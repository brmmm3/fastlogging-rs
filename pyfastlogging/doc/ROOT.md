# API of the ROOT logger

## `root_init()`

Initialize root logger.

## `shutdown(now: bool = False)`

Shutdown fastlogging module. If optional argument `now` is `True` then this call will wait until all writers have written all logs.

## `set_level(wid: int, level: int)`

Set log level for writer with writer id `wid` to `level`.

## `set_domain(domain: str)`

Set log domain.

## `set_level2sym(level2sym: LevelSyms)`

Set log level symbols used for log messages.

## `set_ext_config(ext_config: ExtConfig)`

Set extended formatting configuration.

## `add_logger(logger: Logger)`

## `remove_logger(logger: Logger)`

## `add_writer(config: Config) -> int`

`Config` must be one of:

- [RootConfig](DEF.md#RootConfig)
- [ConsoleWriterConfig](DEF.md#ConsoleWriterConfig)
- [FileWriterConfig](DEF.md#FileWriterConfig)
- [ClientWriterConfig](DEF.md#ClientWriterConfig)
- [ServerConfig](DEF.md#ServerConfig)
- [SyslogWriterConfig](DEF.md#SyslogWriterConfig)
- [CallbackWriterConfig](DEF.md#CallbackWriterConfig)

If config has wrong class type an exception is thrown.
The method returns the `id` of the new writer.

## `remove_writer(wid: int) -> Config | None`

`wid` is the writer id. If valid the configuration of the writer will be returned.

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
The file extension determines the structure type used. Allowed extensions are `json`, `xml`, `yaml`.  
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
