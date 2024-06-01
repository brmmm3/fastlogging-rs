use std::{
    fmt,
    io::{Error, ErrorKind},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{bounded, Receiver, SendError, Sender};
use syslog::{Facility, Formatter3164};

use crate::{CRITICAL, DEBUG, ERROR, EXCEPTION, INFO, SUCCESS, WARNING};

#[derive(Debug)]
pub enum SyslogTypeEnum {
    Message((u8, String)), // level, message
    Sync(f64),             // timeout
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyslogWriterConfig {
    pub level: u8, // Log level
    #[serde(skip_serializing, skip_deserializing)]
    formatter: Formatter3164,
}

impl SyslogWriterConfig {
    pub fn new<S: Into<String>>(level: u8, hostname: Option<String>, pname: S, pid: u32) -> Self {
        Self {
            level,
            formatter: Formatter3164 {
                facility: Facility::LOG_USER,
                hostname,
                process: pname.into(),
                pid,
            },
        }
    }
}

impl fmt::Display for SyslogWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn syslog_writer_thread(
    config: Arc<Mutex<SyslogWriterConfig>>,
    rx: Receiver<SyslogTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = match syslog::unix(config.lock().unwrap().formatter.clone()) {
        Ok(w) => w,
        Err(err) => Err(format!("impossible to connect to syslog: {err:?}"))?,
    };
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        match rx.recv()? {
            SyslogTypeEnum::Message((level, message)) => match level {
                DEBUG => writer.debug(message)?,
                INFO => writer.info(message)?,
                SUCCESS => writer.notice(message)?,
                WARNING => writer.warning(message)?,
                ERROR => writer.err(message)?,
                CRITICAL => writer.crit(message)?,
                EXCEPTION => writer.alert(message)?,
                _ => {}
            },
            SyslogTypeEnum::Sync(_) => {
                sync_tx.send(1)?;
            }
            SyslogTypeEnum::Stop => {
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct SyslogWriter {
    pub(crate) config: Arc<Mutex<SyslogWriterConfig>>,
    tx: Sender<SyslogTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
}

impl SyslogWriter {
    pub fn new(config: SyslogWriterConfig, stop: Arc<Mutex<bool>>) -> Result<Self, Error> {
        let config = Arc::new(Mutex::new(config));
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder::new()
                    .name("SyslogWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = syslog_writer_thread(config, rx, sync_tx, stop) {
                            eprintln!("{err:?}");
                        }
                    })?,
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            self.tx
                .send(SyslogTypeEnum::Stop)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            thr.join().map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), Error> {
        self.tx
            .send(SyslogTypeEnum::Sync(timeout))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        Ok(())
    }

    pub fn set_level(&self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<SyslogTypeEnum>> {
        self.tx.send(SyslogTypeEnum::Message((level, message)))
    }
}
