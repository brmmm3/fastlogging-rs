# `logging::Logger` — Per-Module / Per-Thread Logger

The `logging::Logger` class (declared in `cppfastlogging/h/logger.hpp`) is a C++ RAII
wrapper for a per-module or per-thread logger instance. Unlike `Logging`, which owns
the writers and the background dispatch thread, a `Logger` is a lightweight handle
that must be **registered** with a `Logging` instance via `add_logger` before its log
methods will be dispatched.

A typical use case is creating one `Logger` per thread or per subsystem and sharing a
single `Logging` instance that owns the outputs.

## RAII Semantics

`Logger` is an RAII type:

- **Move-only.** Copy constructor and copy assignment are deleted. Move constructor
  and move assignment are defined and `noexcept`; after a move, the moved-from object
  holds a null handle.
- The destructor releases the underlying handle. The `Logger` should outlive any
  threads that use it; when sharing a `Logger` across threads, keep it alive (e.g.
  with `new`/`delete`) until all threads have joined and it has been removed from the
  `Logging` instance if applicable.

## Constructors

### 1. Basic constructor

```cpp
Logger(uint8_t level, const char *domain)
```

Create a logger with the given log level and domain.

- `level` — One of the global log-level constants (e.g. `DEBUG`, `INFO`). Messages
  below this level are filtered out **locally** before being dispatched (see note
  below).
- `domain` — Domain string for this logger (e.g. `"worker"`, `"db-layer"`).

```cpp
logging::Logger logger(logging::DEBUG, "worker");
```

### 2. Extended constructor with thread info

```cpp
Logger(uint8_t level, const char *domain, int8_t tname, int8_t tid)
```

Create a logger with optional thread name and thread id logging.

- `level` — Log level.
- `domain` — Domain string.
- `tname` — Set to `1` to include the thread name in log messages, `0` to omit.
- `tid` — Set to `1` to include the thread id in log messages, `0` to omit.

```cpp
logging::Logger logger(logging::DEBUG, "LoggerThread", 1, 1);
```

## Methods

### `void set_level(uint8_t level)`

Set the log level for this logger. Messages below `level` will be filtered out
locally before dispatch.

### `void set_domain(const char *domain)`

Set the domain string for this logger.

### Log methods

All log methods are `const`, take `const std::string &message`, and return `int`
(`0` on success, `< 0` on error):

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

> **Note on local level checking:** A `Logger` checks its own level **locally** before
> dispatching a message to the `Logging` instance. This means that if a `Logger` is
> set to `INFO`, calls to `debug`/`trace` on that logger return immediately without
> incurring the cost of crossing the FFI boundary or enqueueing the message. This
> makes per-logger level control an efficient way to suppress verbose output for
> specific subsystems or threads at runtime.

### `rust::Logger *raw() const`

Returns the raw underlying FFI handle (`rust::Logger *`). Intended for advanced use
cases that call the C FFI functions in `cppfastlogging/h/logger.hpp` directly.
Normal application code should not need this.

## Usage Examples

### Registered with a `Logging` instance

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main(void) {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));

    Logger logger(INFO, "worker");
    logging.add_logger(logger);

    logger.info("Logger started");
    logger.debug("This is below the logger's INFO level and is filtered locally");
    return 0;
}
```

### Shared across threads

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>
#include <pthread.h>
using namespace logging;

void *loggerThreadFun(void *vargp) {
    Logger *logger = static_cast<Logger *>(vargp);
    logger->trace("Trace Message");
    logger->debug("Debug Message");
    logger->info("Info Message");
    logger->success("Success Message");
    logger->warning("Warning Message");
    logger->error("Error Message");
    logger->fatal("Fatal Message");
    return nullptr;
}

int main(void) {
    pthread_t thread_id;
    // Extended formatting: String messages, include hostname/pname/pid/tname/tid.
    ExtConfig ext_config(MessageStruct::String, 1, 1, 1, 1, 1);
    WriterConfig configs[] = { ConsoleWriterConfig(DEBUG, true) };
    Logging *logging = new Logging(DEBUG, "root", configs, &ext_config);
    Logger  *logger  = new Logger(DEBUG, "LoggerThread", 1, 1);
    logging->add_logger(logger);

    pthread_create(&thread_id, nullptr, loggerThreadFun,
                   static_cast<void *>(logger));

    logging->trace("Trace Message");
    logging->debug("Debug Message");
    logging->info("Info Message");
    logging->success("Success Message");
    logging->warn("Warning Message");
    logging->error("Error Message");
    logging->fatal("Fatal Message");

    pthread_join(thread_id, nullptr);
    delete logging;  // calls shutdown(false)
    delete logger;
    return 0;
}
```

In this example the same `Logger` pointer is safely used from a worker thread while
the main thread logs directly through the `Logging` instance. The `Logger` must
remain alive until `pthread_join` returns; `delete logging` first ensures the
background dispatch thread has flushed, then `delete logger` releases the logger
handle.

---

For the underlying C FFI declarations, see `cppfastlogging/h/logger.hpp`.
For the main logging entry point, see [LOGGING.md](LOGGING.md).
