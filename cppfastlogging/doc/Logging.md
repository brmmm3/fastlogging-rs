# cppfastlogging: Logger and Logging Class Documentation

## Logging Class

The `Logging` class (see logging.hpp) is the main entry point for multi-backend, high-performance logging. It manages log writers, configuration, and dispatches log messages to all registered outputs.

### Construction
- `logging_new_default()` — Create a default logging instance.
- `logging_new(uint8_t level, const char *domain, WriterConfigEnum *configs_ptr, uint32_t config_cnt, ExtConfig *ext_config, const char *config_path)` — Create a logging instance with custom settings.

### Methods
- `int apply_config(const char *path)` — Load and apply configuration from a file.
- `int shutdown(uint8_t now)` — Shutdown the logging system (waits for writers if `now` is true).
- `int set_level(uint8_t level)` — Set log level for all writers.
- `void set_domain(const char *domain)` — Set the log domain string for all writers.
- `void set_level2sym(uint8_t level2sym)` — Set log level symbol style.
- `void set_ext_config(ExtConfig *ext_config)` — Set extended formatting configuration.
- `void add_logger(Logger *logger)` — Register a Logger instance.
- `void remove_logger(Logger *logger)` — Unregister a Logger instance.
- `int set_root_writer_config(WriterConfigEnum *config)` — Set the root writer from a config.
- `int set_root_writer(WriterEnum writer)` — Set the root writer from an instance.
- `int add_writer_config(WriterConfigEnum *config)` — Add a writer from a config.
- `int add_writer(WriterEnum *writer)` — Add a writer from an instance.
- `void remove_writer(uint32_t wid)` — Remove a writer by id.
- `int add_writer_configs(WriterConfigEnum **configs, uint32_t config_cnt)` — Add multiple writers from configs.
- `int add_writers(WriterEnums *writers)` — Add multiple writer instances.
- `WriterEnums *remove_writers(uint32_t *wids, uint32_t wid_cnt)` — Remove multiple writers by id.
- `int enable(uint32_t wid)` — Enable a writer by id.
- `int disable(uint32_t wid)` — Disable a writer by id.
- `int enable_type(WriterTypeEnum typ)` — Enable all writers of a type.
- `int disable_type(WriterTypeEnum typ)` — Disable all writers of a type.
- `intptr_t sync(WriterTypeEnum *types, uint32_t type_cnt, double timeout)` — Synchronize specific writer types.
- `intptr_t sync_all(double timeout)` — Synchronize all writers.
- `intptr_t rotate(const char *path)` — Rotate log file(s).
- `intptr_t set_encryption(WriterTypeEnum writer, EncryptionMethodEnum encryption, char *key)` — Set encryption for a network writer.
- `void set_debug(uint32_t debug)` — Set debug level for all writers.
- `WriterConfigEnum *get_writer_config(uint32_t wid)` — Get config for a writer.
- `WriterConfigEnum **get_writer_configs()` — Get all writer configs.
- `ServerConfig *get_server_config()` — Get server config for a writer.
- `ServerConfig **get_server_configs()` — Get all server configs.
- `const char *get_root_server_address_port()` — Get root server address:port.
- `Cu32StringVec *get_server_addresses_ports()` — Get all server addresses:ports.
- `Cu32StringVec *get_server_addresses()` — Get all server addresses.
- `Cu32u16Vec *get_server_ports()` — Get all server ports.
- `KeyStruct *get_server_auth_key()` — Get server authentication key.
- `const char *get_config_string()` — Get config as string.
- `int save_config(const char *path)` — Save config to file.
- Logging methods: `trace`, `debug`, `info`, `success`, `warning`, `error`, `critical`, `fatal`, `exception` (all return int).

### Usage Example
```cpp
#include "cppfastlogging.hpp"

int main() {
    auto logging = logging_new_default();
    logging->info("Hello from Logging!");
    logging->shutdown(0);
    return 0;
}
```

---

For more details, see the header files in `cppfastlogging/h/`.
