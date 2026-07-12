# Configuration

## `ExtConfig` — Extended Message Metadata

`ExtConfig` controls what process/thread metadata is appended to every log message.

```rust
pub struct ExtConfig {
    pub structured: MessageStructEnum, // String (default), Json, or Xml
    pub hostname:   bool,              // include system hostname
    pub pname:      bool,              // include process name (argv[0])
    pub pid:        bool,              // include process ID
    pub tname:      bool,              // include calling thread name
    pub tid:        bool,              // include calling thread ID
}
```

ExtConfig::new`

```rust
pub fn new(
    structured: MessageStructEnum,
    hostname: bool,
    pname:    bool,
    pid:      bool,
    tname:    bool,
    tid:      bool,
) -> Self
```

```rust
use fastlogging::{ExtConfig, MessageStructEnum};

let cfg = ExtConfig::new(
    MessageStructEnum::Json,
    true,  // hostname
    true,  // pname
    true,  // pid
    false, // tname
    false, // tid
);
```

### `MessageStructEnum`

| Variant | Effect |
|---|---|
| `String` *(default)* | Plain text messages |
| `Json` | Messages formatted as JSON |
| `Xml` | Messages formatted as XML |

### Applying to a Logger

```rust
use fastlogging::{ConsoleWriterConfig, DEBUG, ExtConfig, Logging, LoggingError, MessageStructEnum};

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        DEBUG, "app",
        Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
        Some(ExtConfig::new(MessageStructEnum::Json, true, true, true, true, true)),
        None,
    )?;
    log.info("structured message")?;
    log.shutdown(false)?;
    Ok(())
}
```

`set_ext_config` can also be called after construction:

```rust
log.set_ext_config(&ExtConfig::new(MessageStructEnum::Xml, false, false, true, true, false));
```

---

## File-Based Configuration

`fastlogging` can save and load its full configuration (writers, levels, extended
settings) to/from JSON, YAML, or XML files.  The file format is detected by
extension.

### Saving

```rust
use fastlogging::{Logging, LoggingError};
use std::path::Path;

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::default();
    log.save_config(Some(Path::new("/tmp/fastlogging.json")))?;
    log.save_config(Some(Path::new("/tmp/fastlogging.yaml")))?;
    log.save_config(Some(Path::new("/tmp/fastlogging.xml")))?;
    log.shutdown(false)?;
    Ok(())
}
```

### Loading at construction

Pass the path as the fifth argument to `Logging::new`:

```rust
use fastlogging::{Logging, LoggingError};
use std::path::PathBuf;

fn main() -> Result<(), LoggingError> {
    let mut log = Logging::new(
        fastlogging::NOTSET,
        "app",
        None,                                              // no inline writers
        None,
        Some(PathBuf::from("/etc/myapp/logging.json")),   // load from file
    )?;
    log.info("loaded from config")?;
    log.shutdown(false)?;
    Ok(())
}
```

### Reloading at runtime

```rust
use std::path::Path;

log.apply_config(Path::new("/tmp/fastlogging-updated.json"))?;
```

### `FileMerge` Semantics

When `apply_config` is called the file is merged with the running configuration
according to the internal `FileMerge` policy:

| Variant | Behaviour |
|---|---|
| `Replace` | Discard current config, use file entirely |
| `Merge` | Add only writers not already present |
| `MergeReplace` | Add new writers *and* replace existing ones |

### Config File Structure (JSON example)

```json
{
  "level": 10,
  "domain": "app",
  "hostname": null,
  "pname": "",
  "pid": 0,
  "tname": false,
  "tid": false,
  "structured": "String",
  "level2sym": "Str",
  "configs": [
    { "Console": { "enabled": true, "level": 10, "colors": true,
                   "target": "StdOut", "debug": 0 } },
    { "File":    { "enabled": true, "level": 10, "path": "/tmp/app.log" } }
  ]
}
```

### Required Crate Features

| File extension | Feature flag |
|---|---|
| `.json` | `config_json` *(on by default)* |
| `.yaml` | `config_yaml` *(on by default)* |
| `.xml`  | `config_xml`  *(on by default)* |
