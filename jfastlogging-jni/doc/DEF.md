# Definitions

This page documents all constants, enums, and configuration classes defined in `org.logging.FastLogging`.

## Log Level Constants

All log level constants are `public static final int` members of `FastLogging`.

| Constant | Value | Description |
| --- | --- | --- |
| `NOLOG` | 100 | No logging |
| `EXCEPTION` | 60 | Log exception messages |
| `CRITICAL` | 50 | Log critical messages. Color: bright red |
| `FATAL` | 50 | Same as `CRITICAL` |
| `ERROR` | 40 | Log error messages. Color: red |
| `WARNING` | 30 | Log warning messages. Color: bright yellow |
| `WARN` | 30 | Same as `WARNING` |
| `SUCCESS` | 25 | Success messages |
| `INFO` | 20 | Log info messages. Color: bright green |
| `DEBUG` | 10 | Log debug messages. Color: white |
| `TRACE` | 5 | Trace messages |
| `NOTSET` | 0 | All messages are logged |

## `Level2Sym`

```java
public static String Level2Sym(int level)
```

Static method that returns the level name as a `String` (e.g. `"DEBUG"`, `"INFO"`). Returns `"?"` for unknown levels.

## Enum `LevelSyms`

Controls the symbol style used when rendering log levels.

```java
public enum LevelSyms {
    Sym(0), Short(1), Str(2);
    // each has getValue() returning the int
}
```

| Value | Name | Description |
| --- | --- | --- |
| `Sym` | 0 | 1-char symbol (`!`, `F`, `E`, `W`, ...) |
| `Short` | 1 | 3-char text (`EXC`, `FTL`, `ERR`, `WRN`, ...) |
| `Str` | 2 | Long text (`EXCEPTION`, `FATAL`, `ERROR`, ...). Default |

## Enum `MessageStructEnum`

Controls the message serialization format.

```java
public enum MessageStructEnum {
    String(0), Json(1), Xml(2);
}
```

## Enum `WriterTypeEnum`

Identifies the writer type. `Root` represents the root logging instance.

```java
public enum WriterTypeEnum {
    Root(0), Console(1), File(2), Client(3), Server(4), Syslog(5);
}
```

## Enum `CompressionMethodEnum`

Compression methods applied to rotated backup files.

```java
public enum CompressionMethodEnum {
    Store(0), Deflate(1), Zstd(2), Lzma(3);
}
```

## Enum `EncryptionMethod`

Encryption methods for network client/server traffic.

```java
public enum EncryptionMethod {
    NONE(0), AuthKey(1), AES(2);
}
```

## Class `ExtConfig`

```java
public static class ExtConfig {
    long instance_ptr;
    public ExtConfig(MessageStructEnum structured, boolean hostname, boolean pname, boolean pid, boolean tname, boolean tid)
}
```

Creates an extended formatting configuration. The boolean flags control whether each field is included in log messages:

- `structured` — output format (`String`, `Json`, or `Xml`)
- `hostname` — include the host name
- `pname` — include the process name
- `pid` — include the process ID
- `tname` — include the thread name
- `tid` — include the thread ID

## Class `ConsoleWriterConfig`

```java
public static class ConsoleWriterConfig {
    long instance_ptr;
    public ConsoleWriterConfig(int level)
    public ConsoleWriterConfig(int level, boolean colors)
}
```

- `level` — log level filter for this writer
- `colors` — whether to emit ANSI color codes (defaults to `false` in the single-arg constructor)

## Class `FileWriterConfig`

```java
public static class FileWriterConfig {
    long instance_ptr;
    public FileWriterConfig(int level, String path)
    public FileWriterConfig(int level, String path, int size, int backlog, long timeout, long time, CompressionMethodEnum compression)
}
```

Full constructor parameters:

- `level` — log level filter
- `path` — path to log file
- `size` — max file size in bytes (0 = no limit)
- `backlog` — max number of backup files
- `timeout` — timeout in seconds (0 = no timeout)
- `time` — time of day for rotation as seconds (0 = no time-based rotation)
- `compression` — compression method for backup files

## Class `ClientWriterConfig`

```java
public static class ClientWriterConfig {
    long instance_ptr;
    public ClientWriterConfig(int level, String address, int port)
    public ClientWriterConfig(int level, String address, int port, EncryptionMethod method, String key)
}
```

- `level` — log level filter
- `address` — remote server address
- `port` — remote server port
- `method` — encryption method (`NONE`, `AuthKey`, or `AES`)
- `key` — encryption key (ignored when `method` is `NONE`)

## Class `ServerConfig`

```java
public static class ServerConfig {
    long instance_ptr;
    public ServerConfig(int level, String address, int port)
    public ServerConfig(int level, String address, int port, EncryptionMethod method, String key)
}
```

- `level` — log level filter
- `address` — bind address
- `port` — listen port
- `method` — encryption method (`NONE`, `AuthKey`, or `AES`)
- `key` — encryption key (ignored when `method` is `NONE`)

## Not Yet Wrapped

`SyslogWriterConfig` and `CallbackWriterConfig` do **not** exist as Java wrapper classes yet, although the underlying JNI/Rust layer supports both writer types. Syslog can be partially used via the `Logging(int level, String domain, int syslog)` constructor. The callback writer has no Java wrapper.
