#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <new>

using namespace std;

template <typename T = void>
struct Box;

template <typename T = void>
struct Option;

struct Error
{
    uint32_t magic;
    char *msg;
    intptr_t code;
};

enum class LevelSyms : uint8_t
{
    Sym,
    Short,
    Str,
};

enum class FileTypeEnum : uint8_t
{
    Message,
    Sync,
    Rotate,
    Stop
};

enum class CompressionMethodEnum : uint8_t
{
    Store,
    Deflate,
    Zstd,
    Lzma
};

enum class WriterTypeEnum : uint8_t
{
    Root,
    Console,
    File,
    Client,
    Server,
    Syslog
};

enum class MessageStructEnum : uint8_t
{
    String,
    Json,
    Xml
};

enum class EncryptionMethod : uint8_t
{
    NONE,
    AuthKey,
    AES
};

namespace rust
{
    /// Forward-declaration of opaque type to use as pointer to the Rust object.
    struct ExtConfig;
    struct WriterConfigEnum;
    struct ConsoleWriterConfig;
    struct FileWriterConfig;
    struct ServerConfig;
    struct ClientWriterConfig;
    struct SyslogWriterConfig;
    struct CallbackWriterConfig;
    struct Logging;
    struct Logger;
} // namespace logging::rust

extern "C"
{
    static const int NOLOG = 100;
    static const int EXCEPTION = 60;
    static const int CRITICAL = 50;
    static const int FATAL = CRITICAL;
    static const int ERROR = 40;
    static const int WARNING = 30;
    static const int WARN = WARNING;
    static const int SUCCESS = 25;
    static const int INFO = 20;
    static const int DEBUG = 10;
    static const int TRACE = 5;
    static const int NOTSET = 0;

    /// We take ownership as we are passing by value, so when function
    /// exits the drop gets run.  Handles being passed null.
    void error_free(Option<Box<Error>>);

    /// Our example "getter" methods which work on the Error type. The value
    /// returned is only valid as long as the Error has not been freed. If C
    /// caller needs a longer lifetime they need to copy the value.
    const char *error_msg(const Error *e);

    intptr_t error_code(const Error *e);

    rust::ExtConfig *ext_config_new(MessageStructEnum structured,
                                    int8_t hostname,
                                    int8_t pname,
                                    int8_t pid,
                                    int8_t tname,
                                    int8_t tid);

    // Console writer

    rust::ConsoleWriterConfig *console_writer_config_new(uint8_t level, int8_t colors);

    // File writer

    rust::FileWriterConfig *file_writer_config_new(uint8_t level,
                                                   const char *path,
                                                   uint32_t size,
                                                   uint32_t backlog,
                                                   int32_t timeout,
                                                   int64_t time,
                                                   CompressionMethodEnum compression);

    // Client writer

    rust::ClientWriterConfig *client_writer_config_new(uint8_t level,
                                                       const char *address,
                                                       EncryptionMethod encryption,
                                                       const char *key);

    // Server

    rust::ServerConfig *server_config_new(uint8_t level,
                                          const char *address,
                                          EncryptionMethod encryption,
                                          const char *key);

    // Syslog writer

    rust::SyslogWriterConfig *syslog_writer_config_new(uint8_t level,
                                                       const char *hostname,
                                                       const char *pname,
                                                       uint32_t pid);

    // Callback writer

    rust::CallbackWriterConfig *callback_writer_config_new(uint8_t level,
                                                           void (*callback)(uint8_t, const char *, const char *));

    rust::Logging *logging_init();

    /// For further reading ...
    /// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098
    rust::Logging *logging_new(uint8_t level,
                               const char *domain,
                               rust::WriterConfigEnum *configs_ptr,
                               uint config_cnt,
                               rust::ExtConfig *ext_config,
                               const char *config_path);

    int logging_apply_config(rust::Logging logging, const char *path);

    int logging_shutdown(rust::Logging *logging, uint8_t now);

    int logging_set_level(rust::Logging *logging, uint8_t level);

    void logging_set_domain(rust::Logging *logging, const char *domain);

    void logging_set_level2sym(rust::Logging *logging, uint8_t level2sym);

    void logging_set_ext_config(rust::Logging *logging, rust::ExtConfig *ext_config);

    void logging_add_logger(rust::Logging *logging, rust::Logger *logger);

    void logging_remove_logger(rust::Logging *logging, rust::Logger *logger);

    int logging_set_root_writer_config(rust::Logging logging, rust::WriterConfigEnum config);

    int logging_set_root_writer(rust::Logging logging, rust::WriterEnum writer);

    int logging_add_writer_config(rust::Logging logging, rust::WriterEnum writer);

    int logging_add_writer(rust::Logging *logging, rust::WriterConfigEnum *writer);

    void logging_remove_writer(rust::Logging *logging, WriterTypeEnum writer);

    int logging_add_writer_configs(rust::Logging logging, rust::WriterConfigEnum *configs, uint config_cnt);

    int logging_add_writers(rust::Logging logging, rust::WriterEnum *writers, uint writer_cnt);

    int logging_remove_writers(rust::Logging logging, uint *wids, uint wid_cnt);

    int logging_enable(rust::Logging logging, uint wid);

    int logging_disable(rust::Logging logging, uint wid);

    int logging_enable_type(rust::Logging logging, rust::WriterTypeEnum typ);

    int logging_disable_type(rust::Logging logging, rust::WriterTypeEnum typ);

    intptr_t logging_sync(const rust::Logging *logging, int console, int file, int client, int syslog, int callback, double timeout);

    intptr_t logging_sync_all(const rust::Logging *logging, double timeout);

    // File writer

    intptr_t logging_rotate(const rust::Logging *logging);

    // Network

    intptr_t logging_set_encryption(rust::Logging *logging, WriterTypeEnum writer, EncryptionMethod encryption, char *key);

    // Config

    void logging_set_debug(uint debug);

    rust::WriterConfigEnum *logging_get_config(rust::Logging *logging, WriterTypeEnum writer);

    rust::WriterConfigEnum *logging_get_writer_configs(Logging logging);

    rust::CServerConfig *logging_get_server_config(rust::Logging *logging);

    rust::CServerConfig *logging_get_server_configs(rust::Logging logging);

    const char *logging_get_root_server_address_port(rust::Logging logging);

    const rust::CusizeStringHashMap *logging_get_server_addresses_ports(rust::Logging logging);

    const rust::CusizeStringHashMap *logging_get_server_addresses(rust::Logging logging);

    const rust::Cusizeu16HashMap *logging_get_server_ports(rust::Logging logging);

    const char *logging_get_server_auth_key(rust::Logging *logging);

    const char *logging_get_config_string(rust::Logging *logging);

    int logging_save_config(rust::Logging *logging, const char *path);

    // Logging calls

    intptr_t logging_trace(const rust::Logging *logging, const char *message);

    intptr_t logging_debug(const rust::Logging *logging, const char *message);

    intptr_t logging_info(const rust::Logging *logging, const char *message);

    intptr_t logging_success(const rust::Logging *logging, const char *message);

    intptr_t logging_warning(const rust::Logging *logging, const char *message);

    intptr_t logging_error(const rust::Logging *logging, const char *message);

    intptr_t logging_critical(const rust::Logging *logging, const char *message);

    intptr_t logging_fatal(const rust::Logging *logging, const char *message);

    intptr_t logging_exception(const rust::Logging *logging, const char *message);

    // Logger

    rust::Logger *logger_new(uint8_t level, const char *domain);

    rust::Logger *logger_new_ext(uint8_t level, const char *domain, int8_t tname, int8_t tid);

    void logger_set_level(rust::Logger *logger, uint8_t level);

    void logger_set_domain(rust::Logger *logger, const char *domain);

    intptr_t logger_trace(const rust::Logger *logger, const char *message);

    intptr_t logger_debug(const rust::Logger *logger, const char *message);

    intptr_t logger_info(const rust::Logger *logger, const char *message);

    intptr_t logger_success(const rust::Logger *logger, const char *message);

    intptr_t logger_warning(const rust::Logger *logger, const char *message);

    intptr_t logger_error(const rust::Logger *logger, const char *message);

    intptr_t logger_critical(const rust::Logger *logger, const char *message);

    intptr_t logger_fatal(const rust::Logger *logger, const char *message);

    intptr_t logger_exception(const rust::Logger *logger, const char *message);

} // extern "C"

namespace logging
{
    class ExtConfig
    {
    public:
        rust::ExtConfig *config = NULL;

        ExtConfig(MessageStructEnum structured,
                  int8_t hostname,
                  int8_t pname,
                  int8_t pid,
                  int8_t tname,
                  int8_t tid)
        {
            config = ext_config_new(structured, hostname, pname, pid, tname, tid);
        }

        ~ExtConfig()
        {
            config = NULL;
        }
    };

    class ConsoleWriterConfig
    {
    public:
        rust::ConsoleWriterConfig *writer = NULL;

        ConsoleWriterConfig(uint8_t level, bool colors)
        {
            writer = console_writer_config_new(level, (int8_t)colors);
        }

        ~ConsoleWriterConfig()
        {
            writer = NULL;
        }
    };

    class ConsoleWriterConfigEnum
    {
    public:
        rust::WriterConfigEnum *writer = NULL;

        ConsoleWriterConfigEnum(uint8_t level, bool colors)
        {
            writer = console_writer_config_enum_new(level, (int8_t)colors);
        }

        ~ConsoleWriterConfigEnum()
        {
            writer = NULL;
        }
    };

    class FileWriterConfig
    {
    public:
        rust::FileWriterConfig *writer = NULL;

        FileWriterConfig(uint8_t level,
                         const char *path,
                         uint32_t size,
                         uint32_t backlog,
                         int32_t timeout,
                         int64_t time,
                         CompressionMethodEnum compression)
        {
            writer = file_writer_config_new(level, path, size, backlog, timeout, time, compression);
        }

        ~FileWriterConfig()
        {
            writer = NULL;
        }
    };

    class FileWriterConfigEnum
    {
    public:
        rust::WriterConfigEnum *writer = NULL;

        FileWriterConfigEnum(uint8_t level,
                             const char *path,
                             uint32_t size,
                             uint32_t backlog,
                             int32_t timeout,
                             int64_t time,
                             CompressionMethodEnum compression)
        {
            writer = file_writer_config_enum_new(level, path, size, backlog, timeout, time, compression);
        }

        ~FileWriterConfigEnum()
        {
            writer = NULL;
        }
    };

    class ClientWriterConfig
    {
    public:
        rust::ClientWriterConfig *writer = NULL;

        ClientWriterConfig(uint8_t level,
                           const char *address,
                           EncryptionMethod encryption,
                           const char *key)
        {
            writer = client_writer_config_new(level, address, encryption, key);
        }

        ~ClientWriterConfig()
        {
            writer = NULL;
        }
    };

    class ServerConfig
    {
    public:
        rust::ServerConfig *writer = NULL;

        ServerConfig(uint8_t level,
                     const char *address,
                     EncryptionMethod encryption,
                     const char *key)
        {
            writer = server_config_new(level, address, encryption, key);
        }

        ~ServerConfig()
        {
            writer = NULL;
        }
    };

    class SyslogWriterConfig
    {
    public:
        rust::SyslogWriterConfig *writer = NULL;

        SyslogWriterConfig(uint8_t level,
                           const char *hostname,
                           const char *pname,
                           uint32_t pid)
        {
            writer = syslog_writer_config_new(level, hostname, pname, pid);
        }

        ~SyslogWriterConfig()
        {
            writer = NULL;
        }
    };

    class CallbackWriterConfig
    {
    public:
        rust::CallbackWriterConfig *writer = NULL;

        CallbackWriterConfig(uint8_t level,
                             void (*callback)(uint8_t, const char *, const char *))
        {
            writer = callback_writer_config_new(level, callback);
        }

        ~CallbackWriterConfig()
        {
            writer = NULL;
        }
    };

    class CallbackWriterConfigEnum
    {
    public:
        rust::WriterConfigEnum *writer = NULL;

        CallbackWriterConfigEnum(uint8_t level,
                                 void (*callback)(uint8_t, const char *, const char *))
        {
            writer = callback_writer_config_enum_new(level, callback);
        }

        ~CallbackWriterConfigEnum()
        {
            writer = NULL;
        }
    };

    class Logger
    {
    public:
        rust::Logger *logger = NULL;

        Logger(uint8_t level, const char *domain)
        {
            logger = logger_new(level, domain);
        }

        Logger(uint8_t level, const char *domain, int8_t tname, int8_t tid)
        {
            logger = logger_new_ext(level, domain, tname, tid);
        }

        ~Logger()
        {
            logger = NULL;
        }

        void set_level(uint8_t level)
        {
            logger_set_level(logger, level);
        }

        void set_domain(char *domain)
        {
            logger_set_domain(logger, domain);
        }

        // Logging calls

        int trace(std::string message)
        {
            return logger_trace(logger, message.c_str());
        }

        int debug(std::string message)
        {
            return logger_debug(logger, message.c_str());
        }

        int info(std::string message)
        {
            return logger_info(logger, message.c_str());
        }

        int success(std::string message)
        {
            return logger_success(logger, message.c_str());
        }

        int warn(std::string message)
        {
            return logger_warning(logger, message.c_str());
        }

        int warning(std::string message)
        {
            return logger_warning(logger, message.c_str());
        }

        int error(std::string message)
        {
            return logger_error(logger, message.c_str());
        }

        int critical(std::string message)
        {
            return logger_critical(logger, message.c_str());
        }

        int fatal(std::string message)
        {
            return logger_fatal(logger, message.c_str());
        }

        int exception(std::string message)
        {
            return logger_exception(logger, message.c_str());
        }
    };

    class Logging
    {
        rust::Logging *logging = NULL;

    public:
        Logging()
        {
            logging = logging_init();
        }

        Logging(uint8_t level,
                const char *domain,
                ExtConfig *ext_config,
                ConsoleWriterConfig *console,
                FileWriterConfig *file,
                ServerConfig *server,
                ClientWriterConfig *connect,
                int8_t syslog,
                const char *config)
        {
            logging = logging_new(level,
                                  domain,
                                  ext_config ? ext_config->config : NULL,
                                  console ? console->writer : NULL,
                                  file ? file->writer : NULL,
                                  server ? server->writer : NULL,
                                  connect ? connect->writer : NULL,
                                  syslog,
                                  config);
        }

        ~Logging()
        {
            logging_shutdown(logging, 0);
            logging = NULL;
        }

        int shutdown(bool now)
        {
            return logging_shutdown(logging, (uint8_t)now);
        }

        void set_level(uint8_t level)
        {
            logging_set_level(logging, level);
        }

        void set_domain(char *domain)
        {
            logging_set_domain(logging, domain);
        }

        void set_level2sym(uint8_t level2sym)
        {
            logging_set_level2sym(logging, level2sym);
        }

        void set_ext_config(rust::ExtConfig *ext_config)
        {
            logging_set_ext_config(logging, ext_config);
        }

        void add_logger(Logger *logger)
        {
            logging_add_logger(logging, logger->logger);
        }

        void remove_logger(Logger *logger)
        {
            logging_remove_logger(logging, logger->logger);
        }

        int add_writer(rust::WriterConfigEnum *writer)
        {
            return logging_add_writer(logging, writer);
        }

        void remove_writer(WriterTypeEnum writer)
        {
            logging_remove_writer(logging, writer);
        }

        int sync(int console, int file, int client, int syslog, int callback, double timeout)
        {
            return logging_sync(logging, console, file, client, syslog, callback, timeout);
        }

        int sync_all(double timeout)
        {
            return logging_sync_all(logging, timeout);
        }

        // File writer

        int rotate()
        {
            return logging_rotate(logging);
        }

        // Network

        int set_encryption(WriterTypeEnum writer, EncryptionMethod encryption, char *key)
        {
            return logging_set_encryption(logging, writer, encryption, key);
        }

        // Config

        rust::WriterConfigEnum *get_config(WriterTypeEnum writer)
        {
            return logging_get_config(logging, writer);
        }

        rust::ServerConfig *get_server_config()
        {
            return logging_get_server_config(logging);
        }

        const char *get_server_address()
        {
            return logging_get_server_address(logging);
        }

        const char *get_server_auth_key()
        {
            return logging_get_server_auth_key(logging);
        }

        const char *get_config_string()
        {
            return logging_get_config_string(logging);
        }

        int save_config(const char *path)
        {
            return logging_save_config(logging, path);
        }

        // Logging calls

        int trace(std::string message)
        {
            return logging_trace(logging, message.c_str());
        }

        int debug(std::string message)
        {
            return logging_debug(logging, message.c_str());
        }

        int info(std::string message)
        {
            return logging_info(logging, message.c_str());
        }

        int success(std::string message)
        {
            return logging_success(logging, message.c_str());
        }

        int warn(std::string message)
        {
            return logging_warning(logging, message.c_str());
        }

        int warning(std::string message)
        {
            return logging_warning(logging, message.c_str());
        }

        int error(std::string message)
        {
            return logging_error(logging, message.c_str());
        }

        int critical(std::string message)
        {
            return logging_critical(logging, message.c_str());
        }

        int fatal(std::string message)
        {
            return logging_fatal(logging, message.c_str());
        }

        int exception(std::string message)
        {
            return logging_exception(logging, message.c_str());
        }
    };
}
