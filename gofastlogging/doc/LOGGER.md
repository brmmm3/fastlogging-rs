# Logger

The `Logger` struct is defined in the `gofastlogging/fastlogging/logger` package. It represents a per-thread or per-domain logging handle that is attached to a `Logging` instance via `AddLogger`.

```go
type Logger struct { Logger C.Logger }
```

```go
import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logger"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)
```

## Constructors

### New

```go
func New(level uint8, domain *string) *Logger
```

Creates a new `Logger`. Returns `*Logger` (nil on failure; no error returned).

| Parameter | Type | Description |
|-----------|------|-------------|
| `level` | `uint8` | Log level filter. Use constants like `fl.DEBUG`, `fl.INFO`. |
| `domain` | `*string` | Log domain. Pass `nil` for none. |

### NewExt

```go
func NewExt(level uint8, domain *string, tname int8, tid int8) *Logger
```

Creates a new `Logger` with extended thread-logging options. Returns `*Logger` (nil on failure).

| Parameter | Type | Description |
|-----------|------|-------------|
| `level` | `uint8` | Log level filter. |
| `domain` | `*string` | Log domain. Pass `nil` for none. |
| `tname` | `int8` | `0` = don't log thread name, `1` = log thread name. |
| `tid` | `int8` | `0` = don't log thread id, `1` = log thread id. |

## Methods

All methods are defined on `*Logger`.

| Method | Signature | Description |
|--------|-----------|-------------|
| `SetLevel` | `(level uint8) error` | Set log level. |
| `SetDomain` | `(domain *string) error` | Set log domain. Pass `nil` to clear. |

## Logging methods

All return `error`.

| Method | Signature |
|--------|-----------|
| `Trace` | `(message string) error` |
| `Debug` | `(message string) error` |
| `Info` | `(message string) error` |
| `Success` | `(message string) error` |
| `Warning` | `(message string) error` |
| `Error` | `(message string) error` |
| `Critical` | `(message string) error` |
| `Fatal` | `(message string) error` |
| `Exception` | `(message string) error` |

## Usage example

```go
console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
writers := []fl.WriterConfigEnum{*console}
log := logging.New(fl.DEBUG, nil, writers, nil, nil)

name := "WorkerThread"
threadLogger := logger.NewExt(fl.DEBUG, &name, 1, 1)
log.AddLogger(*threadLogger)

// In a goroutine:
threadLogger.Info("Message from worker thread")

log.Shutdown(false)
```

> **Note:** `AddLogger` and `RemoveLogger` take `logger.Logger` by value (not pointer). Dereference a `*Logger` with `*` when passing it in.
