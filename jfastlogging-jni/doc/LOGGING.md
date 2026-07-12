# Logging

The `Logging` class is the primary logger instance. It is a nested static class of `FastLogging`: `FastLogging.Logging`.

## Constructors

The `Logging` class has many overloaded constructors. Each combines different writer configs. All take `int level` and `String domain` as the first two parameters. Writer config parameters can be `null` to skip that writer type.

| Constructor | Description |
|---|---|
| `Logging()` | Default: level NOTSET, domain "root", no writers |
| `Logging(int level)` | Level + domain "root", no writers |
| `Logging(int level, String domain)` | Level + domain, no writers |
| `Logging(int level, String domain, ExtConfig extConfig)` | With ExtConfig |
| `Logging(int level, String domain, ConsoleWriterConfig console)` | With console writer |
| `Logging(int level, String domain, FileWriterConfig file)` | With file writer |
| `Logging(int level, String domain, ConsoleWriterConfig console, FileWriterConfig file)` | Console + file |
| `Logging(int level, String domain, FileWriterConfig file, ServerConfig server)` | File + server |
| `Logging(int level, String domain, FileWriterConfig file, ClientWriterConfig client)` | File + client |
| `Logging(int level, String domain, ConsoleWriterConfig console, ClientWriterConfig client)` | Console + client |
| `Logging(int level, String domain, ConsoleWriterConfig console, FileWriterConfig file, ClientWriterConfig client)` | Console + file + client |
| `Logging(int level, String domain, ConsoleWriterConfig console, FileWriterConfig file, ServerConfig server)` | Console + file + server |
| `Logging(int level, String domain, ClientWriterConfig client)` | With client writer only |
| `Logging(int level, String domain, int syslog)` | With syslog (int level, -1 = none) |
| `Logging(int level, String domain, ExtConfig extConfig, ConsoleWriterConfig console, FileWriterConfig file, ClientWriterConfig client, int syslog)` | Full constructor with all options |
| `Logging(String path)` | Load config from file path |

Example:

```java
ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
Logging logging = new Logging(FastLogging.DEBUG, "root", console);
logging.info("Hello");
logging.shutdown();
```

## Methods

### Lifecycle

- `void shutdown()` — shutdown without waiting
- `void shutdown(boolean now)` — shutdown; if `now` is true, waits for all logs to flush

### Level and domain

- `void setLevel(WriterTypeEnum writer, int level)` — set log level for a writer type
- `void setLevel(WriterTypeEnum writer, String key, int level)` — set log level for a specific writer by key
- `void setDomain(String domain)` — set log domain
- `void setLevel2Sym(LevelSyms level2sym)` — set level symbol format
- `void setExtConfig(ExtConfig extConfig)` — set extended formatting config

### Sub-loggers

- `void addLogger(long loggerPtr)` — add a Logger by its native pointer
- `void removeLogger(long loggerPtr)` — remove a Logger by its native pointer

### Writers

- `void addWriter(long writerPtr)` — add a writer by its native pointer
- `void removeWriter(WriterTypeEnum writer)` — remove all writers of a type
- `void removeWriter(WriterTypeEnum writer, String key)` — remove a specific writer by type and key

### Sync

- `void sync(boolean console, boolean file, boolean client, boolean syslog, double timeout)` — sync specific writer types. The boolean flags select which writer types to sync. `timeout` in seconds.
- `void syncAll(double timeout)` — sync all writers. `timeout` in seconds.

### File rotation

- `void rotate(String path)` — rotate log file at path

### Encryption

- `void setEncryption(EncryptionMethod method, String key)` — set encryption for root server
- `void setEncryption(String address, EncryptionMethod method, String key)` — set encryption for specific server by address

### Config

- `String getConfig(WriterTypeEnum writer, String key)` — get config for a writer
- `ServerConfig getServerConfig()` — get server config
- `String getServerAddress()` — get server address
- `String getServerAuthKey()` — get server auth key
- `String getConfigString()` — get complete config as string
- `void getSaveConfig(String path)` — save config to file (note: method name is `getSaveConfig`, not `saveConfig`)

### Logging methods

All do client-side level checking before calling JNI. Each takes a `String message`.

| Method | Level check |
|---|---|
| `void trace(String message)` | `instance_level <= TRACE` |
| `void debug(String message)` | `instance_level <= DEBUG` |
| `void info(String message)` | `instance_level <= INFO` |
| `void success(String message)` | `instance_level <= SUCCESS` |
| `void warning(String message)` | `instance_level <= WARN` |
| `void error(String message)` | `instance_level <= ERROR` |
| `void critical(String message)` | `instance_level <= CRITICAL` |
| `void fatal(String message)` | `instance_level <= FATAL` |
| `void exception(String message)` | `instance_level <= EXCEPTION` |

Note: The client-side level check means if the `Logging` was created with a level higher than the message level, the JNI call is never made.
