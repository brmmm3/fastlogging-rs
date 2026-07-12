# Writer Configurations

Writers are created through `WriterConfig` factory static functions.  Each
factory returns a `rust::Box<WriterConfig>` (an owned, opaque handle to a Rust
`WriterConfigEnum`).  These boxes are collected into a `rust::Vec` and passed
to `Logging::create` or `Logging::add_writer_config`.

```cpp
rust::Vec<rust::Box<WriterConfig>> configs;
configs.push_back(WriterConfig::new_console(DEBUG, true));
configs.push_back(WriterConfig::new_file(DEBUG, "/tmp/app.log", 0, 0, -1, -1,
                                          CompressionMethodEnum::Store));
auto log = Logging::create(DEBUG, "app", std::move(configs));
```

Console Writer

Writes coloured or plain-text messages to stdout.

### `WriterConfig::new_console`

```cpp
static rust::Box<WriterConfig> new_console(uint8_t level, bool colors);
```

| Parameter | Description |
|---|---|
| `level` | Minimum level for this writer |
| `colors` | Enable ANSI colour output |

```cpp
auto console = WriterConfig::new_console(WARNING, false);
```

---

## File Writer

Writes messages to a log file with optional size-based and time-based rotation,
and compression of rotated archives.

### `WriterConfig::new_file`

```cpp
static rust::Box<WriterConfig> new_file(
    uint8_t              level,
    rust::Str            path,
    uint64_t             size,          // max bytes; 0 = unlimited
    uint64_t             backlog,       // backup count (required when size > 0)
    int64_t              timeout_secs,  // rotate after N seconds; < 0 = disabled
    int64_t              time_secs,     // rotate at now+N seconds; < 0 = disabled
    CompressionMethodEnum compression
);  // may throw rust::Error
```

Throws `rust::Error` if `size > 0` but `backlog == 0`, or `backlog > 1000`.

### `CompressionMethodEnum`

```cpp
enum class CompressionMethodEnum : uint8_t {
    Store   = 0,   // no compression (default)
    Deflate = 1,   // zlib/Deflate
    Zstd    = 2,   // Zstandard
    Lzma    = 3,   // LZMA
};
```

```cpp
auto file = WriterConfig::new_file(
    DEBUG,
    "/tmp/app.log",
    5ULL * 1024 * 1024,   // rotate at 5 MB
    4,                    // keep 4 backups
    86400,                // also rotate after 24 h
    -1,                   // no absolute time trigger
    CompressionMethodEnum::Zstd
);
```

---

## Client Writer (Network)

Connects to a remote `fastlogging` server and forwards log messages over TCP.
See [NETWORK.md](NETWORK.md) for a full description.

### `WriterConfig::new_client`

```cpp
static rust::Box<WriterConfig> new_client(
    uint8_t                    level,
    rust::Str                  address,   // "ip:port"
    EncryptionMethodEnum       key_type,
    rust::Slice<const uint8_t> key        // empty slice for NONE
);
```

---

## Server Writer (Network)

Opens a TCP listener and receives log messages from client writers.
See [NETWORK.md](NETWORK.md) for a full description.

### `WriterConfig::new_server`

```cpp
static rust::Box<WriterConfig> new_server(
    uint8_t                    level,
    rust::Str                  address,   // "ip" or "ip:port" (0 = OS-assigned port)
    EncryptionMethodEnum       key_type,
    rust::Slice<const uint8_t> key
);
```

---

## Syslog Writer *(Unix only)*

Sends messages to the system syslog daemon via RFC 3164 (`LOG_USER` facility).

### `WriterConfig::new_syslog`

```cpp
static rust::Box<WriterConfig> new_syslog(
    uint8_t   level,
    rust::Str hostname,   // empty = not set
    rust::Str pname,      // process name
    uint32_t  pid
);
```

```cpp
auto syslog = WriterConfig::new_syslog(
    WARNING, "", "myapp", static_cast<uint32_t>(getpid()));
```

---

## `WriterTypeTag`

Used by `enable_type`, `disable_type`, and `sync_type` to address all writers
of a given category at once.  Unlike the Rust `WriterTypeEnum`, which carries
data for `File`, `Client`, and `Server` variants, the C++ enum is simple; a
separate `rust::Str data` parameter carries the path or address when needed.

```cpp
enum class WriterTypeTag : uint8_t {
    Root     = 0,
    Console  = 1,
    File     = 2,   // data = file path (empty = all)
    Files    = 3,   // all file writers
    Client   = 4,   // data = address:port (empty = all)
    Clients  = 5,   // all client writers
    Server   = 6,   // data = address:port (empty = all)
    Servers  = 7,   // all server writers
    Callback = 8,
    Syslog   = 9,
};
```

```cpp
// Disable all file writers
log->disable_type(WriterTypeTag::Files, "");

// Re-enable
log->enable_type(WriterTypeTag::Files, "");

// Sync only file and console writers
log->sync_type(WriterTypeTag::Files,   "", 5.0);
log->sync_type(WriterTypeTag::Console, "", 5.0);

// Or sync all at once
log->sync_all(5.0);
```
