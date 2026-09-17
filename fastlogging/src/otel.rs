use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{Receiver, RecvTimeoutError, SendError, Sender, bounded};
use parking_lot::RwLock;
use regex::Regex;

use crate::{CRITICAL, DEBUG, ERROR, EXCEPTION, INFO, LoggingError, SUCCESS, TRACE, WARNING};

/// Default OTLP/HTTP endpoint for the OpenTelemetry Collector.
pub const DEFAULT_OTEL_ENDPOINT: &str = "http://localhost:4318";
/// Default service name used as `service.name` resource attribute.
pub const DEFAULT_OTEL_SERVICE_NAME: &str = "fastlogging";
/// Default number of log records collected before they are exported.
pub const DEFAULT_OTEL_BATCH_SIZE: usize = 100;
/// Default interval after which buffered log records are exported.
pub const DEFAULT_OTEL_FLUSH_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub enum OpenTelemetryTypeEnum {
    Message((u8, String, String)), // level, domain, message
    Sync,                          // timeout
    Stop,
}

/// Map a fastlogging log level to the OpenTelemetry severity number.
pub fn level2severity_number(level: u8) -> i32 {
    match level {
        TRACE => 1,
        DEBUG => 5,
        INFO => 9,
        SUCCESS => 10,
        WARNING => 13,
        ERROR => 17,
        CRITICAL => 21,
        EXCEPTION => 24,
        _ => 9,
    }
}

/// Map a fastlogging log level to the OpenTelemetry severity text.
pub fn level2severity_text(level: u8) -> &'static str {
    match level {
        TRACE => "TRACE",
        DEBUG => "DEBUG",
        INFO => "INFO",
        SUCCESS => "INFO2",
        WARNING => "WARN",
        ERROR => "ERROR",
        CRITICAL => "FATAL",
        EXCEPTION => "FATAL4",
        _ => "INFO",
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenTelemetryWriterConfig {
    /// Only export log messages if enabled is true.
    pub enabled: bool,
    /// Log level for filtering log messages.
    pub level: u8,
    /// Optional filter log messages by domain.
    pub domain_filter: Option<String>,
    /// Optional filter log messages by their contents.
    pub message_filter: Option<String>,
    /// OTLP/HTTP endpoint of the OpenTelemetry Collector.
    /// Log records are exported to `{endpoint}/v1/logs`.
    pub endpoint: String,
    /// Service name used as `service.name` resource attribute.
    pub service_name: String,
    /// Number of log records collected before they are exported.
    pub batch_size: usize,
    /// Interval after which buffered log records are exported.
    #[serde(skip_serializing, skip_deserializing)]
    pub timeout: Option<Duration>,
    /// Debug level. Only for developers.
    pub debug: u8,
}

impl OpenTelemetryWriterConfig {
    pub fn new<S: Into<String>>(level: u8, endpoint: S, service_name: S) -> Self {
        Self {
            enabled: true,
            level,
            domain_filter: None,
            message_filter: None,
            endpoint: endpoint.into(),
            service_name: service_name.into(),
            batch_size: DEFAULT_OTEL_BATCH_SIZE,
            timeout: Some(DEFAULT_OTEL_FLUSH_INTERVAL),
            debug: 0,
        }
    }
}

impl Default for OpenTelemetryWriterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: crate::NOTSET,
            domain_filter: None,
            message_filter: None,
            endpoint: DEFAULT_OTEL_ENDPOINT.to_string(),
            service_name: DEFAULT_OTEL_SERVICE_NAME.to_string(),
            batch_size: DEFAULT_OTEL_BATCH_SIZE,
            timeout: Some(DEFAULT_OTEL_FLUSH_INTERVAL),
            debug: 0,
        }
    }
}

impl fmt::Display for OpenTelemetryWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Build the OTLP/HTTP JSON payload for a batch of log records.
fn build_otlp_json(
    service_name: &str,
    records: &[(u8, String, String)],
) -> Result<String, LoggingError> {
    let mut log_records = Vec::with_capacity(records.len());
    for (level, domain, message) in records {
        let mut attributes = serde_json::Map::new();
        attributes.insert(
            "domain".to_string(),
            serde_json::Value::String(domain.clone()),
        );
        log_records.push(serde_json::json!({
            "timeUnixNano": chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default().to_string(),
            "severityNumber": level2severity_number(*level),
            "severityText": level2severity_text(*level),
            "body": { "stringValue": message },
            "attributes": attributes
                .into_iter()
                .map(|(k, v)| serde_json::json!({ "key": k, "value": { "stringValue": v } }))
                .collect::<Vec<_>>(),
        }));
    }
    let payload = serde_json::json!({
        "resourceLogs": [
            {
                "resource": {
                    "attributes": [
                        {
                            "key": "service.name",
                            "value": { "stringValue": service_name }
                        }
                    ]
                },
                "scopeLogs": [
                    {
                        "scope": {},
                        "logRecords": log_records
                    }
                ]
            }
        ]
    });
    serde_json::to_string(&payload)
        .map_err(|e| LoggingError::InvalidValue(format!("Failed to build OTLP payload: {e:?}")))
}

/// Export a batch of log records to the OpenTelemetry Collector via OTLP/HTTP.
fn export_records(
    config: &RwLock<OpenTelemetryWriterConfig>,
    records: &[(u8, String, String)],
) -> Result<(), LoggingError> {
    if records.is_empty() {
        return Ok(());
    }
    let (endpoint, service_name) = {
        let config_read = config.read();
        (
            config_read.endpoint.clone(),
            config_read.service_name.clone(),
        )
    };
    let url = format!("{}/v1/logs", endpoint.trim_end_matches('/'));
    let body = build_otlp_json(&service_name, records)?;
    let response = ureq::post(&url)
        .header("Content-Type", "application/json")
        .send(body.as_bytes())
        .map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to export log records to {url}: {e:?}"))
        })?;
    let status = response.status();
    if status.as_u16() >= 400 {
        return Err(LoggingError::InvalidValue(format!(
            "OpenTelemetry Collector returned status {status} for {url}",
        )));
    }
    Ok(())
}

fn otel_writer_thread(
    config: Arc<RwLock<OpenTelemetryWriterConfig>>,
    rx: Receiver<OpenTelemetryTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    let mut buffer: Vec<(u8, String, String)> = Vec::new();
    let default_delay = DEFAULT_OTEL_FLUSH_INTERVAL;
    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let timeout = config.read().timeout.unwrap_or(default_delay);
        match rx.recv_timeout(timeout) {
            Ok(OpenTelemetryTypeEnum::Message((level, domain, message))) => {
                let config_read = config.read();
                if !config_read.enabled {
                    continue;
                }
                if let Some(ref domain_filter) = config_read.domain_filter {
                    let re = Regex::new(domain_filter).unwrap();
                    if !re.is_match(&domain) {
                        continue;
                    }
                }
                if let Some(ref message_filter) = config_read.message_filter {
                    let re = Regex::new(message_filter).unwrap();
                    if !re.is_match(&message) {
                        continue;
                    }
                }
                let batch_size = config_read.batch_size;
                drop(config_read);
                buffer.push((level, domain, message));
                if buffer.len() >= batch_size {
                    export_records(&config, &buffer)?;
                    buffer.clear();
                }
            }
            Ok(OpenTelemetryTypeEnum::Sync) => {
                export_records(&config, &buffer)?;
                buffer.clear();
                sync_tx.send(1)?;
            }
            Ok(OpenTelemetryTypeEnum::Stop) => {
                export_records(&config, &buffer)?;
                buffer.clear();
                break;
            }
            Err(RecvTimeoutError::Timeout) => {
                if !buffer.is_empty() {
                    export_records(&config, &buffer)?;
                    buffer.clear();
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct OpenTelemetryWriter {
    pub(crate) config: Arc<RwLock<OpenTelemetryWriterConfig>>,
    tx: Sender<OpenTelemetryTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
    pub(crate) debug: u8,
}

impl OpenTelemetryWriter {
    pub fn new(
        config: OpenTelemetryWriterConfig,
        stop: Arc<AtomicBool>,
    ) -> Result<Self, LoggingError> {
        let config = Arc::new(RwLock::new(config));
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder::new()
                    .name("OpenTelemetryWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = otel_writer_thread(config, rx, sync_tx, stop) {
                            eprintln!("otel_writer_thread failed: {err:?}");
                        }
                    })?,
            ),
            debug: 0,
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(OpenTelemetryTypeEnum::Stop).map_err(|e| {
                LoggingError::SendCmdError(
                    "OpenTelemetryWriter".to_string(),
                    "STOP".to_string(),
                    e.to_string(),
                )
            })?;
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "OpenTelemetryWriter".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), LoggingError> {
        self.tx.send(OpenTelemetryTypeEnum::Sync).map_err(|e| {
            LoggingError::SendCmdError(
                "OpenTelemetryWriter".to_string(),
                "SYNC".to_string(),
                e.to_string(),
            )
        })?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| {
                LoggingError::RecvAswError(
                    "OpenTelemetryWriter".to_string(),
                    "SYNC".to_string(),
                    e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn enable(&self) {
        self.config.write().enabled = true;
    }

    pub fn disable(&self) {
        self.config.write().enabled = false;
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.config.write().enabled = enabled;
    }

    pub fn set_level(&self, level: u8) {
        self.config.write().level = level;
    }

    pub fn set_domain_filter(&self, domain_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = domain_filter {
            Regex::new(message)?;
        }
        self.config.write().domain_filter = domain_filter;
        Ok(())
    }

    pub fn set_message_filter(&self, message_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = message_filter {
            Regex::new(message)?;
        }
        self.config.write().message_filter = message_filter;
        Ok(())
    }

    pub fn set_endpoint<S: Into<String>>(&self, endpoint: S) {
        self.config.write().endpoint = endpoint.into();
    }

    pub fn set_service_name<S: Into<String>>(&self, service_name: S) {
        self.config.write().service_name = service_name.into();
    }

    pub fn set_batch_size(&self, batch_size: usize) {
        self.config.write().batch_size = batch_size;
    }

    #[inline]
    pub fn send(
        &self,
        level: u8,
        domain: String,
        message: String,
    ) -> Result<(), SendError<OpenTelemetryTypeEnum>> {
        self.tx
            .send(OpenTelemetryTypeEnum::Message((level, domain, message)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{DEBUG, Logging, NOTSET};

    use super::OpenTelemetryWriterConfig;

    #[test]
    fn otel() {
        let mut logging = Logging::new(
            NOTSET,
            "root",
            Some(vec![
                OpenTelemetryWriterConfig::new(DEBUG, "http://localhost:4318", "test-service")
                    .into(),
            ]),
            None,
            None,
        )
        .unwrap();
        logging.trace("Trace Message").unwrap();
        logging.debug("Debug Message").unwrap();
        logging.info("Info Message").unwrap();
        logging.success("Success Message").unwrap();
        logging.warning("Warning Message").unwrap();
        logging.error("Error Message").unwrap();
        logging.fatal("Fatal Message").unwrap();
        logging.shutdown(false).unwrap();
    }
}
