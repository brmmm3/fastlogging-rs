use std::{
    fmt,
    fs::{rename, File, OpenOptions},
    io::{BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};

use flume::{bounded, Receiver, RecvTimeoutError, Sender};
use regex::Regex;
use zip::{write::SimpleFileOptions, ZipWriter};

use crate::{level2str, LoggingError};

const BACKLOG_MAX: usize = 1000;
const QUEUE_CAPACITY: usize = 10000;
const DEFAULT_DELAY: u64 = 3600;

#[derive(Debug, Clone)]
pub enum FileTypeEnum {
    Message((u8, String, String)), // level, domain,message
    Sync,                          // timeout
    Rotate,
    Stop,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum CompressionMethodEnum {
    Store,
    Deflate,
    Zstd,
    Lzma,
}

impl From<CompressionMethodEnum> for zip::CompressionMethod {
    fn from(val: CompressionMethodEnum) -> Self {
        match val {
            CompressionMethodEnum::Store => zip::CompressionMethod::Stored,
            CompressionMethodEnum::Deflate => zip::CompressionMethod::Deflated,
            CompressionMethodEnum::Zstd => zip::CompressionMethod::Zstd,
            CompressionMethodEnum::Lzma => zip::CompressionMethod::Lzma,
        }
    }
}

impl From<i32> for CompressionMethodEnum {
    fn from(val: i32) -> Self {
        match val {
            0 => CompressionMethodEnum::Store,
            1 => CompressionMethodEnum::Deflate,
            2 => CompressionMethodEnum::Zstd,
            3 => CompressionMethodEnum::Lzma,
            _ => CompressionMethodEnum::Store,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWriterConfig {
    pub enabled: bool,
    pub level: u8, // Log level
    pub domain_filter: Option<String>,
    pub message_filter: Option<String>,
    pub path: PathBuf,                  // Log file path
    size: usize,                        // Maximum size of log file. 0 means no size limit.
    backlog: usize,                     // Maximum number of backup files.
    timeout: Option<Duration>,          // Maximum log file age in seconds.
    time: Option<SystemTime>,           // Time when to backup log file.
    compression: CompressionMethodEnum, // Compression method for backup files.
}

impl FileWriterConfig {
    pub fn new(
        level: u8,
        path: PathBuf,
        size: usize,
        backlog: usize,
        timeout: Option<Duration>,
        time: Option<SystemTime>,
        compression: Option<CompressionMethodEnum>,
    ) -> Result<Self, LoggingError> {
        if size > 0 || timeout.is_some() || time.is_some() {
            if backlog == 0 {
                return Err(LoggingError::InvalidValue(
                    "For rotating file logger backlog depth has to be set!".to_string(),
                ));
            } else if backlog > BACKLOG_MAX {
                return Err(
                    LoggingError::InvalidValue(
                        format!(
                            "For rotating file logger backlog depth {backlog} too big! Maximum value is {BACKLOG_MAX}."
                        )
                    )
                );
            }
        }
        Ok(Self {
            enabled: true,
            level,
            domain_filter: None,
            message_filter: None,
            path,
            size,
            backlog,
            timeout,
            time,
            compression: compression.unwrap_or(CompressionMethodEnum::Store),
        })
    }
}

impl fmt::Display for FileWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn rotate_do(
    path: &Path,
    backlog: usize,
    compression: CompressionMethodEnum,
) -> Result<(), LoggingError> {
    for num in 1..backlog {
        let mut backlog_path_old = path.to_path_buf();
        backlog_path_old.set_extension(format!(".log.{}", backlog - num - 1));
        if backlog_path_old.exists() {
            let mut backlog_path_new = path.to_path_buf();
            backlog_path_new.set_extension(format!(".log.{}", backlog - num));
            rename(backlog_path_old, backlog_path_new)?;
        }
    }
    let mut backlog_path = path.to_path_buf();
    if compression == CompressionMethodEnum::Store {
        backlog_path.set_extension(".log.1");
    } else {
        backlog_path.set_extension(".log.1.gz");
    }
    // Compress current log file
    let file = File::open(path)?;
    let mut buffer = Vec::new();
    std::io::copy(&mut file.take(u64::MAX), &mut buffer)?;
    let zip_file = File::create(&backlog_path)?;
    let mut zip = ZipWriter::new(zip_file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let options = SimpleFileOptions::default()
        .compression_method(compression.into())
        .unix_permissions(0o755);
    zip.start_file(filename, options)?;
    zip.write_all(&buffer)?;
    zip.finish()?;
    Ok(())
}

fn file_writer_thread_worker(
    config: Arc<Mutex<FileWriterConfig>>,
    rx: Receiver<FileTypeEnum>,
    stop: Arc<AtomicBool>,
    sync_tx: Sender<u8>,
) -> Result<(), LoggingError> {
    let path = config.lock().unwrap().path.clone();
    let mut create_time = SystemTime::now();
    let mut file = BufWriter::new(OpenOptions::new().create(true).append(true).open(&path)?);
    let mut size = file.seek(SeekFrom::End(0))? as usize;
    let newline = vec![b'\n'];
    let default_delay = Duration::from_secs(DEFAULT_DELAY);
    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let (max_size, backlog, timeout, time, compression) = {
            let c = config.lock().unwrap();
            (c.size, c.backlog, c.timeout, c.time, c.compression)
        };
        let mut deadline = create_time
            .checked_add(timeout.unwrap_or(default_delay))
            .unwrap_or_else(|| SystemTime::now().checked_add(default_delay).unwrap());
        if let Some(time) = time {
            if time < deadline {
                deadline = time;
            }
        }
        let to = deadline.duration_since(SystemTime::now()).unwrap();
        let message = match rx.recv_timeout(to) {
            Ok(m) => m,
            Err(err) => {
                if err == RecvTimeoutError::Disconnected {
                    break;
                }
                if timeout.is_none() && time.is_none() {
                    continue;
                }
                FileTypeEnum::Rotate
            }
        };
        let rotate = match message {
            FileTypeEnum::Message((_level, domain, message)) => {
                if let Ok(ref config) = config.lock() {
                    if !config.enabled {
                        continue;
                    }
                    if let Some(ref domain_filter) = config.domain_filter {
                        let re = Regex::new(domain_filter).unwrap();
                        if !re.is_match(&domain) {
                            continue;
                        }
                    }
                    if let Some(ref message_filter) = config.message_filter {
                        let re = Regex::new(message_filter).unwrap();
                        if !re.is_match(&domain) {
                            continue;
                        }
                    }
                }
                file.write_all(message.as_bytes())?;
                let _ = file.write(&newline)?;
                size += message.len();
                false
            }
            FileTypeEnum::Rotate => true,
            FileTypeEnum::Sync => {
                sync_tx.send(1)?;
                false
            }
            FileTypeEnum::Stop => {
                break;
            }
        };
        if backlog > 0 && (rotate || (max_size > 0 && size > max_size)) {
            file.flush()?;
            drop(file);
            // Rotate
            if let Err(err) = rotate_do(&path, backlog, compression) {
                eprintln!("Failed to rotate log files: {path:?}\n  {err:?}");
            }
            create_time = SystemTime::now();
            file = BufWriter::new(OpenOptions::new().write(true).truncate(true).open(&path)?);
            size = 0;
        }
    }
    file.flush()?;
    Ok(())
}

fn file_writer_thread(
    config: Arc<Mutex<FileWriterConfig>>,
    rx: Receiver<FileTypeEnum>,
    stop: Arc<AtomicBool>,
    sync_tx: Sender<u8>,
) -> Result<(), LoggingError> {
    if let Err(err) = file_writer_thread_worker(config.clone(), rx, stop, sync_tx) {
        eprintln!("Logging file worker crashed with error: {err:?}");
        eprintln!("{:#?}", config.lock().unwrap());
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileWriter {
    pub(crate) config: Arc<Mutex<FileWriterConfig>>,
    tx: Sender<FileTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
    pub(crate) debug: u8,
}

impl FileWriter {
    pub fn new(config: FileWriterConfig, stop: Arc<AtomicBool>) -> Result<Self, LoggingError> {
        let config = Arc::new(Mutex::new(config));
        let (tx, rx) = bounded(QUEUE_CAPACITY);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder::new()
                    .name("FileWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = file_writer_thread(config, rx, stop, sync_tx) {
                            eprintln!("file_writer_thread failed: {err:?}");
                        }
                    })?,
            ),
            debug: 0,
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(FileTypeEnum::Stop).map_err(|e| {
                LoggingError::SendCmdError(
                    "FileWriter".to_string(),
                    "STOP".to_string(),
                    e.to_string(),
                )
            })?;
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "FileWriter".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), LoggingError> {
        self.tx.send(FileTypeEnum::Sync).map_err(|e| {
            LoggingError::SendCmdError("FileWriter".to_string(), "SYNC".to_string(), e.to_string())
        })?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| {
                LoggingError::RecvAswError(
                    "FileWriter".to_string(),
                    "SYNC".to_string(),
                    e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn enable(&self) {
        self.config.lock().unwrap().enabled = true;
    }

    pub fn disable(&self) {
        self.config.lock().unwrap().enabled = false;
    }

    pub fn set_level(&self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    pub fn set_domain_filter(&self, domain_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = domain_filter {
            Regex::new(message)?;
        }
        self.config.lock().unwrap().domain_filter = domain_filter;
        Ok(())
    }

    pub fn set_message_filter(&self, message_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = message_filter {
            Regex::new(message)?;
        }
        self.config.lock().unwrap().message_filter = message_filter;
        Ok(())
    }

    pub fn set_rotate(
        &self,
        size: usize,
        backlog: usize,
        timeout: Option<Duration>,
        time: Option<SystemTime>,
        compression: Option<CompressionMethodEnum>,
    ) -> Result<(), LoggingError> {
        let mut config = self.config.lock().unwrap();
        config.size = size;
        config.backlog = backlog;
        config.timeout = timeout;
        config.time = time;
        config.compression = compression.unwrap_or(CompressionMethodEnum::Store);
        self.sync(5.0)
    }

    pub fn rotate(&self) -> Result<(), LoggingError> {
        self.tx.send(FileTypeEnum::Rotate).map_err(|e| {
            LoggingError::SendError(format!(
                "Failed to rotate {:?}: {e:?}",
                self.config.lock().unwrap().path
            ))
        })
    }

    #[inline]
    pub fn send(&self, level: u8, domain: String, message: String) -> Result<(), LoggingError> {
        self.tx
            .send(FileTypeEnum::Message((level, domain, message)))
            .map_err(|e| {
                LoggingError::SendError(format!(
                    "FileWriter::send: Failed to send {} message: {e}",
                    level2str(level)
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use crate::{FileWriterConfig, Logging, DEBUG, NOTSET};

    #[test]
    fn file() {
        let temp_dir = TempDir::with_prefix("fastlogging").unwrap();
        let log_file = temp_dir.path().join("file.log");
        let mut logging = Logging::new(
            NOTSET,
            "root",
            vec![
                FileWriterConfig::new(DEBUG, log_file.clone(), 0, 0, None, None, None)
                    .unwrap()
                    .into(),
            ],
            None,
            None,
        )
        .unwrap();
        logging.trace("Trace Message".to_string()).unwrap();
        logging.info("Info Message".to_string()).unwrap();
        logging.success("Success Message".to_string()).unwrap();
        logging.warning("Warning Message".to_string()).unwrap();
        logging.error("Error Message".to_string()).unwrap();
        logging.fatal("Fatal Message".to_string()).unwrap();
        logging.shutdown(false).unwrap();
        let _log_text = std::fs::read_to_string(&log_file).unwrap();
        temp_dir.close().unwrap();
    }
}
