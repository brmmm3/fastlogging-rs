# Log Levels

## Level Constants

All log level constants are defined in `h/def.hpp` in the global namespace as
`static constexpr uint8_t`:

| Constant    | Value | Description                                  |
|-------------|-------|----------------------------------------------|
| `NOLOG`     | 100   | Suppress all logging output                  |
| `EXCEPTION` | 60    | Exception-level messages                     |
| `CRITICAL`  | 50    | Critical conditions                          |
| `FATAL`     | 50    | Alias of `CRITICAL`                          |
| `ERROR`     | 40    | Error conditions                             |
| `WARNING`   | 30    | Warning conditions                           |
| `WARN`      | 30    | Alias of `WARNING`                           |
| `SUCCESS`   | 25    | Success messages                             |
| `INFO`      | 20    | Informational messages                       |
| `DEBUG`     | 10    | Debug-level messages                         |
| `TRACE`     | 5     | Fine-grained trace messages                  |
| `NOTSET`    | 0     | Log everything (no filtering)                |

## Level Filtering

A message is emitted by a writer if:

```text
message_level >= writer_level
```

Equivalently, a writer with level `L` will output all messages whose level is
**at least** `L`. Since the constants increase with severity, a lower numeric
value means more verbose output.

### Examples

| Writer Level | Emits | Suppresses |
|---|---|---|
| `TRACE` (5) | All levels | Nothing |
| `DEBUG` (10) | DEBUG, INFO, SUCCESS, WARNING, ERROR, CRITICAL, EXCEPTION | TRACE |
| `INFO` (20) | INFO, SUCCESS, WARNING, ERROR, CRITICAL, EXCEPTION | TRACE, DEBUG |
| `WARNING` (30) | WARNING, ERROR, CRITICAL, EXCEPTION | TRACE, DEBUG, INFO, SUCCESS |
| `ERROR` (40) | ERROR, CRITICAL, EXCEPTION | Everything below ERROR |
| `NOLOG` (100) | Nothing | Everything |
| `NOTSET` (0) | All levels | Nothing |

### Per-Writer vs. Per-Logger Levels

- **`Logging::set_level(uint32_t wid, uint8_t level)`** sets the level for a
  specific writer (`wid` = 0 for the root writer, or the ID returned by
  `add_writer_config`).
- **`Logger::set_level(uint8_t level)`** sets the level for a specific `Logger`
  instance. Messages below this level are filtered **locally** before being
  dispatched to the `Logging` instance — this is an efficient hot-path
  optimization.

## Level Aliases

Two aliases exist for convenience:

- `FATAL` = `CRITICAL` (50)
- `WARN` = `WARNING` (30)

The log methods `warn()` and `warning()` are also aliases — they are identical.

## Level Symbols (`rust::LevelSyms`)

The `rust::LevelSyms` enum controls how log levels are rendered in output:

| Value   | Integer | Description                    |
|---------|---------|--------------------------------|
| `Sym`   | 0       | Symbol representation          |
| `Short` | 1       | Short string representation    |
| `Str`   | 2       | Full string representation     |

Set via `Logging::set_level2sym(uint8_t level2sym)`:

```cpp
logging.set_level2sym(static_cast<uint8_t>(rust::LevelSyms::Str));
```

## Example

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(INFO, true));

    // This message is emitted (INFO >= INFO)
    logging.info("This will be printed");

    // This message is suppressed (DEBUG < INFO)
    logging.debug("This will NOT be printed");

    return 0;
}
```
