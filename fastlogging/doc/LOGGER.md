# `Logger` — Per-Domain / Per-Thread Handle

`Logger` is a lightweight handle that can be registered with a `Logging` instance
(or the global `ROOT_LOGGER`) and used from any thread without needing access to
the full `Logging` object.
Multiple `Logger` instances can share one `Logging` background thread.

## Creating a Logger

### `Logger::new`

```rust
pub fn new<S: Into<String>>(level: u8, domain: S) -> Self
```

Creates a logger with the given filter level and domain label. Thread-name and
thread-id logging are disabled.

### `Logger::new_ext`

```rust
pub fn new_ext<S: Into<String>>(level: u8, domain: S, tname: bool, tid: bool) -> Self
```

Extended constructor that optionally includes the calling thread's name and/or id
in every message.

## Registration

A `Logger` is inert until registered with a `Logging` instance.
Registration connects its internal channel sender:

```rust
// Register
logging.add_logger(&mut logger);

// Later — unregister
logging.remove_logger(&mut logger);
```

After `add_logger` the logger shares the `Logging` instance's background thread.
After `remove_logger` (or if the `Logging` instance is dropped) logging calls will
return `Err(LoggingError::ConfigError(...))`.

## Configuration

```rust
pub fn set_level(&mut self, level: u8)
pub fn level(&self) -> u8
pub fn set_domain(&mut self, domain: &str)
```

Level changes take effect immediately for subsequent log calls.

## Flushing

```rust
pub fn flush(&self, timeout: f64)
```

Waits until the internal channel is empty, or until `timeout` seconds have passed.
Pass `0.0` to wait indefinitely.

## Logging Methods

All logging methods accept any `S: Into<String>`:

```rust
pub fn trace<S: Into<String>>(&self, message: S)     -> Result<(), LoggingError>
pub fn debug<S: Into<String>>(&self, message: S)     -> Result<(), LoggingError>
pub fn info<S: Into<String>>(&self, message: S)      -> Result<(), LoggingError>
pub fn success<S: Into<String>>(&self, message: S)   -> Result<(), LoggingError>
pub fn warning<S: Into<String>>(&self, message: S)   -> Result<(), LoggingError>
pub fn error<S: Into<String>>(&self, message: S)     -> Result<(), LoggingError>
pub fn critical<S: Into<String>>(&self, message: S)  -> Result<(), LoggingError>
pub fn fatal<S: Into<String>>(&self, message: S)     -> Result<(), LoggingError>
pub fn exception<S: Into<String>>(&self, message: S) -> Result<(), LoggingError>
```

Messages are tagged with the `Logger`'s own `domain`, independent of the parent
`Logging` instance's domain.

## Thread-safety

`Logger` implements neither `Send` nor `Sync` in its raw form, but the underlying
channel sender is `Send`.
To use a logger from a spawned thread, move it by value:

```rust
use fastlogging::{ConsoleWriterConfig, DEBUG, ExtConfig, Logger, Logging,
                  LoggingError, MessageStructEnum};
use std::thread;

fn main() -> Result<(), LoggingError> {
    let mut logging = Logging::default();
    logging.set_ext_config(&ExtConfig::new(
        MessageStructEnum::String, true, true, true, true, true,
    ));
    logging.add_writer_config(&ConsoleWriterConfig::new(DEBUG, true).into())?;

    // Create a logger and register it before spawning.
    let mut logger = Logger::new_ext(DEBUG, "worker", true, true);
    logging.add_logger(&mut logger);

    let handle = thread::Builder::new()
        .name("worker-thread".into())
        .spawn(move || {
            // `logger` is moved into the thread.
            logger.info("hello from worker").unwrap();
            logger.flush(1.0);
        })?;

    logging.info("hello from main")?;
    handle.join().unwrap();
    logging.shutdown(false)?;
    Ok(())
}
```

> **Note:** `add_logger` must be called *before* `spawn`, because it sets the
> internal channel sender.  The `Logger` is then moved into the closure.
