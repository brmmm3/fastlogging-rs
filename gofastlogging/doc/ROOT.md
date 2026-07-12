# Root Logger

The root logger is a process-wide singleton logger managed by the top-level `fastlogging` package (imported as `fl`). It provides package-level functions — no instance needed. This is different from the `Logging` struct in the `logging` sub-package, which creates individual logger instances.

## Functions

All functions live in the top-level `fastlogging` package (e.g. `fl.Init()`).

### Lifecycle

| Function | Description |
|---|---|
| `Init() error` | Initialize the root logger. |
| `Shutdown(now bool) error` | Shutdown the root logger. If `now` is true, waits for all logs to flush. |

### Configuration

| Function | Description |
|---|---|
| `SetLevel(wid uint32, level uint8) error` | Set log level for a writer by ID. |
| `SetDomain(domain *string) error` | Set the log domain (pass `nil` to clear). |
| `SetLevel2Sym(level2sym uint8) error` | Set the level symbol format (use `fl.Sym.Into()`, `fl.Short.Into()`, `fl.Str.Into()`). |
| `SetExtConfig(extConfig ExtConfig) error` | Set extended formatting config. |

### Sub-Loggers

| Function | Description |
|---|---|
| `AddLogger(log logger.Logger) error` | Add a sub-logger (takes `logger.Logger` by value). |
| `RemoveLogger(log logger.Logger) error` | Remove a sub-logger. |

### Writers

| Function | Description |
|---|---|
| `SetRootWriterConfig(config WriterConfigEnum) error` | Set the root writer config. |
| `SetRootWriter(writer WriterEnum) error` | Set the root writer. |
| `AddWriterConfig(config WriterConfigEnum) error` | Add a writer config. |
| `AddWriter(writer WriterEnum) error` | Add a writer. |
| `AddWriterConfigs(configs WriterConfigEnums, configCnt uint32) int` | Add multiple writer configs. |
| `AddWriters(writers WriterEnums, writerCnt uint32) int` | Add multiple writers. |
| `RemoveWriter(wid uint32) error` | Remove a writer by ID. |
| `RemoveWriters(wids []uint32) (WriterEnums, error)` | Remove multiple writers; returns the removed writers. |

### Enable / Disable

| Function | Description |
|---|---|
| `Enable(wid uint32) error` | Enable a writer. |
| `Disable(wid uint32) error` | Disable a writer. |
| `EnableType(typ WriterTypeEnum) error` | Enable all writers of a given type. |
| `DisableType(typ WriterTypeEnum) error` | Disable all writers of a given type. |

### Sync / Rotate / Encryption / Server

| Function | Description |
|---|---|
| `Sync(types []WriterTypeEnum, timeout float64) error` | Sync specific writer types (timeout in seconds). |
| `SyncAll(timeout float64) error` | Sync all writers (timeout in seconds). |
| `Rotate(path string) error` | Rotate a log file. |
| `SetEncryption(wid uint32, key KeyStruct) error` | Set encryption key for a writer by ID. |
| `GetServerConfig() (ServerConfig, error)` | Get the server config. |
| `GetServerAuthKey() (KeyStruct, error)` | Get the server auth/encryption key. |
| `GetConfigString() (string, error)` | Get the config as a string. |
| `SaveConfig(path string) error` | Save the config to a file. |

## Logging Functions

All logging functions return `error`:

| Function | Level |
|---|---|
| `Trace(message string) error` | Trace |
| `Debug(message string) error` | Debug |
| `Info(message string) error` | Info |
| `Success(message string) error` | Success |
| `Warning(message string) error` | Warning |
| `Error(message string) error` | Error |
| `Critical(message string) error` | Critical |
| `Fatal(message string) error` | Fatal |
| `Exception(message string) error` | Exception |

## Example

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "log"
)

func main() {
    err := fl.Init()
    if err != nil {
        log.Fatal(err)
    }
    fl.Trace("Trace message")
    fl.Debug("Debug message")
    fl.Info("Info Message")
    fl.Warning("Warning Message")
    fl.Error("Error Message")
    fl.Fatal("Fatal Message")
    fl.Shutdown(false)
}
```

> **Note:** The root logger does **not** require creating writer configs explicitly — it reads from the config file or uses defaults. To add writers programmatically, use `fl.AddWriterConfig()`.
