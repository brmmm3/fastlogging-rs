# Definitions

## Log level names and their values

`NOLOG` (100) &ensp;&ensp;&ensp;&ensp;&ensp; No logging.  
`EXCEPTION` (60) &nbsp; Log exception messages. In addition to the message text the exception info (output of traceback.format_exc) is logged (SLOW!).  
`CRITICAL` (50) &nbsp;&ensp; Log fatal/critical messages. Default color is bright red.  
`FATAL` (50) &ensp;&ensp;&ensp;&ensp;&ensp; Same as CRITICAL.  
`ERROR` (40) &ensp;&ensp;&ensp;&ensp;&ensp; Log also error messages. Default color is red.  
`WARNING` (30) &ensp;&ensp;&ensp; Log also warning messages. Default color is bright yellow.  
`SUCCESS` (25) &ensp;&ensp;&ensp; Success messages.  
`INFO` (20) &ensp;&ensp;&ensp;&ensp;&ensp;&ensp;&nbsp; Log also info messages. Default color is bright green.  
`DEBUG` (10) &ensp;&ensp;&ensp;&ensp;&ensp;  Log also debug messages. Default color is white.  
`TRACE` (5) &ensp;&ensp;&ensp;&ensp;&ensp;  Trace messages.  
`NOTSET` (0) &ensp;&ensp;&ensp;&ensp; All messages are logged.

## Enum `LevelSyms`

The enum has following values:

```rust
pub enum LevelSyms {
    /// Use 1 character symbol (!, F, E, W, ...)
    Sym,
    /// Use 3 character text (EXC, FTL, ERR, WRN, ...)
    Short,
    /// Use long text (EXCEPTION, FATAL, ERROR, WARNING, ...). This is the default.
    Str,
}
```

## Enum `MessageStructEnum`

The enum has following values:

```rust
pub enum MessageStructEnum {
    /// Log messages without structure information (default).
    String,
    /// Log messages as Json structure.
    Json,
    /// Log messages as Xml structure.
    Xml,
}
```

## Class `ExtConfig`

This class is for configuring extended formatting setting. It has following members:

```rust
pub struct ExtConfig {
    /// Set log message structuring.
    pub structured: MessageStructEnum,
    /// Include hostname in log messages.
    pub hostname: bool,
    /// Include process name in log messages.
    pub pname: bool,
    /// Include process id in log messages.
    pub pid: bool,
    /// Include thread name in log messages.
    pub tname: bool,
    /// Include thread id in log messages.
    pub tid: bool,
}
```

## Class `RootConfig`

```rust
pub struct RootConfig {
    /// Log level for filtering log messages.
    pub level: u8,
    /// Log domain to add to log messages.
    pub domain: String,
    /// Optional hostname to add to log messages.
    pub hostname: Option<String>,
    /// Process name. Logged is not empty.
    pub pname: String,
    /// Process id. Logged if greater than 0.
    pub pid: u32,
    /// Log thread name if `true``.
    pub tname: bool,
    /// Log thread id if `true`.
    pub tid: bool,
    /// Log messages with structure information.
    pub structured: MessageStructEnum,
    /// Select log level names.
    pub level2sym: LevelSyms,
}
```

## Enum `ConsoleTargetEnum`

```rust
pub enum ConsoleTargetEnum {
    /// Write log messages to stdout
    StdOut,
    /// Write log messages to stderr
    StdErr,
    /// Write log messages to stdout and stderr
    Both,
}
```

## Class `ConsoleWriterConfig`

```rust
pub struct ConsoleWriterConfig {
    /// Only write log messages if enabled is true
    pub enabled: bool,
    /// Log level for filtering log messages
    pub level: u8,
    /// Optional filter log messages by domain
    pub domain_filter: Option<String>,
    /// Optional filter log messages by their contents
    pub message_filter: Option<String>,
    /// Colored output if true
    pub colors: bool,
    /// Select log message destination (stdout, stderr)
    pub target: ConsoleTargetEnum,
    /// Debug level. Only for developers.
    pub debug: u8,
}
```

## Class `FileWriterConfig`

```rust
pub struct FileWriterConfig {
    /// Only write log messages if enabled is true
    pub enabled: bool,
    /// Log level for filtering log messages
    pub level: u8,
    /// Optional filter log messages by domain
    pub domain_filter: Option<String>,
    /// Optional filter log messages by their contents
    pub message_filter: Option<String>,
    /// Path to log file
    pub path: PathBuf,
    /// Maximum size of log file. 0 means no size limit.
    size: usize,
    /// Maximum number of backup files.
    backlog: usize,
    /// Maximum log file age in seconds.
    timeout: Option<Duration>,
    /// Time when to backup log file.
    time: Option<SystemTime>,
    /// Compression method for backup files.
    compression: CompressionMethodEnum,
}
```

## Class `ServerConfig`

```rust
pub struct ServerConfig {
    /// Log level for filtering log messages
    pub level: u8,
    /// IP address to listen to
    pub address: String,
    /// IP port
    pub port: u16,
    /// Optional key for authentication and message encryption
    pub key: EncryptionMethod,
    /// Temporary file for key exchange between server and client process
    pub port_file: Option<PathBuf>,
}
```

## Class `ClientWriterConfig`

```rust
pub struct ClientWriterConfig {
    /// Only send log messages if enabled is true
    pub enabled: bool,
    /// Log level for filtering log messages
    pub level: u8,
    /// Optional filter log messages by domain
    pub domain_filter: Option<String>,
    /// Optional filter log messages by their contents
    pub message_filter: Option<String>,
    /// IP address to connect and send log messages
    pub address: String,
    /// IP port
    pub port: u16,
    /// Optional key for authentication and message encryption
    pub key: EncryptionMethod,
    /// Debug level. Only for developers.
    pub debug: u8,
}
```
