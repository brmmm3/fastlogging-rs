# cppfastlogging: Extended Configuration Documentation

This document covers the extended configuration features of cppfastlogging: the `ExtConfig` class for message formatting and metadata inclusion, the `MessageStruct` output-format enum, config-file load/save, encryption configuration, and debug output.

---

## `logging::ExtConfig`

`ExtConfig` controls how log messages are structured (plain string, JSON, or XML) and which metadata fields (hostname, process name, pid, thread name, thread id) are included in each emitted message. Defined in `logging.hpp`:

```cpp
class ExtConfig {
public:
    rust::ExtConfig *config = nullptr;

    ExtConfig(MessageStruct structured,
              int8_t hostname, int8_t pname, int8_t pid,
              int8_t tname, int8_t tid);
    ~ExtConfig();  // sets config = nullptr
};
```

### Constructor Parameters

| Parameter     | Type             | Description                                                       |
|---------------|------------------|-------------------------------------------------------------------|
| `structured`  | `MessageStruct`  | Output format: `String`, `Json`, or `Xml` (see below).            |
| `hostname`    | `int8_t`         | Include hostname? `1` = yes, `0` = no.                            |
| `pname`       | `int8_t`         | Include process name? `1` = yes, `0` = no.                        |
| `pid`         | `int8_t`         | Include process ID? `1` = yes, `0` = no.                          |
| `tname`       | `int8_t`         | Include thread name? `1` = yes, `0` = no.                         |
| `tid`         | `int8_t`         | Include thread ID? `1` = yes, `0` = no.                           |

The `config` field points to the underlying `rust::ExtConfig` struct (see below). The destructor sets `config = nullptr`; the underlying struct is managed by the Rust side once the `ExtConfig` is applied to a `Logging` instance.

### Underlying `rust::ExtConfig` Struct

Defined in `def.hpp`:

```cpp
struct ExtConfig {
    MessageStructEnum structured;
    int8_t hostname;
    int8_t pname;
    int8_t pid;
    int8_t tname;
    int8_t tid;
};
```

You can read the fields directly through the `config` pointer, e.g. `ext_config.config->structured` or `ext_config.config->hostname`.

---

## `logging::MessageStruct` Enum

Selects the output format for structured log messages. Defined in `logging.hpp`:

```cpp
enum class MessageStruct : uint8_t { String = 0, Json = 1, Xml = 2 };
```

| Value    | Integer | Description                                |
|----------|---------|--------------------------------------------|
| `String` | 0       | Plain string format (default).             |
| `Json`   | 1       | JSON-encoded structured log message.       |
| `Xml`    | 2       | XML-encoded structured log message.        |

This mirrors `rust::MessageStructEnum` in `def.hpp`. The `ExtConfig` constructor casts the C++ enum to the Rust enum internally.

---

## Passing `ExtConfig` to a `Logging` Instance

`ExtConfig` can be supplied in two ways:

### 1. Via the `Logging` Constructor

```cpp
explicit Logging(uint8_t level = NOTSET, const char *domain = nullptr,
                 ExtConfig *ext_config = nullptr,
                 const char *config_path = nullptr);
```

Pass a pointer to an `ExtConfig` as the third argument:

```cpp
ExtConfig ext(MessageStruct::Json, 1, 1, 1, 0, 0);
Logging logging(INFO, "app", &ext);
```

The constructor forwards `ext_config->config` to the Rust `logging_new` FFI.

### 2. Via `set_ext_config` After Construction

```cpp
void set_ext_config(ExtConfig *ext_config);
```

```cpp
Logging logging(INFO, "app");
ExtConfig ext(MessageStruct::Xml, 1, 0, 1, 0, 1);
logging.set_ext_config(&ext);
```

This forwards `ext_config->config` to the Rust `logging_set_ext_config` FFI, updating the formatting configuration of an already-running `Logging` instance.

---

## `ExtConfig` Example

This mirrors `examples/ext_config.cpp` and demonstrates reading fields back from the underlying `rust::ExtConfig` struct:

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>
using namespace logging;

int main() {
    ExtConfig ext_config(MessageStruct::Xml, 1, 0, 1, 0, 1);
    printf("config.structured=%d\n", (int)ext_config.config->structured);
    printf("config.hostname=%d\n", ext_config.config->hostname);
    printf("config.pname=%d\n", ext_config.config->pname);
    printf("config.pid=%d\n", ext_config.config->pid);
    printf("config.tname=%d\n", ext_config.config->tname);
    printf("config.tid=%d\n", ext_config.config->tid);
    return 0;
}
```

Expected output:

```text
config.structured=2
config.hostname=1
config.pname=0
config.pid=1
config.tname=0
config.tid=1
```

(Here `structured=2` corresponds to `MessageStruct::Xml`.)

---

## Config File Methods

A `Logging` instance can load and save its full configuration to a file. The configuration includes writer definitions, levels, domains, and other settings.

### Methods

| Method                       | Signature                     | Description                                                         |
|------------------------------|-------------------------------|---------------------------------------------------------------------|
| `apply_config`               | `int apply_config(const char *path)` | Load and apply configuration from a file. Returns `0` on success, negative on error. |
| `get_config_string`          | `const char *get_config_string()`   | Get the current configuration serialized as a string.               |
| `save_config`                | `int save_config(const char *path)` | Save the current configuration to a file. Returns `0` on success, negative on error. |
| Constructor `config_path`    | `Logging(..., const char *config_path = nullptr)` | Load configuration from `config_path` at construction time.  |

### Loading at Construction Time

```cpp
Logging logging(NOTSET, "root", nullptr, "/etc/myapp/logging.conf");
```

When `config_path` is non-null, the constructor passes it to the Rust `logging_new` FFI, which loads and applies the configuration as part of instance creation.

### Loading After Construction

```cpp
Logging logging(INFO, "app");
if (logging.apply_config("/etc/myapp/logging.conf") != 0) {
    // handle error
}
```

### Saving the Current Configuration

```cpp
Logging logging(DEBUG, "app");
logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
logging.add_writer_config(FileWriterConfig(DEBUG, "/tmp/app.log", 1024, 3));

const char *cfg = logging.get_config_string();
printf("Current config:\n%s\n", cfg);

logging.save_config("/tmp/app.conf");
```

The string returned by `get_config_string()` is owned by the Rust side and is valid until the next call or the `Logging` instance is destroyed.

---

## Encryption Configuration

Network writers (client and server) can be secured with a shared key. See [NETWORK.md](NETWORK.md) for the full network usage; this section covers the encryption primitives themselves.

### `rust::KeyStruct`

Defined in `def.hpp`:

```cpp
struct KeyStruct {
    EncryptionMethodEnum typ;  // NONE=0, AuthKey=1, AES=2
    uint32_t             len;
    const char          *key;
};
```

### `rust::EncryptionMethodEnum`

```cpp
enum class EncryptionMethodEnum : uint8_t {
    NONE    = 0,
    AuthKey = 1,
    AES     = 2
};
```

| Value     | Integer | Description                                                            |
|-----------|---------|------------------------------------------------------------------------|
| `NONE`    | 0       | No encryption.                                                         |
| `AuthKey` | 1       | Authentication only (shared-key handshake, plaintext payload).        |
| `AES`     | 2       | Authenticated + AES-encrypted payload.                                 |

### FFI Helpers

Declared in `cppfastlogging.hpp`:

```cpp
rust::KeyStruct *create_key(rust::EncryptionMethodEnum typ, uint32_t len,
                            const uint8_t *key);
rust::KeyStruct *create_random_key(rust::EncryptionMethodEnum typ);
```

- `create_key(typ, len, key)` — Build a `KeyStruct` from `len` raw key bytes pointed to by `key`.
- `create_random_key(typ)` — Build a `KeyStruct` with a freshly generated random key appropriate for `typ`.

Both return a heap-allocated `rust::KeyStruct *` suitable for passing to `ClientWriterConfig`, `ServerConfig`, or `set_encryption`.

### `set_encryption` on `Logging`

Reconfigure the encryption of an existing network writer:

```cpp
int set_encryption(rust::WriterTypeEnum writer, const rust::KeyStruct *key);
```

`writer` selects the writer *type* to reconfigure (e.g. `rust::WriterTypeEnum::Client` or `rust::WriterTypeEnum::Server`). Returns `0` on success, negative on error.

### Example: Creating and Applying an AES Key

```cpp
#include "h/cppfastlogging.hpp"
using namespace logging;

int main() {
    // Generate a random AES key
    rust::KeyStruct *key = create_random_key(rust::EncryptionMethodEnum::AES);

    // Use it on both sides (in a real app the key would be shared securely)
    Logging *server = new Logging(DEBUG, "LOGSRV");
    ServerConfig srv(DEBUG, "127.0.0.1", key);
    server->add_writer_config(srv);
    server->set_root_writer_config(srv);
    server->sync_all(5.0);

    Logging *client = new Logging(DEBUG, "LOGCLIENT");
    client->add_writer_config(
        ClientWriterConfig(DEBUG, server->get_root_server_address_port(), key));
    client->info("Encrypted hello!");
    client->sync_all(1.0);
    server->sync_all(1.0);

    delete client;
    delete server;
    return 0;
}
```

To reconfigure encryption on an already-added client writer:

```cpp
rust::KeyStruct *new_key = create_random_key(rust::EncryptionMethodEnum::AES);
logging.set_encryption(rust::WriterTypeEnum::Client, new_key);
```

---

## Debug Output

The library can emit internal diagnostic output for troubleshooting writer behavior, network activity, and dispatch.

```cpp
void set_debug(uint32_t debug);
```

Pass a non-zero value to enable internal debug output; `0` disables it. The exact meaning of non-zero values is defined by the underlying Rust library and generally increases in verbosity with larger values. This is primarily a development/diagnostic aid.

```cpp
Logging logging(DEBUG, "app");
logging.set_debug(1);
```

---

## See Also

- [WRITERS.md](WRITERS.md) — all writer config classes.
- [NETWORK.md](NETWORK.md) — network logging and the client/server architecture.
- [LOGGING.md](LOGGING.md) — full reference for the `Logging` class.
- Header files in `cppfastlogging/h/` (`logging.hpp`, `def.hpp`, `cppfastlogging.hpp`).
