# Examples

All examples are built against `libcxxfastlogging.a` produced by
`cargo build -p cxxfastlogging`.  See the [README](README.md) for the exact
`g++` command and Makefile.

---

## 1. Default Logger

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    auto log = Logging::new_default();
    log->info("Hello from cxxfastlogging!");
    log->shutdown(false);
    return 0;
}
```

2. Console Logger

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_console(DEBUG, true));

    auto log = Logging::create(DEBUG, "root", std::move(configs));
    log->trace("Trace Message");
    log->debug("Debug Message");
    log->info("Info Message");
    log->success("Success Message");
    log->warning("Warning Message");
    log->error("Error Message");
    log->fatal("Fatal Message");
    log->shutdown(false);
    return 0;
}
```

---

## 3. File Logger with Rotation

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_file(
        DEBUG,
        "/tmp/app.log",
        5ULL * 1024 * 1024,          // rotate at 5 MB
        4,                           // keep 4 backups
        86400,                       // also rotate after 24 h
        -1,                          // no absolute-time trigger
        CompressionMethodEnum::Zstd
    ));

    auto log = Logging::create(DEBUG, "root", std::move(configs));
    log->info("writing to file");
    log->rotate("");      // force rotation of all file writers
    log->shutdown(false);
    return 0;
}
```

---

## 4. Add a Writer After Construction

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    auto log = Logging::create(DEBUG, "root", {});
    log->add_writer_config(WriterConfig::new_console(DEBUG, true));
    log->info("Info Message");
    log->shutdown(false);
    return 0;
}
```

---

## 5. Callback Writer

```cpp
#include "cxxfastlogging/h/fastlogging.h"
#include <cstdio>

void my_sink(int level, const char* domain, size_t domain_len,
             const char* message, size_t message_len) {
    printf("[%d] %.*s: %.*s\n",
           level,
           (int)domain_len,  domain,
           (int)message_len, message);
}

int main() {
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_syslog(DEBUG, "", "myapp", 0));
    // Callback writers are registered through the Rust side;
    // use the root_* free functions or the callbackWriterConfigNew FFM entry point.
    auto log = Logging::create(DEBUG, "root", std::move(configs));
    log->error("something went wrong");
    log->shutdown(false);
    return 0;
}
```

---

## 6. Extended Config (Structured Logging)

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_console(DEBUG, true));

    auto log = Logging::create(DEBUG, "root", std::move(configs));
    log->set_ext_config(ExtConfigFfi {
        MessageStructEnum::Json,
        true,   // hostname
        true,   // process name
        true,   // pid
        true,   // thread name
        true,   // thread id
    });
    log->info("structured JSON message");
    log->shutdown(false);
    return 0;
}
```

---

## 7. Multi-threaded Logging with `Logger`

```cpp
#include "cxxfastlogging/h/fastlogging.h"
#include <thread>

int main() {
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_console(DEBUG, true));
    auto logging = Logging::create(DEBUG, "main", std::move(configs));

    logging->set_ext_config(ExtConfigFfi {
        MessageStructEnum::String, true, true, true, true, true
    });

    // Create and register before spawning
    auto logger = Logger::new_ext(DEBUG, "worker", true, true);
    logging->add_logger(*logger);

    std::thread worker([lg = std::move(logger)]() mutable {
        lg->trace("Trace Message");
        lg->info("Info Message");
        lg->error("Error Message");
    });

    logging->trace("Trace Message");
    logging->info("Info Message");
    logging->error("Error Message");

    worker.join();
    logging->shutdown(false);
    return 0;
}
```

---

## 8. Network Logging (Client / Server)

```cpp
#include "cxxfastlogging/h/fastlogging.h"
#include <thread>
#include <chrono>

int main() {
    // ── Server ───────────────────────────────────────────────────────
    rust::Vec<rust::Box<WriterConfig>> srv_configs;
    srv_configs.push_back(WriterConfig::new_console(DEBUG, true));
    srv_configs.push_back(WriterConfig::new_file(
        DEBUG, "/tmp/net.log", 0, 0, -1, -1, CompressionMethodEnum::Store));

    auto srv = Logging::create(DEBUG, "SERVER", std::move(srv_configs));
    srv->set_root_writer_config(
        WriterConfig::new_server(DEBUG, "127.0.0.1",
                                  EncryptionMethodEnum::NONE, {}));
    srv->sync_all(5.0);

    // ── Client ───────────────────────────────────────────────────────
    auto addr = srv->get_root_server_address_port();
    auto key  = srv->get_server_auth_key();

    rust::Vec<rust::Box<WriterConfig>> cli_configs;
    cli_configs.push_back(WriterConfig::new_client(
        DEBUG, addr,
        EncryptionMethodEnum::AuthKey,
        rust::Slice<const uint8_t>(key.data(), key.size())));

    auto cli = Logging::create(DEBUG, "CLIENT", std::move(cli_configs));

    cli->info("hello from client");
    srv->info("hello from server");

    cli->sync_all(1.0);
    srv->sync_all(1.0);
    std::this_thread::sleep_for(std::chrono::milliseconds(50));
    cli->shutdown(false);
    srv->shutdown(false);
    return 0;
}
```

---

## 9. Root Logger

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    root_init();
    root_info("Hello from root logger");
    root_error("Something went wrong");
    root_sync_all(1.0);
    root_shutdown(false);
    return 0;
}
```

---

## 10. Save and Load Config File

```cpp
#include "cxxfastlogging/h/fastlogging.h"

int main() {
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_console(ERROR, true));
    configs.push_back(WriterConfig::new_file(
        DEBUG, "/tmp/app.log",
        10ULL * 1024 * 1024, 4, -1, -1, CompressionMethodEnum::Store));

    auto log = Logging::create(DEBUG, "app", std::move(configs));
    log->set_ext_config(ExtConfigFfi {
        MessageStructEnum::String, true, true, true, false, false
    });

    // Save in multiple formats
    log->save_config("/tmp/app.json");
    log->save_config("/tmp/app.yaml");
    log->shutdown(false);

    // Reload
    auto log2 = Logging::create(NOTSET, "app", {});
    log2->apply_config("/tmp/app.json");
    log2->info("loaded from file");
    log2->shutdown(false);
    return 0;
}
```
