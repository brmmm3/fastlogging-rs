# Log Levels

This documents the log levels exposed by the Go wrapper around the `fastlogging` Rust library. The top-level package is `gofastlogging/fastlogging`, conventionally imported as `fl`.

## Level constants

All level constants are untyped `int` values originating from C macros. A logger configured at level `L` emits any message whose level is `<= L`; therefore a **lower numeric value means more verbose** output.

| Constant | Value | Color | Description |
| --- | --- | --- | --- |
| `NOLOG` | 100 | — | No logging at all |
| `EXCEPTION` | 60 | — | Log exception messages (includes traceback — slow) |
| `CRITICAL` | 50 | bright red | Log critical messages |
| `FATAL` | 50 | bright red | Same as `CRITICAL` |
| `ERROR` | 40 | red | Log error messages |
| `WARNING` | 30 | bright yellow | Log warning messages |
| `WARN` | 30 | bright yellow | Same as `WARNING` |
| `SUCCESS` | 25 | — | Success messages |
| `INFO` | 20 | bright green | Log info messages |
| `DEBUG` | 10 | white | Log debug messages |
| `TRACE` | 5 | — | Trace messages |
| `NOTSET` | 0 | — | All messages are logged |

## Per-writer filtering

Each writer carries its own level filter, set via `SetLevel(wid uint32, level uint8)`. The root logger / `Logging` instance additionally has a global level. A message must pass **both** the logger-level check (the hot path) and the per-writer level check before it reaches a given writer. This makes it straightforward to, for example, send `TRACE` output to a file writer while restricting the console to `ERROR` and above.

```go
console := writer.ConsoleWriterConfigNew(fl.ERROR, true)
file := writer.FileWriterConfigNew(fl.TRACE, "/tmp/app.log", 0, 0, -1, -1, fl.Store)
writers := []fl.WriterConfigEnum{*console, *file}
log := logging.New(fl.TRACE, nil, writers, nil, nil)
// console shows only ERROR+, file shows everything
```

## Level symbols (`LevelSymbol`)

The `LevelSymbol` enum controls how a level is rendered in output. It lives in the `fl` package:

- `fl.Sym` — 1-character symbol (`!`, `F`, `E`, `W`, ...) — iota `0`
- `fl.Short` — 3-character text (`EXC`, `FTL`, `ERR`, `WRN`, ...) — iota `1`
- `fl.Str` — long text (`EXCEPTION`, `FATAL`, `ERROR`, `WARNING`, ...) — default — iota `2`

Set the active rendering style with:

```go
log.SetLevel2Sym(fl.Sym.Into())
// or
log.SetLevel2Sym(fl.Short.Into())
// or
log.SetLevel2Sym(fl.Str.Into())
```

The `.Into()` method returns the `uint8` value of the underlying C enum.

## Changing levels at runtime

- `Logging.SetLevel(wid uint32, level uint8) error` — change an individual writer's level on a `Logging` instance.
- `Logging.SetDebug(debug uint32) error` — set the developer debug level.
- `fl.SetLevel(wid, level)` — same operation as `Logging.SetLevel`, but applied to the process-wide root logger.
