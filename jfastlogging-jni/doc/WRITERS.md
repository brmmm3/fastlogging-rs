# Writers

Writers are the sinks for log messages. Writer configuration objects are created first, then passed to `Logging` constructors. All writer config classes are nested static classes of `FastLogging` and hold a native pointer (`instance_ptr`).

## Console Writer

```java
public static class ConsoleWriterConfig {
    public ConsoleWriterConfig(int level)
    public ConsoleWriterConfig(int level, boolean colors)
}
```

- `level` — log level filter (e.g. `FastLogging.DEBUG`)
- `colors` — enables colored output (default false in the single-arg constructor)

Example:

```java
ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
Logging logging = new Logging(FastLogging.DEBUG, "root", console);
```

## File Writer

```java
public static class FileWriterConfig {
    public FileWriterConfig(int level, String path)
    public FileWriterConfig(int level, String path, int size, int backlog, long timeout, long time, CompressionMethodEnum compression)
}
```

Parameters:

- `level` — log level filter
- `path` — path to the log file
- `size` — max file size in bytes before rotation (0 = no size limit)
- `backlog` — max number of backup files
- `timeout` — timeout in seconds after last log message before rotation (0 = no timeout)
- `time` — time of day for rotation in seconds (0 = no time-based rotation)
- `compression` — `CompressionMethodEnum.Store`, `.Deflate`, `.Zstd`, `.Lzma`

Example:

```java
FileWriterConfig file = new FileWriterConfig(FastLogging.DEBUG, "/tmp/app.log",
    1048576, 5, 0, 0, CompressionMethodEnum.Store);
Logging logging = new Logging(FastLogging.DEBUG, "root", file);
```

## Syslog Writer

The JNI layer supports syslog (`syslogWriterConfigNew`), but there is no Java `SyslogWriterConfig` wrapper class yet. To use syslog, pass a syslog level to the `Logging(int level, String domain, int syslog)` constructor:

```java
Logging logging = new Logging(FastLogging.DEBUG, "root", FastLogging.DEBUG);
```

The `syslog` parameter is an `int` log level (-1 = no syslog).

## Callback Writer

The JNI layer supports callbacks (`callbackWriterConfigNew`), but there is no Java `CallbackWriterConfig` wrapper class in the current release. This is a known gap.

## Adding/removing writers at runtime

```java
void addWriter(long writerPtr)        // pass writerConfig.instance_ptr
void removeWriter(WriterTypeEnum writer)
void removeWriter(WriterTypeEnum writer, String key)
```
