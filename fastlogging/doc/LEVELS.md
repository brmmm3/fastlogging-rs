# Log Levels

`fastlogging` uses plain `u8` values for log levels. A message is forwarded to
a writer only when `message_level >= writer.level`. Lower numeric values are
*more* verbose.

## Constants

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

```rust
use fastlogging::{NOTSET, TRACE, DEBUG, INFO, SUCCESS, WARNING, ERROR, FATAL, NOLOG};
```

## Helper Functions

level2str(level: u8) -> &'static str`

Returns the full level name as a `&str` (e.g. `"DEBUG"`, `"WARNING"`, `"FATAL"`).

```rust
use fastlogging::{level2str, DEBUG, WARNING};
assert_eq!(level2str(DEBUG),   "DEBUG");
assert_eq!(level2str(WARNING), "WARNING");
```

level2short(level: u8) -> &'static str`

Returns a 3-character abbreviation: `"TRC"`, `"DBG"`, `"INF"`, `"SCS"`, `"WRN"`,
`"ERR"`, `"FTL"`, `"EXC"`, …

### `level2sym(level: u8) -> &'static str`

Returns a single-character symbol: `"T"`, `"D"`, `"I"`, `"S"`, `"W"`, `"E"`,
`"F"`, `"!"`, `"-"`.

### `level2string(levelsym: &LevelSyms, level: u8) -> &'static str`

Dispatches to one of the three functions above depending on the `LevelSyms` value.

## LevelSyms Enum

Controls how level names appear in formatted messages.

| Variant | Example output |
|---|---|
| `LevelSyms::Sym` | `"D"`, `"W"`, `"F"` |
| `LevelSyms::Short` | `"DBG"`, `"WRN"`, `"FTL"` |
| `LevelSyms::Str` *(default)* | `"DEBUG"`, `"WARNING"`, `"FATAL" |

```rust
use fastlogging::{LevelSyms, Logging};

let mut log = Logging::default();
log.set_level2sym(&LevelSyms::Short);
```

## Per-writer Level Filtering

Each writer carries its own level, independent of the global level.  A common
pattern is to write everything to a file but show only warnings on the console:

```rust
use fastlogging::{
    ConsoleWriterConfig, DEBUG, FileWriterConfig, Logging, LoggingError, WARNING,
};
use std::path::PathBuf;

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG,                                                    // global: pass DEBUG+
        "app",
        Some(vec![
            ConsoleWriterConfig::new(WARNING, false).into(),     // console: WARNING+
            FileWriterConfig::new(DEBUG,
                PathBuf::from("/tmp/app.log"), 0, 0, None, None, None)?.into(), // file: DEBUG+
        ]),
        None,
        None,
    )?;
    log.debug("only in file")?;
    log.warning("console and file")?;
    log.shutdown(false)?;
    Ok(())
}
```
