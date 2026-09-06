# gofastlogging — Documentation

Go bindings for the [fastlogging](https://github.com/brmmm3/fastlogging-rs/tree/master/fastlogging) Rust logging library via cgo. Messages are routed through an asynchronous background thread to one or more independent writers (console, file, network client/server, syslog, callback), so the hot path in your code stays a cheap level check followed by a channel send.

## Table of Contents

- [doc/DEF.md](doc/DEF.md) — Top-level definitions: log levels, enums, type wrappers, key creation
- [doc/LEVELS.md](doc/LEVELS.md) — Log levels, symbols, and filtering semantics
- [doc/LOGGING.md](doc/LOGGING.md) — The `Logging` struct (primary logger instance)
- [doc/LOGGER.md](doc/LOGGER.md) — The `Logger` struct (per-thread / per-domain handles)
- [doc/WRITERS.md](doc/WRITERS.md) — Writer configuration factories and writer types
- [doc/NETWORK.md](doc/NETWORK.md) — Network client/server writers and encryption
- [doc/CONFIG.md](doc/CONFIG.md) — Configuration files, extended formatting, runtime config queries
- [doc/ROOT.md](doc/ROOT.md) — Root logger (process-global) functions
- [doc/EXAMPLES.md](doc/EXAMPLES.md) — Runnable examples

> The full guide (quick start, examples, architecture, package layout, important notes) lives in **[doc/README.md](doc/README.md)**.

## Quick Start

The `gofastlogging` module is **not** part of the Cargo workspace. It is built from the `gofastlogging/` directory, after producing the C shared library from the `cfastlogging` package.

#### 1. Build the C library

From the repository root:

```bash
cargo build -p cfastlogging            # debug  -> target/debug/libcfastlogging.so
# or
cargo build -p cfastlogging --release  # release -> target/release/libcfastlogging.so
```

The `cbindgen` step is handled automatically by `cfastlogging`'s build script — no manual header generation needed.

#### 2. Build the Go wrapper and examples

```bash
cd gofastlogging
make build-debug   # uses ../target/debug/libcfastlogging.so
# or
make build         # uses ../target/release/libcfastlogging.so
```

The Makefile copies the shared library from `../target/{debug,release}/libcfastlogging.so` into `gofastlogging/lib/`, then runs `go build` with `-ldflags="-r $(ROOT_DIR)lib"` so the runtime linker can find `libcfastlogging.so` next to the produced binaries in `gofastlogging/bin/`.

#### 3. Minimal console example

```go
package main

import (
    "gofastlogging/fastlogging/logging"
)

func main() {
    logger, err := logging.Default()
    if err != nil {
        panic(err)
    }
    defer logger.Shutdown(false)

    logger.Info("Hello from gofastlogging")
}
```

Run the produced binary from `gofastlogging/bin/` (e.g. `./bin/default`, `./bin/console`), or run an example directly with `go run examples/console/main.go` from the `gofastlogging/` directory.

## Architecture

Log calls never touch I/O on the caller's goroutine. Each call performs a level check (the hot path), and if it passes, the message is handed to a flume-backed channel (`server_tx`). A single `LoggingThread` running in the background drains that channel and dispatches to each writer's own thread.

```mermaid
flowchart TD
    Code["Your code"] --> LevelCheck["level check\n(hot path — single\ninteger compare,\nno lock)"]
    LevelCheck -->|"push message"| Channel["server_tx\n(flume channel)"]
    Channel --> Server["LoggingThread\n(background)"]
    Server --> Console["ConsoleWriter\n(thread)"]
    Server --> File["FileWriter\n(thread)"]
    Server --> Client["ClientWriter\n(thread)"]
    Server --> NetServer["LoggingServer\n(thread)"]
    Server --> Callback["CallbackWriter\n(thread)"]
    Server --> Syslog["SyslogWriter\n(thread, unix)"]
```

Because the channel send is the only work done on the caller's goroutine, logging stays cheap even when writers are slow (network, disk, syslog).

## Package Layout

The Go wrapper is split across four packages. Import paths are rooted at the `gofastlogging` module:

| Package | Import path | Typical alias | Responsibility |
| --- | --- | --- | --- |
| `fastlogging` | `gofastlogging/fastlogging` | `fl` | Log-level constants, enums (`LevelSymbol`, `FileType`, `MessageStruct`, `EncryptionMethod`, `CompressionMethod`), type wrappers (`WriterConfigEnum`, `Key`, `ExtConfig`, ...), root logger functions, and key creation (`CreateKey`, `CreateRandomKey`). |
| `logging` | `gofastlogging/fastlogging/logging` | `logging` | The `Logging` struct — the primary logger instance. `logging.New` and `logging.Default` construct it; methods log messages and manage writers, levels, domain, sync, rotation, and config. |
| `logger` | `gofastlogging/fastlogging/logger` | `logger` | The `Logger` struct — lightweight per-thread / per-domain handles attached to a `Logging` instance. |
| `writer` | `gofastlogging/fastlogging/writer` | `writer` | Writer configuration factory functions: `ConsoleWriterConfigNew`, `FileWriterConfigNew`, `ClientWriterConfigNew`, `ServerConfigNew`, `SyslogWriterConfigNew`, `CallbackWriterConfigNew`. |

## Important Notes

- **Writer factory functions return `*fl.WriterConfigEnum`.** When assembling a `[]fl.WriterConfigEnum` slice for `logging.New` (or `AddWriterConfigs`), dereference each pointer with `*`, e.g. `[]fl.WriterConfigEnum{*console, *file}`. Passing the pointers directly is a type error.
- **`logging.New` returns `*Logging` (no error).** It returns `nil` if the underlying C constructor fails, so check for `nil` if you want to fail loudly. `logging.Default()` returns `(*Logging, error)`.
- **Optional strings use `*string`.** Pass `nil` for "not set" rather than an empty string when the distinction matters (e.g. `domain`, `configPath`).
- **Callback writer is not yet implemented.** `writer.CallbackWriterConfigNew` exists for API symmetry but currently returns an error. The other writer types are functional.
- **Cross-package type wrappers use `unsafe.Pointer` fields.** This is intentional: cgo generates a distinct, non-interchangeable Go type per package for each C type, even when two `import "C"` blocks include the same header. Using `unsafe.Pointer` lets `WriterConfigEnum`, `Key`, `ExtConfig`, etc. flow between the `fastlogging`, `logging`, `logger`, and `writer` packages.
- **Build order matters.** The Makefile expects `libcfastlogging.so` to already exist in `../target/debug` (or `../target/release`). Run `cargo build -p cfastlogging` first, then `make build-debug` / `make build`.
