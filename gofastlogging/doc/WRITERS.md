# Writers

Writers are the sinks for log messages. A writer configuration must be created first using the factory functions in the `writer` package (`gofastlogging/fastlogging/writer`), then passed to `logging.New` or `Logging.AddWriterConfig`. All factory functions return `*fl.WriterConfigEnum` (a pointer) and may return `nil` on failure — no error is returned, so check for `nil` if you want to fail loudly. When assembling a `[]fl.WriterConfigEnum` slice for `logging.New`, dereference each pointer with `*`.

## Console Writer

```go
func ConsoleWriterConfigNew(level uint8, colors bool) *fl.WriterConfigEnum
```

- `level` — log level filter (e.g. `fl.DEBUG`)
- `colors` — enables colored output

### Example

```go
console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
if console == nil {
    panic("failed to create console writer config")
}
writers := []fl.WriterConfigEnum{*console}
logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
logger.Info("hello from console writer")
logger.Shutdown(false)
```

## File Writer

```go
func FileWriterConfigNew(
    level uint8,
    path string,
    size uint32,
    backlog uint32,
    timeout int32,
    time int64,
    compression fl.CompressionMethod,
) *fl.WriterConfigEnum
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `level` | `uint8` | Log level filter |
| `path` | `string` | Path to the log file |
| `size` | `uint32` | Max file size in bytes before rotation (0 = no size limit) |
| `backlog` | `uint32` | Max number of backup files |
| `timeout` | `int32` | Timeout in seconds after last log message before rotation (-1 = none) |
| `time` | `int64` | Time of day for rotation as Unix timestamp seconds (-1 = no time rotation) |
| `compression` | `fl.CompressionMethod` | `fl.Store`, `fl.Deflate`, `fl.Zstd`, or `fl.Lzma` |

### Example

```go
file := writer.FileWriterConfigNew(
    fl.DEBUG,
    "/tmp/myapp.log",
    1024*1024,
    5,
    -1,
    -1,
    fl.Store,
)
if file == nil {
    panic("failed to create file writer config")
}
writers := []fl.WriterConfigEnum{*file}
logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
logger.Info("hello from file writer")
logger.Shutdown(false)
```

## Syslog Writer

```go
func SyslogWriterConfigNew(level uint8, hostname, pname string, pid uint32) *fl.WriterConfigEnum
```

- `level` — log level filter
- `hostname` — hostname added to log messages
- `pname` — process name added to log messages
- `pid` — process ID (0 to skip)

### Example

```go
syslog := writer.SyslogWriterConfigNew(fl.DEBUG, "myhost", "myapp", 0)
if syslog == nil {
    panic("failed to create syslog writer config")
}
writers := []fl.WriterConfigEnum{*syslog}
logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
logger.Info("hello from syslog writer")
logger.Shutdown(false)
```

## Callback Writer

> The callback writer supports one active Go callback because the underlying C callback API has no user-data argument. Keep the returned `CallbackHandle` alive while the writer is active.

```go
func CallbackWriterConfigNew(
    level uint8,
    callback func(level uint8, domain, message string),
) (fl.WriterConfigEnum, CallbackHandle, error)
```

### Intended parameters

- `level` — log level filter
- `callback` — a Go function with the signature `func(level uint8, domain, message string)`, invoked once per log message that passes the level filter:
  - `level` — the level of the message
  - `domain` — the domain string of the message
  - `message` — the formatted message text

### `CallbackHandle`

The function would return a `CallbackHandle` alongside the writer config. The handle exposes:

```go
func (h CallbackHandle) UnregisterCallback()
```

Calling `UnregisterCallback()` removes the callback from the writer, after which messages will no longer be delivered to it.

### Current behavior

Calling `CallbackWriterConfigNew` currently returns a non-nil error:

```text
```

Do not rely on this writer until it is implemented.

## Adding and removing writers at runtime

In addition to passing writers to `logging.New`, you can add or remove writers after a `Logging` instance has been created:

```go
func (l *Logging) AddWriterConfig(config fl.WriterConfigEnum) error
func (l *Logging) RemoveWriter(wid uint32) error
```

- `AddWriterConfig` adds a writer configuration to an existing logger. Pass a `fl.WriterConfigEnum` value (not the pointer returned by the factory functions — dereference with `*` first).
- `RemoveWriter` removes a previously added writer by its writer ID. The writer ID is assigned by the library when the writer is added.

See [LOGGING.md](LOGGING.md) for the full `Logging` API.
