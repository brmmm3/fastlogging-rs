---
# gofastlogging: Idiomatic Go Usage & Examples

## Quick Start

Install dependencies and build the C library as described in the README. Then:

```go
package main

import (
    "log"
    logging "gofastlogging/fastlogging"
)

func main() {
    console, err := logging.ConsoleWriterConfigNew(logging.DEBUG, true)
    if err != nil {
        log.Fatal(err)
    }
    writers := []logging.WriterConfigEnum{console}
    logger := logging.New(logging.DEBUG, nil, writers, nil, nil)
    logger.Info("Hello, world!")
    logger.Shutdown(false)
}
```

## Logging to Console with root logger

```go
package main

import (
    "log"
    "gofastlogging/fastlogging/root"
)

func main() {
    err := root.Init()
    if err != nil {
        log.Fatal(err)
    }
    root.Info("Root logger example")
    root.Shutdown(false)
}
```

## Logging to File

```go
package main

import (
    "log"
    logging "gofastlogging/fastlogging"
)

func main() {
    file, err := logging.FileWriterConfigNew(logging.DEBUG,
        "/tmp/gofastlogging.log",
        1024,
        3,
        -1,
        -1,
        logging.Store)
    if err != nil {
        log.Fatal(err)
    }
    writers := []logging.WriterConfigEnum{file}
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

## Using the Callback Writer

```go
package main

import (
    "fmt"
    "os"
    logging "gofastlogging/fastlogging"
)

func main() {
    callback := func(level uint8, domain, message string) {
        fmt.Fprintf(os.Stdout, "[CALLBACK] Level: %d, Domain: %s, Message: %s\n", level, domain, message)
    }
    writer, handle, err := logging.CallbackWriterConfigNew(logging.DEBUG, callback)
    if err != nil {
        panic(err)
    }
    defer handle.UnregisterCallback()
    logger := logging.New(logging.DEBUG, nil, []logging.WriterConfigEnum{writer}, nil, nil)
    logger.Info("Hello from callback writer!")
    logger.Error("This is an error message.")
    logger.Shutdown(false)
}
```

## Best Practices

- Always check errors when creating writer configs.
- Use `defer handle.UnregisterCallback()` for callback writers to avoid memory leaks.
- Prefer context.Context for advanced use cases (see API docs).
- Use the provided constructors for all writer types for memory safety and idiomatic Go.

---

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
