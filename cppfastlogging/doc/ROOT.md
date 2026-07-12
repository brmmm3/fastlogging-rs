# Root Logger API

The Root Logger is a **global singleton** that provides default logging
configuration for the entire process. Unlike the `logging::Logging` and
`logging::Logger` classes, which are C++ RAII wrappers in the `logging::`
namespace, the Root Logger is exposed **only as C-style free functions**
declared `extern "C"` in `h/root.hpp`.

There is no `Root` class. You call the functions directly.

## How to use

1. Call `root_init()` once, before any other root logger function.
2. Configure the root logger (domain, writers, levels, encryption, …).
3. Log messages via `root_trace`, `root_info`, … , `root_exception`.
4. Call `root_shutdown(0)` (graceful) or `root_shutdown(1)` (immediate) when
   you are done.

```cpp
#include "h/cppfastlogging.hpp"

int main() {
    root_init();
    root_set_domain("example");
    root_info("Hello from the root logger!");
    root_shutdown(0);
    return 0;
}
```

## Log level constants

Defined in `h/def.hpp` (global namespace):

| Constant    | Value |
|-------------|-------|
| `NOLOG`     | 100   |
| `EXCEPTION` | 60    |
| `CRITICAL`  | 50    |
| `FATAL`     | 50    |
| `ERROR`     | 40    |
| `WARNING`   | 30    |
| `WARN`      | 30    |
| `SUCCESS`   | 25    |
| `INFO`      | 20    |
| `DEBUG`     | 10    |
| `TRACE`     | 5     |
| `NOTSET`    | 0     |

## Initialization and Shutdown

| Signature | Description |
|-----------|-------------|
| `void root_init()` | Initialize the root logger. Call before any other root function. |
| `int root_shutdown(int8_t now)` | Shut down the root logger. If `now` is nonzero the shutdown is immediate; otherwise it is graceful (flushes pending messages). Returns a status code. |

## Configuration

| Signature | Description |
|-----------|-------------|
| `int root_set_level(uint32_t wid, uint8_t level)` | Set the log level of writer `wid`. Pass `0` for the root/default writer. Returns a status code. |
| `void root_set_domain(const char *domain)` | Set the global log domain (string, e.g. `"myapp"`). |
| `void root_set_level2sym(uint8_t level2sym)` | Set how log levels are rendered in output (symbol, short string, or full string). See `rust::LevelSyms`. |
| `void root_set_ext_config(rust::ExtConfig *ext_config)` | Set extended configuration (structured messages, hostname, process name, pid, thread name, thread id). Pass the raw `rust::ExtConfig *` from `logging::ExtConfig::config`. |
| `void root_set_debug(uint32_t debug)` | Enable internal debug output of the logging library itself. |

## Writer Management

| Signature | Description |
|-----------|-------------|
| `int root_set_root_writer_config(rust::WriterConfigEnum *config)` | Set the root writer configuration. The config **must** be a `ClientWriterConfig` or `ServerConfig`. Returns a status code. |
| `int root_add_writer_config(rust::WriterConfigEnum *config)` | Add a writer configuration to the root logger. Returns a writer id (`wid`). |
| `int root_remove_writer(uint32_t wid)` | Remove the writer identified by `wid`. |
| `int root_enable(uint32_t wid)` | Enable the writer `wid`. |
| `int root_disable(uint32_t wid)` | Disable the writer `wid`. |
| `int root_enable_type(rust::WriterTypeEnum typ)` | Enable all writers of the given type (e.g. `rust::WriterTypeEnum::Console`). |
| `int root_disable_type(rust::WriterTypeEnum typ)` | Disable all writers of the given type. |

### Passing writer configs to the root API

The root functions accept the raw `rust::WriterConfigEnum *` pointer, not the
`logging::WriterConfig` C++ wrapper class. Construct a wrapper class as usual
and pass its public `config` field:

```cpp
ConsoleWriterConfig cfg(DEBUG, true);
root_add_writer_config(cfg.config);

ServerConfig srv(DEBUG, "127.0.0.1");
root_set_root_writer_config(srv.config);
```

The same applies to `FileWriterConfig`, `ClientWriterConfig`,
`SyslogWriterConfig`, and `CallbackWriterConfig`.

## Logger Management

| Signature | Description |
|-----------|-------------|
| `void root_add_logger(rust::Logger *logger)` | Register a logger with the root logger. |
| `void root_remove_logger(rust::Logger *logger)` | Unregister a logger. |

### Passing `Logger` objects to the root API

The root functions accept the raw `rust::Logger *` pointer. Construct a
`logging::Logger` and pass its `raw()` method:

```cpp
Logger logger(DEBUG, "worker");
root_add_logger(logger.raw());
```

## Synchronization and Rotation

| Signature | Description |
|-----------|-------------|
| `int root_sync(rust::WriterTypeEnum *types, uint32_t type_cnt, double timeout)` | Sync (flush) writers of the given types. `types` is a C array of length `type_cnt`; `timeout` is in seconds. |
| `int root_sync_all(double timeout)` | Sync all writers. `timeout` is in seconds. |
| `int root_rotate(const char *path)` | Rotate the file writer writing to `path`. |

## Encryption

| Signature | Description |
|-----------|-------------|
| `int root_set_encryption(uint32_t wid, const rust::KeyStruct *key)` | Set the encryption key for writer `wid`. The `key` struct specifies the encryption method, key length, and key bytes. |

## Query Methods

| Signature | Description |
|-----------|-------------|
| `rust::WriterConfigEnum *root_get_writer_config(uint32_t wid)` | Get the configuration of writer `wid`. |
| `rust::ServerConfig *root_get_server_config(uint32_t wid)` | Get the server config for writer `wid`. |
| `rust::ServerConfigs *root_get_server_configs()` | Get all server configs. |
| `const char *root_get_root_server_address_port()` | Get the root server's `"address:port"` string. |
| `const rust::Cu32StringVec *root_get_server_addresses_ports()` | Get all server `"address:port"` strings. |
| `const rust::Cu32StringVec *root_get_server_addresses()` | Get all server addresses. |
| `const rust::Cu32u16Vec *root_get_server_ports()` | Get all server ports. |
| `rust::KeyStruct *root_get_server_auth_key()` | Get the server authentication key. |
| `const char *root_get_config_string()` | Get the current configuration as a string. |
| `int root_save_config(const char *path)` | Save the current configuration to a file at `path`. |

The `Cu32StringVec` and `Cu32u16Vec` structs (defined in `h/def.hpp`) are
parallel arrays keyed by writer id:

```c
typedef struct Cu32StringVec {
    uint32_t  cnt;
    uint32_t *keys;
    char    **values;
} Cu32StringVec;

typedef struct Cu32u16Vec {
    uint32_t  cnt;
    uint32_t *keys;
    uint16_t *values;
} Cu32u16Vec;
```

Iterate `cnt` entries, reading `keys[i]` (writer id) and `values[i]` (the
data).

## Log Methods

All of these take `const char *message` and return `int` (a status /
message-id code).

| Function | Level |
|----------|-------|
| `root_trace(const char *message)` | `TRACE` (5) |
| `root_debug(const char *message)` | `DEBUG` (10) |
| `root_info(const char *message)` | `INFO` (20) |
| `root_success(const char *message)` | `SUCCESS` (25) |
| `root_warning(const char *message)` | `WARNING` (30) |
| `root_error(const char *message)` | `ERROR` (40) |
| `root_critical(const char *message)` | `CRITICAL` (50) |
| `root_fatal(const char *message)` | `FATAL` (50) |
| `root_exception(const char *message)` | `EXCEPTION` (60) |

## Usage examples

### Minimal

```cpp
#include "h/cppfastlogging.hpp"

int main() {
    root_init();
    root_set_domain("example");
    root_info("Hello from the root logger!");
    root_shutdown(0);
    return 0;
}
```

### Adding a console writer

```cpp
#include "h/cppfastlogging.hpp"

using namespace logging;

int main() {
    root_init();
    root_set_domain("demo");

    ConsoleWriterConfig cfg(DEBUG, true);
    root_add_writer_config(cfg.config);

    root_info("Info via root logger");
    root_shutdown(0);
    return 0;
}
```

### Registering a `Logger` with the root logger

```cpp
#include "h/cppfastlogging.hpp"

using namespace logging;

int main() {
    root_init();
    root_set_domain("demo");

    Logger logger(DEBUG, "worker");
    root_add_logger(logger.raw());

    logger.info("Info from a registered logger");
    root_info("Info from the root logger");

    root_remove_logger(logger.raw());
    root_shutdown(0);
    return 0;
}
```

### Network server + client via root

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main() {
    root_init();
    root_set_domain("LOGSRV");

    ConsoleWriterConfig console(DEBUG, true);
    root_add_writer_config(console.config);

    ServerConfig srv(DEBUG, "127.0.0.1");
    root_add_writer_config(srv.config);
    root_set_root_writer_config(srv.config);
    root_sync_all(5.0);

    const char *address_port = root_get_root_server_address_port();
    rust::KeyStruct *key = root_get_server_auth_key();

    // Client side (could be a different process)
    root_set_domain("LOGCLIENT");
    ClientWriterConfig client(DEBUG, address_port, key);
    root_add_writer_config(client.config);

    root_info("Info from client via root");
    root_sync_all(1.0);
    root_shutdown(0);
    return 0;
}
```

## Root logger vs. `logging::Logging`

| Aspect | Root Logger | `logging::Logging` |
|--------|-------------|---------------------|
| Style | C-style free functions (`extern "C"`) | C++ RAII class in `namespace logging` |
| Lifetime | Global singleton; explicit `root_init()` / `root_shutdown()` | Construct/destruct instances; destructor calls `logging_shutdown` automatically |
| Ownership of writers | You pass raw `rust::WriterConfigEnum *` pointers | `add_writer_config()` accepts `WriterConfig&` / `WriterConfig&&` and forwards `.config` |
| Number of instances | Exactly one per process | As many as you need |
| Thread safety | Shared global state | Each instance is independent |
| Use when… | You want a single process-wide logger without managing instances, or you're integrating with C code | You want scoped, RAII-managed logging; multiple independent logger hierarchies; or you prefer C++ ergonomics |

In particular, `logging::Logging`'s destructor automatically calls
`logging_shutdown`, so you do not need an explicit shutdown call. The root
logger has no destructor, so you must call `root_shutdown()` yourself.
