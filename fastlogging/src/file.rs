use std::{
    fs::{ rename, File, OpenOptions },
    io::{ BufWriter, Error, ErrorKind, Read, Seek, SeekFrom, Write },
    path::{ Path, PathBuf },
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
    time::{ Duration, Instant },
};

use flume::{ bounded, Receiver, RecvTimeoutError, Sender };
use zip::{ write::SimpleFileOptions, CompressionMethod, ZipWriter };

use crate::MessageTypeEnum;

const BACKLOG_MAX: usize = 1000;
const QUEUE_CAPACITY: usize = 10000;
const DEFAULT_DELAY: u64 = 3600;

#[derive(Debug, Clone)]
pub struct FileWriterConfig {
    level: u8, // Log level
    path: PathBuf, // Log file path
    size: usize, // Maximum size of log file. 0 means no size limit.
    backlog: usize, // Maximum number of backup files.
    timeout: Option<Duration>, // Maximum log file age in seconds.
    time: Option<Instant>, // Time when to backup log file.
    compression: CompressionMethod, // Compression method for backup files.
}

impl FileWriterConfig {
    pub fn new(
        level: u8,
        path: PathBuf,
        size: usize,
        backlog: usize,
        timeout: Option<Duration>,
        time: Option<Instant>,
        compression: Option<CompressionMethod>
    ) -> Result<Self, Error> {
        if size > 0 || timeout.is_some() || time.is_some() {
            if backlog == 0 {
                return Err(
                    Error::new(
                        ErrorKind::InvalidInput,
                        "For rotating file logger backlog depth has to be set!"
                    )
                );
            } else if backlog > BACKLOG_MAX {
                return Err(
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "For rotating file logger backlog depth {backlog} too big! Maximum value is {BACKLOG_MAX}."
                        )
                    )
                );
            }
        }
        Ok(Self {
            level,
            path,
            size,
            backlog,
            timeout,
            time,
            compression: compression.unwrap_or(CompressionMethod::STORE),
        })
    }
}

fn rotate_do(path: &Path, backlog: usize, compression: CompressionMethod) -> Result<(), Error> {
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
    if compression == CompressionMethod::STORE {
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
        .compression_method(compression)
        .unix_permissions(0o755);
    zip.start_file(filename, options)?;
    zip.write_all(&buffer)?;
    zip.finish()?;
    Ok(())
}

fn file_writer_thread_worker(
    config: Arc<Mutex<FileWriterConfig>>,
    rx: Receiver<Option<MessageTypeEnum>>,
    stop: Arc<Mutex<bool>>,
    sync_tx: Sender<u8>
) -> Result<(), Box<dyn std::error::Error>> {
    let path = config.lock().unwrap().path.clone();
    let mut create_time = Instant::now();
    let mut file = BufWriter::new(OpenOptions::new().create(true).append(true).open(&path)?);
    let mut size = file.seek(SeekFrom::End(0))? as usize;
    let newline = vec![b'\n'];
    let default_delay = Duration::from_secs(DEFAULT_DELAY);
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        let (max_size, backlog, timeout, time, compression) = {
            let c = config.lock().unwrap();
            (c.size, c.backlog, c.timeout, c.time, c.compression)
        };
        let mut to = create_time
            .checked_add(timeout.unwrap_or(default_delay))
            .unwrap_or_else(|| Instant::now().checked_add(default_delay).unwrap());
        if let Some(time) = time {
            if time < to {
                to = time;
            }
        }
        let message = match rx.recv_deadline(to) {
            Ok(m) =>
                match m {
                    Some(m) => m,
                    None => {
                        break;
                    }
                }
            Err(err) => {
                if err == RecvTimeoutError::Disconnected {
                    break;
                }
                if timeout.is_none() && time.is_none() {
                    continue;
                }
                MessageTypeEnum::Rotate
            }
        };
        let rotate = match message {
            MessageTypeEnum::Message((_level, message)) => {
                file.write_all(message.as_bytes())?;
                let _ = file.write(&newline)?;
                size += message.len();
                false
            }
            MessageTypeEnum::Rotate => { true }
            MessageTypeEnum::Sync(_) => {
                sync_tx.send(1)?;
                false
            }
            MessageTypeEnum::Stop => {
                break;
            }
        };
        if backlog > 0 && (rotate || (max_size > 0 && size > max_size)) {
            file.flush()?;
            drop(file);
            // Rotate
            if let Err(err) = rotate_do(&path, backlog, compression) {
                eprintln!("Failed to rotate log files: {err:?}");
            }
            create_time = Instant::now();
            file = BufWriter::new(OpenOptions::new().write(true).truncate(true).open(&path)?);
            size = 0;
        }
    }
    file.flush()?;
    Ok(())
}

fn file_writer_thread(
    config: Arc<Mutex<FileWriterConfig>>,
    rx: Receiver<Option<MessageTypeEnum>>,
    stop: Arc<Mutex<bool>>,
    sync_tx: Sender<u8>
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = file_writer_thread_worker(config, rx, stop, sync_tx) {
        eprintln!("Logging file worker crashed with error: {err:?}");
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileWriter {
    config: Arc<Mutex<FileWriterConfig>>,
    tx: Sender<Option<MessageTypeEnum>>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
}

impl FileWriter {
    pub fn new(config: FileWriterConfig, stop: Arc<Mutex<bool>>) -> Result<Self, Error> {
        let config = Arc::new(Mutex::new(config));
        let (tx, rx) = bounded(QUEUE_CAPACITY);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("FileWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = file_writer_thread(config, rx, stop, sync_tx) {
                            eprintln!("{err:?}");
                        }
                    })?
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(None).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            thr.join().map_err(|e|
                Error::new(ErrorKind::Other, e.downcast_ref::<&str>().unwrap().to_string())
            )
        } else {
            Ok(())
        }
    }

    pub fn set_level(&self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    pub fn set_rotate(
        &self,
        size: usize,
        backlog: usize,
        timeout: Option<Duration>,
        time: Option<Instant>,
        compression: Option<CompressionMethod>
    ) -> Result<(), Error> {
        let mut config = self.config.lock().unwrap();
        config.size = size;
        config.backlog = backlog;
        config.timeout = timeout;
        config.time = time;
        config.compression = compression.unwrap_or(CompressionMethod::STORE);
        self.sync(5.0)
    }

    pub fn rotate(&self) -> Result<(), Error> {
        self.tx
            .send(Some(MessageTypeEnum::Rotate))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn sync(&self, timeout: f64) -> Result<(), Error> {
        self.tx
            .send(Some(MessageTypeEnum::Sync(timeout)))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        Ok(())
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), Error> {
        self.tx
            .send(Some(MessageTypeEnum::Message((level, message))))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }
}
