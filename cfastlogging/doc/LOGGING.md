# API of the LOGGING module

## `logging_new(level: c_char, domain: *const c_char, configs_ptr: *const *mut WriterConfigEnum, configs_cnt: c_uint, ext_config: *mut ExtConfig, config_path: *const c_char) -> *mut Logging`

Create `Logging` instance.  
`level` if not provided is set to `NOTSET`.  
`domain` if not provided is set to `root`.  
`configs` contains a list of writers configs.  
`ext_config` if provided sets the extended formatting configuration.
`config_path` if provided the configuration is loaded from a file.

## `logging_apply_config(logging: &mut Logging, path: *const c_char) -> isize`

## `logging_shutdown(logging: &mut Logging, now: i8) -> isize`

Shutdown fastlogging module. If optional argument `now` is `True` then this call will wait until all writers have written all logs.

## `logging_set_level(logging: &mut Logging, wid: c_uint, level: u8) -> isize`

Set log level for writer with writer id `wid` to `level`.

## `logging_set_domain(logging: &mut Logging, domain: *const c_char)`

Set log domain.

## `logging_set_level2sym(logging: &mut Logging, level2sym: u8)`

Set log level symbols used for log messages.

## `logging_set_ext_config(logging: &mut Logging, ext_config: &ExtConfig)`

Set extended formatting configuration.

## `logging_add_logger(logging: &mut Logging, logger: &mut Logger)`

## `logging_remove_logger(logging: &mut Logging, logger: &mut Logger)`

## `logging_set_root_writer_config(logging: &mut Logging, config: *mut WriterConfigEnum,) -> isize`

`Config` must be one of:

- [ClientWriterConfig](DEF.md#ClientWriterConfig)
- [ServerConfig](DEF.md#ServerConfig)

If config has wrong class type an exception is thrown.

## `logging_set_root_writer(logging: &mut Logging, writer: *mut WriterEnum,) -> isize`

## `logging_add_writer_config(logging: &mut Logging, config: *mut WriterConfigEnum,) -> isize`

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

## `logging_add_writer(logging: &mut Logging, writer: *mut WriterEnum,) -> usize`

## `remove_writer(wid: int) -> Config | None`

`wid` is the writer id. If valid the configuration of the writer will be returned.

## `add_writers(configs: List[Config]) -> int`

`Config` must be one of:

- [RootConfig](DEF.md#RootConfig)
- [ConsoleWriterConfig](DEF.md#ConsoleWriterConfig)
- [FileWriterConfig](DEF.md#FileWriterConfig)
- [ClientWriterConfig](DEF.md#ClientWriterConfig)
- [ServerConfig](DEF.md#ServerConfig)
- [SyslogWriterConfig](DEF.md#SyslogWriterConfig)
- [CallbackWriterConfig](DEF.md#CallbackWriterConfig)

If a config has wrong class type an exception is thrown.
The method returns a list of `id` of the new writers.

## `logging_remove_writer(logging: &mut Logging, id: usize,) -> *const WriterEnum`

Remove list of `wid` writer ids if provided or all writers if `None`. List of writer configurations will be returned.

## `logging_add_writer_configs(logging: &mut Logging, configs: *mut WriterConfigEnum, config_cnt: usize) -> isize`

## `logging_add_writers(logging: &mut Logging, writers: *mut WriterEnum, writer_cnt: usize,) -> *mut CusizeVec`

## `logging_remove_writers(logging: &mut Logging, wids: *mut u32, wid_cnt: u32,) -> *mut CWriterEnums`

## `logging_enable(logging: &mut Logging, wid: usize) -> isize`

Enable writer with id `wid`. If `wid` is invalid an exception will be thrown.

## `logging_disable(logging: &mut Logging, wid: usize) -> isize`

Disable writer with id `wid`. If `wid` is invalid an exception will be thrown.

## `logging_enable_type(logging: &mut Logging, typ: *mut WriterTypeEnum,) -> isize`

Enable all writers with type `typ`. If no type with `typ` was found an exception will be thrown.

## `logging_disable_type(logging: &mut Logging, typ: *mut WriterTypeEnum,) -> isize`

Disable all writers with type `typ`. If no type with `typ` was found an exception will be thrown.

## `logging_sync(logging: &Logging, types: *mut WriterTypeEnum, type_cnt: c_uint, timeout: c_double) -> isize`

Sync all writers listed in `types`. If `timeout` is provided and waiting takes longer then an exception is thrown.

## `logging_sync_all(logging: &Logging, timeout: c_double) -> isize`

Sync all writers. If `timeout` is provided and waiting takes longer then an exception is thrown.

## `logging_rotate(logging: &Logging, path: *mut PathBuf) -> isize`

Rotate log file with path `path` or all log files if `path` is `None`.
An exception is thrown if file rotation fails.

## `logging_set_encryption(logging: &mut Logging, wid: c_uint, key: *mut CKeyStruct,) -> isize`

Set authentication or AES encryption key for network client writer or server with id `wid`.
An exception is thrown if either `wid` doesn't exist or `key` contains invalid invalid data.

## `logging_get_writer_config(logging: &Logging, wid: c_uint,) -> *const WriterConfigEnum`

Get configuration for writer `wid`. Returns `None` if `wid` is invalid.

## `logging_get_writer_configs(logging: &Logging,) -> *const CWriterConfigEnums`

Get configuration for writer `wid`. Returns `None` if `wid` is invalid.

## `logging_get_server_config(logging: &Logging, wid: usize,) -> *mut CServerConfig`

Get server configuration with id `wid`. An exception is thrown if either `wid` is not found or instance is not a server.

## `logging_get_server_configs(logging: &Logging) -> *const CServerConfigs`

Get all server configurations. Key is `wid`.

## `logging_get_root_server_address_port(logging: &Logging) -> *const char`

Get all server addresses and ports. Key is `wid`. Value has syntax `IP:Port`.

## `logging_get_server_addresses_ports(logging: &LogginG) -> *const Cu32StringVec`

Get all server addresses and ports. Key is `wid`. Value has syntax `IP:Port`.

## `logging_get_server_addresses(logging: &Logging) -> *const Cu32StringVec`

Get all server addresses. Key is `wid`. Value has syntax `IP`.

## `logging_get_server_ports(logging: &Logging) -> *const Cu32u16Vec`

Get all server ports. Key is `wid`. Value is port.

## `logging_get_server_auth_key(logging: &Logging) -> *mut CKeyStruct`

Get authentication or AES encryption key of root server instance.

## `logging_get_config_string(logging: &Logging) -> *const c_char`

Get complete configuration as string.

## `logging_save_config(logging: &mut Logging, path: *const c_char) -> isize`

Save configuration to file. If `path` is provided then configuration is written to this new path. Otherwise the default path in the configuration is used.
An exception is thrown is saving the configuration failed.

## `logging_trace(logging: &Logging, message: *const c_char) -> isize`

Log **TRACE** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_debug(logging: &Logging, message: *const c_char) -> isize`

Log **DEBUG** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_info(logging: &Logging, message: *const c_char) -> isize`

Log **INFO** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_success(logging: &Logging, message: *const c_char) -> isize`

Log **SUCCESS** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_warning(logging: &Logging, message: *const c_char) -> isize`

Log **WARNING** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_error(logging: &Logging, message: *const c_char) -> isize`

Log **ERROR** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_critical(logging: &Logging, message: *const c_char) -> isize`

Log **CRITICAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_fatal(logging: &Logging, message: *const c_char) -> isize`

Log **FATAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_exception(logging: &Logging, message: *const c_char) -> isize`

Log **EXCEPTION** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `logging_set_debug(logging: &mut Logging, debug: u8)`

Set debug level for logging module. This is only for developers.
