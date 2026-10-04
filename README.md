# fastlogging-rs

![fastlogging-rs](docs/title.png)

[![crates.io](https://img.shields.io/crates/v/fastlogging?style=flat-square&logo=rust)](https://crates.io/crates/fastlogging)
[![docs.rs](https://img.shields.io/badge/docs.rs-fastlogging-4c8bfd?logo=docs.rs&style=flat-square)](https://docs.rs/fastlogging)
[![PyPI](https://img.shields.io/pypi/v/pyfastlogging?style=flat-square&logo=pypi&logoColor=white)](https://pypi.org/project/pyfastlogging/)
[![CI](https://github.com/brmmm3/fastlogging-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/brmmm3/fastlogging-rs/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](LICENSE-MIT)

**Non-blocking logging for Rust, with near-identical bindings for C, C++, C#, Go, Java and Python.**

`fastlogging` is written in Rust and keeps blocking I/O off your hot path: a log call does a
single integer level check and hands the message to a background thread. One Rust core powers
every binding, so the same logger, writers and configuration model are available from eight
languages — useful for mixed-language applications that want consistent logs
without shipping eight different logging stacks.

## Why another logging library?

- **The call site never blocks.** Each writer owns a thread and consumes from a bounded channel,
  so a slow disk, a congested network or a backed-up syslog daemon cannot stall your request path.
- **One core, eight languages.** Rust, C, C++, C#, Go, Java (JNI *and* FFM) and Python share the
  same API shape, the same writers and the same config files — see the table below.
- **Sub-processes log into the parent automatically.** A child process detects the parent's
  logging server and forwards its messages there, at any nesting depth, with no extra setup. This can be disabled via argument, environment variable, or configuration file.
- **Writers are pluggable and composable.** Console, file (with rotation and compression),
  TCP client/server, syslog/EventLog, callback, and OpenTelemetry export — several at once.
- **Configuration file support.** Load and save full configuration as JSON, XML or YAML.

Pick it if you want one logging design across a mixed-language codebase, or if log volume is high
enough that per-call latency matters.

## Quick start

### Rust

```toml
# Cargo.toml
[dependencies]
fastlogging = "0.9"
```

```rust
use fastlogging::{logging_new_default, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = logging_new_default()?;   // console writer at NOTSET
    log.info("Hello, fastlogging!")?;
    log.shutdown(false)?;                   // flush and join the writer threads
    Ok(())
}
```

An explicit logger with a coloured console writer:

```rust
use fastlogging::{ConsoleWriterConfig, DEBUG, Logging, LoggingError};

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG,
        "myapp",
        Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
        None,   // ExtConfig
        None,   // config file path
    )?;
    log.debug("starting up")?;
    log.info("ready")?;
    log.shutdown(false)?;
    Ok(())
}
```

### Python

```bash
pip install pyfastlogging    # requires Python >= 3.10
```

```python
from pyfastlogging import TRACE, ConsoleWriterConfig, Logging

logger = Logging(TRACE, "main", [ConsoleWriterConfig(TRACE, True)])
logger.info("Hello, fastlogging!")
logger.shutdown()
```

### Other languages

Each binding comes with runnable examples and its own documentation:

| Language | Binding | Documentation |
|---|---|---|
| Rust | `fastlogging` (crate) | [fastlogging/README.md](fastlogging/README.md) · [API docs](https://docs.rs/fastlogging) |
| Python | `pyfastlogging` (PyO3) | [pyfastlogging/README.md](pyfastlogging/README.md) |
| C | `cfastlogging` (raw `extern "C"`) | [cfastlogging/README.md](cfastlogging/README.md) |
| C++ | `cppfastlogging` (C++17 RAII) | [cppfastlogging/README.md](cppfastlogging/README.md) |
| C++ | `cxxfastlogging` (`cxx` bridge) | [cxxfastlogging/README.md](cxxfastlogging/README.md) |
| Go | `gofastlogging` (cgo) | [gofastlogging/README.md](gofastlogging/README.md) |
| Java (JNI) | `jfastlogging-jni` | [jfastlogging-jni/README.md](jfastlogging-jni/README.md) |
| Java (FFM) | `jfastlogging-ffm` | [jfastlogging-ffm/README.md](jfastlogging-ffm/README.md) |
| C# | `csharpfastlogging` (P/Invoke) | [csharpfastlogging/README.md](csharpfastlogging/README.md) |

Runnable examples live next to each binding — for Rust, `cargo run --example <name>` in
[`fastlogging/examples`](fastlogging/examples) (console, file, callback, threads, network,
syslog, OpenTelemetry, fork/spawn, config files).

## Features

- Non-blocking logging — writers run in background threads, the caller only enqueues
- Thread-safe log calls
- Multiple writers per logger: console, file, network, syslog/EventLog, callback, OpenTelemetry
- File rotation (by size or age) with optional `Deflate` / `Zstd` / `Lzma` compression
- TCP client and server writers, optional authentication key and AES encryption
- OpenTelemetry export over OTLP/HTTP (batched, non-blocking)
- Automatic forwarding of log messages from sub-processes to the main process
- Structured messages (`String`, `Json` or `Xml`) with optional hostname, process name, PID,
  thread name and thread ID
- Configuration via API or via a JSON / XML / YAML file, loadable and savable at runtime
- Level filtering per logger *and* per writer: `NOTSET`, `TRACE`, `DEBUG`, `INFO`, `SUCCESS`,
  `WARNING`, `ERROR`, `CRITICAL`, `FATAL`, `EXCEPTION`

### Writers

Writers are sinks for log data. Every writer runs in its own background thread, so writer speed
never slows the application down as long as the queue is not full.

| Writer | Notes |
|---|---|
| Console | Optional colours; stdout, stderr or both |
| File | Optional size- or time-based rotation, optional compression of rotated archives |
| TCP client | Optional authentication key, optional AES encryption |
| TCP server | Accepts remote clients and routes their messages through local writers |
| Syslog | Unix only (`SyslogWriter`) |
| EventLog | Windows only (`EventLogWriter`, same type names) |
| Callback | Calls your closure for every message |
| OpenTelemetry | OTLP/HTTP, batches posted to `{endpoint}/v1/logs` |

## Benchmarks

Every figure below comes from raw data committed in this repository
([`docs/benchmarks`](docs/benchmarks)); the interactive version with charts and tables is
published at **[brmmm3.github.io/fastlogging-rs](https://brmmm3.github.io/fastlogging-rs/)**.

**Test conditions.** Linux, Ryzen 5, Ubuntu 26.04, Python 3.13, release build.
12,500 messages per measurement (500 iterations × 25 messages, cycling all five levels), single
producer thread, written to a real file; the rotating variant rotates at 1 MiB keeping 8
backups. Best of 5 runs — reproduce with:

```bash
cd pyfastlogging && ./.venv/bin/python ../tools/readme_benchmark.py
```

| Scenario | Python stdlib `logging` | `pyfastlogging` |
|---|---|---|
| Plain file | ~0.19–0.24 s | ~0.022–0.028 s |
| Rotating file (1 MiB, 8 backups) | ~0.25 s | ~0.022 s |

The `pyfastlogging` timings were stable across repeated runs; the stdlib timings varied by
roughly ±30 % run to run, so treat the ~8–12× gap as the meaningful signal rather than the
exact decimals.

The committed data additionally covers log4j and log4j2 (Java) plus the C, C++ and Go bindings.
From [`linux_log4j.json`](docs/benchmarks/linux_log4j.json) and
[`linux_jfastlogging.json`](docs/benchmarks/linux_jfastlogging.json) — long messages, no
exceptions, `INFO`, plain file: **log4j 1.532 s vs jfastlogging 0.148 s**.

Regenerating everything:

```bash
# Rust (Criterion)
cargo bench -p fastlogging

# Python (writes pyfastlogging/doc/benchmarks/python_<platform>.json)
cd pyfastlogging && ./.venv/bin/python benches/benchmark.py

# C and C++ (vs zlog and loguru) — build the native libs first
make -C cfastlogging/benches run
make -C cppfastlogging/benches run

# C# (vs Serilog and NLog)
dotnet run --project csharpfastlogging/benchmarks/CSharpBenchmark.csproj -- 5000

# Charts and the HTML overview from the raw JSON
cd docs/benchmarks && python3 benchmarks2charts.py && python3 benchmarks2html.py
```

Absolute numbers depend on hardware, filesystem, log level and message size; treat them as
indicative, and reproduce them on your own machine before drawing conclusions.

## Configuration

Besides the API, the full configuration (writers, levels, extended settings) can be saved to and
loaded from a file. Supported formats are JSON, XML and YAML; the file must be named
`fastlogging.<EXT>` and live in the current working directory, or its path must be set in the
`FASTLOGGING_CONFIG_FILE` environment variable.

Example configuration files (minimal and full) are in [`docs/configs`](docs/configs):

```json
{
  "level": 10,
  "domain": "root",
  "console": { "level": 40, "colors": true },
  "file": { "level": 10, "path": "/tmp/app.log", "size": 1048576, "backlog": 4 }
}
```

Config-file support is behind cargo features — `config_json`, `config_xml` and `config_yaml`, all
enabled by default. For a dependency-light build:

```toml
fastlogging = { version = "0.9", default-features = false }
```

## Supported versions and platforms

| Item | Value |
|---|---|
| Rust | Edition 2024, so **Rust 1.85 or newer**. CI builds and tests on nightly. |
| Python | 3.10 – 3.14 and PyPy 3.11; wheels for CPython and PyPy |
| Operating systems | Linux (glibc and musl), Windows, macOS |
| Linux wheel targets | x86_64, x86 (i686), aarch64, armv7, s390x, ppc64le |
| Java | Requires **JDK 25** and Maven 3.9+ (the FFM bindings use the `java.lang.foreign` API, final since JDK 22) |
| License | `MIT OR Apache-2.0` — [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) |

### Known limitations

- `SyslogWriter` / `SyslogWriterConfig` exist on **Unix** only. On Windows the equivalent is
  `eventlog`-backed and exposed under the same type names.
- Only the `fastlogging` crate is published to [crates.io](https://crates.io/crates/fastlogging);
  the bindings are used as a path or git dependency and built from source.
- The Python wheels target glibc 2.34 (`manylinux_2_34`); on older distributions build from source.
- Sub-process forwarding relies on a port file in the system temp directory. This file contains the port number for the sub-process to connect to. It is enabled by
  default and can be turned off.
- `jfastlogging` has no Java wrapper class for the callback writer, and only partial syslog support.

## FAQ / troubleshooting

**Nothing shows up in the log file.**
Check the level: it is compared against both the logger level and the writer level, so a writer
set to `WARNING` will not display `INFO` messages. Also confirm the writer is not disabled.

**Messages appear only on shutdown.**
Writers are asynchronous. Call `shutdown(false)` (or `sync_all(timeout)`) before your process
exits, otherwise buffered messages can be lost.

**Which Rust version do I need?**
Edition 2024 requires Rust 1.85 or newer. There is no `rust-version` field in `Cargo.toml`, so
Cargo will not warn you if you are too old — the build will simply fail to parse.

**Where does my configuration file go?**
Name it `fastlogging.<json|xml|yaml>` and place it in the working directory, or point
`FASTLOGGING_CONFIG_FILE` at it.

**`cargo build` fails in `cppfastlogging` or `gofastlogging`.**
Both are outside the Cargo workspace and must be built with `make`. Run `cargo build -p cfastlogging`
first, then `make build-debug` / `make build` in the binding directory — build order matters.

**Python: `ImportError` for `pyfastlogging`.**
The import name is `pyfastlogging` (the package name), not `fastlogging`. Python 3.10 or newer
is required.

**Is this thread-safe / safe from subprocesses?**
Log calls are thread-safe. For sub-processes use the process-wide root logger instead — `ROOT_LOGGER`
in Rust, or the module-level functions in Python (`fl.info(...)`, `fl.add_writer(...)`). It
automatically forwards messages to a parent process if one is running.

## Documentation

- [Rust guide](fastlogging/doc/README.md) · [levels](fastlogging/doc/LEVELS.md) ·
  [`Logging`](fastlogging/doc/LOGGING.md) · [`Logger`](fastlogging/doc/LOGGER.md) ·
  [writers](fastlogging/doc/WRITERS.md) · [networking](fastlogging/doc/NETWORK.md) ·
  [configuration](fastlogging/doc/CONFIG.md) · [root logger](fastlogging/doc/ROOT.md) ·
  [examples](fastlogging/doc/EXAMPLES.md)
- Generated API reference: **[docs.rs/fastlogging](https://docs.rs/fastlogging)**
- Examples: [`fastlogging/examples`](fastlogging/examples) and each binding's `examples/` directory

## Project links

- **Changelog**: [CHANGELOG.md](CHANGELOG.md) (plus per-binding changelogs)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **License**: [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) — your choice
- **Benchmarks overview**: [brmmm3.github.io/fastlogging-rs](https://brmmm3.github.io/fastlogging-rs/)
- **Funding**: [GitHub Sponsors](https://github.com/sponsors/brmmm3) · [Ko-fi](https://ko-fi.com/brmmm3)

## A note to LLM usage

Parts of this crate are created by an LLM. I want to give you an overview:

- The core and the Python wrapper are 100% hand coded.
- The wrappers for the other programming languages, are more than 50% hand coded.
- The documentation is around 50% hand written.
