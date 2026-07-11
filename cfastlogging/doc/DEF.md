# Definitions

## Log level names and their values

`NOLOG` (100) &ensp;&ensp;&ensp;&ensp;&ensp; No logging.  
`EXCEPTION` (60) &nbsp; Log exception messages. In addition to the message text the exception info (output of traceback.format_exc) is logged (SLOW!).  
`CRITICAL` (50) &nbsp;&ensp; Log fatal/critical messages. Default color is bright red.  
`FATAL` (50) &ensp;&ensp;&ensp;&ensp;&ensp; Same as CRITICAL.  
`ERROR` (40) &ensp;&ensp;&ensp;&ensp;&ensp; Log also error messages. Default color is red.  
`WARNING` (30) &ensp;&ensp;&ensp; Log also warning messages. Default color is bright yellow.  
`SUCCESS` (25) &ensp;&ensp;&ensp; Success messages.  
`INFO` (20) &ensp;&ensp;&ensp;&ensp;&ensp;&ensp;&nbsp; Log also info messages. Default color is bright green.  
`DEBUG` (10) &ensp;&ensp;&ensp;&ensp;&ensp;  Log also debug messages. Default color is white.  
`TRACE` (5) &ensp;&ensp;&ensp;&ensp;&ensp;  Trace messages.  
`NOTSET` (0) &ensp;&ensp;&ensp;&ensp; All messages are logged.


## Enum `CWriterEnum`

Represents the type of log writer. Used to select or identify a writer backend.

```c
typedef enum {
    Root,
    Console,
    File,
    Client,
    Server,
    Callback,
    Syslog
} CWriterEnum;
```

---


## Struct `CWriterEnums`

Holds an array of writer types.

```c
typedef struct {
    unsigned int cnt;           // Number of writers
    const CWriterEnum* values;  // Pointer to array of writer enums
} CWriterEnums;
```

*Memory management:* The caller is responsible for freeing any dynamically allocated arrays if returned by the API.

---


## Struct `CEncryptionMethod`

Describes an encryption method and key for network writers.

```c
typedef struct {
    CEncryptionMethodEnum typ;  // Encryption type (see enum)
    uint32_t len;               // Length of key
    const uint8_t* key;         // Pointer to key bytes
} CEncryptionMethod;
```

*Pointer usage:* The key pointer must remain valid for the lifetime of the config. If allocated, free after use.

---


## Struct `CServerConfig`

Describes a server writer configuration.

```c
typedef struct {
    uint8_t level;                  // Log level for server
    const char* address;            // Server address string
    uint16_t port;                  // Server port
    const CEncryptionMethod* key;   // Pointer to encryption method
    const char* port_file;          // Optional: file to write port info
} CServerConfig;
```

*Pointer usage:* All string pointers must be valid UTF-8 null-terminated strings. Do not free until config is unused.

---


## Struct `CServerConfigs`

Holds an array of server configs, indexed by key.

```c
typedef struct {
    unsigned int cnt;               // Number of configs
    const uint32_t* keys;           // Array of keys (writer IDs)
    const CServerConfig* values;    // Array of server configs
} CServerConfigs;
```

*Memory management:* If returned by API, free arrays after use if documented as heap-allocated.

---


## `ext_config_new(structured: c_uchar, hostname: c_char, pname: c_char, pid: c_char, tname: c_char, tid: c_char) -> *const ExtConfig`

Create an `ExtConfig` instance for advanced formatting and metadata.

**Parameters:**
- `structured`: 0 = plain string, 1 = structured/JSON
- `hostname`: Hostname string
- `pname`: Process name string
- `pid`: Process ID string
- `tname`: Thread name string
- `tid`: Thread ID string

**Returns:** Pointer to a new ExtConfig struct. Free with `ext_config_free` when done.

---

## Memory Management Notes

- All pointers returned by the API that are not owned by the caller must not be freed.
- If the API allocates memory (e.g., for arrays or config structs), free with the provided `*_free` function or `free()` if documented.
- All string pointers must be valid UTF-8 null-terminated strings.
