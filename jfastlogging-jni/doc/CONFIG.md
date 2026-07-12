# Configuration

jfastlogging can be configured either programmatically or via configuration files.

## Configuration Files

Supported types: JSON, YAML, XML. File name: `fastlogging.<EXT>`. Location: current working directory, or path specified by `FASTLOGGING_CONFIG_FILE` environment variable.

To load from a config file, use the `Logging(String path)` constructor:

```java
Logging logging = new Logging("/path/to/fastlogging.json");
```

Example JSON config

```json
{
  "level": 0,
  "domain": "root",
  "hostname": "bender",
  "pname": "write_config_file",
  "pid": 935659,
  "tname": false,
  "tid": false,
  "structured": "String",
  "console": { "level": 40, "colors": true },
  "file": {
    "level": 10, "path": "/tmp/write_config_file.log",
    "size": 1048576, "backlog": 4,
    "timeout": { "secs": 3600, "nanos": 0 },
    "time": { "secs_since_epoch": 1717081855, "nanos_since_epoch": 211877680 },
    "compression": "Deflate"
  },
  "server": null,
  "connect": { "level": 50, "address": "127.0.0.1:12346", "port": 12346, "key": "NONE" },
  "syslog": null,
  "level2sym": "Sym"
}
```

## ExtConfig (programmatic)

```java
public static class ExtConfig {
    public ExtConfig(MessageStructEnum structured, boolean hostname, boolean pname, boolean pid, boolean tname, boolean tid)
}
```

Parameters:

- `structured` — `MessageStructEnum.String`, `.Json`, or `.Xml`
- `hostname` — include hostname
- `pname` — include process name
- `pid` — include process ID
- `tname` — include thread name
- `tid` — include thread ID

Example:

```java
ExtConfig extConfig = new ExtConfig(MessageStructEnum.Xml, true, false, true, false, true);
Logging logging = new Logging(FastLogging.DEBUG, "root", extConfig);
```

## Saving config

```java
logging.getSaveConfig("/path/to/fastlogging.json"); // note: method is getSaveConfig, not saveConfig
String configString = logging.getConfigString();
```

## Programmatic vs file config

Programmatic:

```java
ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
FileWriterConfig file = new FileWriterConfig(FastLogging.DEBUG, "/tmp/app.log");
Logging logging = new Logging(FastLogging.DEBUG, "root", console, file);
```

File-based:

```java
Logging logging = new Logging("/path/to/fastlogging.json");
```

Both approaches produce an equivalent `Logging` instance. Use programmatic configuration when structure is dynamic or computed at runtime; use file-based configuration when settings are fixed or need to be changed without recompiling.
