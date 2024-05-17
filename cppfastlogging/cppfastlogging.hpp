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

namespace logging::rust
{
    /// Forward-declaration of opaque type to use as pointer to the Rust object.
    struct Logging;
    struct Logger;
} // namespace logging::rust

extern "C"
{
    static const int NOLOG = 70;
    static const int EXCEPTION = 60;
    static const int CRITICAL = 50;
    static const int FATAL = CRITICAL;
    static const int ERROR = 40;
    static const int WARNING = 30;
    static const int WARN = WARNING;
    static const int INFO = 20;
    static const int DEBUG = 10;
    static const int NOTSET = 0;

    /// We take ownership as we are passing by value, so when function
    /// exits the drop gets run.  Handles being passed null.
    void error_free(Option<Box<Error>>);

    /// Our example "getter" methods which work on the Error type. The value
    /// returned is only valid as long as the Error has not been freed. If C
    /// caller needs a longer lifetime they need to copy the value.
    const char *error_msg(const Error *e);

    intptr_t error_code(const Error *e);

    logging::rust::Logging *logging_init();

    /// For further reading ...
    /// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098
    logging::rust::Logging *logging_new(uint8_t level,
                                        const char *domain,
                                        int console,
                                        const char *file,
                                        const char *server,
                                        const char *connect,
                                        int max_size,
                                        int backlog);

    intptr_t logging_shutdown(logging::rust::Logging *logging, uint8_t now);

    void logging_add_logger(logging::rust::Logging *logging, logging::rust::Logger *logger);

    void logging_remove_logger(logging::rust::Logging *logging, logging::rust::Logger *logger);

    void logging_set_level(logging::rust::Logging *logging, uint8_t level);

    void logging_set_domain(logging::rust::Logging *logging, const char *domain);

    void logging_set_level2sym(logging::rust::Logging *logging, uint8_t level2sym);

    intptr_t logging_set_console_writer(logging::rust::Logging *logging, int8_t level);

    void logging_set_console_colors(logging::rust::Logging *logging, uint8_t colors);

    intptr_t logging_set_file_writer(logging::rust::Logging *logging,
                                     int8_t level,
                                     const char *path,
                                     int max_size,
                                     int backlog);

    intptr_t logging_rotate(const logging::rust::Logging *logging);

    intptr_t logging_sync(const logging::rust::Logging *logging, double timeout);

    intptr_t logging_connect(logging::rust::Logging *logging,
                             const char *address,
                             uint8_t level,
                             const char *key);

    intptr_t logging_disconnect(logging::rust::Logging *logging, const char *address);

    intptr_t logging_set_client_level(logging::rust::Logging *logging, const char *address, uint8_t level);

    intptr_t logging_set_client_encryption(logging::rust::Logging *logging, const char *address, const char *key);

    intptr_t logging_server_start(logging::rust::Logging *logging,
                                  const char *address,
                                  uint8_t level,
                                  const char *key);

    intptr_t logging_server_shutdown(logging::rust::Logging *logging);

    intptr_t logging_set_server_level(logging::rust::Logging *logging, uint8_t level);

    intptr_t logging_set_server_encryption(logging::rust::Logging *logging, const char *key);

    intptr_t logging_debug(const logging::rust::Logging *logging, const char *message);

    intptr_t logging_info(const logging::rust::Logging *logging, const char *message);

    intptr_t logging_warning(const logging::rust::Logging *logging, const char *message);

    intptr_t logging_error(const logging::rust::Logging *logging, const char *message);

    intptr_t logging_critical(const logging::rust::Logging *logging, const char *message);

    intptr_t logging_fatal(const logging::rust::Logging *logging, const char *message);

    intptr_t logging_exception(const logging::rust::Logging *logging, const char *message);

    logging::rust::Logger *logger_new(uint8_t level, const char *domain);

    void logger_set_level(logging::rust::Logger *logger, uint8_t level);

    void logger_set_domain(logging::rust::Logger *logger, const char *domain);

    intptr_t logger_debug(const logging::rust::Logger *logger, const char *message);

    intptr_t logger_info(const logging::rust::Logger *logger, const char *message);

    intptr_t logger_warning(const logging::rust::Logger *logger, const char *message);

    intptr_t logger_error(const logging::rust::Logger *logger, const char *message);

    intptr_t logger_critical(const logging::rust::Logger *logger, const char *message);

    intptr_t logger_fatal(const logging::rust::Logger *logger, const char *message);

    intptr_t logger_exception(const logging::rust::Logger *logger, const char *message);

} // extern "C"

namespace logging
{
    class Logger
    {
        rust::Logger *logger = NULL;

    public:
        Logger(uint8_t level, char *domain)
        {
            logger = logger_new(level, domain);
        }

        ~Logger()
        {
            logger = NULL;
        }

        rust::Logger *ptr()
        {
            return logger;
        }

        void set_level(uint8_t level)
        {
            logger_set_level(logger, level);
        }

        void set_domain(char *domain)
        {
            logger_set_domain(logger, domain);
        }

        int debug(std::string message)
        {
            return logger_debug(logger, message.c_str());
        }

        int info(std::string message)
        {
            return logger_info(logger, message.c_str());
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
            logging = logging_new(NOTSET, NULL, 1, NULL, NULL, NULL, 0, 0);
        }

        Logging(uint8_t level, char *domain, int console, char *file, char *server, char *connect, uint32_t max_size, uint32_t backlog)
        {
            logging = logging_new(level, domain, console, file, server, connect, max_size, backlog);
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

        void add_logger(Logger &logger)
        {
            logging_add_logger(logging, logger.ptr());
        }

        void remove_logger(Logger &logger)
        {
            logging_remove_logger(logging, logger.ptr());
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

        int set_console_writer(int8_t level)
        {
            return logging_set_console_writer(logging, level);
        }

        void set_console_colors(bool colors)
        {
            logging_set_console_colors(logging, (uint8_t)colors);
        }

        int set_file_writer(int8_t level, const char *path, int max_size, int backlog)
        {
            return logging_set_file_writer(logging, level, path, max_size, backlog);
        }

        int rotate()
        {
            return logging_rotate(logging);
        }

        int sync(double timeout)
        {
            return logging_sync(logging, timeout);
        }

        int connect(const char *address, uint8_t level, const char *key)
        {
            return logging_connect(logging, address, level, key);
        }

        int disconnect(const char *address)
        {
            return logging_disconnect(logging, address);
        }

        void set_client_level(const char *address, uint8_t level)
        {
            logging_set_client_level(logging, address, level);
        }

        void set_client_encryption(const char *address, const char *key)
        {
            logging_set_client_encryption(logging, address, key);
        }

        int server_start(const char *address,
                         uint8_t level,
                         const char *key)
        {
            return logging_server_start(logging, address, level, key);
        }

        int server_shutdown()
        {
            return logging_server_shutdown(logging);
        }

        void set_server_level(uint8_t level)
        {
            logging_set_server_level(logging, level);
        }

        void set_server_encryption(const char *key)
        {
            logging_set_server_encryption(logging, key);
        }

        int debug(std::string message)
        {
            return logging_debug(logging, message.c_str());
        }

        int info(std::string message)
        {
            return logging_info(logging, message.c_str());
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
