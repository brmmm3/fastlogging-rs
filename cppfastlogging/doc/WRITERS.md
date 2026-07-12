# cppfastlogging: Writer Configuration Documentation

## Overview

cppfastlogging uses an asynchronous writer model. Every `Logging` instance owns a background dispatch thread that fans log messages out to one or more **writers**. Writers are added to a `Logging` instance via writer *config* objects; the underlying Rust runtime then constructs the actual writer and takes ownership of the config.

A single `Logging` instance may have any number of writers attached. Each writer independently filters by log level (set per-writer at config time) and formats/delivers the message through its own backend (console, file, syslog, network, callback, ...).

### Log Levels

All writer configs take a `uint8_t level` argument that sets the minimum severity the writer will emit. Messages with a level strictly less than the writer's level are dropped. The level constants (defined in `def.hpp`, global namespace) are:

| Constant     | Value | Notes                         |
|--------------|-------|-------------------------------|
| `NOLOG`      | 100   | Suppress all logging          |
| `EXCEPTION`  | 60    |                               |
| `CRITICAL`   | 50    |                               |
| `FATAL`      | 50    | Alias for `CRITICAL`          |
| `ERROR`      | 40    |                               |
| `WARNING`    | 30    |                               |
| `WARN`       | 30    | Alias for `WARNING`           |
| `SUCCESS`    | 25    |                               |
| `INFO`       | 20    |                               |
| `DEBUG`      | 10    |                               |
| `TRACE`      | 5     |                               |
| `NOTSET`     | 0     | Log everything                |

---

## The `WriterConfig` Base Class and Ownership Model

All writer config classes inherit from `WriterConfig` (defined in `writer.hpp`):

```cpp
class WriterConfig {
public:
    rust::WriterConfigEnum *config = nullptr;
    WriterConfig() = default;
    WriterConfig(const WriterConfig &) = default;
    WriterConfig &operator=(const WriterConfig &) = default;
    virtual ~WriterConfig() = default;
};
```

The public `config` field is an opaque handle (`rust::WriterConfigEnum *`) created by the C FFI (`console_writer_config_new`, `file_writer_config_new`, ...). The handle is **owned by the caller** only until it is passed to `Logging::add_writer_config` (or one of its overloads), at which point **the Rust side takes ownership**.

Because of this ownership transfer:

- The `WriterConfig` destructor does **not** free the `config` pointer. Rust owns and frees it after `add_writer_config` is called.
- A given config object should be consumed by `add_writer_config` **exactly once**. Re-using a consumed `config` pointer is undefined behavior.
- It is safe to create a config as a temporary and pass it to `add_writer_config` in the same expression, e.g. `logging.add_writer_config(ConsoleWriterConfig(DEBUG));`.

### `add_writer_config` Overloads

`logging::Logging` provides three overloads for adding a writer:

```cpp
int add_writer_config(WriterConfig &config);   // lvalue: transfer this config
int add_writer_config(WriterConfig &&config);   // rvalue/temporary convenience
int add_writer_config(WriterConfig *config);    // pointer (nullptr-safe, returns -1)
```

All three forward the inner `config` pointer to the Rust `logging_add_writer_config` FFI. The return value is the new **writer id** (`wid`) on success, or a negative value on error. The `wid` can later be used with `set_level(wid, ...)`, `enable(wid)`, `disable(wid)`, `remove_writer(wid)`, etc.

### Adding Multiple Writers

Writers are added one-by-one, or in a batch via the array constructor overload:

```cpp
// One-by-one
Logging logging(DEBUG, "root");
logging.add_writer_config(ConsoleWriterConfig(DEBUG));
logging.add_writer_config(FileWriterConfig(DEBUG, "/tmp/app.log"));

// Batch via the array constructor (N is deduced)
WriterConfig configs[] = {
    ConsoleWriterConfig(DEBUG),
    FileWriterConfig(DEBUG, "/tmp/app.log")
};
Logging logging2(DEBUG, "root", configs);
```

---

## `CompressionMethod` Enum

Used by `FileWriterConfig` to select how rotated log files are compressed. Defined in `writer.hpp` in the global namespace:

```cpp
enum class CompressionMethod : uint8_t {
    Store   = 0,
    Deflate = 1,
    Zstd    = 2,
    Lzma    = 3
};
```

| Value      | Integer | Description                                  |
|------------|---------|----------------------------------------------|
| `Store`    | 0       | No compression (stored as-is). Default.      |
| `Deflate`  | 1       | DEFLATE/zlib compression.                    |
| `Zstd`     | 2       | Zstandard compression.                       |
| `Lzma`     | 3       | LZMA/XZ compression.                         |

This mirrors `rust::CompressionMethodEnum` in `def.hpp`. The `FileWriterConfig` constructor casts the C++ enum to the Rust enum internally.

---

## `ConsoleWriterConfig`

Writes log messages to the console (stdout). Defined in `writer.hpp`:

```cpp
class ConsoleWriterConfig : public WriterConfig {
public:
    ConsoleWriterConfig(uint8_t level, bool colors = false);
};
```

### Parameters

| Parameter | Type      | Default | Description                                  |
|-----------|-----------|---------|----------------------------------------------|
| `level`   | `uint8_t` | —       | Minimum log level for this writer.           |
| `colors`  | `bool`    | `false` | Enable ANSI color codes in console output.   |

### Example

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true)); // colored
    logging.info("Hello from the console writer!");
    return 0;
}
```

---

## `FileWriterConfig`

Writes log messages to a file, with optional size- and time-based rotation, a backlog of backup files, and compression of rotated files. Defined in `writer.hpp`:

```cpp
class FileWriterConfig : public WriterConfig {
public:
    FileWriterConfig(uint8_t level, const char *path, uint32_t size = 0,
                     uint32_t backlog = 0, int32_t timeout = -1,
                     int64_t time = -1,
                     CompressionMethod compression = CompressionMethod::Store);
};
```

### Parameters

| Parameter      | Type                | Default                    | Description                                                                 |
|----------------|---------------------|----------------------------|-----------------------------------------------------------------------------|
| `level`        | `uint8_t`           | —                          | Minimum log level for this writer.                                          |
| `path`         | `const char *`      | —                          | Path to the log file.                                                       |
| `size`         | `uint32_t`          | `0`                        | Max file size in bytes before rotation. `0` = no size-based rotation.       |
| `backlog`      | `uint32_t`          | `0`                        | Number of rotated backup files to keep.                                     |
| `timeout`      | `int32_t`           | `-1`                       | Sync timeout in seconds. `-1` = use library default.                        |
| `time`         | `int64_t`           | `-1`                       | Time-based rotation interval in seconds. `-1` = disabled.                   |
| `compression`  | `CompressionMethod` | `CompressionMethod::Store` | Compression method applied to rotated files (see table above).              |

### Rotation Behavior

- **Size-based rotation:** When `size > 0` and the current log file grows beyond `size` bytes, the file is rotated. The current file is moved into the backlog and a new file is opened at `path`.
- **Time-based rotation:** When `time > 0` (in seconds), the file is rotated at that interval regardless of size.
- **Backlog:** `backlog` controls how many rotated backup files are retained. Older backups beyond this count are deleted. Backup files may be compressed according to `compression`.
- **Timeout:** `timeout` is the flush/sync timeout used by the underlying writer. Use `-1` to accept the library default.

### Example

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging(DEBUG, "root");
    // Rotate at 1024 bytes, keep 3 backups, no compression
    logging.add_writer_config(FileWriterConfig(DEBUG, "/tmp/cppfastlogging.log", 1024, 3));
    logging.info("Hello from the file writer!");
    return 0;
}
```

### Example with Compression and Time-Based Rotation

```cpp
Logging logging(DEBUG, "root");
logging.add_writer_config(
    FileWriterConfig(DEBUG, "/tmp/app.log",
                     1024 * 1024,   // size: 1 MiB
                     5,             // backlog: keep 5 rotated files
                     -1,            // timeout: default
                     3600,          // time: rotate every hour
                     CompressionMethod::Zstd));
```

---

## `SyslogWriterConfig` (Unix only)

Writes log messages to the system syslog. Only available on Unix-like systems. Defined in `writer.hpp`:

```cpp
class SyslogWriterConfig : public WriterConfig {
public:
    SyslogWriterConfig(uint8_t level, const char *hostname = nullptr,
                       const char *pname = nullptr, uint32_t pid = 0);
};
```

### Parameters

| Parameter   | Type           | Default | Description                                              |
|-------------|----------------|---------|----------------------------------------------------------|
| `level`     | `uint8_t`      | —       | Minimum log level for this writer.                       |
| `hostname`  | `const char *` | `nullptr` | Hostname identifying the sender. `nullptr` = omit/auto. |
| `pname`     | `const char *` | `nullptr` | Process name identifying the sender. `nullptr` = omit/auto. |
| `pid`       | `uint32_t`     | `0`     | Process ID. `0` = omit/auto.                             |

### Example

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(SyslogWriterConfig(DEBUG, "hostname", "pname", 1234));
    logging.info("Hello from syslog!");
    return 0;
}
```

---

## `CallbackWriterConfig`

Invokes a user-supplied C function for each log message that passes the writer's level filter. This allows fully custom processing of log messages (forwarding to a third-party system, in-memory capture for tests, etc.). Defined in `writer.hpp`:

```cpp
class CallbackWriterConfig : public WriterConfig {
public:
    CallbackWriterConfig(uint8_t level,
                         void (*callback)(uint8_t, const char *, const char *));
};
```

### Parameters

| Parameter  | Type     | Description                                      |
|------------|----------|--------------------------------------------------|
| `level`    | `uint8_t` | Minimum log level for this writer.              |
| `callback` | `void (*)(uint8_t, const char *, const char *)` | Function pointer invoked per message. |

### Callback Signature

```cpp
void callback(uint8_t level, const char *domain, const char *message);
```

- `level`   — the numeric log level of the message (see the log level table above).
- `domain`  — the log domain string.
- `message` — the formatted log message string.

The callback is invoked on the writer's background dispatch thread. It must be thread-safe and must not block for long periods, as this would stall all writers on the same `Logging` instance.

### Example

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>
using namespace logging;

void writer_callback(uint8_t level, const char *domain, const char *message) {
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

int main() {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(CallbackWriterConfig(DEBUG, writer_callback));
    logging.info("Hello from the callback writer!");
    return 0;
}
```

---

## Related Types

### `rust::WriterTypeEnum`

Identifies the *kind* of writer. Used by `enable_type`, `disable_type`, `set_encryption`, and `sync`. Defined in `def.hpp`:

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

Note: there is no separate `WriterTypeEnum` value for the callback writer; callback writers are managed individually by `wid`.

---

## See Also

- [NETWORK.md](NETWORK.md) — documentation for `ClientWriterConfig` and `ServerConfig` (network logging).
- [CONFIG.md](CONFIG.md) — extended configuration (`ExtConfig`), config files, and encryption.
- [LOGGING.md](LOGGING.md) — full reference for the `Logging` class.
- Header files in `cppfastlogging/h/`.
