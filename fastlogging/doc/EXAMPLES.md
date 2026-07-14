# Examples

All examples assume `fastlogging` is in `Cargo.toml`:

```toml
[dependencies]
fastlogging = "0.3"
```

1. Default Logger (one line)

```rust
use fastlogging::{logging_new_default, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = logging_new_default()?;
    log.info("Hello, fastlogging!")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## 2. Console Logger

```rust
use fastlogging::{ConsoleWriterConfig, DEBUG, Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG,
        "root",
        Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
        None,
        None,
    )?;
    log.trace("Trace Message")?;
    log.debug("Debug Message")?;
    log.info("Info Message")?;
    log.success("Success Message")?;
    log.warning("Warning Message")?;
    log.error("Error Message")?;
    log.fatal("Fatal Message")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## 3. File Logger with Rotation

```rust
use fastlogging::{CompressionMethodEnum, DEBUG, FileWriterConfig, Logging, LoggingError};
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG,
        "root",
        Some(vec![
            FileWriterConfig::new(
                DEBUG,
                PathBuf::from("/tmp/app.log"),
                5 * 1024 * 1024,           // rotate at 5 MB
                4,                         // keep 4 backups
                Some(Duration::from_secs(86400)), // or after 24 h
                None,
                Some(CompressionMethodEnum::Zstd),
            )?
            .into(),
        ]),
        None,
        None,
    )?;
    log.info("Info Message")?;
    log.rotate(None)?;   // force rotation right now
    log.shutdown(false)?;
    Ok(())
}
```

---

## 4. Add a Writer After Construction

```rust
use fastlogging::{ConsoleWriterConfig, DEBUG, Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(DEBUG, "root", None, None, None)?;
    log.add_writer_config(&ConsoleWriterConfig::new(DEBUG, true).into())?;
    log.info("Info Message")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## 5. Callback Writer

```rust
use fastlogging::{CallbackWriterConfig, DEBUG, Logging, LoggingError};

fn my_sink(level: u8, domain: String, message: String) -> Result<(), LoggingError> {
    println!("[{level}] {domain}: {message}");
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG,
        "root",
        Some(vec![
            CallbackWriterConfig::new(DEBUG, Some(Box::new(my_sink))).into(),
        ]),
        None,
        None,
    )?;
    log.info("routed through callback")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## 6. Extended Config (Structured Logging)

```rust
use fastlogging::{
    ConsoleWriterConfig, DEBUG, ExtConfig, Logging, LoggingError, MessageStructEnum,
};

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG,
        "root",
        Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
        Some(ExtConfig::new(
            MessageStructEnum::Json, // format messages as JSON
            true,   // hostname
            true,   // process name
            true,   // pid
            true,   // thread name
            true,   // thread id
        )),
        None,
    )?;
    log.info("structured message")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## 7. Multi-threaded Logging with `Logger`

```rust
use fastlogging::{
    ConsoleWriterConfig, DEBUG, ExtConfig, Logger, Logging, LoggingError, MessageStructEnum,
};
use std::thread;

fn main() -> Result<(), LoggingError> {
    let mut logging = Logging::default();
    logging.set_ext_config(&ExtConfig::new(
        MessageStructEnum::String, true, true, true, true, true,
    ));
    logging.add_writer_config(&ConsoleWriterConfig::new(DEBUG, true).into())?;

    // Logger for the background thread
    let mut logger = Logger::new_ext(DEBUG, "worker", true, true);
    logging.add_logger(&mut logger); // connect to the Logging thread

    let handle = thread::Builder::new()
        .name("worker-thread".into())
        .spawn(move || {
            logger.trace("Trace Message").unwrap();
            logger.info("Info Message").unwrap();
            logger.error("Error Message").unwrap();
            logger.flush(1.0);
        })?;

    logging.trace("Trace Message")?;
    logging.info("Info Message")?;
    logging.error("Error Message")?;

    handle.join().unwrap();
    logging.shutdown(false)?;
    Ok(())
}
```

---

## 8. Network Logging (Client / Server)

```rust
use std::{thread, time::Duration};
use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, DEBUG, EncryptionMethod,
    FileWriterConfig, Logging, LoggingError, ServerConfig,
};
use std::path::PathBuf;

fn main() -> Result<(), LoggingError> {
    // ── Server ──
    let mut server = Logging::new(
        DEBUG, "SERVER",
        Some(vec![
            ConsoleWriterConfig::new(DEBUG, true).into(),
            FileWriterConfig::new(
                DEBUG, PathBuf::from("/tmp/net.log"), 0, 0, None, None, None,
            )?.into(),
        ]),
        None, None,
    )?;
    server.set_root_writer_config(
        &ServerConfig::new(DEBUG, "127.0.0.1", EncryptionMethod::NONE).into(),
    )?;
    server.sync_all(5.0)?;

    // ── Client ──
    let mut client = Logging::new(
        DEBUG, "CLIENT",
        Some(vec![
            ClientWriterConfig::new(
                DEBUG,
                server.get_root_server_address_port().unwrap(),
                server.get_server_auth_key(),
            ).into(),
        ]),
        None, None,
    )?;

    client.info("from client")?;
    server.info("from server")?;

    client.sync_all(1.0)?;
    server.sync_all(1.0)?;
    thread::sleep(Duration::from_millis(50));
    client.shutdown(false)?;
    server.shutdown(false)?;
    Ok(())
}
```

---

## 9. Root Logger

```rust
use fastlogging::{LoggingError, ROOT_LOGGER};

fn main() -> Result<(), LoggingError> {
    let log = ROOT_LOGGER.read().unwrap();
    log.info("Hello from ROOT_LOGGER")?;
    log.error("Something went wrong")?;
    log.sync_all(1.0)?;
    Ok(())
}
```

---

## 10. Save and Load Config File

```rust
use fastlogging::{
    ConsoleWriterConfig, DEBUG, ERROR, ExtConfig, FileWriterConfig,
    Logging, LoggingError, MessageStructEnum,
};
use std::path::{Path, PathBuf};

fn main() -> Result<(), LoggingError> {
    // Build a rich config
    let mut log = Logging::new(
        DEBUG, "app",
        Some(vec![
            ConsoleWriterConfig::new(ERROR, true).into(),
            FileWriterConfig::new(
                DEBUG, PathBuf::from("/tmp/app.log"),
                10 * 1024 * 1024, 4, None, None, None,
            )?.into(),
        ]),
        Some(ExtConfig::new(
            MessageStructEnum::String, true, true, true, false, false,
        )),
        None,
    )?;

    // Persist
    log.save_config(Some(Path::new("/tmp/app.json")))?;
    log.save_config(Some(Path::new("/tmp/app.yaml")))?;
    log.shutdown(false)?;

    // Reload
    let mut log2 = Logging::new(
        fastlogging::NOTSET, "app", None, None,
        Some(PathBuf::from("/tmp/app.json")),
    )?;
    log2.info("loaded from file")?;
    log2.shutdown(false)?;
    Ok(())
}
```
