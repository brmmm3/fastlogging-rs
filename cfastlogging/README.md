# cfastlogging — Documentation

C bindings for the [`fastlogging`](../fastlogging) Rust logging library, exposing a raw `extern "C"` ABI (`cfastlogging.h`). This is the base C layer used by other language bindings ([`cppfastlogging`](../cppfastlogging), [`gofastlogging`](../gofastlogging)) — it is intentionally minimal, with plain C structs, enums, and free functions, and no automatic memory management.

Messages are routed through an asynchronous background thread to one or more independent writers (console, file, network client/server, syslog, callback), so the hot path in your code stays a cheap level check followed by a channel send.

## Table of Contents

- [doc/DEF.md](doc/DEF.md) — Top-level definitions: log levels, enums, structs
- [doc/LOGGING.md](doc/LOGGING.md) — The `Logging` API (primary logger)
- [doc/LOGGER.md](doc/LOGGER.md) — The `Logger` API (per-thread / per-domain handles)
- [doc/ROOT.md](doc/ROOT.md) — Root logger (process-global) functions
- [doc/CONFIG.md](doc/CONFIG.md) — Configuration files and extended formatting
- [doc/EXAMPLES.md](doc/EXAMPLES.md) — Runnable C examples

## Quick Start

### 1. Build the shared library

From the repository root:

```bash
cargo build -p cfastlogging            # debug  -> target/debug/libcfastlogging.so
# or
cargo build -p cfastlogging --release  # release -> target/release/libcfastlogging.so
```

The C header is generated automatically by the build script (cbindgen); the checked-in headers live in `cfastlogging/h/`.

### 2. Build and run the examples

```bash
cd cfastlogging
make build-debug   # links against ../target/debug/libcfastlogging.so
# or
make build         # links against ../target/release/libcfastlogging.so
```

The Makefile compiles the C examples in `examples/` (using `gcc`) and links them directly against the shared library. The produced binaries land in `cfastlogging/bin/` (e.g. `./bin/console`).

### Minimal console example

```c
#include "h/cfastlogging.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  WriterConfigEnum writers[] = {console_writer_config_new(DEBUG, 1)};
  Logging logging = logging_new(DEBUG, NULL, writers, 1, NULL, NULL);
  logging_info(logging, "Info Message");
  logging_warning(logging, "Warning Message");
  logging_error(logging, "Error Message");
  logging_shutdown(logging, 0);
  return 0;
}
```

## Headers

| Header | Contents |
|---|---|
| `h/cfastlogging.h` | Umbrella header — includes all of the below |
| `h/def.h` | Log-level constants, `CWriterEnum`, structs (`CEncryptionMethod`, `CServerConfig`, ...) |
| `h/logging.h` | `Logging` opaque handle and `logging_*` functions |
| `h/logger.h` | `Logger` opaque handle and `logger_*` functions |
| `h/root.h` | Process-global root logger functions |
| `h/writer.h` | Writer config factories (`console_writer_config_new`, `file_writer_config_new`, ...) |

## Important Notes

- **This is the raw C ABI.** There is no RAII — you are responsible for freeing dynamically allocated resources and keeping pointer arguments alive (see `doc/DEF.md` for per-struct memory-management notes).
- **`Logging` constructors take an array of writer configs.** Pass the array plus its element count to `logging_new` (see the example above).
- **Client-side level filtering happens in C.** `logging_*` message functions compare the message level against the instance level before handing the message to the native layer.
- **Build order matters.** The Makefile expects `libcfastlogging.so` to already exist in `../target/{debug,release}`. Run `cargo build -p cfastlogging` first, then `make build-debug` / `make build`.
- **For a type-safe, memory-safe C++ wrapper** over this C API, see [`cppfastlogging`](../cppfastlogging).
