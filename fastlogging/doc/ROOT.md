# Root Logger

`fastlogging` provides a process-wide singleton logger initialised automatically
on first access.  It is suitable for applications that want a zero-setup, global
logger without managing a `Logging` instance manually.

## `ROOT_LOGGER`

```rust
pub static ROOT_LOGGER: Lazy<RwLock<Logging>>
```

The root logger is a `Lazy<RwLock<Logging>>`. It is initialised the first time it
is accessed and performs the following setup automatically:

1. Creates a `Logging` instance with a `ServerConfig` root writer on `127.0.0.1`
   (OS-assigned port) with `AuthKey` encryption.
2. Writes the bound port to a temp file so child processes (spawned via `fork` or
   `std::process::Command`) can detect and connect to it.
3. Checks whether the *parent* process has a running `ROOT_LOGGER` server by
   reading the parent's temp file, and if so adds a `ClientWriter` pointing to it.
4. Falls back to a `ConsoleWriter` if no parent server is found, or loads a
   default config file if one exists.

### Usage

```rust
use fastlogging::{LoggingError, ROOT_LOGGER};

fn main() -> Result<(), LoggingError> {
    let log = ROOT_LOGGER.read().unwrap();
    log.trace("Trace Message")?;
    log.debug("Debug Message")?;
    log.info("Info Message")?;
    log.success("Success Message")?;
    log.error("Error Message")?;
    log.fatal("Fatal Message")?;
    log.sync_all(1.0)?;
    Ok(())
}
```

## `root` Module (Free Functions)

The `root` module exposes free functions that delegate to `ROOT_LOGGER` without
exposing the mutex guard.

```rust
use fastlogging::root;

// Initialise explicitly (optional — happens automatically on first access)
root::root_init();

// Configuration
root::shutdown(false)?;
root::set_level(wid, level)?;
root::set_domain(domain)?;
root::set_ext_config(ext_config)?;

// Writer management
root::add_writer_config(&config)?;
root::remove_writer(wid)?;
root::enable(wid)?;
root::disable(wid)?;

// Sync
root::sync(vec![WriterTypeEnum::Console], 5.0)?;
root::sync_all(5.0)?;

// Logging
root::trace("message")?;
root::debug("message")?;
root::info("message")?;
root::success("message")?;
root::warning("message")?;
root::error("message")?;
root::critical("message")?;
root::fatal("message")?;
root::exception("message")?;
```

## Parent-Child Process Logging

When a child process is started (e.g. via `std::process::Command::spawn` or
`fork(2)` on Unix), the child's `ROOT_LOGGER` will detect the parent's server
port file in the system temp directory and automatically connect its
`ClientWriter` to the parent's `LoggingServer`.  All log messages from the child
are then forwarded to the parent's writers (file, console, etc.) with no extra
setup required.

```rust
// examples/spawn.rs pattern
use fastlogging::{LoggingError, ROOT_LOGGER};

fn main() -> Result<(), LoggingError> {
    {
        let log = ROOT_LOGGER.read().unwrap();
        log.info("parent starting")?;
    }
    let child = std::process::Command::new(std::env::current_exe()?)
        .arg("--child")
        .spawn()?;
    // Child's ROOT_LOGGER detects parent and forwards log messages.
    child.wait_with_output()?;
    let log = ROOT_LOGGER.read().unwrap();
    log.info("parent done")?;
    log.sync_all(1.0)?;
    Ok(())
}
```

## Thread Safety

`ROOT_LOGGER` is protected by a `RwLock`. Lock it for each group of related calls
and drop the guard promptly to avoid contention:

```rust
// Good — lock, log, release
{
    let log = ROOT_LOGGER.read().unwrap();
    log.info("one")?;
    log.info("two")?;
} // guard dropped here

// Also fine — explicit drop
let log = ROOT_LOGGER.read().unwrap();
log.info("three")?;
drop(log);
```

For high-frequency logging from multiple threads, create a dedicated `Logging`
instance with a `ClientWriter` pointing to the root server, so the mutex is not
a bottleneck.
