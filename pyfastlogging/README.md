# pyfastlogging

Python wrapper for [fastlogging](https://github.com/brmmm3/fastlogging-rs/tree/master/fastlogging).

## Features

- Extremely fast, non-blocking logging (writers run in background threads)
- Thread-safe logging calls
- Multiple writers per logger: console (optional colored), file (rotation/compression), network (TCP), syslog, callback
- Optional AES encryption for network logging
- Support for multiprocessing (`spawn`) and threads
- Configuration via API or configuration file (JSON, XML, YAML)
- Requires Python >= 3.10

## Installation

```bash
pip install pyfastlogging
```

## Quickstart

```python
from pyfastlogging import TRACE, Logging, ConsoleWriterConfig

logger = Logging(
    TRACE,
    "main",
    [ConsoleWriterConfig(TRACE, True)],
)
logger.trace("Trace Message")
logger.debug("Debug Message")
logger.info("Info Message")
logger.shutdown()
```

Or with the root logger:

```python
import pyfastlogging as fl
from pyfastlogging import TRACE, ConsoleWriterConfig

fl.add_writer(ConsoleWriterConfig(TRACE, True))
fl.trace("Trace Message")
fl.debug("Debug Message")
fl.info("Info Message")
```

## Building the wheels

This is the Python layer for [fastlogging](https://github.com/brmmm3/fastlogging-rs/tree/master/fastlogging). This package is for creating Python wheels.  
For simplicity `build_wheels.py` can be used to build the wheels. The Python script uses `pyenv` to choose different Python versions and `maturin` to build the wheels.  
If you run the script without options it will build the Python module for the current used Python interpreter.  
When using the `--versions` option you can provide:

- a comma separated list of Python versions
- `*` to build Python modules for all versions installed by `pyenv`. Put it in quotation marks to avoid unexpected behavior.

**Note:**
As of now [manylinux](https://github.com/pypa/manylinux) wheels are failing to build with `cibuildwheel`, because the spec for libc 2.34 is still not released.

## Documentation

- Class and enum definitions: [doc/DEF.md](doc/DEF.md)
- Root logger: [doc/ROOT.md](doc/ROOT.md)
- Logging class: [doc/LOGGING.md](doc/LOGGING.md)
- Logger class: [doc/LOGGER.md](doc/LOGGER.md)
- Examples: [examples](examples) and [doc/EXAMPLES.md](doc/EXAMPLES.md)
