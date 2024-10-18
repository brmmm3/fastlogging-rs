#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <new>

using namespace std;

#include "def.hpp"

namespace rust
{
    /// Forward-declaration of opaque type to use as pointer to the Rust object.
    struct Logger;
} // namespace logging::rust

extern "C"
{
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
}
