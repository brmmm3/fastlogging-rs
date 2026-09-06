# fastlogging-rs

`fastlogging-rs` is a very fast and versatile logging framework. It supports the following programming languages with similar APIs:

- [Rust](fastlogging/README.md) (of course, as it is written in Rust ;-) )
- [Python](pyfastlogging/README.md) >=3.10 (using pyo3)
- [C](cfastlogging/README.md) (FFI)
- [C++](cppfastlogging/README.md) (FFI and CXX)
- [Go](gofastlogging/README.md)
- [Java](jfastlogging/README.md) (JNI and FFM)

## Features

- Extremely fast — logging calls are non-blocking and writers run in background threads
- Thread-safe logging calls
- Multiple writers (sinks) per logger: console, file, network, syslog, callback, …
- Optional file rotation and compression
- Optional AES encryption for network logging
- Configuration via API or configuration file (JSON, XML, YAML)
- Automatic forwarding of log messages from sub processes to the main process

## Writers

Writers are sinks for the logging data. The following writers are available:

- Console (optional colored)
- File (optional rotation and compression)
- TCP client (optional authentication key and AES encryption)
- Syslog (Linux), EventLog (Windows)
- Callback function

All writers run in background threads. So the speed / slowness of the writers doesn't slow down the application
as long as the queue is not running full.

## Threads

Logging calls are thread safe.

## Processes

`fastlogging-rs` supports logging from sub processes to the main process automatically.
So if a sub process logs messages, these messages are forwarded to the main process.
This also works with higher nesting levels. This feature is enabled by default and can be disabled.

## Configuration

As an alternative to API calls, configuration can be done through a configuration file.
Supported formats are JSON, XML and YAML. The configuration file must have the filename `fastlogging.<EXT>`,
where `EXT` is one of `json`, `xml` or `yaml`.

Example configuration files (default and full) are available in [`doc/configs`](doc/configs).

## Benchmarks

To give you an idea how fast `fastlogging-rs` is, here are some benchmarks:

### Writing to a file

| Framework       | Time  |
| --------------- | ----- |
| Python logging  | 29.37s|
| log4j           | 1.48s |
| fastlogging-rs  | 0.2s  |

### Rotating file logging

| Framework       | Time  |
| --------------- | ----- |
| Python logging  | 35.24s|
| log4j           | 1.56s |
| fastlogging-rs  | 0.17s |

More benchmarks can be found in `doc/benchmarks`.

You can explore the full benchmark results with interactive charts and tables: **[Benchmarks overview](doc/benchmarks.html)** (generated from the raw JSON data).

## Usage

### Rust

```rust
use fastlogging::{logging_new_default, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = logging_new_default()?;
    log.info("Hello, fastlogging!")?;
    log.shutdown(false)?;
    Ok(())
}
```

### Python

```python
from fastlogging import Logging

log = Logging()
log.info("Hello, fastlogging!")
log.shutdown(False)
```

## Documentation

Detailed documentation is available for each language binding:

- Rust: [fastlogging/README.md](fastlogging/README.md)
- Python: [pyfastlogging/README.md](pyfastlogging/README.md)
- C: [cfastlogging/README.md](cfastlogging/README.md)
- C++: [cppfastlogging/README.md](cppfastlogging/README.md)
- Go: [gofastlogging/README.md](gofastlogging/README.md)
- Java: [jfastlogging/README.md](jfastlogging/README.md)
