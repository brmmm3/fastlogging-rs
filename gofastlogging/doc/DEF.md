# Definitions

This document describes every definition exported by the top-level `gofastlogging/fastlogging` package (conventionally imported as `fl`). These are the constants, enums, type wrappers, and helper functions shared across the `logging`, `logger`, and `writer` packages.

---

## Log level constants

All log level constants are untyped `int` values sourced from C macros. They are ordered so that a lower numeric value means a more verbose level: a logger at level `L` emits messages whose level is `<= L`.

| Constant | Value | Meaning |
| --- | --- | --- |
| `NOLOG` | 100 | No logging. |
| `EXCEPTION` | 60 | Log exception messages. (Also logs exception info / traceback — slow.) |
| `CRITICAL` | 50 | Log critical messages. Default color is bright red. |
| `FATAL` | 50 | Same as `CRITICAL`. |
| `ERROR` | 40 | Log error messages. Default color is red. |
| `WARNING` | 30 | Log warning messages. Default color is bright yellow. |
| `WARN` | 30 | Same as `WARNING`. |
| `SUCCESS` | 25 | Success messages. |
| `INFO` | 20 | Log info messages. Default color is bright green. |
| `DEBUG` | 10 | Log debug messages. Default color is white. |
| `TRACE` | 5 | Trace messages. |
| `NOTSET` | 0 | All messages are logged. |

See [LEVELS.md](LEVELS.md) for filtering semantics and per-writer levels.

---

## Enum `LevelSymbol`

Controls how a log level is rendered in output.

```go
type LevelSymbol int

const (
    Sym   LevelSymbol = iota // 1-char symbol (!, F, E, W, ...)
    Short                    // 3-char text (EXC, FTL, ERR, WRN, ...)
    Str                      // long text (EXCEPTION, FATAL, ERROR, WARNING, ...). Default.
)
```

func (s LevelSymbol) Into() uint8`

Converts to the underlying C enum value. This returns a plain `uint8` rather than a cgo-generated type so it can be used from the `logging`, `logger`, and `writer` packages (cgo creates a distinct, non-interchangeable Go type per package for every C type). Intended for internal use.

---

## Enum `FileType`

Identifies the kind of operation queued to a writer thread.

```go
type FileType int

const (
    MessageOp FileType = iota // a log message
    SyncOp                     // sync/flush request
    RotateOp                   // log file rotation request
    StopOp                     // stop the writer
)
```

### `func (s FileType) Into() uint8`

Converts to the underlying C enum value. Internal use.

---

## Enum `MessageStruct`

Selects whether and how log messages carry structure information.

```go
type MessageStruct int

const (
    String MessageStruct = iota // No structure info (default)
    Json                        // Log as JSON
    Xml                         // Log as XML
)
```

### `func (s MessageStruct) Into() uint8`

Converts to the underlying C enum value. Internal use.

---

## Enum `EncryptionMethod`

Selects the encryption / authentication scheme for network writers and servers.

```go
type EncryptionMethod int

const (
    NONE    EncryptionMethod = iota // No encryption
    AuthKey                          // Authentication key
    AES                              // AES encryption
)
```

### `func (s EncryptionMethod) Into() uint8`

Converts to the underlying C enum value. Internal use.

---

## Enum `CompressionMethod`

Selects the compression algorithm used by file writers.

```go
type CompressionMethod int

const (
    Store   CompressionMethod = iota // No compression
    Deflate                           // Deflate compression
    Zstd                              // Zstandard compression
    Lzma                              // LZMA compression
)
```

### `func (s CompressionMethod) Into() uint8`

Converts to the underlying C enum value. Internal use.

---

## Type wrappers

All wrapper structs hold an `unsafe.Pointer` to the underlying C handle. This is deliberate: cgo generates a distinct, non-interchangeable Go type per package for every C type, even when two `import "C"` blocks include the same header. Using `unsafe.Pointer` lets these values cross between the `fastlogging`, `logging`, `logger`, and `writer` packages.

### `WriterConfigEnum`

```go
type WriterConfigEnum struct {
    Config unsafe.Pointer // wraps a C WriterConfigEnum handle (a void* in C)
}
```

A single writer configuration. Returned **by pointer** (`*fl.WriterConfigEnum`) from the `writer` factory functions; dereference with `*` when building a `[]WriterConfigEnum` slice.

### `WriterConfigs`

```go
type WriterConfigs struct {
    Configs unsafe.Pointer // wraps multiple writer configs
}
```

### `WriterConfigEnums`

```go
type WriterConfigEnums = WriterConfigs // type alias
```

### `WriterType`

```go
type WriterType struct {
    Typ uint8 // writer type enum value
}
```

### `WriterTypeEnum`

```go
type WriterTypeEnum = WriterType // type alias
```

### `Writer`

```go
type Writer struct {
    Writer unsafe.Pointer // wraps a C CWriterEnum handle (a void* in C)
}
```

### `WriterEnum`

```go
type WriterEnum = Writer // type alias
```

### `Writers`

```go
type Writers struct {
    Writers unsafe.Pointer // wraps multiple writers
}
```

### `WriterEnums`

```go
type WriterEnums = Writers // type alias
```

### `ServerConfig`

```go
type ServerConfig struct {
    Config unsafe.Pointer // wraps the C server config pointer
}
```

### `ServerConfigs`

```go
type ServerConfigs struct {
    Config unsafe.Pointer // wraps the C server configs pointer
}
```

### `Key`

```go
type Key struct {
    Key unsafe.Pointer // wraps a C key struct
}
```

### `KeyStruct`

```go
type KeyStruct = Key // type alias
```

### `ExtConfig`

```go
type ExtConfig struct {
    Config unsafe.Pointer // wraps the C extended config pointer
}
```

Holds extended formatting settings (see `NewExtConfig`).

---

## Functions

### `func NewWriters(writers unsafe.Pointer) Writers`

Constructs a `Writers` wrapper from a raw C pointer. Internal/low-level use.

### `func WriterEnumsNew(writers unsafe.Pointer) WriterEnums`

Alias for `NewWriters`. Returns a `WriterEnums` (which is a type alias for `Writers`).

### `func NewExtConfig(structured MessageStruct, hostname, pname, pid, tname, tid bool) ExtConfig`

Creates an extended formatting configuration. The bool flags control whether each field is included in log messages:

| Parameter | Type | When `true` |
| --- | --- | --- |
| `structured` | `MessageStruct` | `String` (default), `Json`, or `Xml` — selects message structure format |
| `hostname` | `bool` | Include the hostname |
| `pname` | `bool` | Include the process name |
| `pid` | `bool` | Include the process id |
| `tname` | `bool` | Include the thread name |
| `tid` | `bool` | Include the thread id |

Pass the result to `logging.New` (as `*fl.ExtConfig`) or to `Logging.SetExtConfig`.

```go
ext := fl.NewExtConfig(fl.String, true, true, true, false, false)
logger := logging.New(fl.DEBUG, nil, writers, &ext, nil)
```

### `func CreateKey(typ EncryptionMethod, key []byte) KeyStruct`

Creates an encryption key of the given type from raw key bytes. If `key` is empty, a random key is generated (equivalent to `CreateRandomKey`). The returned `KeyStruct` can be passed to network writer/server factory functions or to `SetEncryption`.

```go
key := fl.CreateKey(fl.AES, []byte{0x01, 0x02, 0x03, /* ... */})
client := writer.ClientWriterConfigNew(fl.DEBUG, "127.0.0.1:12345", &key)
```

### `func CreateRandomKey(typ EncryptionMethod) KeyStruct`

Creates a random encryption key of the given type. Convenience wrapper around `CreateKey` with an empty key slice.

```go
key := fl.CreateRandomKey(fl.AES)
```

---

## Helper types

These mirror C vector structs returned by queries that map `uint32` keys to values. They are used by `Logging` / root methods such as `GetRootServerAddressesPorts` and `GetRootServerPorts`.

### `Cu32StringVec`

```go
type Cu32StringVec struct {
    Cnt    uint32
    Keys   []uint32
    Values []string
}
```

A vector of `uint32` keys paired with `string` values.

### `Cu32u16Vec`

```go
type Cu32u16Vec struct {
    Cnt    uint32
    Keys   []uint32
    Values []uint16
}
```

A vector of `uint32` keys paired with `uint16` values (e.g. server ports keyed by writer id).
