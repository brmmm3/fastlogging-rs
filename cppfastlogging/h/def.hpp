#pragma once

#include <cstdint>

// Log-level constants
static constexpr uint8_t NOLOG = 100;
static constexpr uint8_t EXCEPTION = 60;
static constexpr uint8_t CRITICAL = 50;
static constexpr uint8_t FATAL = CRITICAL;
static constexpr uint8_t ERROR = 40;
static constexpr uint8_t WARNING = 30;
static constexpr uint8_t WARN = WARNING;
static constexpr uint8_t SUCCESS = 25;
static constexpr uint8_t INFO = 20;
static constexpr uint8_t DEBUG = 10;
static constexpr uint8_t TRACE = 5;
static constexpr uint8_t NOTSET = 0;

namespace rust {

// ---- Simple value-type enums (match the C uint8_t-backed enums exactly) ----

enum class LevelSyms : uint8_t { Sym = 0, Short = 1, Str = 2 };

enum class CompressionMethodEnum : uint8_t {
  Store   = 0,
  Deflate = 1,
  Zstd    = 2,
  Lzma    = 3
};

/// Writer-type selector.  Values must match cfastlogging's CWriterTypeEnum.
enum class WriterTypeEnum : uint8_t {
  Root    = 0,
  Console = 1,
  File    = 2,
  Files   = 3,
  Client  = 4,
  Clients = 5,
  Server  = 6,
  Servers = 7,
  Syslog  = 8
};

enum class MessageStructEnum : uint8_t { String = 0, Json = 1, Xml = 2 };

enum class EncryptionMethodEnum : uint8_t { NONE = 0, AuthKey = 1, AES = 2 };

// ---- Opaque handle types (forward-declared; used only via pointer) ----
// These mirror the C API's "typedef void *Foo;" typedefs, but give C++ type
// safety: a WriterConfigEnum * is not accidentally mixed with a Logger *.

/// Opaque handle returned by console_writer_config_new, file_writer_config_new,
/// etc.  Ownership is transferred to the Logging instance via add_writer_config.
struct WriterConfigEnum;

/// Opaque active-writer handle (not the same as a config).
struct WriterEnum;

// ---- Concrete data structs shared between C and C++ ----

typedef struct Cu32StringVec {
  uint32_t  cnt;
  uint32_t *keys;
  char    **values;
} Cu32StringVec;

typedef struct Cu32u16Vec {
  uint32_t  cnt;
  uint32_t *keys;
  uint16_t *values;
} Cu32u16Vec;

typedef struct WriterEnums {
  uint32_t    cnt;
  WriterEnum *values;
} WriterEnums;

typedef struct ExtConfig {
  MessageStructEnum structured;
  int8_t hostname;
  int8_t pname;
  int8_t pid;
  int8_t tname;
  int8_t tid;
} ExtConfig;

typedef struct KeyStruct {
  EncryptionMethodEnum typ;
  uint32_t             len;
  const char          *key;
} KeyStruct;

typedef struct ServerConfig {
  uint8_t     level;
  const char *address;
  uint16_t    port;
  KeyStruct  *key;
  const char *port_file;
} ServerConfig;

typedef struct ServerConfigs {
  uint32_t      cnt;
  uint32_t     *keys;
  ServerConfig *values;
} ServerConfigs;

// Opaque handle types for Rust objects
struct Logging;
struct Logger;

} // namespace rust

// Convenience aliases so examples can write Cu32StringVec_t without "rust::"
using Cu32StringVec_t = rust::Cu32StringVec;
using Cu32u16Vec_t    = rust::Cu32u16Vec;
