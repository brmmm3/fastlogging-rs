# Examples

## Logging to Console with Logging instance

```go
package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import logging "gofastlogging/fastlogging"

func main() {
    writers := []logging.WriterConfigEnum{logging.ConsoleWriterConfigNew(logging.DEBUG, true)}
    logger := logging.New(logging.DEBUG, nil, writers, nil, nil)
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Fatal("Fatal Message")
    logger.Shutdown(false)
}
```

## Logging to Console with root logger

```go
package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
    "gofastlogging/fastlogging/root"
)

func main() {
    root.Init()
    root.Trace("Trace message")
    root.Debug("Debug message")
    root.Info("Info Message")
    root.Warning("Warning Message")
    root.Error("Error Message")
    root.Fatal("Fatal Message")
    root.Shutdown(false)
}
```

## Logging to File

```go
package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import logging "gofastlogging/fastlogging"

func main() {
    writers := []logging.WriterConfigEnum{logging.FileWriterConfigNew(logging.DEBUG,
        "/tmp/gofastlogging.log",
        1024,
        3,
        -1,
        -1,
        logging.Store)}
    logger := logging.New(logging.DEBUG, nil, writers, nil, nil)
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Fatal("Fatal Message")
    logger.Shutdown(false)
}
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
