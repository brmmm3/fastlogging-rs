#pragma once

#include <cstdint>
#include <cstdlib>
#include <string>
#include "def.hpp"
#include "logger.hpp"
#include "writer.hpp"

// ---- C API declarations (must match cfastlogging/h/logging.h exactly) -------

extern "C" {

rust::ExtConfig *ext_config_new(rust::MessageStructEnum structured,
                                int8_t hostname, int8_t pname, int8_t pid,
                                int8_t tname, int8_t tid);

rust::Logging *logging_new_default();

rust::Logging *logging_new(uint8_t level, const char *domain,
                            rust::WriterConfigEnum *configs_ptr,
                            uint32_t config_cnt,
                            rust::ExtConfig *ext_config,
                            const char *config_path);

int  logging_apply_config(rust::Logging *logging, const char *path);
int  logging_shutdown(rust::Logging *logging, int8_t now);

// wid = writer id returned by logging_add_writer_config (0 = root writer)
int  logging_set_level(rust::Logging *logging, uint32_t wid, uint8_t level);
void logging_set_domain(rust::Logging *logging, const char *domain);
void logging_set_level2sym(rust::Logging *logging, uint8_t level2sym);
void logging_set_ext_config(rust::Logging *logging, rust::ExtConfig *ext_config);

void logging_add_logger(rust::Logging *logging, rust::Logger *logger);
void logging_remove_logger(rust::Logging *logging, rust::Logger *logger);

int logging_set_root_writer_config(rust::Logging *logging,
                                   rust::WriterConfigEnum *config);
int logging_add_writer_config(rust::Logging *logging,
                              rust::WriterConfigEnum *config);
void logging_remove_writer(rust::Logging *logging, uint32_t wid);

int logging_enable(rust::Logging *logging, uint32_t wid);
int logging_disable(rust::Logging *logging, uint32_t wid);
int logging_enable_type(rust::Logging *logging, rust::WriterTypeEnum typ);
int logging_disable_type(rust::Logging *logging, rust::WriterTypeEnum typ);

intptr_t logging_sync(rust::Logging *logging, rust::WriterTypeEnum *types,
                      uint32_t type_cnt, double timeout);
intptr_t logging_sync_all(rust::Logging *logging, double timeout);
intptr_t logging_rotate(rust::Logging *logging, const char *path);

// wid identifies which writer to reconfigure (use WriterTypeEnum cast to select)
int logging_set_encryption(rust::Logging *logging, rust::WriterTypeEnum writer,
                           const rust::KeyStruct *key);

void logging_set_debug(rust::Logging *logging, uint32_t debug);

rust::WriterConfigEnum  *logging_get_writer_config(rust::Logging *logging, uint32_t wid);
rust::ServerConfig      *logging_get_server_config(rust::Logging *logging, uint32_t wid);
rust::ServerConfigs     *logging_get_server_configs(rust::Logging *logging);
const char              *logging_get_root_server_address_port(rust::Logging *logging);
const rust::Cu32StringVec *logging_get_server_addresses_ports(rust::Logging *logging);
const rust::Cu32StringVec *logging_get_server_addresses(rust::Logging *logging);
const rust::Cu32u16Vec    *logging_get_server_ports(rust::Logging *logging);
rust::KeyStruct           *logging_get_server_auth_key(rust::Logging *logging);
const char                *logging_get_config_string(rust::Logging *logging);

int logging_save_config(rust::Logging *logging, const char *path);

intptr_t logging_trace(const rust::Logging *logging, const char *message);
intptr_t logging_debug(const rust::Logging *logging, const char *message);
intptr_t logging_info(const rust::Logging *logging, const char *message);
intptr_t logging_success(const rust::Logging *logging, const char *message);
intptr_t logging_warning(const rust::Logging *logging, const char *message);
intptr_t logging_error(const rust::Logging *logging, const char *message);
intptr_t logging_critical(const rust::Logging *logging, const char *message);
intptr_t logging_fatal(const rust::Logging *logging, const char *message);
intptr_t logging_exception(const rust::Logging *logging, const char *message);

} // extern "C"

// ---- C++ wrapper classes -----------------------------------------------

namespace logging {

enum class MessageStruct : uint8_t { String = 0, Json = 1, Xml = 2 };

class ExtConfig {
public:
  rust::ExtConfig *config = nullptr;

  ExtConfig(MessageStruct structured,
            int8_t hostname, int8_t pname, int8_t pid,
            int8_t tname, int8_t tid) {
    config = ext_config_new(
        static_cast<rust::MessageStructEnum>(structured),
        hostname, pname, pid, tname, tid);
  }
  ~ExtConfig() { config = nullptr; }
};

class Logging {
public:
  // ---- Constructors ----

  /// Create a Logging instance with no writers (add them via add_writer_config).
  explicit Logging(uint8_t level = NOTSET, const char *domain = nullptr,
                   ExtConfig *ext_config = nullptr,
                   const char *config_path = nullptr)
      : logging_(nullptr) {
    if (!domain) domain = "root";
    rust::ExtConfig *ec = ext_config ? ext_config->config : nullptr;
    logging_ = logging_new(level, domain, nullptr, 0, ec, config_path);
  }

  /// Create a Logging instance and pre-configure N writer configs supplied as a
  /// C array.  The template parameter N is deduced automatically:
  ///
  ///   WriterConfig writers[] = { ConsoleWriterConfig(DEBUG), ... };
  ///   Logging logging(DEBUG, "root", writers);
  template <std::size_t N>
  Logging(uint8_t level, const char *domain,
          WriterConfig (&configs)[N],
          ExtConfig *ext_config = nullptr,
          const char *config_path = nullptr)
      : logging_(nullptr) {
    if (!domain) domain = "root";
    rust::ExtConfig *ec = ext_config ? ext_config->config : nullptr;
    logging_ = logging_new(level, domain, nullptr, 0, ec, config_path);
    for (std::size_t i = 0; i < N; ++i)
      logging_add_writer_config(logging_, configs[i].config);
  }

  /// Default-initialised logger with a console writer at DEBUG level.
  static Logging Default() {
    Logging l;
    l.logging_ = logging_new_default();
    return l;
  }

  ~Logging() {
    if (logging_) {
      logging_shutdown(logging_, 0);
      logging_ = nullptr;
    }
  }

  Logging(const Logging &) = delete;
  Logging &operator=(const Logging &) = delete;
  Logging(Logging &&other) noexcept : logging_(other.logging_) {
    other.logging_ = nullptr;
  }
  Logging &operator=(Logging &&other) noexcept {
    if (this != &other) {
      logging_ = other.logging_;
      other.logging_ = nullptr;
    }
    return *this;
  }

  // ---- Configuration ----

  int  apply_config(const char *path) { return logging_apply_config(logging_, path); }
  int  shutdown(bool now) { return logging_shutdown(logging_, now ? 1 : 0); }

  /// Set log level of writer `wid` (pass 0 for the root/default writer).
  int  set_level(uint32_t wid, uint8_t level) {
    return logging_set_level(logging_, wid, level);
  }
  void set_domain(const char *domain) { logging_set_domain(logging_, domain); }
  void set_level2sym(uint8_t level2sym) {
    logging_set_level2sym(logging_, level2sym);
  }
  void set_ext_config(ExtConfig *ext_config) {
    logging_set_ext_config(logging_, ext_config->config);
  }

  void add_logger(Logger &logger) {
    logging_add_logger(logging_, logger.raw());
  }
  void add_logger(Logger *logger) {
    if (logger) logging_add_logger(logging_, logger->raw());
  }
  void remove_logger(Logger &logger) {
    logging_remove_logger(logging_, logger.raw());
  }
  void remove_logger(Logger *logger) {
    if (logger) logging_remove_logger(logging_, logger->raw());
  }

  /// Transfer ownership of config to this Logging instance.
  int add_writer_config(WriterConfig &config) {
    return logging_add_writer_config(logging_, config.config);
  }
  /// Convenience overload for temporaries: Logging::add_writer_config(ConsoleWriterConfig(DEBUG))
  int add_writer_config(WriterConfig &&config) {
    return logging_add_writer_config(logging_, config.config);
  }
  int add_writer_config(WriterConfig *config) {
    return config ? logging_add_writer_config(logging_, config->config) : -1;
  }

  /// Set the root writer config (must be a Client or Server config).
  int set_root_writer_config(WriterConfig &config) {
    return logging_set_root_writer_config(logging_, config.config);
  }
  int set_root_writer_config(WriterConfig *config) {
    return config ? logging_set_root_writer_config(logging_, config->config) : -1;
  }

  void remove_writer(uint32_t wid) { logging_remove_writer(logging_, wid); }
  int  enable(uint32_t wid)        { return logging_enable(logging_, wid); }
  int  disable(uint32_t wid)       { return logging_disable(logging_, wid); }
  int  enable_type(rust::WriterTypeEnum typ)  { return logging_enable_type(logging_, typ); }
  int  disable_type(rust::WriterTypeEnum typ) { return logging_disable_type(logging_, typ); }

  int sync(rust::WriterTypeEnum *types, uint32_t cnt, double timeout) {
    return logging_sync(logging_, types, cnt, timeout);
  }
  int sync_all(double timeout)     { return logging_sync_all(logging_, timeout); }
  int rotate(const char *path)     { return logging_rotate(logging_, path); }

  int set_encryption(rust::WriterTypeEnum writer, const rust::KeyStruct *key) {
    return logging_set_encryption(logging_, writer, key);
  }
  void set_debug(uint32_t debug)   { logging_set_debug(logging_, debug); }

  // ---- Queries ----

  rust::ServerConfig     *get_server_config(uint32_t wid = 0) {
    return logging_get_server_config(logging_, wid);
  }
  rust::ServerConfigs    *get_server_configs() {
    return logging_get_server_configs(logging_);
  }
  const char             *get_root_server_address_port() {
    return logging_get_root_server_address_port(logging_);
  }
  const rust::Cu32StringVec *get_server_addresses_ports() {
    return logging_get_server_addresses_ports(logging_);
  }
  const rust::Cu32StringVec *get_server_addresses() {
    return logging_get_server_addresses(logging_);
  }
  const rust::Cu32u16Vec    *get_server_ports() {
    return logging_get_server_ports(logging_);
  }
  rust::KeyStruct *get_server_auth_key() {
    return logging_get_server_auth_key(logging_);
  }
  const char *get_config_string() {
    return logging_get_config_string(logging_);
  }
  int save_config(const char *path) { return logging_save_config(logging_, path); }

  // ---- Log calls ----

  int trace(const std::string &m)     const { return logging_trace(logging_, m.c_str()); }
  int debug(const std::string &m)     const { return logging_debug(logging_, m.c_str()); }
  int info(const std::string &m)      const { return logging_info(logging_, m.c_str()); }
  int success(const std::string &m)   const { return logging_success(logging_, m.c_str()); }
  int warn(const std::string &m)      const { return logging_warning(logging_, m.c_str()); }
  int warning(const std::string &m)   const { return logging_warning(logging_, m.c_str()); }
  int error(const std::string &m)     const { return logging_error(logging_, m.c_str()); }
  int critical(const std::string &m)  const { return logging_critical(logging_, m.c_str()); }
  int fatal(const std::string &m)     const { return logging_fatal(logging_, m.c_str()); }
  int exception(const std::string &m) const { return logging_exception(logging_, m.c_str()); }

  rust::Logging *raw() const { return logging_; }

private:
  rust::Logging *logging_ = nullptr;
};

} // namespace logging
