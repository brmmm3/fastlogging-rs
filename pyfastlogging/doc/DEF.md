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

```python
class LevelSyms(IntEnum):
    # Use 1 character symbol (!, F, E, W, ...)
    Sym = 0
    # Use 3 character text (EXC, FTL, ERR, WRN, ...)
    Short = 1
    # Use long text (EXCEPTION, FATAL, ERROR, WARNING, ...). This is the default.
    Str = 2
```

## Enum `MessageStructEnum`

The enum has following values:

```python
class MessageStructEnum(IntEnum):
    # Log messages without structure information (default).
    String = 0
    # Log messages as Json structure.
    Json = 1
    # Log messages as Xml structure.
    Xml = 2
```

## Class `ExtConfig`

This class is for configuring extended formatting setting. It has following members:

```python
class ExtConfig:
    # Set log message structuring.
    structured: MessageStructEnum
    # Include hostname in log messages.
    hostname: bool,
    # Include process name in log messages.
    pname: bool,
    # Include process id in log messages.
    pid: bool,
    # Include thread name in log messages.
    tname: bool,
    # Include thread id in log messages.
    tid: bool,
```

## Class `RootConfig`

```python
class RootConfig:
    # Log level for filtering log messages.
    level: int
    # Log domain to add to log messages.
    domain: str
    # Optional hostname to add to log messages.
    hostname: str | None
    # Process name. Logged is not empty.
    pname: str
    # Process id. Logged if greater than 0.
    pid: int
    # Log thread name if `true``.
    tname: bool
    # Log thread id if `true`.
    tid: bool
    # Log messages with structure information.
    structured: MessageStructEnum
    # Select log level names.
    level2sym: LevelSyms
```

## Enum `ConsoleTargetEnum`

```python
class ConsoleTargetEnum(IntEnum):
    # Write log messages to stdout
    StdOut = 0
    # Write log messages to stderr
    StdErr = 1
    # Write log messages to stdout and stderr
    Both = 2
```

## Class `ConsoleWriterConfig`

```python
class ConsoleWriterConfig:
    # Only write log messages if enabled is true
    enabled: bool
    # Log level for filtering log messages
    level: int
    # Optional filter log messages by domain
    domain_filter: str | None
    # Optional filter log messages by their contents
    message_filter: str | None
    # Colored output if true
    colors: bool
    # Select log message destination (stdout, stderr)
    target: ConsoleTargetEnum
    # Debug level. Only for developers.
    debug: int
```

## Enum `CompressionMethodEnum`

```python
class CompressionMethodEnum(IntEnum):
    # Do not compress the log files
    Store = 0
    # Compress the log files by the Deflate algorithm
    Deflate = 1
    # Compress the log files by the Zstandard algorithm
    Zstd = 2
    # Compress the log files by the Lzma algorithm
    Lzma = 3
```

## Class `FileWriterConfig`

```python
class FileWriterConfig:
    # Only write log messages if enabled is true
    enabled: bool
    # Log level for filtering log messages
    level: int
    # Optional filter log messages by domain
    domain_filter: str | None
    # Optional filter log messages by their contents
    message_filter: str | None
    # Path to log file
    path: str
    # Maximum size of log file. 0 means no size limit.
    size: int
    # Maximum number of backup files.
    backlog: int
    # Maximum log file age in seconds.
    timeout: Duration | None
    # Time when to backup log file.
    time: SystemTime | None
    # Compression method for backup files.
    compression: CompressionMethodEnum
```

## Class `ServerConfig`

```python
class ServerConfig:
    # Log level for filtering log messages
    level: int
    # IP address to listen to
    address: str
    # IP port
    port: int
    # Optional key for authentication and message encryption
    key: EncryptionMethod
    # Temporary file for key exchange between server and client process
    port_file: str | None
```

## Class `ClientWriterConfig`

```python
class ClientWriterConfig:
    # Only send log messages if enabled is true
    enabled: bool
    # Log level for filtering log messages
    level: int
    # Optional filter log messages by domain
    domain_filter: str | None
    # Optional filter log messages by their contents
    message_filter: str | None
    # IP address to connect and send log messages
    address: str
    # IP port
    port: int
    # Optional key for authentication and message encryption
    key: EncryptionMethod
    # Debug level. Only for developers.
    debug: int
```
