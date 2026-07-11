#pragma once

#include <cstdint>
#include "def.hpp"

// ---- C API declarations (must match cfastlogging/h/writer.h exactly) --------

extern "C" {

/// Returns an opaque WriterConfigEnum * (= void * in C ABI) that is owned by
/// the caller until it is passed to logging_new / logging_add_writer_config,
/// after which the Rust side takes ownership.

rust::WriterConfigEnum *console_writer_config_new(uint8_t level,
                                                  int8_t  colors);

rust::WriterConfigEnum *file_writer_config_new(
    uint8_t level, const char *path, uint32_t size, uint32_t backlog,
    int32_t timeout, int64_t time,
    rust::CompressionMethodEnum compression);

rust::WriterConfigEnum *client_writer_config_new(uint8_t level,
                                                 const char *address,
                                                 const rust::KeyStruct *key);

rust::WriterConfigEnum *server_config_new(uint8_t level, const char *address,
                                          const rust::KeyStruct *key);

rust::WriterConfigEnum *syslog_writer_config_new(uint8_t   level,
                                                 const char *hostname,
                                                 const char *pname,
                                                 uint32_t   pid);

rust::WriterConfigEnum *callback_writer_config_new(
    uint8_t level,
    void (*callback)(uint8_t, const char *, const char *));

} // extern "C"

// ---- C++ helper enums (subset of rust:: enums with friendlier names) --------

/// Compression algorithm for file writers.
enum class CompressionMethod : uint8_t {
  Store   = 0,
  Deflate = 1,
  Zstd    = 2,
  Lzma    = 3
};

// ---- C++ RAII wrapper classes -----------------------------------------------
// Each class obtains an opaque WriterConfigEnum * from the C API and stores it
// in the public `config` field.  The pointer is intended to be consumed (i.e.
// transferred to a Logging instance) exactly once; the destructor does not free
// it because the Rust side takes ownership upon add_writer_config.

class WriterConfig {
public:
  /// Opaque handle; valid until consumed by Logging::add_writer_config.
  rust::WriterConfigEnum *config = nullptr;

  WriterConfig() = default;
  WriterConfig(const WriterConfig &) = default;
  WriterConfig &operator=(const WriterConfig &) = default;
  virtual ~WriterConfig() = default;
};

class ConsoleWriterConfig : public WriterConfig {
public:
  ConsoleWriterConfig(uint8_t level, bool colors = false) {
    config = console_writer_config_new(level, static_cast<int8_t>(colors));
  }
};

class FileWriterConfig : public WriterConfig {
public:
  FileWriterConfig(uint8_t level, const char *path, uint32_t size = 0,
                   uint32_t backlog = 0, int32_t timeout = -1,
                   int64_t time = -1,
                   CompressionMethod compression = CompressionMethod::Store) {
    rust::CompressionMethodEnum c =
        static_cast<rust::CompressionMethodEnum>(compression);
    config = file_writer_config_new(level, path, size, backlog, timeout, time, c);
  }
};

class ClientWriterConfig : public WriterConfig {
public:
  ClientWriterConfig(uint8_t level, const char *address,
                     const rust::KeyStruct *key = nullptr) {
    config = client_writer_config_new(level, address, key);
  }
};

class ServerConfig : public WriterConfig {
public:
  ServerConfig(uint8_t level, const char *address,
               const rust::KeyStruct *key = nullptr) {
    config = server_config_new(level, address, key);
  }
};

class SyslogWriterConfig : public WriterConfig {
public:
  SyslogWriterConfig(uint8_t level, const char *hostname = nullptr,
                     const char *pname = nullptr, uint32_t pid = 0) {
    config = syslog_writer_config_new(level, hostname, pname, pid);
  }
};

class CallbackWriterConfig : public WriterConfig {
public:
  CallbackWriterConfig(uint8_t level,
                       void (*callback)(uint8_t, const char *, const char *)) {
    config = callback_writer_config_new(level, callback);
  }
};
