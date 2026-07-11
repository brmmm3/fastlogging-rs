#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

///  Some C code uses magic values in structures to determine if the pointer
/// is of the correct type.
constexpr static const uint32_t ERROR_MAGIC = 3735928559;

enum class EncryptionMethodEnum
{
  NONE,
  AuthKey,
  AES,
};

enum class WriterEnum
{
  Root,
  Console,
  File,
  Client,
  Server,
  Callback,
  Syslog,
};

template <typename T = void>
struct Box;

template <typename T = void>
struct Option;

template <typename T = void>
struct Vec;

struct KeyStruct
{
  EncryptionMethodEnum typ;
  unsigned int len;
  const uint8_t *key;
};

struct Error
{
  uint32_t magic;
  CString msg;
  intptr_t code;
};

using Logging = void *;

using Logger = void *;

struct CusizeVec
{
  unsigned int cnt;
  Vec<uintptr_t> values;
};

struct WriterEnums
{
  unsigned int cnt;
  const WriterEnum *values;
};

struct WriterConfigEnums
{
  unsigned int cnt;
  Vec<uintptr_t> keys;
  Vec<WriterConfigEnum> values;
};

struct EncryptionMethod
{
  EncryptionMethodEnum typ;
  uint32_t len;
  const uint8_t *key;
};

struct ServerConfig
{
  uint8_t level;
  const uint32_t *address;
  uint16_t port;
  const EncryptionMethod *key;
  const uint32_t *port_file;
};

struct ServerConfigs
{
  unsigned int cnt;
  const uint32_t *keys;
  const ServerConfig *values;
};

struct Cu32StringVec
{
  unsigned int cnt;
  const unsigned int *keys;
  const char *const *values;
};

struct Cu32u16Vec
{
  unsigned int cnt;
  const unsigned int *keys;
  const unsigned short *values;
};

struct CKeyStruct
{
  EncryptionMethodEnum typ;
  unsigned int len;
  const uint8_t *key;
};

extern "C"
{

  extern const Lazy<Vec<uint8_t>> AUTH_KEY;

  extern const intptr_t EINIT;

  extern const intptr_t EINVAL;

  extern const Lazy<Mutex<void (*)(unsigned char, const char *, const char *)>> CALLBACK_C_FUNC;

  /// # Safety
  ///
  /// Create encryption key.
  const KeyStruct *create_key(EncryptionMethodEnum typ, unsigned int len, const uint8_t *key);

  /// # Safety
  ///
  /// Create encryption key.
  const KeyStruct *create_random_key(EncryptionMethodEnum typ);

  /// # Safety
  ///
  /// Drop error.
  /// We take ownership as we are passing by value, so when function
  /// exits the drop gets run.  Handles being passed null.
  void error_free(Option<Box<Error>>);

  /// # Safety
  ///
  /// Return error message.
  /// Our example "getter" methods which work on the Error type. The value
  /// returned is only valid as long as the Error has not been freed. If C
  /// caller needs a longer lifetime they need to copy the value.
  const char *error_msg(const Error *e);

  intptr_t error_code(const Error *e);

  /// For further reading ...
  /// [](https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098)
  ///
  /// # Safety
  ///
  /// Create new logging instance.
  Logging *logging_new_default();

  /// # Safety
  ///
  /// Create new logging instance.
  Logging *logging_new(char level,
                       const char *domain,
                       WriterConfigEnum **configs,
                       uintptr_t config_count,
                       ExtConfig *ext_config,
                       const char *config_path);

  /// # Safety
  ///
  /// Shutdown logging.
  intptr_t logging_apply_config(Logging *logging, const char *path);

  /// # Safety
  ///
  /// Shutdown logging.
  intptr_t logging_shutdown(Logging *logging, int8_t now);

  /// # Safety
  ///
  /// Set logging level.
  intptr_t logging_set_level(Logging *logging, unsigned int wid, uint8_t level);

  /// # Safety
  ///
  /// Set logging domain.
  void logging_set_domain(Logging *logging, const char *domain);

  /// # Safety
  ///
  /// Set log level symbols.
  void logging_set_level2sym(Logging *logging, uint8_t level2sym);

  /// # Safety
  ///
  /// Set extended configuration.
  void logging_set_ext_config(Logging *logging, const ExtConfig *ext_config);

  /// # Safety
  ///
  /// Add logger.
  void logging_add_logger(Logging *logging, Logger *logger);

  /// # Safety
  ///
  /// Remove logger.
  void logging_remove_logger(Logging *logging, Logger *logger);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_set_root_writer_config(Logging *logging, WriterConfigEnum *config);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_set_root_writer(Logging *logging, WriterEnum *writer);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_add_writer_config(Logging *logging, WriterConfigEnum *config);

  /// # Safety
  ///
  /// Add writer.
  uintptr_t logging_add_writer(Logging *logging, WriterEnum *writer);

  /// # Safety
  ///
  /// Remove writer.
  const WriterEnum *logging_remove_writer(Logging *logging, uintptr_t wid);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_add_writer_configs(Logging *logging, Vec<WriterConfigEnum> *const *configs_ptr);

  /// # Safety
  ///
  /// Add writer.
  CusizeVec *logging_add_writers(Logging *logging, Vec<WriterEnum> *writers_ptr);

  /// # Safety
  ///
  /// Remove writers.
  WriterEnums *logging_remove_writers(Logging *logging, uint32_t *wids, uint32_t wid_cnt);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_enable(Logging *logging, uintptr_t wid);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_disable(Logging *logging, uintptr_t wid);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_enable_type(Logging *logging, WriterTypeEnum *typ);

  /// # Safety
  ///
  /// Add writer.
  intptr_t logging_disable_type(Logging *logging, WriterTypeEnum *typ);

  /// # Safety
  ///
  /// Sync specific writers.
  intptr_t logging_sync(const Logging *logging, Vec<WriterTypeEnum> *types, double timeout);

  /// # Safety
  ///
  /// Sync all writers.
  intptr_t logging_sync_all(const Logging *logging, double timeout);

  /// # Safety
  ///
  /// Rotate file.
  intptr_t logging_rotate(const Logging *logging, PathBuf *path);

  /// # Safety
  ///
  /// Set encryption.
  intptr_t logging_set_encryption(Logging *logging, unsigned int wid, KeyStruct *key);

  /// # Safety
  ///
  /// Get configuration.
  const WriterConfigEnum *logging_get_writer_config(const Logging *logging, unsigned int wid);

  /// # Safety
  ///
  /// Get configuration.
  const WriterConfigEnums *logging_get_writer_configs(const Logging *logging);

  /// # Safety
  ///
  /// Get server configuration.
  ServerConfig *logging_get_server_config(const Logging *logging, uintptr_t wid);

  /// # Safety
  ///
  /// Get configuration.
  const ServerConfigs *logging_get_server_configs(const Logging *logging);

  /// # Safety
  ///
  /// Get server configuration.
  const uint32_t *logging_get_root_server_address_port(const Logging *logging);

  /// # Safety
  ///
  /// Get server configuration.
  const Cu32StringVec *logging_get_server_addresses_ports(const Logging *logging);

  /// # Safety
  ///
  /// Get server configuration.
  const Cu32StringVec *logging_get_server_addresses(const Logging *logging);

  /// # Safety
  ///
  /// Get server configuration.
  const Cu32u16Vec *logging_get_server_ports(const Logging *logging);

  /// # Safety
  ///
  /// Get server authentification key.
  KeyStruct *logging_get_server_auth_key(const Logging *logging);

  /// # Safety
  ///
  /// Get configuration as string.
  const char *logging_get_config_string(const Logging *logging);

  /// # Safety
  ///
  /// Save configuration.
  intptr_t logging_save_config(Logging *logging, const char *path);

  /// # Safety
  ///
  /// trace message.
  intptr_t logging_trace(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// debug message.
  intptr_t logging_debug(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// info message.
  intptr_t logging_info(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// success message.
  intptr_t logging_success(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// warning message.
  intptr_t logging_warning(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// error message.
  intptr_t logging_error(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// critical message.
  intptr_t logging_critical(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// fatal message.
  intptr_t logging_fatal(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// exception message.
  intptr_t logging_exception(const Logging *logging, const char *message);

  /// # Safety
  ///
  /// Set debug level.
  void logging_set_debug(Logging *logging, uint8_t debug);

  /// # Safety
  ///
  /// Create extended configuration.
  const ExtConfig *ext_config_new(unsigned char structured,
                                  char hostname,
                                  char pname,
                                  char pid,
                                  char tname,
                                  char tid);

  /// # Safety
  ///
  /// Create and return new config for console writer.
  WriterConfigEnum *console_writer_config_new(unsigned char level, char colors);

  /// # Safety
  ///
  /// Create and return new config for file writer.
  WriterConfigEnum *file_writer_config_new(unsigned char level,
                                           const char *path,
                                           unsigned int size,
                                           unsigned int backlog,
                                           int timeout,
                                           long long time,
                                           CompressionMethodEnum *compression);

  /// # Safety
  ///
  /// Create and return new config for client writer.
  WriterConfigEnum *client_writer_config_new(unsigned char level,
                                             const char *address,
                                             CKeyStruct *key);

  /// # Safety
  ///
  /// Create and return new config for server.
  WriterConfigEnum *server_config_new(unsigned char level, const char *address, CKeyStruct *key);

  /// # Safety
  ///
  /// Create and return new config for syslog writer.
  WriterConfigEnum *syslog_writer_config_new(unsigned char level,
                                             const char *hostname,
                                             const char *pname,
                                             unsigned int pid);

  /// # Safety
  ///
  /// Create and return new config for callback writer.
  WriterConfigEnum *callback_writer_config_new(unsigned char level, void (*callback)(unsigned char,
                                                                                     const char *,
                                                                                     const char *));

  /// # Safety
  ///
  /// Create new logger.
  Logger *logger_new(unsigned char level, const char *domain);

  /// # Safety
  ///
  /// Create new logger with extended configuration.
  Logger *logger_new_ext(unsigned char level, const char *domain, char tname, char tid);

  /// # Safety
  ///
  /// Set log level.
  void logger_set_level(Logger *logger, uint8_t level);

  /// # Safety
  ///
  /// Set domain.
  void logger_set_domain(Logger *logger, const char *domain);

  /// # Safety
  ///
  /// trace message.
  intptr_t logger_trace(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// debug message.
  intptr_t logger_debug(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// info message.
  intptr_t logger_info(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// success message.
  intptr_t logger_success(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// warning message.
  intptr_t logger_warning(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// error message.
  intptr_t logger_error(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// critical message.
  intptr_t logger_critical(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// fatal message.
  intptr_t logger_fatal(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// exception message.
  intptr_t logger_exception(const Logger *logger, const char *message);

  /// # Safety
  ///
  /// Create new logging instance.
  void root_init();

  /// # Safety
  ///
  /// Shutdown root
  intptr_t root_shutdown(bool now);

  /// # Safety
  ///
  /// Set logging level.
  intptr_t root_set_level(unsigned int wid, uint8_t level);

  /// # Safety
  ///
  /// Set logging domain.
  void root_set_domain(const char *domain);

  /// # Safety
  ///
  /// Set log level symbols.
  void root_set_level2sym(uint8_t level2sym);

  /// # Safety
  ///
  /// Set extended configuration.
  void root_set_ext_config(const ExtConfig *ext_config);

  /// # Safety
  ///
  /// Add logger.
  void root_add_logger(Logger *logger);

  /// # Safety
  ///
  /// Remove logger.
  void root_remove_logger(Logger *logger);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_set_root_writer_config(WriterConfigEnum *config);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_set_root_writer(WriterEnum *writer);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_add_writer_config(WriterConfigEnum *config);

  /// # Safety
  ///
  /// Add writer.
  uintptr_t root_add_writer(WriterEnum *writer);

  /// # Safety
  ///
  /// Remove writer.
  const WriterEnum *root_remove_writer(uintptr_t wid);

  /// # Safety
  ///
  /// Add writers.
  intptr_t root_add_writer_configs(Vec<WriterConfigEnum> *const *configs_ptr);

  /// # Safety
  ///
  /// Add writers top root logger.
  CusizeVec *root_add_writers(Vec<WriterEnum> *writers_ptr);

  /// # Safety
  ///
  /// Remove writers.
  WriterEnums *root_remove_writers(uint32_t *wids, uint32_t wid_cnt);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_enable(uintptr_t wid);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_disable(uintptr_t wid);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_enable_type(WriterTypeEnum *typ);

  /// # Safety
  ///
  /// Add writer.
  intptr_t root_disable_type(WriterTypeEnum *typ);

  /// # Safety
  ///
  /// Sync specific writers.
  intptr_t root_sync(Vec<WriterTypeEnum> *types, double timeout);

  /// # Safety
  ///
  /// Sync all writers.
  intptr_t root_sync_all(double timeout);

  /// # Safety
  ///
  /// Rotate file.
  intptr_t root_rotate(PathBuf *path);

  /// # Safety
  ///
  /// Set encryption.
  intptr_t root_set_encryption(unsigned int wid, KeyStruct *key);

  /// # Safety
  ///
  /// Get configuration.
  const WriterConfigEnum *root_get_writer_config(unsigned int wid);

  /// # Safety
  ///
  /// Get configuration.
  const WriterConfigEnums *root_get_writer_configs();

  /// # Safety
  ///
  /// Get server configuration.
  ServerConfig *root_get_server_config(uintptr_t wid);

  /// # Safety
  ///
  /// Get configuration.
  const ServerConfigs *root_get_server_configs();

  /// # Safety
  ///
  /// Get server configuration.
  const uint32_t *root_get_root_server_address_port();

  /// # Safety
  ///
  /// Get server configuration.
  const Cu32StringVec *root_get_server_addresses_ports();

  /// # Safety
  ///
  /// Get server configuration.
  const Cu32StringVec *root_get_server_addresses();

  /// # Safety
  ///
  /// Get server configuration.
  const Cu32u16Vec *root_get_server_ports();

  /// # Safety
  ///
  /// Get server authentification key.
  KeyStruct *root_get_server_auth_key();

  /// # Safety
  ///
  /// Get configuration as string.
  const char *root_get_config_string();

  /// # Safety
  ///
  /// Save configuration.
  intptr_t root_save_config(const char *path);

  /// # Safety
  ///
  /// trace message.
  intptr_t root_trace(const char *message);

  /// # Safety
  ///
  /// debug message.
  intptr_t root_debug(const char *message);

  /// # Safety
  ///
  /// info message.
  intptr_t root_info(const char *message);

  /// # Safety
  ///
  /// success message.
  intptr_t root_success(const char *message);

  /// # Safety
  ///
  /// warning message.
  intptr_t root_warning(const char *message);

  /// # Safety
  ///
  /// error message.
  intptr_t root_error(const char *message);

  /// # Safety
  ///
  /// critical message.
  intptr_t root_critical(const char *message);

  /// # Safety
  ///
  /// fatal message.
  intptr_t root_fatal(const char *message);

  /// # Safety
  ///
  /// exception message.
  intptr_t root_exception(const char *message);

  /// # Safety
  ///
  /// Set debug level.
  void root_set_debug(uint8_t debug);

} // extern "C"
