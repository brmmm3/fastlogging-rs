# cppfastlogging C++ API Documentation

This document describes the main C++ API for cppfastlogging, a modern, idiomatic C++ wrapper for the fastlogging library. The API is organized into several modules:

- Core types and enums (def.hpp)
- Writer configuration and types (writer.hpp)
- Logger class (logger.hpp)
- Logging class (logging.hpp)
- Root logger API (root.hpp)
- C++ entrypoint and FFI helpers (cppfastlogging.hpp)

---

## Core Types and Enums (def.hpp)
- Log level constants: `NOLOG`, `EXCEPTION`, `CRITICAL`, `FATAL`, `ERROR`, `WARNING`, `SUCCESS`, `INFO`, `DEBUG`, `TRACE`, `NOTSET`.
- Error handling: `error_free`, `error_msg`, `error_code`.
- Rust FFI types: `Box<T>`, `Option<T>`, `Error` struct.
- String/port vector types: `Cu32StringVec`, `Cu32u16Vec`.
- Enums: `LevelSyms`, `FileTypeEnum`, `CompressionMethodEnum`.

## Writer Types and Configs (writer.hpp)
- Writer type variants: `Root`, `Console`, `File`, `Files`, `Client`, `Clients`, `Server`, `Servers`, `Callback`, `Syslog`.
- Writer config variants: `RootConfig`, `ConsoleWriterConfig`, `FileWriterConfig`, `ClientWriterConfig`, `ServerConfig`, `CallbackWriterConfig`, `SyslogWriterConfig`.
- Console/file/client writer config constructors.

## Logger Class (logger.hpp)
- `logging::Logger` — C++ RAII wrapper for a logger instance.
    - Constructor: `Logger(uint8_t level, const char *domain)`
    - Extended constructor: `Logger(uint8_t level, const char *domain, int8_t tname, int8_t tid)`
    - Move-only, non-copyable.
    - Methods: `set_level`, `set_domain`, `trace`, `debug`, `info`, `success`, `warn`, `warning`, `error`, `critical`, `fatal`, `exception`.
    - All log methods return int (0 = success, <0 = error).

## Logging Class (logging.hpp)
- C++ interface to the main logging instance.
    - Constructors: `logging_new_default`, `logging_new` (with level, domain, writer configs, ext config, config path)
    - Methods: `apply_config`, `shutdown`, `set_level`, `set_domain`, `set_level2sym`, `set_ext_config`, `add_logger`, `remove_logger`, `set_root_writer_config`, `set_root_writer`, `add_writer_config`, `add_writer`, `remove_writer`, `add_writer_configs`, `add_writers`, `remove_writers`, `enable`, `disable`, `enable_type`, `disable_type`, `sync`, `sync_all`, `rotate`, `set_encryption`, `set_debug`, `get_writer_config`, `get_writer_configs`, `get_server_config`, `get_server_configs`, `get_root_server_address_port`, `get_server_addresses_ports`, `get_server_addresses`, `get_server_ports`, `get_server_auth_key`, `get_config_string`, `save_config`, and all log methods (`trace`, `debug`, ...).
    - All log methods return int (0 = success, <0 = error).

## Root Logger API (root.hpp)
- C++ interface to the global root logger singleton.
    - Functions: `root_init`, `root_shutdown`, `root_set_level`, `root_set_domain`, `root_set_level2sym`, `root_set_ext_config`, `root_add_logger`, `root_remove_logger`, `root_set_root_writer_config`, `root_set_root_writer`, `root_add_writer_config`, `root_add_writer`, `root_remove_writer`, `root_add_writer_configs`, `root_add_writers`, `root_remove_writers`, `root_enable`, `root_disable`, `root_enable_type`, `root_disable_type`, `root_sync`, `root_sync_all`, `root_rotate`, `root_set_encryption`, `root_set_debug`, `root_get_writer_config`, `root_get_writer_configs`, `root_get_server_config`, `root_get_server_configs`, `root_get_root_server_address_port`, `root_get_server_addresses_ports`, `root_get_server_addresses`, `root_get_server_ports`, `root_get_server_auth_key`, `root_get_config_string`, `root_save_config`, and all log methods (`root_trace`, `root_debug`, ...).

## C++ Entrypoint and FFI Helpers (cppfastlogging.hpp)
- Includes all main headers.
- FFI helpers: `create_key`, `create_random_key` for encryption.

---

## Usage Example

```cpp
#include "cppfastlogging.hpp"

int main() {
    logging::Logger logger(logging::DEBUG, "example");
    logger.info("Hello from cppfastlogging!");
    return 0;
}
```

---

For detailed API reference, see the header files in `cppfastlogging/h/`.
