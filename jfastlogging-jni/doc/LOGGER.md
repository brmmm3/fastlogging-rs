# Logger

The `Logger` class is a non-static inner class of `FastLogging`: `FastLogging.Logger`. It represents a per-thread or per-domain logging handle attached to a `Logging` instance via `addLogger()`.

Note: Because `Logger` is a non-static inner class, you need a `FastLogging` instance to create one. In practice, since `FastLogging` uses `System.loadLibrary` in its static initializer, you typically just need to reference the class first.

## Constructors

| Constructor | Description |
|---|---|
| `Logger()` | Default: level NOTSET, domain null |
| `Logger(int level)` | Level, domain null |
| `Logger(String domain)` | Level NOTSET, domain set |
| `Logger(int level, String domain)` | Level + domain |
| `Logger(int level, String domain, boolean tname, boolean tid)` | Level + domain + thread name/id logging |

Example:

```java
FastLogging fastLogging = new FastLogging(); // triggers static initializer
FastLogging.Logger logger = fastLogging.new Logger(FastLogging.DEBUG, "WorkerThread", true, true);
```

Note: Creating a `FastLogging` instance is not typically necessary (the class only has a static initializer for loading the native library). You can also create a `Logger` from within a `Logging` context or by directly calling the constructor if you have a `FastLogging` instance.

## Methods

- `void setLevel(int level)` — set log level
- `void setDomain(String domain)` — set log domain

### Logging methods

All do client-side level checking. Each takes a `String message`.

| Method | Level check |
|---|---|
| `void trace(String message)` | `instance_level <= TRACE` |
| `void debug(String message)` | `instance_level <= DEBUG` |
| `void info(String message)` | `instance_level <= INFO` |
| `void success(String message)` | `instance_level <= SUCCESS` |
| `void warning(String message)` | `instance_level <= WARN` |
| `void error(String message)` | `instance_level <= ERROR` |
| `void critical(String message)` | `instance_level <= CRITICAL` |
| `void fatal(String message)` | `instance_level <= FATAL` |
| `void exception(String message)` | `instance_level <= EXCEPTION` |

### Attaching to a Logging instance

```java
ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
Logging logging = new Logging(FastLogging.DEBUG, "root", console);

FastLogging fastLogging = new FastLogging();
FastLogging.Logger logger = fastLogging.new Logger(FastLogging.DEBUG, "WorkerThread");
logging.addLogger(logger.instance_ptr);

// In another thread:
logger.info("Message from worker thread");

logging.shutdown();
```

**Note:** `addLogger` and `removeLogger` take `long loggerPtr` — pass `logger.instance_ptr`.
