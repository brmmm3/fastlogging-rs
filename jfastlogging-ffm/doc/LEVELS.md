# Log Levels

## Level constants

All are `public static final int` in `FastLogging`. A logger at level L emits messages whose level is <= L (lower = more verbose).

| Constant | Value | Color | Description |
|---|---|---|---|
| `FastLogging.NOLOG` | 100 | — | No logging |
| `FastLogging.EXCEPTION` | 60 | — | Log exception messages |
| `FastLogging.CRITICAL` | 50 | bright red | Log critical messages |
| `FastLogging.FATAL` | 50 | bright red | Same as CRITICAL |
| `FastLogging.ERROR` | 40 | red | Log error messages |
| `FastLogging.WARNING` | 30 | bright yellow | Log warning messages |
| `FastLogging.WARN` | 30 | bright yellow | Same as WARNING |
| `FastLogging.SUCCESS` | 25 | — | Success messages |
| `FastLogging.INFO` | 20 | bright green | Log info messages |
| `FastLogging.DEBUG` | 10 | white | Log debug messages |
| `FastLogging.TRACE` | 5 | — | Trace messages |
| `FastLogging.NOTSET` | 0 | — | All messages are logged |

## `Level2Sym(int level)` static method

Returns the level name as a `String` (e.g. `"DEBUG"`, `"INFO"`). Returns `"?"` for unknown levels.

## Per-writer filtering

Each writer has its own level filter set via `setLevel(WriterTypeEnum, int)`. The `Logging` instance also has an `instance_level`. The logging methods do a client-side check: `if (instance_level <= TRACE)` before calling JNI. Each writer then does its own filtering.

Example:

```java
ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.ERROR, true);
FileWriterConfig file = new FileWriterConfig(FastLogging.TRACE, "/tmp/app.log");
Logging logging = new Logging(FastLogging.TRACE, "root", console, file);
// console shows only ERROR+, file shows everything
```

Level symbols (`LevelSyms`)

Controls how levels are rendered:

- `LevelSyms.Sym` (0) — 1-char symbol
- `LevelSyms.Short` (1) — 3-char text
- `LevelSyms.Str` (2) — long text (default)

Set via:

```java
logging.setLevel2Sym(LevelSyms.Sym);
```

## Changing levels at runtime

- `logging.setLevel(WriterTypeEnum writer, int level)` — change a writer type's level
- `logging.setLevel(WriterTypeEnum writer, String key, int level)` — change a specific writer's level
