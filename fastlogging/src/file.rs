use std::{
    fs::{ rename, File, OpenOptions },
    io::{ BufWriter, Error, ErrorKind, Read, Seek, SeekFrom, Write },
    path::{ Path, PathBuf },
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
};

use flume::{ bounded, Receiver, Sender };
use zip::{ write::SimpleFileOptions, CompressionMethod, ZipWriter };

use crate::BACKLOG_MAX;

pub enum FileMessageType {
    Message(String),
    Rotate,
    Sync,
}

fn do_rotate(path: &Path, backlog: usize) -> Result<(), Error> {
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
    backlog_path.set_extension(".log.1");
    // Compress current log file
    let file = File::open(path)?;
    let mut buffer = Vec::new();
    std::io::copy(&mut file.take(u64::MAX), &mut buffer)?;
    let zip_file = File::create(&backlog_path)?;
    let mut zip = ZipWriter::new(zip_file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::DEFLATE)
        .unix_permissions(0o755);
    zip.start_file(filename, options)?;
    zip.write_all(&buffer)?;
    zip.finish()?;
    Ok(())
}

fn logger_thread_worker(
    path: PathBuf,
    max_size: usize,
    backlog: usize,
    rx: Receiver<Option<FileMessageType>>,
    stop: Arc<Mutex<bool>>,
    sync_tx: Sender<u8>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = BufWriter::new(OpenOptions::new().create(true).append(true).open(&path)?);
    let mut size = file.seek(SeekFrom::End(0))? as usize;
    let newline = vec![b'\n'];
    while let Some(message) = rx.recv()? {
        if *stop.lock().unwrap() {
            break;
        }
        let rotate = match message {
            FileMessageType::Message(m) => {
                file.write_all(m.as_bytes())?;
                let _ = file.write(&newline)?;
                size += m.len();
                false
            }
            FileMessageType::Rotate => { true }
            FileMessageType::Sync => {
                sync_tx.send(1)?;
                false
            }
        };
        if backlog > 0 && (rotate || (max_size > 0 && size > max_size)) {
            file.flush()?;
            drop(file);
            // Rotate
            if let Err(err) = do_rotate(&path, backlog) {
                eprintln!("Failed to rotate log files: {err:?}");
            }
            file = BufWriter::new(OpenOptions::new().write(true).truncate(true).open(&path)?);
            size = 0;
        }
    }
    file.flush()?;
    Ok(())
}

fn logger_thread(
    path: PathBuf,
    max_size: usize,
    backlog: usize,
    rx: Receiver<Option<FileMessageType>>,
    stop: Arc<Mutex<bool>>,
    sync_tx: Sender<u8>
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = logger_thread_worker(path, max_size, backlog, rx, stop, sync_tx) {
        eprintln!("Logging file worker crashed with error: {err:?}");
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileLogging {
    level: u8,
    tx: Sender<Option<FileMessageType>>,
    thr: Option<JoinHandle<()>>,
}

impl FileLogging {
    pub fn new(
        level: u8,
        path: PathBuf,
        max_size: usize,
        backlog: usize,
        stop: Arc<Mutex<bool>>,
        sync_tx: Sender<u8>
    ) -> Result<Self, Error> {
        if max_size > 0 {
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
        let (tx, rx) = bounded(10000);
        Ok(Self {
            level,
            tx,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("FileLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = logger_thread(path, max_size, backlog, rx, stop, sync_tx) {
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

    pub fn rotate(&self) -> Result<(), Error> {
        self.tx
            .send(Some(FileMessageType::Rotate))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn send(&self, level: u8, message: String) -> Result<(), Error> {
        if level >= self.level {
            (
                if level == 255 {
                    self.tx.send(Some(FileMessageType::Sync))
                } else {
                    self.tx.send(Some(FileMessageType::Message(message)))
                }
            ).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
        } else {
            Ok(())
        }
    }
}
