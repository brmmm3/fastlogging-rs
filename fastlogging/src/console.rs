use std::{
    io::{ Error, ErrorKind, Write },
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
    time::Duration,
};

use flume::{ bounded, Receiver, SendError, Sender };
use termcolor::{ BufferWriter, Color, ColorChoice, ColorSpec, WriteColor };

use crate::{ MessageTypeEnum, DEBUG, INFO, WARNING, ERROR, CRITICAL, EXCEPTION };

#[derive(Debug, Clone)]
pub struct ConsoleWriterConfig {
    pub level: u8, // Log level
    colors: bool,
}

impl ConsoleWriterConfig {
    pub fn new(level: u8, colors: bool) -> Self {
        Self { level, colors }
    }
}

fn console_writer_thread(
    config: Arc<Mutex<ConsoleWriterConfig>>,
    rx: Receiver<MessageTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<Mutex<bool>>
) -> Result<(), Box<dyn std::error::Error>> {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        match rx.recv()? {
            MessageTypeEnum::Message((level, message)) => {
                if let Ok(ref config) = config.lock() {
                    if config.colors {
                        buffer.set_color(
                            ColorSpec::new().set_fg(
                                Some(match level {
                                    DEBUG => Color::Green,
                                    INFO => Color::White,
                                    WARNING => Color::Yellow,
                                    ERROR => Color::Magenta,
                                    CRITICAL => Color::Red,
                                    EXCEPTION => Color::Red,
                                    _ => Color::White,
                                })
                            )
                        )?;
                        writeln!(&mut buffer, "{message}")?;
                        bufwtr.print(&buffer)?;
                    } else {
                        println!("{message}");
                    }
                } else {
                    break;
                }
            }
            MessageTypeEnum::Sync(_) => {
                sync_tx.send(1)?;
            }
            MessageTypeEnum::Stop => {
                break;
            }
            MessageTypeEnum::Rotate => {
                eprint!("Console received invalid message type Rotate");
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct ConsoleWriter {
    config: Arc<Mutex<ConsoleWriterConfig>>,
    tx: Sender<MessageTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
}

impl ConsoleWriter {
    pub fn new(config: ConsoleWriterConfig, stop: Arc<Mutex<bool>>) -> Result<Self, Error> {
        let config = Arc::new(Mutex::new(config));
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("ConsoleWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = console_writer_thread(config.clone(), rx, sync_tx, stop) {
                            eprintln!("{err:?}");
                        }
                    })?
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            self.tx
                .send(MessageTypeEnum::Stop)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            thr.join().map_err(|e|
                Error::new(ErrorKind::Other, e.downcast_ref::<&str>().unwrap().to_string())
            )
        } else {
            Ok(())
        }
    }

    pub fn set_colors(&self, colors: bool) {
        self.config.lock().unwrap().colors = colors;
    }

    pub fn sync(&self, timeout: f64) -> Result<(), Error> {
        self.tx
            .send(MessageTypeEnum::Sync(timeout))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        Ok(())
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<MessageTypeEnum>> {
        self.tx.send(MessageTypeEnum::Message((level, message)))
    }
}
