
# API of the LOGGING Module

The LOGGING API provides the main entry point for multi-backend, high-performance logging. Supports multiple writers, dynamic configuration, and thread-safe operation.

---

## Function Summary

| Function | Purpose |
|----------|---------|
| `logging_new` | Create a new Logging instance |
| `logging_apply_config` | Load/apply config from file |
| `logging_shutdown` | Shutdown logging, flush writers |
| `logging_set_level` | Set log level for a writer |
| `logging_set_domain` | Set log domain |
| `logging_set_level2sym` | Set log level symbol style |
| `logging_set_ext_config` | Set extended formatting config |
| `logging_add_logger` | Register a Logger instance |
| `logging_remove_logger` | Unregister a Logger |
| `logging_set_root_writer_config` | Set root writer from config |
| `logging_set_root_writer` | Set root writer from instance |
| `logging_add_writer_config` | Add writer from config |
| `logging_add_writer` | Add writer from instance |
| `logging_remove_writer` | Remove writer by id |
| `logging_add_writer_configs` | Add multiple writers from configs |
| `logging_add_writers` | Add multiple writer instances |
| `logging_remove_writers` | Remove multiple writers |
| `logging_enable` | Enable writer by id |
| `logging_disable` | Disable writer by id |
| `logging_enable_type` | Enable all writers of a type |
| `logging_disable_type` | Disable all writers of a type |
| `logging_sync` | Sync specific writer types |
| `logging_sync_all` | Sync all writers |
| `logging_rotate` | Rotate file loggers |
| `logging_set_encryption` | Set encryption for network writer |
| `logging_get_writer_config` | Get config for a writer |
| `logging_get_writer_configs` | Get all writer configs |
| `logging_get_server_config` | Get server config for a writer |
| `logging_get_server_configs` | Get all server configs |
| `logging_get_root_server_address_port` | Get root server address:port |
| `logging_get_server_addresses_ports` | Get all server addresses:ports |
| `logging_get_server_addresses` | Get all server addresses |
| `logging_get_server_ports` | Get all server ports |
| `logging_get_server_auth_key` | Get server auth key |
| `logging_get_config_string` | Get config as string |
| `logging_save_config` | Save config to file |
| `logging_trace` | Log TRACE message |
| `logging_debug` | Log DEBUG message |
| `logging_info` | Log INFO message |
| `logging_success` | Log SUCCESS message |
| `logging_warning` | Log WARNING message |
| `logging_error` | Log ERROR message |

---


## `logging_new(level: c_char, domain: *const c_char, configs_ptr: *const *mut WriterConfigEnum, configs_cnt: c_uint, ext_config: *mut ExtConfig, config_path: *const c_char) -> *mut Logging`
Create a new Logging instance.
- `level`: Default log level (use NOTSET for all messages)
- `domain`: Log domain string (default: "root")
- `configs_ptr`/`configs_cnt`: Array of writer configs (see below)
- `ext_config`: Optional extended formatting config
- `config_path`: Optional config file path (overrides other params)

**Writer Config Usage:**
You can add multiple writers (console, file, network, etc.) by passing an array of WriterConfigEnum objects. See DEF.md for config struct details.

---


## `logging_apply_config(logging: &mut Logging, path: *const c_char) -> isize`
Load and apply configuration from a file at `path`.


## `logging_shutdown(logging: &mut Logging, now: i8) -> isize`
Shutdown the logging system. If `now` is true, waits until all writers have flushed logs.


## `logging_set_level(logging: &mut Logging, wid: c_uint, level: u8) -> isize`
Set log level for writer with id `wid`.


## `logging_set_domain(logging: &mut Logging, domain: *const c_char)`
Set the log domain string for all writers.


## `logging_set_level2sym(logging: &mut Logging, level2sym: u8)`
Set log level symbol style (0 = symbol, 1 = short, 2 = long).


## `logging_set_ext_config(logging: &mut Logging, ext_config: &ExtConfig)`
Set extended formatting configuration (hostname, process, thread info, etc).


## `logging_add_logger(logging: &mut Logging, logger: &mut Logger)`
Register a Logger instance for use with this Logging instance.


## `logging_remove_logger(logging: &mut Logging, logger: &mut Logger)`
Unregister a Logger instance.


## `logging_set_root_writer_config(logging: &mut Logging, config: *mut WriterConfigEnum) -> isize`
Set the root writer from a config. Config must be a ClientWriterConfig or ServerConfig.


## `logging_set_root_writer(logging: &mut Logging, writer: *mut WriterEnum) -> isize`
Set the root writer from a WriterEnum instance.


## `logging_add_writer_config(logging: &mut Logging, config: *mut WriterConfigEnum) -> isize`
Add a writer from a config. Config must be a valid writer config (see DEF.md). Returns the new writer id.


## `logging_add_writer(logging: &mut Logging, writer: *mut WriterEnum) -> usize`
Add a writer from a WriterEnum instance. Returns the new writer id.


## `remove_writer(wid: int) -> Config | None`
Remove a writer by id. Returns the config if successful.


## `add_writers(configs: List[Config]) -> int`
Add multiple writers from configs. Returns a list of new writer ids.


## `logging_remove_writer(logging: &mut Logging, id: usize) -> *const WriterEnum`
Remove a writer by id. Returns the writer config.


## `logging_add_writer_configs(logging: &mut Logging, configs: *mut WriterConfigEnum, config_cnt: usize) -> isize`
Add multiple writers from configs.


## `logging_add_writers(logging: &mut Logging, writers: *mut WriterEnum, writer_cnt: usize) -> *mut CusizeVec`
Add multiple writer instances.


## `logging_remove_writers(logging: &mut Logging, wids: *mut u32, wid_cnt: u32) -> *mut CWriterEnums`
Remove multiple writers by id.


## `logging_enable(logging: &mut Logging, wid: usize) -> isize`
Enable a writer by id.


## `logging_disable(logging: &mut Logging, wid: usize) -> isize`
Disable a writer by id.


## `logging_enable_type(logging: &mut Logging, typ: *mut WriterTypeEnum) -> isize`
Enable all writers of a given type.


## `logging_disable_type(logging: &mut Logging, typ: *mut WriterTypeEnum) -> isize`
Disable all writers of a given type.


## `logging_sync(logging: &Logging, types: *mut WriterTypeEnum, type_cnt: c_uint, timeout: c_double) -> isize`
Synchronize all writers of the given types. Waits up to `timeout` seconds.


## `logging_sync_all(logging: &Logging, timeout: c_double) -> isize`
Synchronize all writers. Waits up to `timeout` seconds.


## `logging_rotate(logging: &Logging, path: *mut PathBuf) -> isize`
Rotate log file with path `path`, or all log files if `path` is `NULL`.


## `logging_set_encryption(logging: &mut Logging, wid: c_uint, key: *mut CKeyStruct) -> isize`
Set authentication or AES encryption key for a network client writer or server.


## `logging_get_writer_config(logging: &Logging, wid: c_uint) -> *const WriterConfigEnum`
Get configuration for writer `wid`. Returns `NULL` if `wid` is invalid.


## `logging_get_writer_configs(logging: &Logging) -> *const WriterConfigEnums`
Get all writer configurations.


## `logging_get_server_config(logging: &Logging, wid: usize) -> *mut CServerConfig`
Get server configuration with id `wid`. Throws if not found or not a server.


## `logging_get_server_configs(logging: &Logging) -> *const CServerConfigs`
Get all server configurations. Key is `wid`.


## `logging_get_root_server_address_port(logging: &Logging) -> *const char`
Get root server address and port as a string (`IP:Port`).


## `logging_get_server_addresses_ports(logging: &Logging) -> *const Cu32StringVec`
Get all server addresses and ports. Key is `wid`, value is `IP:Port`.


## `logging_get_server_addresses(logging: &Logging) -> *const Cu32StringVec`
Get all server addresses. Key is `wid`, value is `IP`.


## `logging_get_server_ports(logging: &Logging) -> *const Cu32u16Vec`
Get all server ports. Key is `wid`, value is port.


## `logging_get_server_auth_key(logging: &Logging) -> *mut CKeyStruct`
Get authentication or AES encryption key of root server instance.


## `logging_get_config_string(logging: &Logging) -> *const c_char`
Get complete configuration as a string.


## `logging_save_config(logging: &mut Logging, path: *const c_char) -> isize`
Save configuration to file. If `path` is provided, writes to that path; otherwise uses the default path.


## Logging Methods

All logging methods return 0 on success, or a negative error code on failure. See DEF.md for log level values.

### `logging_trace(logging: &Logging, message: *const c_char) -> isize`
Log **TRACE** message.


### `logging_debug(logging: &Logging, message: *const c_char) -> isize`
Log **DEBUG** message.


### `logging_info(logging: &Logging, message: *const c_char) -> isize`
Log **INFO** message.


### `logging_success(logging: &Logging, message: *const c_char) -> isize`
Log **SUCCESS** message.


### `logging_warning(logging: &Logging, message: *const c_char) -> isize`
Log **WARNING** message.


### `logging_error(logging: &Logging, message: *const c_char) -> isize`
Log **ERROR** message.

---

## Usage Example

```c
#include <stdio.h>
#include "h/cfastlogging.h"

int main(void) {
	WriterConfigEnum writers[] = { console_writer_config_new(DEBUG, 1) };
	Logging logging = logging_new(DEBUG, NULL, writers, 1, NULL, NULL);
	if (logging_info(logging, "Hello from cfastlogging!") != 0) fprintf(stderr, "Log failed\n");
	logging_shutdown(logging, 0);
	return 0;
}
```

---

## Multi-Writer and Shutdown Semantics

- You can add multiple writers (console, file, network, etc.) to a single Logging instance.
- All writers receive log messages in parallel.
- Always call `logging_shutdown` before program exit to flush and close all writers.
- Use `logging_sync_all` to force flush with a timeout.

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
