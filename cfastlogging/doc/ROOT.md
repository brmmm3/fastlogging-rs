
# API of the ROOT Logger

The ROOT logger API provides global logging functions for simple applications or quick setup. It is a singleton and not intended for advanced multi-instance or multi-backend use. For more flexibility, use the LOGGING API.

---

## Function Summary

| Function | Purpose |
|----------|---------|
| `root_init` | Initialize root logger |
| `root_shutdown` | Shutdown root logger |
| `root_set_level` | Set log level for a writer |
| `root_set_domain` | Set log domain |
| `root_set_level2sym` | Set log level symbol style |
| `root_set_ext_config` | Set extended formatting config |
| `root_add_logger` | Register a Logger |
| `root_remove_logger` | Unregister a Logger |
| `root_set_root_writer_config` | Set root writer from config |
| `root_set_root_writer` | Set root writer from instance |
| `root_add_writer_config` | Add writer from config |
| `root_add_writer` | Add writer from instance |
| `root_remove_writer` | Remove writer by id |
| `root_add_writer_configs` | Add multiple writers from configs |
| `root_add_writers` | Add multiple writer instances |
| `root_remove_writers` | Remove multiple writers |
| `root_enable` | Enable writer by id |
| `root_disable` | Disable writer by id |
| `root_enable_type` | Enable all writers of a type |
| `root_disable_type` | Disable all writers of a type |
| `root_sync` | Sync specific writer types |
| `root_sync_all` | Sync all writers |
| `root_rotate` | Rotate file loggers |
| `root_set_encryption` | Set encryption for network writer |
| `root_get_writer_config` | Get config for a writer |
| `root_get_writer_configs` | Get all writer configs |
| `root_get_server_config` | Get server config for a writer |
| `root_get_server_configs` | Get all server configs |
| `root_get_root_server_address_port` | Get root server address:port |
| `root_get_server_addresses_ports` | Get all server addresses:ports |
| `root_get_server_addresses` | Get all server addresses |
| `root_get_server_ports` | Get all server ports |
| `root_get_server_auth_key` | Get server auth key |
| `root_get_config_string` | Get config as string |
| `root_save_config` | Save config to file |
| `root_trace` | Log TRACE message |
| `root_debug` | Log DEBUG message |
| `root_info` | Log INFO message |
| `root_success` | Log SUCCESS message |
| `root_warning` | Log WARNING message |
| `root_error` | Log ERROR message |
| `root_critical` | Log CRITICAL message |
| `root_fatal` | Log FATAL message |
| `root_exception` | Log EXCEPTION message |
| `root_set_debug` | Set debug level |

---


## `root_init()`
Initialize the root logger singleton. Call before any other root_* function.


## `root_shutdown(now: bool)`
Shutdown the root logger. If `now` is true, waits until all writers have flushed logs.


## `root_set_level(wid: c_uint, level: u8) -> isize`
Set log level for writer with id `wid`.


## `root_set_domain(domain: *const c_char)`
Set the log domain string for all writers.


## `root_set_level2sym(level2sym: u8)`
Set log level symbol style (0 = symbol, 1 = short, 2 = long).


## `root_set_ext_config(ext_config: &ExtConfig)`
Set extended formatting configuration (hostname, process, thread info, etc).


## `root_add_logger(logger: &mut Logger)`
Register a Logger instance for use with the root logger.


## `root_remove_logger(logger: &mut Logger)`
Unregister a Logger instance.


## `root_set_root_writer_config(config: *mut WriterConfigEnum) -> isize`
Set the root writer from a config. Returns 0 on success.


## `root_set_root_writer(writer: *mut WriterEnum) -> isize`
Set the root writer from a WriterEnum instance. Returns 0 on success.


## `root_add_writer_config(config: *mut WriterConfigEnum) -> isize`
Add a writer from a config. Returns the new writer id.


## `root_add_writer(writer: *mut WriterEnum) -> usize`
Add a writer from a WriterEnum instance. Returns the new writer id.


## `root_remove_writer(wid: usize) -> *const WriterEnum`
Remove a writer by id. Returns the writer config.


## `root_add_writer_configs(configs: *mut WriterConfigEnum, config_cnt: usize) -> isize`
Add multiple writers from configs. Returns pointer to list of new writer ids.


## `root_add_writers(writers: *mut WriterEnum, writer_cnt: usize) -> isize`
Add multiple writer instances. Returns pointer to list of new writer ids.


## `root_remove_writers(wids: *mut u32, wid_cnt: u32) -> *mut CWriterEnums`
Remove multiple writers by id. Returns pointer to list of removed writers.


## `root_enable(wid: usize) -> isize`
Enable a writer by id. Returns 0 on success.


## `root_disable(wid: usize) -> isize`
Disable a writer by id. Returns 0 on success.


## `root_enable_type(typ: *mut WriterTypeEnum) -> isize`
Enable all writers of a given type. Returns 0 on success.


## `root_disable_type(typ: *mut WriterTypeEnum) -> isize`
Disable all writers of a given type. Returns 0 on success.


## `root_sync(types: *mut WriterTypeEnum, type_cnt: c_uint, timeout: c_double) -> isize`
Synchronize all writers of the given types. Waits up to `timeout` seconds. Returns 0 on success.


## `root_sync_all(timeout: c_double) -> isize`
Synchronize all writers. Waits up to `timeout` seconds. Returns 0 on success.


## `root_rotate(path: *mut PathBuf) -> isize`
Rotate log file with path `path`, or all log files if `path` is `NULL`. Returns 0 on success.


## `root_set_encryption(wid: c_uint, key: *mut CKeyStruct) -> isize`
Set authentication or AES encryption key for a network client writer or server. Returns 0 on success.


## `root_get_writer_config(wid: c_uint) -> *const WriterConfigEnum`
Get configuration for writer `wid`. Returns NULL if `wid` is invalid.


## `root_get_writer_configs() -> *const WriterConfigEnums`
Get all writer configurations.


## `root_get_server_config(wid: usize) -> *mut ServerConfig`
Get server configuration with id `wid`. Returns NULL if not found or not a server.


## `root_get_server_configs() -> *const ServerConfigs`
Get all server configurations.


## `root_get_root_server_address_port() -> *const char`
Get address and port to parent process connection as a string (`IP:Port`).


## `root_get_server_addresses_ports() -> *const Cu32StringVec`
Get list of connections (address and port) to root server.


## `root_get_server_addresses() -> *const Cu32StringVec`
Get list of connections (only address) to root server.


## `root_get_server_ports() -> *const Cu32u16Vec`
Get list of connections (only ports) to root server.


## `root_get_server_auth_key() -> *mut KeyStruct`
Get authentication or AES encryption key of root server instance.


## `root_get_config_string() -> *const c_char`
Get complete configuration as a string.


## `root_save_config(path: *const c_char) -> isize`
Save configuration to file. If `path` is provided, writes to that path; otherwise uses the default path.


## Logging Methods

All logging methods return 0 on success, or a negative error code on failure. See DEF.md for log level values.

### `root_trace(message: *const c_char) -> isize`
Log **TRACE** message.


### `root_debug(message: *const c_char) -> isize`
Log **DEBUG** message.


### `root_info(message: *const c_char) -> isize`
Log **INFO** message.


### `root_success(message: *const c_char) -> isize`
Log **SUCCESS** message.


### `root_warning(message: *const c_char) -> isize`
Log **WARNING** message.


### `root_error(message: *const c_char) -> isize`
Log **ERROR** message.


### `root_critical(message: *const c_char) -> isize`
Log **CRITICAL** message.


### `root_fatal(message: *const c_char) -> isize`
Log **FATAL** message.


### `root_exception(message: *const c_char) -> isize`
Log **EXCEPTION** message.


## `root_set_debug(debug: u8)`
Set debug level for root logger. For development use only.

---

## Usage Example

```c
#include <stdio.h>
#include "h/cfastlogging.h"

int main(void) {
	root_init();
	if (root_info("Hello from root logger!") != 0) fprintf(stderr, "Root log failed\n");
	root_shutdown(0);
	return 0;
}
```

---

## Root Logger vs. Logging Instance

- The root logger is a singleton: only one instance per process.
- Use the root logger for simple applications or quick setup.
- For advanced use (multiple log domains, dynamic writers, per-module loggers), use the LOGGING API.

---

## Global State and Limitations

- The root logger holds global state; not suitable for libraries or multi-tenant applications.
- All configuration changes affect the entire process.
- Only one set of writers and config can be active at a time.
