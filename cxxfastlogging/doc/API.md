# cxxfastlogging API Documentation

C++ bindings for `fastlogging`, generated via `cxx` from `src/lib.rs`.

## Types

- `WriterConfig` — opaque, built via `WriterConfig::new_console/new_file/new_client/new_server/new_syslog`.
- `Logging` — opaque, the main entry point (`Logging::new_default()`, `Logging::new(level, domain, configs)`, plus methods for shutdown, writer management, sync/rotate, and the `trace`..`exception` log calls).
- `Logger` — opaque, lightweight handle registered via `Logging::add_logger`/`root_add_logger`.
- `root_*` free functions mirror `fastlogging::root` (the process-wide singleton logger).

## Shared enums/structs

`EncryptionMethodEnum`, `CompressionMethodEnum`, `MessageStructEnum`, `LevelSymsEnum`,
`WriterTypeTag`, `ExtConfigFfi`, `ServerConfigInfo`, `IdString`, `IdU16` — see `src/lib.rs`
for exact fields; they mirror the corresponding `fastlogging` types.

## C++ example

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    auto console = WriterConfig::new_console(10 /* DEBUG */, true);
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(std::move(console));
    auto logging = Logging::create(0 /* NOTSET */, "root", std::move(configs));
    logging->info("Hello from C++!");
    logging->shutdown(false);
    return 0;
}
```
