use std::{ io::{ Error, ErrorKind, Write }, sync::{ Arc, Mutex }, thread::{ self, JoinHandle } };

use flume::{ bounded, Receiver, SendError, Sender };
use termcolor::{ BufferWriter, Color, ColorChoice, ColorSpec, WriteColor };

use crate::{ MessageType, DEBUG, INFO, WARNING, ERROR, CRITICAL, EXCEPTION };

#[derive(Debug)]
pub struct Config {
    pub colors: bool,
}

fn logger_thread(
    config: Arc<Mutex<Config>>,
    rx: Receiver<MessageType>,
    stop: Arc<Mutex<bool>>
) -> Result<(), Box<dyn std::error::Error>> {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    while let Some((level, message)) = rx.recv()? {
        if *stop.lock().unwrap() {
            break;
        }
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
    Ok(())
}

#[derive(Debug)]
pub struct ConsoleLogging {
    config: Arc<Mutex<Config>>,
    level: u8,
    tx: Sender<MessageType>,
    thr: Option<JoinHandle<()>>,
}

impl ConsoleLogging {
    pub fn new(level: u8, stop: Arc<Mutex<bool>>) -> Result<Self, Error> {
        let config = Arc::new(Mutex::new(Config { colors: false }));
        let (tx, rx) = bounded(1000);
        Ok(Self {
            config: config.clone(),
            level,
            tx,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("ConsoleLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = logger_thread(config.clone(), rx, stop) {
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

    pub fn set_colors(&self, colors: bool) {
        self.config.lock().unwrap().colors = colors;
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<MessageType>> {
        if level >= self.level { self.tx.send(Some((level, message))) } else { Ok(()) }
    }
}
