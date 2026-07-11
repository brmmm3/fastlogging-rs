#pragma once

#include <cstdint>
#include <string>
#include "def.hpp"

extern "C" {
rust::Logger *logger_new(uint8_t level, const char *domain);
rust::Logger *logger_new_ext(uint8_t level, const char *domain,
                              int8_t tname, int8_t tid);
void logger_set_level(rust::Logger *logger, uint8_t level);
void logger_set_domain(rust::Logger *logger, const char *domain);
int  logger_trace(const rust::Logger *logger, const char *message);
int  logger_debug(const rust::Logger *logger, const char *message);
int  logger_info(const rust::Logger *logger, const char *message);
int  logger_success(const rust::Logger *logger, const char *message);
int  logger_warning(const rust::Logger *logger, const char *message);
int  logger_error(const rust::Logger *logger, const char *message);
int  logger_critical(const rust::Logger *logger, const char *message);
int  logger_fatal(const rust::Logger *logger, const char *message);
int  logger_exception(const rust::Logger *logger, const char *message);
} // extern "C"

namespace logging {

class Logger {
public:
  Logger(uint8_t level, const char *domain)
      : logger_(logger_new(level, domain)) {}

  Logger(uint8_t level, const char *domain, int8_t tname, int8_t tid)
      : logger_(logger_new_ext(level, domain, tname, tid)) {}

  ~Logger() { logger_ = nullptr; }

  Logger(const Logger &) = delete;
  Logger &operator=(const Logger &) = delete;
  Logger(Logger &&other) noexcept : logger_(other.logger_) {
    other.logger_ = nullptr;
  }
  Logger &operator=(Logger &&other) noexcept {
    if (this != &other) {
      logger_ = other.logger_;
      other.logger_ = nullptr;
    }
    return *this;
  }

  void set_level(uint8_t level)       { logger_set_level(logger_, level); }
  void set_domain(const char *domain) { logger_set_domain(logger_, domain); }

  int trace(const std::string &m)     const { return logger_trace(logger_, m.c_str()); }
  int debug(const std::string &m)     const { return logger_debug(logger_, m.c_str()); }
  int info(const std::string &m)      const { return logger_info(logger_, m.c_str()); }
  int success(const std::string &m)   const { return logger_success(logger_, m.c_str()); }
  int warn(const std::string &m)      const { return logger_warning(logger_, m.c_str()); }
  int warning(const std::string &m)   const { return logger_warning(logger_, m.c_str()); }
  int error(const std::string &m)     const { return logger_error(logger_, m.c_str()); }
  int critical(const std::string &m)  const { return logger_critical(logger_, m.c_str()); }
  int fatal(const std::string &m)     const { return logger_fatal(logger_, m.c_str()); }
  int exception(const std::string &m) const { return logger_exception(logger_, m.c_str()); }

  rust::Logger *raw() const { return logger_; }

private:
  rust::Logger *logger_ = nullptr;
};

} // namespace logging
