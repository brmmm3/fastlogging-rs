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

The enum has following values:

```rust
pub enum CWriterEnum {
    Root,
    Console,
    File,
    Client,
    Server,
    Callback,
    Syslog,
}
```

## Enum `CWriterEnums`

The enum has following values:

```rust
pub struct CWriterEnums {
    pub cnt: c_uint,
    pub values: *const CWriterEnum,
}
```

## Enum `CWriterConfigEnums`

The enum has following values:

```rust
pub struct CEncryptionMethod {
    typ: CEncryptionMethodEnum,
    len: u32,
    key: *const u8,
}
```

## Enum `CServerConfig`

The enum has following values:

```rust
pub struct CServerConfig {
    level: u8,
    address: *const char,
    port: u16,
    key: *const CEncryptionMethod,
    port_file: *const char,
}
```

## Enum `CServerConfigs`

The enum has following values:

```rust
pub struct CServerConfigs {
    pub cnt: c_uint,
    pub keys: *const u32,
    pub values: *const CServerConfig,
}
```

## `ext_config_new(structured: c_uchar, hostname: c_char, pname: c_char, pid: c_char, tname: c_char, tid: c_char) -> *const ExtConfig`

Create `ExtConfig` instance.
