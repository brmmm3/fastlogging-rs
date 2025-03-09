# Examples

## Logging to Console with Logging instance

```rust
use fastlogging::{ConsoleWriterConfig, Logging, LoggingError, DEBUG};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new(
        DEBUG,
        "root",
        vec![ConsoleWriterConfig::new(DEBUG, true).into()],
        None,
        None,
    )?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.shutdown(false)?;
    Ok(())
}
```

## Logging to Console with root logger

```rust
use fastlogging::{LoggingError, ROOT_LOGGER};

fn main() -> Result<(), LoggingError> {
    let logger = ROOT_LOGGER.lock().unwrap();
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.sync_all(1.0)?;
    Ok(())
}
```

## Logging with fork

```rust
use std::{process, thread, time::Duration};

use fastlogging::{root, LoggingError};
use fork::{fork, Fork};

fn run_parent(child: i32) -> Result<(), LoggingError> {
    println!("# Run parent. Child has pid {child}.");
    root::debug("Debug Message from parent")?;
    root::info("Info Message from parent")?;
    println!("# Parent finished");
    Ok(())
}

fn run_child() -> Result<(), LoggingError> {
    println!("# Run child with pid {}", process::id());
    thread::sleep(Duration::from_millis(20));
    root::debug("Debug Message from child")?;
    root::info("Info Message from child")?;
    println!("# Child finished");
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    println!("# Start main with pid {}", process::id());
    match fork() {
        Ok(Fork::Parent(child)) => run_parent(child)?,
        Ok(Fork::Child) => run_child()?,
        Err(_) => println!("Fork failed"),
    }
    println!("# Continue main with pid {}", process::id());
    root::debug("Debug Message from main")?;
    println!("# main finished");
    thread::sleep(Duration::from_millis(100));
    Ok(())
}
```

## Logging to File using root logger

```rust
use std::path::PathBuf;

use fastlogging::{CompressionMethodEnum, FileWriterConfig, Logging, LoggingError, DEBUG};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new(
        DEBUG,
        "root",
        vec![FileWriterConfig::new(
            DEBUG,
            PathBuf::from("/tmp/cfastlogging.log"),
            1024,
            3,
            None,
            None,
            Some(CompressionMethodEnum::Store),
        )?
        .into()],
        None,
        None,
    )?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.shutdown(false)?;
    Ok(())
}
```

## Logging via network sockets

```rust
use std::{thread, time::Duration};

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, FileWriterConfig, Logging,
    LoggingError, ServerConfig, DEBUG,
};
use tempfile::TempDir;

fn main() -> Result<(), LoggingError> {
    let temp_dir = TempDir::with_prefix("fastlogging").unwrap();
    let log_file = temp_dir.path().join("file.log");
    // Server
    let mut logging_server = Logging::new(
        DEBUG,
        "LOGSRV",
        vec![
            ConsoleWriterConfig::new(DEBUG, true).into(),
            FileWriterConfig::new(DEBUG, log_file.clone(), 0, 0, None, None, None)
                .unwrap()
                .into(),
        ],
        None,
        None,
    )?;
    // Set root writer
    logging_server.set_root_writer_config(
        &ServerConfig::new(DEBUG, "127.0.0.1", EncryptionMethod::NONE).into(),
    )?;
    logging_server.sync_all(5.0).unwrap();
    // Client
    let mut logging_client = Logging::new(
        DEBUG,
        "LOGCLIENT",
        vec![
            ClientWriterConfig::new(
                DEBUG,
                logging_server.get_root_server_address_port().unwrap(),
                logging_server.get_server_auth_key(),
            )
            .into(),
        ],
        None,
        None,
    )?;
    logging_client.trace("Trace Message".to_string()).unwrap();
    logging_client.debug("Debug Message".to_string()).unwrap();
    logging_client.info("Info Message".to_string()).unwrap();

    logging_server.trace("Trace Message".to_string()).unwrap();
    logging_server.debug("Debug Message".to_string()).unwrap();
    logging_server.info("Info Message".to_string()).unwrap();

    logging_client.sync_all(1.0)?;
    logging_server.sync_all(1.0)?;
    // Give client some time to send the log messages
    thread::sleep(Duration::from_millis(50));
    println!("Shutdown Loggers");
    logging_client.shutdown(false).unwrap();
    logging_server.shutdown(false).unwrap();
    let _log_text = std::fs::read_to_string(&log_file).unwrap();
    temp_dir.close().unwrap();
    println!("-------- Finished --------");
    Ok(())
}
```

## Logging using callback

```rust
use fastlogging::{CallbackWriterConfig, Logging, LoggingError, DEBUG};

fn writer_callback(level: u8, domain: String, message: String) -> Result<(), LoggingError> {
    println!("CB: {level} {domain}: {message}");
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    let mut logging = Logging::new(
        DEBUG,
        "root",
        vec![CallbackWriterConfig::new(DEBUG, Some(Box::new(writer_callback))).into()],
        None,
        None,
    )
    .unwrap();
    logging.trace("Trace Message".to_string()).unwrap();
    logging.debug("Debug Message".to_string()).unwrap();
    logging.info("Info Message".to_string()).unwrap();
    logging.shutdown(false).unwrap();
    Ok(())
}
```

## Logging to syslog

```rust
use fastlogging::{Logging, LoggingError, DEBUG};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::new(DEBUG, "root", Vec::new(), None, None)?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    logger.shutdown(false)?;
    Ok(())
}
```

## Logging and threads

```rust
use std::thread;

use fastlogging::{
    ConsoleWriterConfig, ExtConfig, Logger, Logging, LoggingError, MessageStructEnum, DEBUG,
};

fn main() -> Result<(), LoggingError> {
    let mut logger = Logging::default();
    logger.set_ext_config(&mut ExtConfig::new(
        MessageStructEnum::String,
        true,
        true,
        true,
        true,
        true,
    ));
    logger.add_writer_config(&ConsoleWriterConfig::new(DEBUG, true).into())?;
    let mut logger2 = Logger::new_ext(DEBUG, "LoggerThread", true, true);
    logger.add_logger(&mut logger2);
    let thr = thread::Builder::new()
        .name("SomeThread".to_string())
        .spawn(move || {
            logger2
                .trace("Trace Message")
                .expect("Failed to log message");
            logger2
                .debug("Debug Message")
                .expect("Failed to log message");
            logger2.info("Info Message").expect("Failed to log message");
        })?;
    logger.trace("Trace Message")?;
    logger.debug("Debug Message")?;
    logger.info("Info Message")?;
    thr.join().unwrap();
    logger.shutdown(false)?;
    Ok(())
}
```
