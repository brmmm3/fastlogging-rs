
# `Logging` — Primary API

`Logging` is the central type in `fastlogging`.
One instance owns a background logging thread and an arbitrary number of writers.
All log calls are non-blocking:
the message is pushed onto a bounded channel and the thread dispatches it.

## Creating a `Logging` Instance

### `Logging::new`

```rust
pub fn new<S: Into<String>>(
    level:       u8,
    domain:      S,
    configs:     Option<Vec<WriterConfigEnum>>,
    ext_config:  Option<ExtConfig>,
    config_path: Option<PathBuf>,
) -> Result<Self, LoggingError>
```

| Parameter | Description |
|---|---|
| `level` | Global filter level.  Messages below this are dropped before they enter the channel. |
| `domain` | String prepended to every log message, e.g. `"myapp"` or `"server"`. |
| `configs` | Zero or more writer configurations.  Pass `None` for no writers. |
| `ext_config` | Optional extended metadata (hostname, pid, thread id, structured format). |
| `config_path` | Optional path to a JSON/YAML/XML config file that is merged after the inline config. |

### `Logging::init

```rust
pub fn init() -> Result<Self, LoggingError>
```

Convenience constructor equivalent to `Logging::new(NOTSET, "root", Some(vec![ConsoleWriterConfig::new(NOTSET, false).into()]), None, None)`.

### `Logging::default`

```rust
impl Default for Logging
```

Same as `Logging::init()` but panics on failure (useful in tests / quick scripts).

### `logging_new_default` (free function)

```rust
pub fn logging_new_default() -> Result<Logging, LoggingError>
```

Creates a `Logging` instance with a console writer at `NOTSET` level.

Lifecycle

### `shutdown`

```rust
pub fn shutdown(&mut self, now: bool) -> Result<(), LoggingError>
```

Stops the background thread. If `now` is `true` the stop flag is set immediately
and queued messages may be lost; if `false` a graceful stop message is sent and the
thread drains the channel before exiting.

`Logging` implements `Drop`, which calls `shutdown(false)` automatically.

### `apply_config`

```rust
pub fn apply_config(&mut self, path: &Path) -> Result<(), LoggingError>
```

Reload configuration from a file at runtime.

### `save_config`

```rust
pub fn save_config(&mut self, path: Option<&Path>) -> Result<(), LoggingError>
```

Persist the current configuration.
Extension determines format: `.json`, `.yaml`, `.xml`.
Pass `None` to reuse the path from the last `apply_config` call.

## Configuration Methods

### Level and Domain

```rust
pub fn set_level(&mut self, wid: usize, level: u8) -> Result<(), LoggingError>
pub fn set_domain(&mut self, domain: &str)
pub fn set_level2sym(&mut self, level2sym: &LevelSyms)
pub fn set_ext_config(&mut self, ext_config: &ExtConfig)
pub fn set_debug(&mut self, debug: u8)
```

`set_level` targets a specific writer by its id (`wid`).
Writer ids are returned by the `add_writer_config` / `add_writer` methods. `wid = 0` is always the root writer (Client or Server type).

## Writer Management

```rust
// Add at construction time via configs vec, or dynamically:
pub fn add_writer_config(&mut self, config: &WriterConfigEnum) -> Result<usize, LoggingError>
pub fn add_writer(&mut self, writer: WriterEnum) -> usize
pub fn add_writer_configs(&mut self, configs: Vec<WriterConfigEnum>) -> Result<Vec<usize>, LoggingError>
pub fn add_writers(&mut self, writers: Vec<WriterEnum>) -> Vec<usize>

pub fn remove_writer(&mut self, wid: usize) -> Option<WriterEnum>
pub fn remove_writers(&mut self, wids: Option<Vec<usize>>) -> Vec<WriterEnum>

pub fn enable(&self, wid: usize)  -> Result<(), LoggingError>
pub fn disable(&self, wid: usize) -> Result<(), LoggingError>
pub fn enable_type(&self,  typ: WriterTypeEnum) -> Result<(), LoggingError>
pub fn disable_type(&self, typ: WriterTypeEnum) -> Result<(), LoggingError>
```

### Root Writer

The root writer (wid = 0) must be a `Client` or `Server` config:

```rust
pub fn set_root_writer_config(&mut self, config: &WriterConfigEnum) -> Result<(), LoggingError>
pub fn set_root_writer(&mut self, writer: WriterEnum) -> Result<(), LoggingError>
```

## Logger Management

`Logger` handles can be registered to share the same background thread:

```rust
pub fn add_logger(&mut self, logger: &mut Logger)
pub fn remove_logger(&mut self, logger: &mut Logger)
```

## Sync and Rotate

```rust
pub fn sync(&self, types: Vec<WriterTypeEnum>, timeout: f64) -> Result<(), LoggingError>
pub fn sync_all(&self, timeout: f64) -> Result<(), LoggingError>
pub fn rotate(&self, path: Option<PathBuf>) -> Result<(), LoggingError>
```

`sync_all` flushes Console, Files, Clients, Servers, Callback, and Syslog writers.
`rotate` triggers log-file rotation for all `FileWriter`s (or just those whose path
matches, if `path` is `Some`).

## Encryption

```rust
pub fn set_encryption(&mut self, wid: usize, method: EncryptionMethod) -> Result<(), LoggingError>
```

Reconfigure the encryption of a Client or Server writer at runtime.

## Query Methods

```rust
pub fn get_writer_config(&self, wid: usize) -> Option<WriterConfigEnum>
pub fn get_writer_configs(&self) -> HashMap<usize, WriterConfigEnum>
pub fn get_server_config(&self, wid: usize) -> Result<ServerConfig, LoggingError>
pub fn get_server_configs(&self) -> HashMap<usize, ServerConfig>
pub fn get_root_server_address_port(&self) -> Option<String>
pub fn get_server_addresses_ports(&self) -> HashMap<usize, String>
pub fn get_server_addresses(&self) -> HashMap<usize, String>
pub fn get_server_ports(&self) -> HashMap<usize, u16>
pub fn get_server_auth_key(&self) -> EncryptionMethod
pub fn get_config_string(&self) -> String
```

## Logging Methods

All logging methods accept any `S: Into<String>` and return `Result<(), LoggingError>`.
They are **no-ops** when `self.level > message_level` — no heap allocation occurs.

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

## Error Handling

All fallible methods return `Result<_, LoggingError>`.
The variants are:

| Variant | Meaning |
|---|---|
| `Io` | Underlying I/O failure |
| `Utf8Error` | Invalid UTF-8 string |
| `SyslogError` | Syslog connection problem |
| `SendError` / `SendCmdError` | Channel send failure |
| `RecvError` / `RecvAswError` | Channel receive failure |
| `InvalidValue` | Bad argument (e.g. unknown writer id) |
| `InvalidFile` | Config file not found or wrong format |
| `InvalidEncryption` | Unsupported encryption configuration |
| `JoinError` | Background thread failed to join |
| `ConfigError` | Config file parse error |
| `ArchiveError` | Zip compression failure |
