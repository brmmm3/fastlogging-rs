#ifndef CFASTLOGGING
#define CFASTLOGGING

// File: cfastlogging.h

// Lets use some types which we can easily pair with rust types.
#include <stdint.h>

enum LevelSyms
{
    Sym = 0,
    Short = 1,
    Str = 2
};

typedef void *Logging;
typedef void *Logger;

// Logging module

Logging logging_init();

Logging logging_new(uint8_t level, char *domain, int console, char *file, char *server, char *connect, uint32_t max_size, uint32_t backlog);

int logging_shutdown(Logging logging, uint8_t now);

void logging_add_logger(Logging logging, Logger logger);

void logging_remove_logger(Logging logging, Logger logger);

void logging_set_level(Logging logging, uint8_t level);

void logging_set_domain(Logging logging, char *domain);

void logging_set_level2sym(Logging logging, uint8_t level2sym);

// Console writer

int logging_set_console_writer(Logging logging, int8_t level);

void logging_set_console_colors(Logging logging, uint8_t colors);

// File writer

int logging_set_file_writer(Logging logging, int8_t level, const char *path, int max_size, int backlog);

int logging_rotate(Logging logging);

int logging_sync(Logging logging, double timeout);

// Network client

int logging_connect(Logging logging, const char *address, uint8_t level, const char *key);

int logging_disconnect(Logging logging, const char *address);

int logging_set_client_level(Logging logging, const char *address, uint8_t level);

int logging_set_client_encryption(Logging logging, const char *address, const char *key);

// Network server

int logging_server_start(Logging logging, const char *address, uint8_t level, const char *key);

int logging_server_shutdown(Logging logging);

int logging_set_server_level(Logging logging, uint8_t level);

int logging_set_server_encryption(Logging logging, const char *key);

// Logging calls

int logging_debug(Logging logging, char *message);

int logging_info(Logging logging, char *message);

int logging_warning(Logging logging, char *message);

int logging_error(Logging logging, char *message);

int logging_critical(Logging logging, char *message);

int logging_fatal(Logging logging, char *message);

int logging_exception(Logging logging, char *message);

// Logger module

Logger logger_new(uint8_t level, char *domain);

void logger_set_level(Logger logger, uint8_t level);

void logger_set_domain(Logger logger, char *domain);

int logger_debug(Logger logger, char *message);

int logger_info(Logger logger, char *message);

int logger_warning(Logger logger, char *message);

int logger_error(Logger logger, char *message);

int logger_critical(Logger logger, char *message);

int logger_fatal(Logger logger, char *message);

int logger_exception(Logger logger, char *message);

#endif
