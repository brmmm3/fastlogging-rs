# Examples

The following examples are available in `gofastlogging/examples/`. Build them with:

```sh
cd gofastlogging && make build-debug
```

**Prerequisite:** Build the C library first with `cargo build -p cfastlogging`.

## 1. Default Logger (`examples/default`)

```go
package main

import (
    "fmt"
    "gofastlogging/fastlogging/logging"
    "log"
)

func main() {
    logger, err := logging.Default()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Default logger created: %+v\n", logger)
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Critical("Critical Message")
    logger.Shutdown(false)
}
```

## 2. Console Logger (`examples/console`)

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)

func main() {
    console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
    if console == nil {
        panic("Failed to create writer")
    }
    writers := []fl.WriterConfigEnum{*console}
    logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
    if logger == nil {
        panic("Failed to create logger")
    }
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Fatal("Fatal Message")
    logger.Shutdown(false)
}
```

## 3. Root Logger (`examples/console_root`)

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "log"
)

func main() {
    err := fl.Init()
    if err != nil {
        log.Fatal(err)
    }
    fl.Trace("Trace message")
    fl.Debug("Debug message")
    fl.Info("Info Message")
    fl.Warning("Warning Message")
    fl.Error("Error Message")
    fl.Fatal("Fatal Message")
    fl.Shutdown(false)
}
```

## 4. File Logger (`examples/file`)

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)

func main() {
    file := writer.FileWriterConfigNew(
        fl.DEBUG,
        "/tmp/cfastlogging.log",
        1024, 3, -1, -1,
        fl.Store)
    if file == nil {
        panic("Failed to create file writer")
    }
    writers := []fl.WriterConfigEnum{*file}
    logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
    if logger == nil {
        panic("Failed to create logger")
    }
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Fatal("Fatal Message")
    logger.Shutdown(false)
}
```

## 5. Extended Config (`examples/ext_config`)

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)

func main() {
    console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
    if console == nil {
        panic("Failed to create writer")
    }
    writers := []fl.WriterConfigEnum{*console}
    logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
    if logger == nil {
        panic("Failed to create logger")
    }
    extConfig := fl.NewExtConfig(fl.Xml, true, false, true, false, true)
    logger.SetExtConfig(extConfig)
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Fatal("Fatal Message")
    logger.Shutdown(false)
}
```

## 6. Syslog (`examples/syslog`)

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)

func main() {
    syslogWriter := writer.SyslogWriterConfigNew(fl.DEBUG, "hostname", "pname", 1234)
    if syslogWriter == nil {
        panic("Failed to create syslog writer")
    }
    writers := []fl.WriterConfigEnum{*syslogWriter}
    logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
    if logger == nil {
        panic("Failed to create logger")
    }
    logger.Trace("Trace message")
    logger.Debug("Debug message")
    logger.Info("Info Message")
    logger.Warning("Warning Message")
    logger.Error("Error Message")
    logger.Fatal("Fatal Message")
    logger.Shutdown(false)
}
```

## 7. Threads (`examples/threads`)

```go
package main

import (
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logger"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
    "sync"
)

func loggerThread(lt *logger.Logger, wg *sync.WaitGroup) {
    lt.Trace("Trace message")
    lt.Debug("Debug message")
    lt.Info("Info Message")
    lt.Warning("Warning Message")
    lt.Error("Error Message")
    lt.Fatal("Fatal Message")
    wg.Done()
}

func main() {
    console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
    if console == nil {
        panic("Failed to create console writer")
    }
    writers := []fl.WriterConfigEnum{*console}
    loggerMain := logging.New(fl.DEBUG, nil, writers, nil, nil)
    if loggerMain == nil {
        panic("Failed to create logger")
    }
    name := "LoggerThread"
    threadLogger := logger.NewExt(fl.DEBUG, &name, 1, 1)
    if threadLogger == nil {
        panic("Failed to create thread logger")
    }
    var wg sync.WaitGroup
    wg.Add(1)
    go loggerThread(threadLogger, &wg)
    loggerMain.Trace("Trace message")
    loggerMain.Debug("Debug message")
    loggerMain.Info("Info Message")
    loggerMain.Warning("Warning Message")
    loggerMain.Error("Error Message")
    loggerMain.Fatal("Fatal Message")
    wg.Wait()
    loggerMain.Shutdown(false)
}
```

## 8. Network — Unencrypted (`examples/net_unencrypted_one_client`)

```go
package main

import (
    "fmt"
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)

func main() {
    console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
    file := writer.FileWriterConfigNew(fl.DEBUG, "/tmp/cfastlogging.log", 1024, 3, -1, -1, fl.Store)
    serverWriters := []fl.WriterConfigEnum{*console, *file}
    serverDomain := "LOGSRV"
    loggingServer := logging.New(fl.DEBUG, &serverDomain, serverWriters, nil, nil)

    server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", nil)
    loggingServer.SetRootWriterConfig(*server)
    loggingServer.SyncAll(5.0)

    addrPort := loggingServer.GetRootServerAddressPort()
    fmt.Printf("address_port=%s\n", addrPort)

    client := writer.ClientWriterConfigNew(fl.DEBUG, addrPort, nil)
    clientWriters := []fl.WriterConfigEnum{*client}
    clientDomain := "LOGCLIENT"
    loggingClient := logging.New(fl.DEBUG, &clientDomain, clientWriters, nil, nil)

    loggingClient.Trace("Trace message")
    loggingClient.Debug("Debug message")
    loggingClient.Info("Info Message")

    loggingServer.Trace("Trace message")
    loggingServer.Debug("Debug message")
    loggingServer.Info("Info Message")

    loggingClient.SyncAll(1.0)
    loggingServer.SyncAll(1.0)
    loggingClient.Shutdown(false)
    loggingServer.Shutdown(false)
}
```

## 9. Network — Encrypted (`examples/net_unencrypted_one_client_enc`)

```go
package main

import (
    "fmt"
    fl "gofastlogging/fastlogging"
    "gofastlogging/fastlogging/logging"
    "gofastlogging/fastlogging/writer"
)

func main() {
    console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
    file := writer.FileWriterConfigNew(fl.DEBUG, "/tmp/cfastlogging.log", 1024, 3, -1, -1, fl.Store)
    serverWriters := []fl.WriterConfigEnum{*console, *file}
    serverDomain := "LOGSRV"
    loggingServer := logging.New(fl.DEBUG, &serverDomain, serverWriters, nil, nil)

    // Generate random AES key
    serverKey := fl.CreateRandomKey(fl.AES)
    server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", &serverKey)
    loggingServer.SetRootWriterConfig(*server)
    loggingServer.SyncAll(5.0)

    addrPort := loggingServer.GetRootServerAddressPort()
    fmt.Printf("address_port=%s\n", addrPort)

    // Retrieve key for client (don't reuse serverKey — it's consumed)
    authKey := loggingServer.GetServerAuthKey()
    client := writer.ClientWriterConfigNew(fl.DEBUG, addrPort, &authKey)
    clientWriters := []fl.WriterConfigEnum{*client}
    clientDomain := "LOGCLIENT"
    loggingClient := logging.New(fl.DEBUG, &clientDomain, clientWriters, nil, nil)

    loggingClient.Trace("Trace message")
    loggingClient.Info("Info Message")
    loggingServer.Info("Server message")

    loggingClient.SyncAll(1.0)
    loggingServer.SyncAll(1.0)
    loggingClient.Shutdown(false)
    loggingServer.Shutdown(false)
}
```

## Available Examples

| Example | Description |
|---|---|
| `default` | Default logger (reads config file or defaults) |
| `console` | Console writer |
| `console_root` | Root logger (package-level functions) |
| `console_add_writer` | Adding a writer at runtime |
| `file` | File writer with rotation |
| `file_add_writer` | Adding file writer at runtime |
| `syslog` | Syslog writer |
| `callback` | Callback writer (not yet implemented) |
| `threads` | Multi-threaded logging with `Logger` |
| `ext_config` | Extended formatting config |
| `net_unencrypted_one_client` | Network logging, unencrypted |
| `net_unencrypted_one_client_enc` | Network logging, AES encrypted |
| `get_server_addresses_ports` | Querying server addresses/ports |
| `get_server_addresses_ports_enc` | Querying server addresses/ports (encrypted) |
| `get_server_configs` | Querying server configs |
