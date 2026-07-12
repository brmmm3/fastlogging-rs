# Root Logger

`cxxfastlogging` exposes the `fastlogging` process-wide singleton logger via a
set of `root_*` free functions.  They are suitable for applications that want
a zero-setup, global logger without managing a `Logging` instance.

The root logger is initialised automatically on first use and:

1. Creates a `Logging` instance with a server writer on `127.0.0.1`
   (OS-assigned port) with `AuthKey` encryption.
2. Writes the bound port to a temp file so child processes can detect and
   connect to it automatically.
3. Checks whether the parent process has a running root logger server; if so,
   adds a `ClientWriter` pointing to it.
4. Falls back to a `ConsoleWriter` if no parent server is found.

## Free Functions

```cpp
#include "cxxfastlogging/h/fastlogging.h"

// Initialise explicitly (optional — happens automatically on first access)
void root_init();

// Lifecycle
void root_shutdown(bool now);               // may throw rust::Error

// Configuration
void root_set_level(uint64_t wid, uint8_t level);   // may throw
void root_set_domain(rust::Str domain);
void root_set_level2sym(LevelSymsEnum level2sym);
void root_set_ext_config(ExtConfigFfi ext_config);

// Writer management
uint64_t root_add_writer_config(rust::Box<WriterConfig> config);  // may throw
bool     root_remove_writer(uint64_t wid);
void     root_enable(uint64_t wid);    // may throw
void     root_disable(uint64_t wid);   // may throw
void     root_enable_type(WriterTypeTag tag, rust::Str data);   // may throw
void     root_disable_type(WriterTypeTag tag, rust::Str data);  // may throw
void     root_sync_type(WriterTypeTag tag, rust::Str data, double timeout);  // may throw
void     root_sync_all(double timeout);  // may throw
void     root_rotate(rust::Str path);    // may throw (empty = all files)

// Encryption
void root_set_encryption(uint64_t wid, EncryptionMethodEnum key_type,
                          rust::Slice<const uint8_t> key);  // may throw

// Queries
rust::Vec<uint8_t> root_get_server_auth_key();
rust::String       root_get_config_string();
void               root_save_config(rust::Str path);  // may throw

// Logging
void root_trace(rust::Str message);
void root_debug(rust::Str message);
void root_info(rust::Str message);
void root_success(rust::Str message);
void root_warning(rust::Str message);
void root_error(rust::Str message);
void root_critical(rust::Str message);
void root_fatal(rust::Str message);
void root_exception(rust::Str message);
```

Basic Usage

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    root_init();   // optional
    root_debug("starting up");
    root_info("ready");
    root_warning("something unexpected");
    root_error("something failed");
    root_sync_all(1.0);
    root_shutdown(false);
    return 0;
}
```

## Concurrent Access

The root logger is protected by a Rust `Mutex` internally.  Calls from
multiple C++ threads are safe; each `root_*` call locks the mutex for its
duration.  For high-frequency multi-threaded logging, prefer creating a
dedicated `Logging` instance with a `ClientWriter` pointing at the root
server to avoid mutex contention:

```cpp
// High-frequency pattern
auto key  = root_get_server_auth_key();
// (get address from root_get_config_string or keep a dedicated channel)
// Then create a per-thread Logging with a ClientWriter.
```
