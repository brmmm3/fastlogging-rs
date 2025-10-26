# API of the LOGGER

## `Logger(level: int, domain: str, indent: Tuple[int, int, int] = None, tname: bool = False, tid: bool = False)`

Create `Logger` instance. `level` and `domain` are clear and need no further explanation.
With `indent`, if provided, log messages are indented with the following parameters:  
`indent = (offset, increment, maximum)`  
`offset` = Initial indent level  
`increment` = Increment of indent by call level  
`maximum` = Maximum increment  
With `tname` is `True` the thread name is added to the log messages.  
With `tid` is `True` the thread id is added to the log messages.

## `set_level(level: int)`

Set log level to `level`.

## `level() -> int`

Get current log level.

## `set_domain(domain: str)`

Set log domain.

## `trace(obj: Py<PyAny>)`

Log **TRACE** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `debug(obj: Py<PyAny>)`

Log **DEBUG** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `info(obj: Py<PyAny>)`

Log **INFO** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `success(obj: Py<PyAny>)`

Log **SUCCESS** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `warning(obj: Py<PyAny>)`

Log **WARNING** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `error(obj: Py<PyAny>)`

Log **ERROR** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `critical(obj: Py<PyAny>)`

Log **CRITICAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `fatal(obj: Py<PyAny>)`

Log **FATAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `exception(obj: Py<PyAny>)`

Log **EXCEPTION** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.
