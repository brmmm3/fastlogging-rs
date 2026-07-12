# Configuration

gofastlogging can be configured either programmatically (recommended for Go) or via configuration files.

## Configuration Files

Supported types: **JSON**, **YAML**, **XML**.

- **File name:** `fastlogging.<EXT>` (e.g. `fastlogging.json`)
- **Location:** current working directory, or a path specified by the `FASTLOGGING_CONFIG_FILE` environment variable.

```sh
export FASTLOGGING_CONFIG_FILE=/path/to/fastlogging.json
```

## Example JSON Config

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
  "console": {
    "level": 40,
    "colors": true
  },
  "file": {
    "level": 10,
    "path": "/tmp/write_config_file.log",
    "size": 1048576,
    "backlog": 4,
    "timeout": { "secs": 3600, "nanos": 0 },
    "time": { "secs_since_epoch": 1717081855, "nanos_since_epoch": 211877680 },
    "compression": "Deflate"
  },
  "server": null,
  "connect": {
    "level": 50,
    "address": "127.0.0.1:12346",
    "port": 12346,
    "key": "NONE"
  },
  "syslog": null,
  "level2sym": "Sym"
}
```

## ExtConfig (Programmatic)

Use `fl.NewExtConfig` to create extended formatting configuration:

```go
func NewExtConfig(structured MessageStruct, hostname, pname, pid, tname, tid bool) ExtConfig
```

Parameters:

- `structured` — `fl.String`, `fl.Json`, or `fl.Xml`
- `hostname` — include hostname in log messages
- `pname` — include process name
- `pid` — include process ID
- `tname` — include thread name
- `tid` — include thread ID

Example:

```go
extConfig := fl.NewExtConfig(fl.Xml, true, false, true, false, true)
log := logging.New(fl.DEBUG, nil, writers, &extConfig, nil)
// or set later:
log.SetExtConfig(extConfig)
```

## Loading Config via `Default()`

```go
log, err := logging.Default() // reads config file if FASTLOGGING_CONFIG_FILE is set
```

## Saving Config

```go
log.SaveConfig("/path/to/fastlogging.json") // extension determines format
configString := log.GetConfigString()       // get config as string
```

## Programmatic vs. File Config

```go
// From config file
log, err := logging.Default()

// Or build in Go
console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
writers := []fl.WriterConfigEnum{*console}
log2 := logging.New(fl.DEBUG, nil, writers, nil, nil)
```
