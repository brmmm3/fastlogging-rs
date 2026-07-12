# `logging::Logging` — Main Logging Entry Point

The `logging::Logging` class (declared in `cppfastlogging/h/logging.hpp`) is the main
entry point for multi-backend, high-performance logging in the C++ wrapper around the
Rust `fastlogging` library. A `Logging` instance manages log writers, configuration,
and dispatches log messages to all registered outputs via an asynchronous background
thread.

All log-level constants live in the global namespace (see `def.hpp`):

| Constant      | Value | Notes                        |
|---------------|-------|------------------------------|
| `NOLOG`       | 100   | Suppress all logging         |
| `EXCEPTION`   | 60    |                              |
| `CRITICAL`    | 50    |                              |
| `FATAL`       | 50    | Alias of `CRITICAL`          |
| `ERROR`       | 40    |                              |
| `WARNING`     | 30    |                              |
| `WARN`        | 30    | Alias of `WARNING`           |
| `SUCCESS`     | 25    |                              |
| `INFO`        | 20    |                              |
| `DEBUG`       | 10    |                              |
| `TRACE`       | 5     |                              |
| `NOTSET`      | 0     | Default; inherits everything |

## RAII Semantics

`Logging` is an RAII type:

- **Destructor** automatically calls `shutdown(false)`, i.e. it waits for pending
  messages to be flushed before tearing down the background thread. You therefore do
  not normally need to call `shutdown` yourself — it is only required if you want
  immediate (`now = true`) shutdown or need the return code.
- **Move-only.** Copy constructor and copy assignment are deleted. Move constructor
  and move assignment are defined and `noexcept`; after a move, the moved-from object
  holds a null handle and its destructor is a no-op.

## Constructors

### 1. Default-constructible form

```cpp
explicit Logging(uint8_t level   = NOTSET,
                 const char *domain       = nullptr,
                 ExtConfig  *ext_config   = nullptr,
                 const char *config_path  = nullptr)
```

Creates a `Logging` instance with **no writers**. Add writers afterwards with
`add_writer_config`.

- `level` — Root log level. `NOTSET` (0) means "inherit everything"; messages at or
  above this level are dispatched to writers.
- `domain` — Log domain string. If `nullptr`, defaults to `"root"`.
- `ext_config` — Optional pointer to a `logging::ExtConfig` controlling extended
  message formatting (message structure, hostname/process/thread fields). May be
  `nullptr`.
- `config_path` — Optional path to a configuration file applied at construction time.
  May be `nullptr`.

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.info("Hello, world!");
    return 0;  // destructor calls shutdown(false)
}
```

### 2. Template array constructor

```cpp
template <std::size_t N>
Logging(uint8_t level,
        const char *domain,
        WriterConfig (&configs)[N],
        ExtConfig  *ext_config  = nullptr,
        const char *config_path = nullptr)
```

Creates a `Logging` instance and immediately registers N writer configs supplied as a
C array. The template parameter `N` is deduced automatically from the array — you do
not need to specify it.

- `level`, `domain`, `ext_config`, `config_path` — same as above.
- `configs` — Reference to a C array of `WriterConfig` objects (e.g.
  `ConsoleWriterConfig`, `FileWriterConfig`, ...). Each config's ownership is
  transferred to the `Logging` instance.

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    WriterConfig configs[] = { ConsoleWriterConfig(DEBUG, true) };
    Logging logging(DEBUG, "root", configs);
    logging.info("Hello from the array constructor!");
    return 0;
}
```

### 3. `static Logging Default()`

```cpp
static Logging Default()
```

Returns a default-initialized `Logging` with a single console writer at `DEBUG` level.
This is the quickest way to get a working logger.

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging = Logging::Default();
    logging.trace("Trace Message");
    logging.info("Info Message");
    return 0;
}
```

## Methods

Methods are grouped by category. Unless noted, integer return values are `0` on
success and `< 0` on error.

### Configuration

#### `int shutdown(bool now)`

Shut down the logging system.

- `now` — If `true`, shutdown is immediate and does **not** wait for pending messages
  to be flushed. If `false`, blocks until queued messages have been written.
- Returns `0` on success, `< 0` on error.
- Called automatically with `false` by the destructor; you only need to call this
  manually for immediate shutdown or to inspect the return code. After `shutdown`,
  further log calls are no-ops.

#### `int apply_config(const char *path)`

Load and apply a configuration file.

- `path` — Path to the configuration file.
- Returns `0` on success, `< 0` on error.

#### `int set_level(uint32_t wid, uint8_t level)`

Set the log level for a specific writer.

- `wid` — Writer ID. Pass `0` for the root/default writer; positive values are the
  IDs returned by `add_writer_config`.
- `level` — New log level (one of the global level constants).
- Returns `0` on success, `< 0` on error.

> **Note on `wid`:** Writer IDs are how the API addresses individual writers. The
> root/default writer is always `0`. When you call `add_writer_config`, it returns a
> positive `wid` that you can use with `set_level`, `enable`, `disable`,
> `remove_writer`, and `get_server_config`.

#### `void set_domain(const char *domain)`

Set the log domain string for all writers.

- `domain` — New domain string (e.g. `"my-app"`).

#### `void set_level2sym(uint8_t level2sym)`

Set the log-level symbol style used when formatting messages. See
`rust::LevelSyms` (e.g. `Sym`, `Short`, `Str`).

- `level2sym` — Symbol style value.

#### `void set_ext_config(ExtConfig *ext_config)`

Set the extended formatting configuration.

- `ext_config` — Pointer to a `logging::ExtConfig`. Must not be `nullptr`.

#### `void set_debug(uint32_t debug)`

Set the internal debug level for all writers. Useful for diagnostics from the
library itself.

- `debug` — Debug verbosity level.

### Logger Management

These methods register or unregister `logging::Logger` instances with this
`Logging` instance. A `Logger` must be registered before its log methods will be
dispatched by this `Logging` instance.

#### `void add_logger(Logger &logger)`

#### `void add_logger(Logger *logger)`

Register a `Logger`. The pointer overload is a no-op if `logger` is `nullptr`.

#### `void remove_logger(Logger &logger)`

#### `void remove_logger(Logger *logger)`

Unregister a `Logger`. The pointer overload is a no-op if `logger` is `nullptr`.

### Writer Management

#### `int add_writer_config(WriterConfig &config)`

Add a writer from a `WriterConfig`. Ownership of the config is transferred to the
`Logging` instance.

- Returns the new writer's `wid` (a positive integer) on success, `< 0` on error.

Overloads:

- `int add_writer_config(WriterConfig &&config)` — Convenience overload for
  temporaries, e.g. `logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));`.
- `int add_writer_config(WriterConfig *config)` — Pointer overload; returns `-1` if
  `config` is `nullptr`.

#### `int set_root_writer_config(WriterConfig &config)`

#### `int set_root_writer_config(WriterConfig *config)`

Set the **root writer** from a config. The config must be a Client or Server config.

- Returns `0` on success, `< 0` on error. The pointer overload returns `-1` on
  `nullptr`.

#### `void remove_writer(uint32_t wid)`

Remove a writer by ID.

- `wid` — Writer ID returned by `add_writer_config` (`0` for the root writer).

#### `int enable(uint32_t wid)` / `int disable(uint32_t wid)`

Enable or disable a single writer by ID.

- `wid` — Writer ID.
- Returns `0` on success, `< 0` on error.

#### `int enable_type(rust::WriterTypeEnum typ)` / `int disable_type(rust::WriterTypeEnum typ)`

Enable or disable **all** writers of a given type (e.g. all `Console` writers, all
`File` writers). See `rust::WriterTypeEnum` below.

- Returns `0` on success, `< 0` on error.

### Synchronization & Rotation

#### `int sync(rust::WriterTypeEnum *types, uint32_t cnt, double timeout)`

Synchronize specific writer types, waiting up to `timeout` seconds for them to flush.

- `types` — Pointer to an array of `rust::WriterTypeEnum` values.
- `cnt` — Number of entries in `types`.
- `timeout` — Maximum wait time in seconds.
- Returns `0` on success, `< 0` on error/timeout.

#### `int sync_all(double timeout)`

Synchronize **all** writers, waiting up to `timeout` seconds.

- `timeout` — Maximum wait time in seconds.
- Returns `0` on success, `< 0` on error/timeout.

#### `int rotate(const char *path)`

Rotate the log file(s) at the given path.

- `path` — Path of the log file to rotate.
- Returns `0` on success, `< 0` on error.

### Encryption

#### `int set_encryption(rust::WriterTypeEnum writer, const rust::KeyStruct *key)`

Set the encryption key for a network writer type (Client/Clients/Server/Servers).

- `writer` — The writer type to reconfigure.
- `key` — Pointer to a `rust::KeyStruct` describing the encryption method and key.
  May be `nullptr` to clear encryption.
- Returns `0` on success, `< 0` on error.

### Query Methods

#### `rust::ServerConfig *get_server_config(uint32_t wid = 0)`

Get the server config for a specific writer. Defaults to the root writer (`wid = 0`).

- Returns a pointer to a `rust::ServerConfig`, or `nullptr` if the writer is not a
  server writer.

#### `rust::ServerConfigs *get_server_configs()`

Get all server configs.

- Returns a pointer to a `rust::ServerConfigs` struct containing a count, keys, and
  values array.

#### `const char *get_root_server_address_port()`

Get the root server's `address:port` string.

#### `const rust::Cu32StringVec *get_server_addresses_ports()`

Get all server `address:port` strings.

#### `const rust::Cu32StringVec *get_server_addresses()`

Get all server address strings (without ports).

#### `const rust::Cu32u16Vec *get_server_ports()`

Get all server ports.

#### `rust::KeyStruct *get_server_auth_key()`

Get the server authentication key.

#### `const char *get_config_string()`

Get the current configuration serialized as a string.

#### `int save_config(const char *path)`

Save the current configuration to a file.

- `path` — Destination file path.
- Returns `0` on success, `< 0` on error.

### Log Methods

All log methods are `const`, take `const std::string &message`, and return `int`
(`0` on success, `< 0` on error). They dispatch the message to all registered
outputs asynchronously via the background thread.

| Method        | Level       |
|---------------|-------------|
| `trace`       | `TRACE`     |
| `debug`       | `DEBUG`     |
| `info`        | `INFO`      |
| `success`     | `SUCCESS`   |
| `warn`        | `WARNING`   |
| `warning`     | `WARNING`   |
| `error`       | `ERROR`     |
| `critical`    | `CRITICAL`  |
| `fatal`       | `FATAL`     |
| `exception`   | `EXCEPTION` |

`warn` and `warning` are aliases — they are identical.

```cpp
logging.trace("Trace Message");
logging.debug("Debug Message");
logging.info("Info Message");
logging.success("Success Message");
logging.warn("Warning Message");
logging.error("Error Message");
logging.fatal("Fatal Message");
```

### Advanced FFI

#### `rust::Logging *raw() const`

Returns the raw underlying FFI handle (`rust::Logging *`). This is intended for
advanced use cases where you need to call the C FFI functions in
`cppfastlogging/h/logging.hpp` directly. Normal application code should not need
this.

## `rust::` Helper Structs

These structs (defined in `cppfastlogging/h/def.hpp`) are used by the query methods:

### `rust::WriterTypeEnum`

```cpp
enum class WriterTypeEnum : uint8_t {
  Root    = 0,
  Console = 1,
  File    = 2,
  Files   = 3,
  Client  = 4,
  Clients = 5,
  Server  = 6,
  Servers = 7,
  Syslog  = 8
};
```

### `rust::ServerConfig`

```cpp
struct ServerConfig {
  uint8_t     level;
  const char *address;
  uint16_t    port;
  KeyStruct  *key;
  const char *port_file;
};
```

### `rust::ServerConfigs`

```cpp
struct ServerConfigs {
  uint32_t      cnt;
  uint32_t     *keys;
  ServerConfig *values;
};
```

### `rust::Cu32StringVec`

```cpp
struct Cu32StringVec {
  uint32_t  cnt;
  uint32_t *keys;
  char    **values;
};
```

### `rust::Cu32u16Vec`

```cpp
struct Cu32u16Vec {
  uint32_t  cnt;
  uint32_t *keys;
  uint16_t *values;
};
```

### `rust::KeyStruct`

```cpp
struct KeyStruct {
  EncryptionMethodEnum typ;
  uint32_t             len;
  const char          *key;
};
```

## Complete Usage Examples

### Console writer (added after construction)

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main(void) {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

### Default logging

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main(void) {
    Logging logging = Logging::Default();
    logging.trace("Trace Message");
    logging.info("Info Message");
    return 0;
}
```

### File writer

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main(void) {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(
        FileWriterConfig(DEBUG, "/tmp/cppfastlogging.log", 1024, 3));
    logging.info("Logging to a file");
    return 0;
}
```

### Array constructor

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main(void) {
    WriterConfig configs[] = { ConsoleWriterConfig(DEBUG, true) };
    Logging logging(DEBUG, "root", configs);
    logging.info("Hello from the array constructor!");
    return 0;
}
```

### Threads with `Logger` and `ExtConfig`

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>
#include <pthread.h>
using namespace logging;

void *loggerThreadFun(void *vargp) {
    Logger *logger = static_cast<Logger *>(vargp);
    logger->trace("Trace Message");
    logger->info("Info Message");
    logger->warning("Warning Message");
    logger->fatal("Fatal Message");
    return nullptr;
}

int main(void) {
    pthread_t thread_id;
    ExtConfig ext_config(MessageStruct::String, 1, 1, 1, 1, 1);
    WriterConfig configs[] = { ConsoleWriterConfig(DEBUG, true) };
    Logging *logging = new Logging(DEBUG, "root", configs, &ext_config);
    Logger  *logger  = new Logger(DEBUG, "LoggerThread", 1, 1);
    logging->add_logger(logger);
    pthread_create(&thread_id, nullptr, loggerThreadFun,
                   static_cast<void *>(logger));
    logging->info("Message from main thread");
    pthread_join(thread_id, nullptr);
    delete logging;
    delete logger;
    return 0;
}
```

---

For the underlying C FFI declarations, see `cppfastlogging/h/logging.hpp`.
For per-module/per-thread logging, see [LOGGER.md](LOGGER.md).
