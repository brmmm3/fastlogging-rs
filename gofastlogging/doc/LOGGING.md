
# gofastlogging: Go API Reference

This document describes the idiomatic Go API for the fastlogging package.

---

## type Logging

```go
type Logging struct { /* ... */ }
```

### func New(level uint8, domain *string, writers []WriterConfigEnum, extConfig *ExtConfig, configPath *string) *Logging

Creates a new logger instance. Use writer constructors for all writers. Example:

```go
console, err := fastlogging.ConsoleWriterConfigNew(fastlogging.DEBUG, true)
if err != nil { log.Fatal(err) }
logger := fastlogging.New(fastlogging.DEBUG, nil, []fastlogging.WriterConfigEnum{console}, nil, nil)
```

### func (l *Logging) Trace(msg string)
### func (l *Logging) Debug(msg string)
### func (l *Logging) Info(msg string)
### func (l *Logging) Success(msg string)
### func (l *Logging) Warning(msg string)
### func (l *Logging) Error(msg string)
### func (l *Logging) Fatal(msg string)

Log a message at the specified level.

### func (l *Logging) Shutdown(now bool)

Shutdown the logger. If `now` is true, waits for all logs to flush.

### func (l *Logging) SetLevel(writerID int, level uint8)

Set log level for a writer by ID.

### func (l *Logging) SetDomain(domain string)

Set the log domain for this logger.

### func (l *Logging) SetLevelSymbols(levelSyms LevelSymbol)

Set the log level symbol format (Sym, Short, Str).

### func (l *Logging) SetExtConfig(extConfig *ExtConfig)

Set extended formatting configuration.

### func (l *Logging) AddWriter(config WriterConfigEnum) int

Add a writer. Returns the writer ID.

### func (l *Logging) RemoveWriter(writerID int) error

Remove a writer by ID.

### func (l *Logging) AddWriters(configs []WriterConfigEnum) []int

Add multiple writers. Returns their IDs.

---

## Writer Constructors

- ConsoleWriterConfigNew(level uint8, colors bool) (WriterConfigEnum, error)
- FileWriterConfigNew(level uint8, path string, size uint32, backlog uint32, timeout int32, time int64, compression CompressionMethodEnum) (WriterConfigEnum, error)
- ServerConfigNew(level uint8, address string, key *KeyStruct) (WriterConfigEnum, error)
- ClientWriterConfigNew(level uint8, address string, key *KeyStruct) (WriterConfigEnum, error)
- SyslogWriterConfigNew(level uint8, hostname, pname string, pid uint32) (WriterConfigEnum, error)
- CallbackWriterConfigNew(level uint8, callback func(level uint8, domain, message string)) (WriterConfigEnum, CallbackHandle, error)

See WRITERS.md for details.

---

## Best Practices

- Always check errors when creating writer configs.
- Use `defer handle.UnregisterCallback()` for callback writers.
- Use the Go API for dynamic configuration, or config files for static setups.


## `remove_writers(wid: List[int] = None) -> Config | None`

Remove list of `wid` writer ids if provided or all writers if `None`. List of writer configurations will be returned.

## `enable(wid: int)`

Enable writer with id `wid`. If `wid` is invalid an exception will be thrown.

## `disable(wid: int)`

Disable writer with id `wid`. If `wid` is invalid an exception will be thrown.

## `enable_type(typ: WriterTypeEnum)`

Enable all writers with type `typ`. If no type with `typ` was found an exception will be thrown.

## `disable_type(typ: WriterTypeEnum)`

Disable all writers with type `typ`. If no type with `typ` was found an exception will be thrown.

## `sync(types: List[WriterTypeEnum], timeout: float = None)`

Sync all writers listed in `types`. If `timeout` is provided and waiting takes longer then an exception is thrown.

## `sync_all(timeout: float = None)`

Sync all writers. If `timeout` is provided and waiting takes longer then an exception is thrown.

## `rotate(path: str = None)`

Rotate log file with path `path` or all log files if `path` is `None`.
An exception is thrown if file rotation fails.

## `set_encryption(wid: int, key: EncryptionMethod)`

Set authentication or AES encryption key for network client writer or server with id `wid`.
An exception is thrown if either `wid` doesn't exist or `key` contains invalid invalid data.

## `get_writer_config(wid: int) -> WriterConfigEnum | None`

Get configuration for writer `wid`. Returns `None` if `wid` is invalid.

## `get_server_config(wid: int) -> ServerConfig`

Get server configuration with id `wid`. An exception is thrown if either `wid` is not found or instance is not a server.

## `get_server_configs() -> Dict[int, ServerConfig]`

Get all server configurations. Key is `wid`.

## `get_server_addresses_ports() -> Dict[int, str]`

Get all server addresses and ports. Key is `wid`. Value has syntax `IP:Port`.

## `get_server_addresses() -> Dict[int, ServerConfig]`

Get all server addresses. Key is `wid`. Value has syntax `IP`.

## `get_server_ports() -> Dict[int, int]`

Get all server ports. Key is `wid`. Value is port.

## `get_server_auth_key() -> EncryptionMethod`

Get authentication or AES encryption key of root server instance.

## `get_config_string() -> str`

Get complete configuration as string.

## `save_config(path: str = None)`

Save configuration to file. If `path` is provided then configuration is written to this new path. Otherwise the default path in the configuration is used.
An exception is thrown is saving the configuration failed.

## `get_parent_pid() -> int | None`

Get process id of parent process for logging or `None` if there is no parent logger.

## `get_parent_client_writer_config() -> ClientWriterConfig | None`

Get configuration of client writer instance which writes logs to the parent process or `None` if there is no parent logger.

## `get_parent_pid_client_writer_config() -> Tuple[int, ClientWriterConfig] | None`

Get parent process id and configuration of client writer instance which writes logs to the parent process or `None` if there is no parent logger.

## `trace(obj: Py<PyAny>)`

Log **TRACE** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `debug(obj: Py<PyAny>)`

Log **DEBUG** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `info(obj: Py<PyAny>)`

Log **INFO** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `success(obj: Py<PyAny>)`

Log **SUCCESS** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `warning(obj: Py<PyAny>)`

Log **WARNING** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `error(obj: Py<PyAny>)`

Log **ERROR** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `critical(obj: Py<PyAny>)`

Log **CRITICAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `fatal(obj: Py<PyAny>)`

Log **FATAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `exception(obj: Py<PyAny>)`

Log **EXCEPTION** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `set_debug(debug: int)`

Set debug level for root logger. This is only for developers.
