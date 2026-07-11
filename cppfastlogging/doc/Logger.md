# cppfastlogging: Logger and Logging Class Documentation

## Logger Class

The `logging::Logger` class is a modern C++ RAII wrapper for a logger instance, providing per-module or per-thread logging. It is move-only and non-copyable, and must be registered with a `Logging` instance for use.

### Constructor
- `Logger(uint8_t level, const char *domain)`
    - Create a logger with the given log level and domain.
- `Logger(uint8_t level, const char *domain, int8_t tname, int8_t tid)`
    - Create a logger with thread name and thread id logging enabled/disabled.

### Methods
- `void set_level(uint8_t level)` — Set the log level for this logger.
- `void set_domain(const char *domain)` — Set the log domain for this logger.
- `int trace(const std::string &message) const` — Log a trace message.
- `int debug(const std::string &message) const` — Log a debug message.
- `int info(const std::string &message) const` — Log an info message.
- `int success(const std::string &message) const` — Log a success message.
- `int warn(const std::string &message) const` — Log a warning message.
- `int warning(const std::string &message) const` — Log a warning message (alias).
- `int error(const std::string &message) const` — Log an error message.
- `int critical(const std::string &message) const` — Log a critical message.
- `int fatal(const std::string &message) const` — Log a fatal message.
- `int exception(const std::string &message) const` — Log an exception message.
- `rust::Logger *raw() const` — Get the raw FFI pointer.

### Usage Example
```cpp
#include "cppfastlogging.hpp"

int main() {
    logging::Logger logger(logging::DEBUG, "worker");
    logger.info("Logger started");
    return 0;
}
```

---

For more details, see the header files in `cppfastlogging/h/`.
