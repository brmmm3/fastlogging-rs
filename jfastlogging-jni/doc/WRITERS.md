# Writers

Writers are the sinks for the log messages. To add a writer a writer configuration must be created first and then with the `add_writer` method a new writer is created. To remove a writer the `remove_writer` method must be called.

## `ConsoleWriterConfig(level: int, colors: bool)`

Create new console writer configuration.  
`level` sets the log level filter.  
If `colors` is `True` the console output will be colored.

## `FileWriterConfig(level: int, path: str, size: int = None, backlog: int = None, timeout: Duration = None, time: SystemTime = None, compression: CompressionMethodEnum = None)`

Create new file writer configuration.  
`level` sets the log level filter.  
`path` is the path to the log file.  
`size` if provided sets the maximum size of a the log file in bytes. If the file size is reached a file rotation will take place.  
`backlog` if provided sets the maximum number of backup files.  
`timeout` if provided sets the timeout after the last log messages. If the timeout is reached a file rotation will take place.
`time` if provided sets the time of the day when a file rotation will take place.
`compression` if provided sets the compression method for the backup file. Valid values are `Store`, `Deflate`, `Zstd`, `Lzma`.

## `ServerConfig(level: int, address: str, key: EncryptionMethod = None)`

Create new server configuration.  
`level` sets the log level filter.  
`address` is the listening IP address.  
`key` if provided sets the authentication or AES encryption key.

## `ClientWriterConfig(level: int, address: str, key: EncryptionMethod = None)`

Create new client writer configuration.  
`level` sets the log level filter.  
`address` is the target IP address.  
`key` if provided sets the authentication or AES encryption key.

## `SyslogWriterConfig(level: int, hostname: str = None, pname: str = None, pid: int = 0)`

Create new syslog writer configuration.  
`level` sets the log level filter.  
`hostname` if provided sets the hostname to be added to the log messages.
`pname` if provided sets the process name to be added to the log messages.
`pid` if provided sets the process id to be added to the log messages.

## `CallbackWriterConfig(level: int, callback: Py<PyAny>)`

Create new callback writer configuration. This writer can be used for individual log message handling.  
`level` sets the log level filter.  
`callback` is the Python callback function to be called for every log message. The callback function must have the followin definition: `fct(level: int, domain: str, message: str)`.

### `set_callback(callback: Py<PyAny> = None)`

Set a new callback function if provided. In case of `None` the current callback function will be removed, which disabled the callback writer.
