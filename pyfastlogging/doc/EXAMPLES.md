# Examples

## Logging to Console with Logging instance

```python
from pyfastlogging import (
    TRACE,
    Logging,
    ConsoleWriterConfig,
)

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

## Logging to Console with root logger

```python
import pyfastlogging as fl
from pyfastlogging import (
    TRACE,
    ConsoleWriterConfig,
)

fl.add_writer(ConsoleWriterConfig(TRACE, True))
fl.trace("Trace Message")
fl.debug("Debug Message")
fl.info("Info Message")
```

## Logging to Console with multiprocessing

```python
from pyfastlogging import (
    TRACE,
    Logging,
    ConsoleWriterConfig,
)

def ChildProcess():
    logger = Logging(
        TRACE,
        "child",
        [ConsoleWriterConfig(TRACE, True)],
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.shutdown()

if __name__ == "__main__":
    import multiprocessing
    from multiprocessing import Pool, freeze_support

    multiprocessing.set_start_method("spawn")
    freeze_support()
    with Pool() as pool:
        pool.apply(ChildProcess)
        pool.apply(ChildProcess)
    logger = Logging(
        TRACE,
        "main",
        console=ConsoleWriterConfig(TRACE, True),
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.shutdown()
```

## Logging to File using root logger

```python
import os

import pyfastlogging as fl
from pyfastlogging import (
    TRACE,
    CompressionMethodEnum,
    FileWriterConfig,
)

if __name__ == "__main__":
    pathName = (
        "C:\\temp\\pyfastlogging.log" if os.name == "nt" else "/tmp/pyfastlogging.log"
    )
    fl.add_writer(
        FileWriterConfig(TRACE, pathName, compression=CompressionMethodEnum.Deflate)
    )
    fl.trace("Trace Message")
    fl.debug("Debug Message")
    fl.info("Info Message")
    fl.success("Success Message")
    fl.warning("Warning Message")
    fl.error("Error Message")
    fl.fatal("Fatal Message")
```

## Logging via network sockets

```python
import tempfile

from pyfastlogging import (
    TRACE,
    DEBUG,
    Logging,
    ConsoleWriterConfig,
    FileWriterConfig,
    ServerConfig,
    ClientWriterConfig,
)

def SomeThread(logger):
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")

if __name__ == "__main__":
    tmpDir = tempfile.mkdtemp(prefix="fastlogging")
    logging_server = Logging(
        TRACE,
        "LOGSRV",
        [
            ConsoleWriterConfig(TRACE, True),
            FileWriterConfig(TRACE, f"{tmpDir}/fastlogging.log"),
            ServerConfig(TRACE, "127.0.0.1"),
        ],
    )
    logging_server.sync_all(5.0)
    address = logging_server.get_server_address()
    print(address)
    key = logging_server.get_server_auth_key()
    print(key)
    logging_client = Logging(
        TRACE, "LOGCLIENT", [ClientWriterConfig(DEBUG, address, key)]
    )
    logging_client.trace("Trace Message")
    logging_client.debug("Debug Message")
    logging_client.info("Info Message")

    logging_server.trace("Trace Message")
    logging_server.debug("Debug Message")
    logging_server.info("Info Message")

    logging_client.sync_all(1.0)
    logging_server.sync_all(1.0)

    logging_client.shutdown()
    logging_server.shutdown()
```

## Logging using callback

```python
from pyfastlogging import (
    TRACE,
    DEBUG,
    Logging,
    ConsoleWriterConfig,
    CallbackWriterConfig,
)

def writer_callback(level: int, domain: str, message: str):
    print(f"--> {level} {domain} {message}")

if __name__ == "__main__":
    logger = Logging(
        TRACE,
        "main",
        [
            ConsoleWriterConfig(TRACE, True),
            CallbackWriterConfig(DEBUG, writer_callback),
        ],
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.shutdown()
```

## Logging to syslog

```python
from pyfastlogging import (
    TRACE,
    Logging,
)

if __name__ == "__main__":
    logger = Logging(
        TRACE,
        "main",
        syslog=TRACE,
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.shutdown()
```

## Logging and threads

```python
import tempfile
from threading import Thread

from pyfastlogging import (
    TRACE,
    DEBUG,
    MessageStructEnum,
    Logging,
    Logger,
    ExtConfig,
    ConsoleWriterConfig,
    FileWriterConfig,
)

def SomeThread(logger):
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")

if __name__ == "__main__":
    tmpDir = tempfile.mkdtemp(prefix="fastlogging")
    logger = Logging(TRACE)
    logger.set_ext_config(
        ExtConfig(MessageStructEnum.String, True, True, True, True, True)
    )
    logger.add_writer(ConsoleWriterConfig(TRACE, True))
    logger.add_writer(FileWriterConfig(TRACE, f"{tmpDir}/fastlogging.log"))
    logger2 = Logger(DEBUG, "LoggerThread", None, True, True)
    logger.add_logger(logger2)
    thr = Thread(target=SomeThread, args=(logger2,), daemon=True)
    thr.start()
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    thr.join()
    logger.shutdown()
```
