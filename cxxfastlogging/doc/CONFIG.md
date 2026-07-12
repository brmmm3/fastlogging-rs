# Configuration

## `ExtConfigFfi` — Extended Message Metadata

`ExtConfigFfi` controls what process/thread metadata is appended to every log
message.  It is a plain C++ struct (shared across the cxx FFI boundary):

```cpp
struct ExtConfigFfi {
    MessageStructEnum structured;  // String (default), Json, or Xml
    bool              hostname;    // include system hostname
    bool              pname;       // include process name (argv[0])
    bool              pid;         // include process ID
    bool              tname;       // include calling thread name
    bool              tid;         // include calling thread ID
};
```

MessageStructEnum`

```cpp
enum class MessageStructEnum : uint8_t {
    String = 0,   // plain text (default)
    Json   = 1,   // JSON-structured messages
    Xml    = 2,   // XML-structured messages
};
```

### Passing at Construction

```cpp
ExtConfigFfi ext {
    MessageStructEnum::Json,
    true,   // hostname
    true,   // pname
    true,   // pid
    false,  // tname
    false,  // tid
};

rust::Vec<rust::Box<WriterConfig>> configs;
configs.push_back(WriterConfig::new_console(DEBUG, true));
auto log = Logging::create(DEBUG, "app", std::move(configs));
log->set_ext_config(ext);
```

### Setting After Construction

```cpp
log->set_ext_config(ExtConfigFfi {
    MessageStructEnum::Xml, false, false, true, true, false
});
```

---

## File-Based Configuration

`cxxfastlogging` inherits `fastlogging`'s ability to save and reload its full
configuration (writers, levels, extended settings) to/from JSON, YAML, or XML.
The file format is detected by extension.

### Saving

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    auto log = Logging::new_default();
    log->save_config("/tmp/fastlogging.json");
    log->save_config("/tmp/fastlogging.yaml");
    log->save_config("/tmp/fastlogging.xml");
    log->shutdown(false);
    return 0;
}
```

### Reloading at Runtime

```cpp
log->apply_config("/tmp/fastlogging-updated.json");
```

### Config File Structure (JSON example)

The JSON layout mirrors the Rust serialisation format:

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

### Required Rust Features

File-based configuration requires the corresponding Rust crate features to be
enabled when `cxxfastlogging` was compiled:

| Extension | Rust feature |
|---|---|
| `.json` | `config_json` *(on by default)* |
| `.yaml` | `config_yaml` *(on by default)* |
| `.xml`  | `config_xml`  *(on by default)* |

These are all enabled by default, so no special action is needed unless
`cxxfastlogging` was compiled with `--no-default-features`.
