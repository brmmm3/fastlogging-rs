
# API of the LOGGER

The LOGGER API provides per-module or per-thread logging. All functions are thread-safe. Always check return values for error handling.

---

## Function Summary

| Function | Purpose |
|----------|---------|
| `logger_new` | Create a new Logger |
| `logger_new_ext` | Create Logger with thread name/id options |
| `logger_set_level` | Set log level |
| `logger_set_domain` | Set log domain |
| `logger_trace` | Log TRACE message |
| `logger_debug` | Log DEBUG message |
| `logger_info` | Log INFO message |
| `logger_success` | Log SUCCESS message |
| `logger_warning` | Log WARNING message |
| `logger_error` | Log ERROR message |
| `logger_critical` | Log CRITICAL message |
| `logger_fatal` | Log FATAL message |
| `logger_exception` | Log EXCEPTION message |

---


## `logger_new(level: c_uchar, domain: *const c_char) -> *mut Logger`
Create a new Logger instance. `level` is the minimum log level; `domain` is a string label for the logger.


## `logger_new_ext(level: c_uchar, domain: *const c_char, tname: c_char, tid: c_char) -> *mut Logger`
Create a Logger instance. If `tname` is true, the thread name is included in log messages. If `tid` is true, the thread id is included.


## `logger_set_level(logger: &mut Logger, level: u8)`
Set the minimum log level for this logger.


## `logger_set_domain(logger: &mut Logger, domain: *const c_char)`
Set the log domain string for this logger.


## Logging Methods

All logging methods return 0 on success, or a negative error code on failure. Common error codes:
- `-1`: Logger not registered with a Logging instance
- `-2`: Invalid arguments
- `-3`: Internal error

### `logger_trace(logger: &Logger, message: *const c_char) -> isize`
Log **TRACE** message.


### `logger_debug(logger: &Logger, message: *const c_char) -> isize`
Log **DEBUG** message.


### `logger_info(logger: &Logger, message: *const c_char) -> isize`
Log **INFO** message.


### `logger_success(logger: &Logger, message: *const c_char) -> isize`
Log **SUCCESS** message.


### `logger_warning(logger: &Logger, message: *const c_char) -> isize`
Log **WARNING** message.


### `logger_error(logger: &Logger, message: *const c_char) -> isize`
Log **ERROR** message.


### `logger_critical(logger: &Logger, message: *const c_char) -> isize`
Log **CRITICAL** message.


### `logger_fatal(logger: &Logger, message: *const c_char) -> isize`
Log **FATAL** message.


### `logger_exception(logger: &Logger, message: *const c_char) -> isize`
Log **EXCEPTION** message.

---

## Usage Example

```c
#include <stdio.h>
#include "h/cfastlogging.h"

int main(void) {
	Logger logger = logger_new(DEBUG, "worker");
	// Register logger with a Logging instance before use
	// logging_add_logger(logging, logger);
	if (logger_info(logger, "Logger started") != 0) fprintf(stderr, "Logger info failed\n");
	return 0;
}
```

---

## Thread Safety and Lifecycle

- All LOGGER functions are thread-safe.
- Logger instances can be used from multiple threads if registered with a Logging instance.
- Always call `logging_add_logger` before using a logger.
- Free logger instances with the appropriate API if dynamically allocated.
