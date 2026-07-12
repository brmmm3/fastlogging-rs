# `Logging` — Primary API

`Logging` is the central class in `cxxfastlogging`.  Each instance owns a
background Rust thread and zero or more writers.  All log calls are
non-blocking: the message is pushed onto a bounded channel and the thread
dispatches it to each writer.  `Logging` is an opaque Rust type; C++ code
holds it through `rust::Box<Logging>`.

## Constructors

### `Logging::new_default`

```cpp
static rust::Box<Logging> new_default();  // may throw rust::Error
```

Creates a `Logging` instance with a single console writer at `NOTSET` level.

```cpp
auto log = Logging::new_default();
log->info("hello");
log->shutdown(false);
```

### `Logging::create`

```cpp
static rust::Box<Logging> create(
    uint8_t                          level,
    rust::Str                        domain,
    rust::Vec<rust::Box<WriterConfig>> configs
);  // may throw rust::Error
```

| Parameter | Description |
|---|---|
| `level` | Global filter level. Messages below this are dropped before entering the channel. |
| `domain` | Label prepended to every log message (e.g. `"myapp"`, `"server"`). |
| `configs` | Zero or more writer configs.  Build the vector with `WriterConfig::new_*` factory functions. |

> **Note:** The constructor is named `create` rather than `new` because `new`
> is a reserved keyword in C++.

```cpp
rust::Vec<rust::Box<WriterConfig>> configs;
configs.push_back(WriterConfig::new_console(DEBUG, true));

auto log = Logging::create(DEBUG, "myapp", std::move(configs));
log->info("started");
log->shutdown(false);
```

## Lifecycle

### `shutdown`

```cpp
int shutdown(bool now);
```

Stops the background thread.  If `now` is `true` the stop flag is set
immediately and queued messages may be lost; if `false` a graceful stop is
sent and the thread drains its channel before exiting.

The destructor of `rust::Box<Logging>` drops the Rust `Logging` value, which
calls `shutdown(false)` automatically via `impl Drop for Logging`.

### `apply_config`

```cpp
int apply_config(rust::Str path);  // may throw rust::Error
```

Reload configuration from a JSON, YAML, or XML file at runtime.

### `save_config`

```cpp
int save_config(rust::Str path);  // may throw rust::Error
```

Persist the current configuration.  An empty string reuses the path from the
last `apply_config` call.  The file format is determined by extension
(`.json`, `.yaml`, `.xml`).

## Configuration Methods

```cpp
int  set_level(uint64_t wid, uint8_t level);  // wid = writer id (0 = root)
void set_domain(rust::Str domain);
void set_level2sym(LevelSymsEnum level2sym);
void set_ext_config(ExtConfigFfi ext_config);
void set_debug(uint8_t debug);
```

`set_level` targets a specific writer by its numeric id.  Writer ids are
returned by `add_writer_config`.  `wid = 0` is always the root writer
(Client or Server type).

## Writer Management

```cpp
// Add a writer; returns its id
uint64_t add_writer_config(rust::Box<WriterConfig> config);  // may throw

// Add multiple writers at once; returns their ids
rust::Vec<uint64_t> add_writer_configs(
    rust::Vec<rust::Box<WriterConfig>> configs);              // may throw

// Remove a writer by id; returns true if a writer was removed
bool remove_writer(uint64_t wid);

// Enable / disable individual writers
int  enable(uint64_t wid);   // may throw
int  disable(uint64_t wid);  // may throw

// Enable / disable all writers of a given type
// `data` carries the file path or address for File/Client/Server variants;
// pass an empty string for other variants.
int  enable_type(WriterTypeTag  tag, rust::Str data);  // may throw
int  disable_type(WriterTypeTag tag, rust::Str data);  // may throw
```

### Root Writer

A server or client writer installed as *root writer* (wid = 0) causes the
background `LoggingServer` or `ClientWriter` thread to start listening /
connecting:

```cpp
int set_root_writer_config(rust::Box<WriterConfig> config);  // may throw
```

## Logger Management

```cpp
void add_logger(Logger& logger);
void remove_logger(Logger& logger);
```

## Sync and Rotate

```cpp
// Sync writers of one type; data = path / address or empty
int sync_type(WriterTypeTag tag, rust::Str data, double timeout);  // may throw

// Sync all writers (Console, Files, Clients, Servers, Callback, Syslog)
int sync_all(double timeout);  // may throw

// Trigger file rotation; empty path = rotate all file writers
int rotate(rust::Str path);    // may throw
```

## Encryption

```cpp
int set_encryption(uint64_t wid,
                   EncryptionMethodEnum key_type,
                   rust::Slice<const uint8_t> key);  // may throw
```

Reconfigure the encryption of a Client or Server writer at runtime.

## Query Methods

```cpp
// Server config for a specific writer
ServerConfigInfo get_server_config(uint64_t wid);  // may throw

// All server writer configs
rust::Vec<ServerConfigInfo> get_server_configs();

// "ip:port" of the root server writer, or "" if none
rust::String get_root_server_address_port();

// All server writers: {id, "ip:port"}
rust::Vec<IdString> get_server_addresses_ports();
rust::Vec<IdString> get_server_addresses();
rust::Vec<IdU16>    get_server_ports();

// Auth key bytes used by the root server
rust::Vec<uint8_t>  get_server_auth_key();

// Human-readable config dump
rust::String get_config_string();
```

### `ServerConfigInfo`

```cpp
struct ServerConfigInfo {
    uint64_t             id;
    uint8_t              level;
    rust::String         address;
    uint16_t             port;
    EncryptionMethodEnum key_type;
    rust::Vec<uint8_t>   key;
    rust::String         port_file;  // empty if not set
};
```

### `IdString` / `IdU16`

```cpp
struct IdString { uint64_t id; rust::String value; };
struct IdU16    { uint64_t id; uint16_t     value; };
```

## Logging Methods

All methods accept `rust::Str` and throw `rust::Error` on channel failure.
They are **no-ops** when the global level filters the message — no allocation
occurs on the fast path.

```cpp
int trace(rust::Str message);
int debug(rust::Str message);
int info(rust::Str message);
int success(rust::Str message);
int warning(rust::Str message);
int error(rust::Str message);
int critical(rust::Str message);
int fatal(rust::Str message);
int exception(rust::Str message);
```

## Error Handling

All `int`-returning methods propagate errors as `rust::Error` exceptions.  The
`.what()` method returns the Rust error message string:

```cpp
try {
    log->set_level(99, DEBUG);  // unknown wid — will throw
} catch (const rust::Error& e) {
    std::cerr << e.what() << "\n";
}
```
