# Root Logger API Documentation

This document describes the C++ Root Logger API provided by cppfastlogging (root.hpp). The Root Logger is a global singleton that manages the default logging configuration, writers, and log output for the entire application. It is suitable for applications that do not require multiple independent loggers or want to configure logging globally.

---

## Overview
- The Root Logger provides a set of C-style functions for global logging configuration and output.
- It manages writers, log levels, domains, encryption, and server/network settings.
- All log messages sent via the Root Logger are handled according to the current global configuration.

---

## Initialization and Shutdown
- `void root_init();`  
  Initialize the root logger. Call before any other root logger functions.
- `int root_shutdown(int8_t now);`  
  Shutdown the root logger. If `now` is nonzero, shutdown is immediate.

---

## Configuration
- `int root_set_level(uint32_t wid, uint8_t level);`  
  Set log level for a writer.
- `void root_set_domain(const char *domain);`  
  Set the global log domain.
- `void root_set_level2sym(uint8_t level2sym);`  
  Enable/disable symbolic log levels.
- `void root_set_ext_config(rust::ExtConfig *ext_config);`  
  Set extended configuration.
- `void root_set_debug(uint32_t debug);`  
  Enable debug output (nonzero = on).

---

## Writer Management
- `int root_set_root_writer_config(rust::WriterConfigEnum config);`  
  Set the root writer configuration.
- `int root_set_root_writer(rust::WriterEnum writer);`  
  Set the root writer type.
- `int root_add_writer_config(rust::WriterConfigEnum config);`  
  Add a writer configuration.
- `int root_add_writer(rust::WriterEnum writer);`  
  Add a writer.
- `int root_remove_writer(uint32_t wid);`  
  Remove a writer by ID.
- `int root_add_writer_configs(rust::WriterConfigEnum **configs, uint32_t config_cnt);`  
  Add multiple writer configs.
- `int root_add_writers(rust::WriterEnum **writers, uint32_t writer_cnt);`  
  Add multiple writers.
- `int root_remove_writers(uint32_t *wids, uint32_t wid_cnt);`  
  Remove multiple writers by ID.
- `int root_enable(uint32_t wid);`  
  Enable a writer.
- `int root_disable(uint32_t wid);`  
  Disable a writer.
- `int root_enable_type(rust::WriterTypeEnum typ);`  
  Enable all writers of a type.
- `int root_disable_type(rust::WriterTypeEnum typ);`  
  Disable all writers of a type.

---

## Synchronization and Rotation
- `int root_sync(rust::WriterTypeEnum *types, uint32_t type_cnt, double timeout);`  
  Sync writers of given types.
- `int root_sync_all(double timeout);`  
  Sync all writers.
- `int root_rotate(const char *path);`  
  Rotate file writer at path.

---

## Encryption
- `int root_set_encryption(uint32_t wid, const rust::KeyStruct *key);`  
  Set encryption key for a writer.

---

## Server and Network
- `rust::WriterConfigEnum *root_get_writer_config(uint32_t wid);`  
  Get writer config by ID.
- `rust::WriterConfigEnum **root_get_writer_configs();`  
  Get all writer configs.
- `ServerConfig *root_get_server_config();`  
  Get server config.
- `ServerConfig **root_get_server_configs();`  
  Get all server configs.
- `const char *root_get_root_server_address_port();`  
  Get root server address:port.
- `Cu32StringVec *root_get_server_addresses_ports();`  
  Get all server addresses:ports.
- `Cu32StringVec *root_get_server_addresses();`  
  Get all server addresses.
- `Cu32u16Vec *root_get_server_ports();`  
  Get all server ports.
- `rust::KeyStruct *root_get_server_auth_key();`  
  Get server authentication key.

---

## Configuration Export/Import
- `const char *root_get_config_string();`  
  Get current config as a string.
- `int root_save_config(const char *path);`  
  Save config to file.

---

## Logging Methods
- `int root_trace(const char *message);`
- `int root_debug(const char *message);`
- `int root_info(const char *message);`
- `int root_success(const char *message);`
- `int root_warning(const char *message);`
- `int root_error(const char *message);`
- `int root_critical(const char *message);`
- `int root_fatal(const char *message);`
- `int root_exception(const char *message);`

All log methods return 0 on success, <0 on error.

---

## Usage Example

```cpp
#include "cppfastlogging.hpp"

int main() {
    root_init();
    root_set_domain("example");
    root_info("Hello from the root logger!");
    root_shutdown(0);
    return 0;
}
```

---

For detailed parameter types, see `root.hpp` and related headers.
