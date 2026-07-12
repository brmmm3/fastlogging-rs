# Logging

The `Logging` struct is the primary logger instance in the `gofastlogging/fastlogging/logging` package. It owns a set of writers and optional sub-loggers, and exposes the full logging API (`Info`, `Debug`, `Error`, etc.).

```go
import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)
```

## Constructors

### Default

```go
func Default() (*Logging, error)
```

Creates a `Logging` with default configuration. If the environment variable `FASTLOGGING_CONFIG_FILE` is set, the configuration is read from that file. Otherwise a console writer is created at `INFO` level.

Returns `(*Logging, error)`.

### New

```go
func New(level uint8, domain *string, configs []fl.WriterConfigEnum, extConfig *fl.ExtConfig, configPath *string) *Logging
```

Creates a new `Logging` instance. Returns `*Logging` (nil on failure). Does **not** return an error.

Parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| `level` | `uint8` | Log level filter. Use constants like `fl.DEBUG`, `fl.INFO`. |
| `domain` | `*string` | Log domain string, or `nil`. |
| `configs` | `[]fl.WriterConfigEnum` | Slice of writer configs. Create with `writer.XxxWriterConfigNew` and **dereference pointers** with `*` when building the slice. |
| `extConfig` | `*fl.ExtConfig` | Extended formatting config, or `nil`. |
| `configPath` | `*string` | Path to a config file, or `nil`. |

Example:

```go
console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
writers := []fl.WriterConfigEnum{*console}
domain := "myapp"
log := logging.New(fl.DEBUG, &domain, writers, nil, nil)
```

## Methods

All methods are defined on `*Logging`.

### Lifecycle

| Method | Signature | Description |
|--------|-----------|-------------|
| `Shutdown` | `(now bool) error` | Shut down the logger. If `now` is true, blocks until all queued logs are flushed. |

### Sub-loggers

| Method | Signature | Description |
|--------|-----------|-------------|
| `AddLogger` | `(log logger.Logger) error` | Attach a sub-logger. Pass by value, so dereference a `*Logger` with `*`. |
| `RemoveLogger` | `(log logger.Logger) error` | Detach a sub-logger. |

### Level and domain

| Method | Signature | Description |
|--------|-----------|-------------|
| `SetLevel` | `(wid uint32, level uint8) error` | Set log level for a specific writer by ID. |
| `SetDomain` | `(domain *string) error` | Set the log domain. Pass `nil` to clear. |
| `SetLevel2Sym` | `(level2sym uint8) error` | Set level symbol format. Use `fl.Sym.Into()`, `fl.Short.Into()`, or `fl.Str.Into()`. |
| `SetExtConfig` | `(extConfig fl.ExtConfig) error` | Set extended formatting config. |
| `SetDebug` | `(debug uint32) error` | Set debug level. Developers only. |

### Writers

| Method | Signature | Description |
|--------|-----------|-------------|
| `SetRootWriterConfig` | `(config fl.WriterConfigEnum) error` | Set the root writer config (used for server/client setups). |
| `SetRootWriter` | `(writer fl.WriterEnum) error` | Set the root writer. |
| `AddWriterConfig` | `(config fl.WriterConfigEnum) error` | Add a writer at runtime. |
| `AddWriterConfigs` | `(configs []fl.WriterConfigEnum) error` | Add multiple writers at runtime. |
| `AddWriters` | `(writers []fl.WriterEnum) error` | Add multiple existing writers. |
| `RemoveWriter` | `(wid uint32) error` | Remove a writer by ID. |
| `RemoveWriters` | `(wids []uint32) fl.WriterEnums` | Remove multiple writers. Returns the removed writers. |
| `Enable` | `(wid uint32) error` | Enable a writer. |
| `Disable` | `(wid uint32) error` | Disable a writer. |
| `EnableType` | `(typ fl.WriterTypeEnum) error` | Enable all writers of a type. |
| `DisableType` | `(typ fl.WriterTypeEnum) error` | Disable all writers of a type. |

### Sync and rotation

| Method | Signature | Description |
|--------|-----------|-------------|
| `Sync` | `(types []fl.WriterTypeEnum, timeout float64) error` | Sync specific writer types. `timeout` is in seconds. |
| `SyncAll` | `(timeout float64) error` | Sync all writers. `timeout` is in seconds. |
| `Rotate` | `(path string) error` | Rotate the log file. |

### Encryption

| Method | Signature | Description |
|--------|-----------|-------------|
| `SetEncryption` | `(typ fl.WriterTypeEnum, key fl.KeyStruct) error` | Set encryption key for a writer type. |

### Server introspection

| Method | Signature | Description |
|--------|-----------|-------------|
| `GetServerConfig` | `() fl.ServerConfig` | Get root server config. |
| `GetServerConfigs` | `() fl.ServerConfigs` | Get all server configs. |
| `GetRootServerAddressPort` | `() string` | Get root server `address:port`. |
| `GetRootServerAddressesPorts` | `() map[uint32]string` | Get all server addresses:ports (key = writer ID). |
| `GetRootServerAddresses` | `() map[uint32]string` | Get all server addresses (key = writer ID). |
| `GetRootServerPorts` | `() map[uint32]uint16` | Get all server ports (key = writer ID). |
| `GetServerAuthKey` | `() fl.KeyStruct` | Get server auth/encryption key. |

### Config persistence

| Method | Signature | Description |
|--------|-----------|-------------|
| `GetConfigString` | `() string` | Get the complete config as a string. |
| `SaveConfig` | `(path string) error` | Save config to a file. |

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

## Helper functions

The `logging` package provides convenience constructors as alternatives to the `writer` package factories. They return values (not pointers) and apply defaults.

| Function | Signature | Description |
|----------|-----------|-------------|
| `ConsoleWriterConfigHelper` | `(level uint8, color bool) fl.WriterConfigEnum` | Console writer config. |
| `FileWriterConfigHelper` | `(filepath string, compression uint32) fl.WriterConfigEnum` | File writer config with defaults. |
| `ServerConfigHelper` | `(host string, port uint16, key *fl.KeyStruct) fl.WriterConfigEnum` | **Client** writer config pointing at `host:port`. Despite the name, this creates a client writer, not a server. |

> **Note:** The `writer` package factories are preferred over these helpers. They return pointers, accept more parameters, and are more flexible.
