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

## `set_level(level: u8)`

Set log level to `level`.

## `level() -> u8`

Get current log level.

## `set_domain(domain: str)`

Set log domain.

## `trace(obj: PyObject)`

Log **TRACE** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `debug(obj: PyObject)`

Log **DEBUG** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `info(obj: PyObject)`

Log **INFO** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `success(obj: PyObject)`

Log **SUCCESS** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `warning(obj: PyObject)`

Log **WARNING** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `error(obj: PyObject)`

Log **ERROR** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `critical(obj: PyObject)`

Log **CRITICAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `fatal(obj: PyObject)`

Log **FATAL** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.

## `exception(obj: PyObject)`

Log **EXCEPTION** message. `obj` can be any object which can be converted into a string.
An exception is thrown if `obj` cannot be converted into a string.
