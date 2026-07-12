# Writer Configurations

Every writer is configured through a `*WriterConfig` struct that is converted into
a `WriterConfigEnum` variant with `.into()` and passed to `Logging::new` or
`Logging::add_writer_config`.

## `WriterConfigEnum` Variants

```rust
pub enum WriterConfigEnum {
    Root(RootConfig),
    Console(ConsoleWriterConfig),
    File(FileWriterConfig),
    Client(ClientWriterConfig),   // see NETWORK.md
    Server(ServerConfig),         // see NETWORK.md
    Callback(CallbackWriterConfig),
    Syslog(SyslogWriterConfig),   // unix only
}
```

`From<T> for WriterConfigEnum` is implemented for every concrete config type, so `config.into()` always works.

---

## Console Writer

Writes coloured or plain-text messages to stdout, stderr, or both.

### `ConsoleWriterConfig`

```rust
pub struct ConsoleWriterConfig {
    pub enabled:        bool,
    pub level:          u8,
    pub domain_filter:  Option<String>, // regex
    pub message_filter: Option<String>, // regex
    pub colors:         bool,
    pub target:         ConsoleTargetEnum,
    pub debug:          u8,
}
```

### `ConsoleTargetEnum`

| Variant | Output destination |
|---|---|
| `StdOut` *(default)* | Standard output |
| `StdErr` | Standard error |
| `Both` | Both stdout and stderr |

### `ConsoleWriterConfig::new`

```rust
pub fn new(level: u8, colors: bool) -> Self
```

```rust
use fastlogging::{ConsoleWriterConfig, DEBUG, Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG, "app",
        Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
        None, None,
    )?;
    log.info("coloured console output")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## File Writer

Writes messages to a log file with optional size-based or time-based rotation and
compression of rotated archives.

### `FileWriterConfig`

```rust
pub struct FileWriterConfig {
    pub enabled:        bool,
    pub level:          u8,
    pub domain_filter:  Option<String>,
    pub message_filter: Option<String>,
    pub path:           PathBuf,
    // private: size, backlog, timeout, time, compression
}
```

### `FileWriterConfig::new`

```rust
pub fn new(
    level:       u8,
    path:        PathBuf,
    size:        usize,          // max file size in bytes; 0 = unlimited
    backlog:     usize,          // number of rotated backups to keep (required when size > 0)
    timeout:     Option<Duration>, // rotate after this age
    time:        Option<SystemTime>, // rotate at this absolute time
    compression: Option<CompressionMethodEnum>,
) -> Result<Self, LoggingError>
```

Returns an error if `size > 0 || timeout.is_some() || time.is_some()` but `backlog == 0`,
or if `backlog > 1000`.

### `CompressionMethodEnum`

| Variant | Zip algorithm |
|---|---|
| `Store` *(default)* | No compression |
| `Deflate` | Deflate / zlib |
| `Zstd` | Zstandard |
| `Lzma` | LZMA |

```rust
use fastlogging::{CompressionMethodEnum, DEBUG, FileWriterConfig, Logging, LoggingError};
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG, "app",
        Some(vec![
            FileWriterConfig::new(
                DEBUG,
                PathBuf::from("/tmp/app.log"),
                5 * 1024 * 1024,             // rotate at 5 MB
                4,                           // keep 4 backups
                Some(Duration::from_secs(86400)), // also rotate after 24 h
                None,
                Some(CompressionMethodEnum::Zstd),
            )?.into(),
        ]),
        None, None,
    )?;
    log.info("writing to file")?;
    log.rotate(None)?;      // force rotation of all file writers
    log.shutdown(false)?;
    Ok(())
}
```

---

## Callback Writer

Invokes a user-supplied Rust closure for every log message.  Useful for testing
or for integrating with third-party logging pipelines.

### `CallbackWriterConfig`

```rust
pub fn new(level: u8, callback: Option<CallbackFn>) -> Self
```

`CallbackFn` is `Box<dyn Fn(u8, String, String) -> Result<(), LoggingError> + Send + Sync>`.

```rust
use fastlogging::{CallbackWriterConfig, DEBUG, Logging, LoggingError};

fn my_handler(level: u8, domain: String, message: String) -> Result<(), LoggingError> {
    eprintln!("[{level}] {domain}: {message}");
    Ok(())
}

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG, "app",
        Some(vec![
            CallbackWriterConfig::new(DEBUG, Some(Box::new(my_handler))).into(),
        ]),
        None, None,
    )?;
    log.error("something went wrong")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## Syslog Writer *(Unix only)*

Sends messages to the system syslog via the RFC 3164 `Formatter3164` at
`Facility::LOG_USER`.

### `SyslogWriterConfig::new`

```rust
pub fn new<S: Into<String>>(
    level:    u8,
    hostname: Option<String>,
    pname:    S,       // process name
    pid:      u32,
) -> Self
```

```rust
#[cfg(target_family = "unix")]
fn main() -> Result<(), fastlogging::LoggingError> {
    use fastlogging::{DEBUG, Logging, SyslogWriterConfig};
    let mut log = Logging::new(
        DEBUG, "app",
        Some(vec![
            SyslogWriterConfig::new(DEBUG, None, "myapp", std::process::id()).into(),
        ]),
        None, None,
    )?;
    log.warning("sent to syslog")?;
    log.shutdown(false)?;
    Ok(())
}
```

---

## `WriterTypeEnum`

Used by `enable_type`, `disable_type`, and `sync` to address all writers of a
given category at once.

```rust
pub enum WriterTypeEnum {
    Root,
    Console,
    File(String),    // carries the file path
    Files,           // all file writers
    Client(String),  // carries address:port
    Clients,         // all client writers
    Server(String),  // carries address:port
    Servers,         // all server writers
    Callback,
    Syslog,
}
```

```rust
// Disable all file writers
log.disable_type(WriterTypeEnum::Files)?;
// Re-enable
log.enable_type(WriterTypeEnum::Files)?;
// Sync only file and console writers
log.sync(vec![WriterTypeEnum::Files, WriterTypeEnum::Console], 5.0)?;
```
