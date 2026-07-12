# Log Levels

`cxxfastlogging` uses `uint8_t` level values, identical to the underlying Rust
library.  A message is forwarded to a writer only when
`message_level >= writer_level`.  Lower values are *more* verbose.

## Constants

The level constants are defined as `constexpr uint8_t` in `def.hpp`
(automatically included via `cxxfastlogging/h/fastlogging.h`):

| Constant | Value | Description |
|---|---|---|
| `NOTSET` | `0` | No filter — every message passes |
| `TRACE` | `5` | Fine-grained diagnostic trace |
| `DEBUG` | `10` | Developer debugging information |
| `INFO` | `20` | Routine informational messages |
| `SUCCESS` | `25` | Operation completed successfully |
| `WARNING` / `WARN` | `30` | Unexpected but recoverable situation |
| `ERROR` | `40` | Error that should be investigated |
| `CRITICAL` / `FATAL` | `50` | Severe error; program may not continue |
| `EXCEPTION` | `60` | Unhandled exception |
| `NOLOG` | `100` | Silence all output |

`FATAL` is an alias for `CRITICAL`.  `WARN` is an alias for `WARNING`.

```cpp
#include "cxxfastlogging/h/fastlogging.h"

// Use level constants directly
auto log = Logging::create(DEBUG, "app", {});
log->info("level check uses uint8_t comparison");
```

LevelSymsEnum`

Controls how level names appear in formatted log messages:

| Value | Example output |
|---|---|
| `LevelSymsEnum::Sym` | `"D"`, `"W"`, `"F"` |
| `LevelSymsEnum::Short` | `"DBG"`, `"WRN"`, `"FTL"` |
| `LevelSymsEnum::Str` *(default)* | `"DEBUG"`, `"WARNING"`, `"FATAL"` |

```cpp
log->set_level2sym(LevelSymsEnum::Short);
```

## Per-writer Level Filtering

The global level on `Logging` is a gate: messages below it are dropped before
they even enter the internal channel.  Each writer also has its own level, so
you can write `DEBUG` to a file while showing only `WARNING` and above on the
console:

```cpp
rust::Vec<rust::Box<WriterConfig>> configs;
configs.push_back(WriterConfig::new_console(WARNING, false));   // console: WARNING+
configs.push_back(WriterConfig::new_file(
    DEBUG, "/tmp/app.log", 0, 0, -1, -1, CompressionMethodEnum::Store));  // file: DEBUG+

auto log = Logging::create(DEBUG, "app", std::move(configs));
log->debug("only in file");
log->warning("console and file");
log->shutdown(false);
```
