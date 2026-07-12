# cppfastlogging C++ API Reference

This document is a concise reference for the main C++ API. For detailed
documentation, see the individual topic files.

## def.hpp — Core Types and Enums

### Log Level Constants (global namespace)

`NOLOG=100`, `EXCEPTION=60`, `CRITICAL=50`, `FATAL=50` (alias),
`ERROR=40`, `WARNING=30`, `WARN=30` (alias), `SUCCESS=25`, `INFO=20`,
`DEBUG=10`, `TRACE=5`, `NOTSET=0`.

### Enums (`rust::` namespace)

| Enum | Values |
|---|---|
| `LevelSyms` | `Sym=0`, `Short=1`, `Str=2` |
| `CompressionMethodEnum` | `Store=0`, `Deflate=1`, `Zstd=2`, `Lzma=3` |
| `WriterTypeEnum` | `Root=0`, `Console=1`, `File=2`, `Files=3`, `Client=4`, `Clients=5`, `Server=6`, `Servers=7`, `Syslog=8` |
| `MessageStructEnum` | `String=0`, `Json=1`, `Xml=2` |
| `EncryptionMethodEnum` | `NONE=0`, `AuthKey=1`, `AES=2` |

### Structs (`rust::` namespace)

- `ExtConfig { MessageStructEnum structured; int8_t hostname, pname, pid, tname, tid; }`
- `KeyStruct { EncryptionMethodEnum typ; uint32_t len; const char *key; }`
- `ServerConfig { uint8_t level; const char *address; uint16_t port; KeyStruct *key; const char *port_file; }`
- `ServerConfigs { uint32_t cnt; uint32_t *keys; ServerConfig *values; }`
- `Cu32StringVec { uint32_t cnt; uint32_t *keys; char **values; }`
- `Cu32u16Vec { uint32_t cnt; uint32_t *keys; uint16_t *values; }`
- `WriterEnums { uint32_t cnt; WriterEnum *values; }`

### Type Aliases (global namespace)

`using Cu32StringVec_t = rust::Cu32StringVec;`
`using Cu32u16Vec_t = rust::Cu32u16Vec;`

### Opaque Handle Types

`rust::WriterConfigEnum`, `rust::WriterEnum`, `rust::Logging`, `rust::Logger`
(used via pointer only).

---

## writer.hpp — Writer Config Classes

### `CompressionMethod` Enum (global namespace)

`Store=0`, `Deflate=1`, `Zstd=2`, `Lzma=3`.

### Writer Config Classes (global namespace, inherit from `WriterConfig`)

| Class | Constructor |
|---|---|
| `WriterConfig` | Base class; holds `rust::WriterConfigEnum *config` |
| `ConsoleWriterConfig` | `(uint8_t level, bool colors = false)` |
| `FileWriterConfig` | `(uint8_t level, const char *path, uint32_t size = 0, uint32_t backlog = 0, int32_t timeout = -1, int64_t time = -1, CompressionMethod compression = CompressionMethod::Store)` |
| `ClientWriterConfig` | `(uint8_t level, const char *address, const rust::KeyStruct *key = nullptr)` |
| `ServerConfig` | `(uint8_t level, const char *address, const rust::KeyStruct *key = nullptr)` |
| `SyslogWriterConfig` | `(uint8_t level, const char *hostname = nullptr, const char *pname = nullptr, uint32_t pid = 0)` |
| `CallbackWriterConfig` | `(uint8_t level, void (*callback)(uint8_t, const char *, const char *))` |

Ownership: the `config` pointer is transferred to the `Logging` instance via
`add_writer_config`. The destructor does not free it.

---

## logging.hpp — `logging::Logging` and `logging::ExtConfig`

### `logging::MessageStruct` Enum

`String=0`, `Json=1`, `Xml=2`.

### `logging::ExtConfig`

```cpp
ExtConfig(MessageStruct structured, int8_t hostname, int8_t pname,
          int8_t pid, int8_t tname, int8_t tid);
```

Has public `rust::ExtConfig *config` field.

### `logging::Logging` (RAII, move-only)

**Constructors:**

- `explicit Logging(uint8_t level = NOTSET, const char *domain = nullptr, ExtConfig *ext_config = nullptr, const char *config_path = nullptr)`
- `Logging(uint8_t level, const char *domain, WriterConfig (&configs)[N], ExtConfig *ext_config = nullptr, const char *config_path = nullptr)` (template, N deduced)
- `static Logging Default()`

Destructor calls `shutdown(false)` automatically.

**Methods:**

| Category | Methods |
|---|---|
| Lifecycle | `shutdown(bool now)`, `apply_config(const char *path)` |
| Configuration | `set_level(uint32_t wid, uint8_t level)`, `set_domain(const char *domain)`, `set_level2sym(uint8_t level2sym)`, `set_ext_config(ExtConfig *)`, `set_debug(uint32_t debug)` |
| Logger Management | `add_logger(Logger &/*)`, `remove_logger(Logger &/*)` |
| Writer Management | `add_writer_config(WriterConfig &/*/*&&)`, `set_root_writer_config(WriterConfig &/*)`, `remove_writer(uint32_t wid)`, `enable(uint32_t wid)`, `disable(uint32_t wid)`, `enable_type(rust::WriterTypeEnum)`, `disable_type(rust::WriterTypeEnum)` |
| Sync & Rotation | `sync(rust::WriterTypeEnum *, uint32_t cnt, double timeout)`, `sync_all(double timeout)`, `rotate(const char *path)` |
| Encryption | `set_encryption(rust::WriterTypeEnum writer, const rust::KeyStruct *key)` |
| Queries | `get_server_config(uint32_t wid = 0)`, `get_server_configs()`, `get_root_server_address_port()`, `get_server_addresses_ports()`, `get_server_addresses()`, `get_server_ports()`, `get_server_auth_key()`, `get_config_string()`, `save_config(const char *path)` |
| Log Methods | `trace`, `debug`, `info`, `success`, `warn`, `warning`, `error`, `critical`, `fatal`, `exception` (all `const`, take `const std::string &`, return `int`) |
| FFI | `rust::Logging *raw() const` |

---

## logger.hpp — `logging::Logger` (RAII, move-only)

**Constructors:**

- `Logger(uint8_t level, const char *domain)`
- `Logger(uint8_t level, const char *domain, int8_t tname, int8_t tid)`

**Methods:**

- `set_level(uint8_t level)`, `set_domain(const char *domain)`
- Log methods: `trace`, `debug`, `info`, `success`, `warn`, `warning`, `error`, `critical`, `fatal`, `exception` (all `const`, take `const std::string &`, return `int`)
- `rust::Logger *raw() const`

Must be registered with a `Logging` instance via `add_logger` to be used.

---

## root.hpp — Root Logger (C-style functions, no C++ wrapper)

**Initialization:** `root_init()`, `root_shutdown(int8_t now)`

**Configuration:** `root_set_level(uint32_t wid, uint8_t level)`, `root_set_domain(const char *)`, `root_set_level2sym(uint8_t)`, `root_set_ext_config(rust::ExtConfig *)`, `root_set_debug(uint32_t)`

**Writer Management:** `root_set_root_writer_config(rust::WriterConfigEnum *)`, `root_add_writer_config(rust::WriterConfigEnum *)`, `root_remove_writer(uint32_t wid)`, `root_enable(uint32_t wid)`, `root_disable(uint32_t wid)`, `root_enable_type(rust::WriterTypeEnum)`, `root_disable_type(rust::WriterTypeEnum)`

**Logger Management:** `root_add_logger(rust::Logger *)`, `root_remove_logger(rust::Logger *)`

**Sync & Rotation:** `root_sync(rust::WriterTypeEnum *, uint32_t, double)`, `root_sync_all(double)`, `root_rotate(const char *)`

**Encryption:** `root_set_encryption(uint32_t wid, const rust::KeyStruct *key)`

**Queries:** `root_get_writer_config(uint32_t wid)`, `root_get_server_config(uint32_t wid)`, `root_get_server_configs()`, `root_get_root_server_address_port()`, `root_get_server_addresses_ports()`, `root_get_server_addresses()`, `root_get_server_ports()`, `root_get_server_auth_key()`, `root_get_config_string()`, `root_save_config(const char *)`

**Log Methods:** `root_trace`, `root_debug`, `root_info`, `root_success`, `root_warning`, `root_error`, `root_critical`, `root_fatal`, `root_exception` (all take `const char *`, return `int`)

> **Note:** The root API accepts raw pointers. Pass `cfg.config` from writer
> config classes and `logger.raw()` from `Logger` instances.

---

## cppfastlogging.hpp — FFI Helpers

```cpp
rust::KeyStruct *create_key(rust::EncryptionMethodEnum typ, uint32_t len, const uint8_t *key);
rust::KeyStruct *create_random_key(rust::EncryptionMethodEnum typ);
```

---

## Minimal Usage Example

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.info("Hello from cppfastlogging!");
    return 0;  // destructor calls shutdown(false)
}
```

For detailed documentation, see the individual topic files and
[EXAMPLES.md](EXAMPLES.md).
