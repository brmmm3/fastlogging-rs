# API of the LOGGER

## `logger_new(level: c_uchar, domain: *const c_char) -> *mut Logger`

Create `Logger` instance. `level` and `domain` are clear and need no further explanation.

## `logger_new_ext(level: c_uchar, domain: *const c_char, tname: c_char, tid: c_char) -> *mut Logger`

Create `Logger` instance. `level` and `domain` are clear and need no further explanation.
With `tname` is `true` the thread name is added to the log messages.  
With `tid` is `true` the thread id is added to the log messages.

## `logger_set_level(logger: &mut Logger, level: u8)`

Set log level to `level`.

## `logger_set_domain(logger: &mut Logger, domain: *const c_char)`

Set log domain.

## `logger_trace(logger: &Logger, message: *const c_char) -> isize`

Log **TRACE** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_debug(logger: &Logger, message: *const c_char) -> isize`

Log **DEBUG** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_info(logger: &Logger, message: *const c_char) -> isize`

Log **INFO** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_success(logger: &Logger, message: *const c_char) -> isize`

Log **SUCCESS** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_warning(logger: &Logger, message: *const c_char) -> isize`

Log **WARNING** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_error(logger: &Logger, message: *const c_char) -> isize`

Log **ERROR** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_critical(logger: &Logger, message: *const c_char) -> isize`

Log **CRITICAL** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_fatal(logger: &Logger, message: *const c_char) -> isize`

Log **FATAL** message.
An error code is returned in case of failure. On success 0 is returned.

## `logger_exception(logger: &Logger, message: *const c_char) -> isize`

Log **EXCEPTION** message.
An error code is returned in case of failure. On success 0 is returned.
