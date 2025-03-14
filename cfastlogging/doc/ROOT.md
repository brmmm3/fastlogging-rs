# API of the ROOT logger

## `root_init()`

Initialize root logger.

## `root_shutdown(now: bool)`

Shutdown fastlogging module. If argument `now` is `True` then this call will wait until all writers have written all logs.

## `root_set_level(wid: c_uint, level: u8) -> isize`

Set log level for writer with writer id `wid` to `level`. If setting log level fails an error value is returned. On success 0 is returned.

## `root_set_domain(domain: *const c_char)`

Set log domain.

## `root_set_level2sym(level2sym: u8)`

Set log level symbols used for log messages.
Following value for `level2sym` are allowed:

- 0 = LevelSyms::Sym -> single character symbol
- 1 = LevelSyms::Short -> short level name
- 2 = LevelSyms::Str -> long level name

## `root_set_ext_config(ext_config: &ExtConfig)`

Set extended formatting configuration.

## `root_add_logger(logger: &mut Logger)`

## `root_remove_logger(logger: &mut Logger)`

## `root_set_root_writer_config(config: *mut WriterConfigEnum) -> isize`

Set configuration of root writer. If setting configuration fails an error value is returned. On success 0 is returned.

## `root_set_root_writer(writer: *mut WriterEnum) -> isize`

Set root writer. If setting writer fails an error value is returned. On success 0 is returned.

## `root_add_writer_config(config: *mut WriterConfigEnum) -> isize`

Add new writer providing the writer configuration.

`config` must be one of:

- [RootConfig](DEF.md#RootConfig)
- [ConsoleWriterConfig](DEF.md#ConsoleWriterConfig)
- [FileWriterConfig](DEF.md#FileWriterConfig)
- [ClientWriterConfig](DEF.md#ClientWriterConfig)
- [ServerConfig](DEF.md#ServerConfig)
- [SyslogWriterConfig](DEF.md#SyslogWriterConfig)
- [CallbackWriterConfig](DEF.md#CallbackWriterConfig)

If config has wrong class type an error value is returned. On success the method returns the `id` of the new writer.

## `root_add_writer(writer: *mut WriterEnum) -> usize`

Add new writer.

`writer` must be one of:

- [Root](DEF.md#RootConfig)
- [ConsoleWriter](DEF.md#ConsoleWriterConfig)
- [FileWriter](DEF.md#FileWriterConfig)
- [ClientWriter](DEF.md#ClientWriterConfig)
- [Server](DEF.md#ServerConfig)
- [SyslogWriter](DEF.md#SyslogWriterConfig)
- [CallbackWriter](DEF.md#CallbackWriterConfig)

If writer has wrong class type an error value is returned. On success the method returns the `id` of the new writer.

## `root_remove_writer(wid: usize) -> *const WriterEnum`

`wid` is the writer id. If valid the configuration of the writer will be returned.

## `root_add_writer_configs(configs: *mut WriterConfigEnum, config_cnt: usize,) -> isize`

Add a list of writer through providing a list of configurations.
For valid configuration types see `root_add_writer_config`. In case of a failure an error code is returned. In case of success the pointer to a list of writers ids (`wid`) is returned.

## `root_add_writers(writers: *mut WriterEnum, writer_cnt: usize,) -> isize`

Add a list of writers.
For valid writer types see `root_add_writer`. In case of a failure an error code is returned. In case of success the pointer to a list of writers ids (`wid`) is returned.

## `root_remove_writers(wids: *mut u32, wid_cnt: u32) -> *mut CWriterEnums`

Remove a list of writers. `wids` is a pointer to a list of writer ids (`wid`).
In case of a failure an error code is returned. In case of success the pointer to a list of writers is returned.

## `root_enable(wid: usize) -> isize`

Enable writer with id `wid`. If `wid` is invalid an error code is returned.
On succes 0 is returned.

## `root_disable(wid: usize) -> isize`

Disable writer with id `wid`. If `wid` is invalid an error code is returned.
On succes 0 is returned.

## `root_enable_type(typ: *mut WriterTypeEnum) -> isize`

Enable all writers with type `typ`. If no type with `typ` was found an error code is returned. On succes 0 is returned.

## `root_disable_type(typ: *mut WriterTypeEnum) -> isize`

Disable all writers with type `typ`. If no type with `typ` was found an error code is returned. On succes 0 is returned.

## `root_sync(types: *mut WriterTypeEnum, type_cnt: c_uint, timeout: c_double,) -> isize`

Sync all writers listed in `types`. If `timeout` is provided and waiting takes longer then an error code is returned. On success 0 is returned.

## `root_sync_all(timeout: c_double) -> isize`

Sync all writers. If `timeout` is provided and waiting takes longer then an error code is returned. On success 0 is returned.

## `root_rotate(path: *mut PathBuf) -> isize`

Rotate log file with path `path` or all log files if `path` is `None`.
An error code is returned if file rotation failed. On success 0 is returned.

## `root_set_encryption(wid: c_uint, key: *mut CKeyStruct) -> isize`

Set authentication or AES encryption key for network client writer or server with id `wid`.
An error code is returned if either `wid` doesn't exist or `key` contains invalid invalid data. On success 0 is returned.

## `root_get_writer_config(wid: c_uint) -> *const WriterConfigEnum`

Get configuration for writer `wid`. Returns NULL if `wid` is invalid.

## `root_get_writer_configs() -> *const CWriterConfigEnums`

Get list of writer configurations.

## `root_get_server_config(wid: usize) -> *mut CServerConfig`

Get server configuration with id `wid`. NULL is returned if either `wid` is not found or instance is not a server.

## `root_get_server_configs() -> *const CServerConfigs`

List of server configurations is returned.

## `root_get_root_server_address_port() -> *const char`

Get address and port to parent process connection.

## `root_get_server_addresses_ports() -> *const Cu32StringVec`

Get list of connections (address and port) to root server.

## `root_get_server_addresses() -> *const Cu32StringVec`

Get list of connections (only address) to root server.

## `root_get_server_ports() -> *const Cu32u16Vec`

Get list of connections (only ports) to root server.

## `root_get_server_auth_key() -> *mut CKeyStruct`

Get authentication or AES encryption key of root server instance.

## `root_get_config_string() -> *const c_char`

Get complete configuration as string.

## `root_save_config(path: *const c_char) -> isize`

Save configuration to file. If `path` is provided then configuration is written to this new path. Otherwise the default path in the configuration is used.
An exception is thrown is saving the configuration failed.

## `root_trace(message: *const c_char) -> isize`

Log **TRACE** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_debug(message: *const c_char) -> isize`

Log **DEBUG** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_info(message: *const c_char) -> isize`

Log **INFO** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_success(message: *const c_char) -> isize`

Log **SUCCESS** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_warning(message: *const c_char) -> isize`

Log **WARNING** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_error(message: *const c_char) -> isize`

Log **ERROR** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_critical(message: *const c_char) -> isize`

Log **CRITICAL** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_fatal(message: *const c_char) -> isize`

Log **FATAL** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_exception(message: *const c_char) -> isize`

Log **EXCEPTION** message.
An error code is returned in case of failure. On success 0 is returned.

## `root_set_debug(debug: u8)`

Set debug level for root logger. This is only for developers.
