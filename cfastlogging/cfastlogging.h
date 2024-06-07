#ifndef CFASTLOGGING
#define CFASTLOGGING

// File: cfastlogging.h

// Lets use some types which we can easily pair with rust types.
#include <stdint.h>

typedef enum
{
    Sym = 0,
    Short = 1,
    Str = 2
} LevelSyms;

typedef enum
{
    Message = 0,
    Sync = 1,
    Rotate = 2,
    Stop = 3
} FileTypeEnum;

typedef enum
{
    Store = 0,
    Deflate = 1,
    Zstd = 2,
    Lzma = 3
} CompressionMethodEnum;

typedef enum
{
    Root = 0,
    Console = 1,
    File = 2,
    Client = 3,
    Server = 4,
    Syslog = 5
} WriterTypeEnum;

typedef void *WriterConfigEnum;

typedef enum
{
    String = 0,
    Json = 1,
    Xml = 2
} MessageStructEnum;

typedef enum
{
    NONE = 0,
    AuthKey = 1,
    AES = 2
} EncryptionMethod;

typedef void *ExtConfig;

ExtConfig ext_config_new(MessageStructEnum structured,
                         int8_t hostname,
                         int8_t pname,
                         int8_t pid,
                         int8_t tname,
                         int8_t tid);

// Console writer

typedef void *ConsoleWriterConfig;

ConsoleWriterConfig console_writer_config_new(uint8_t level, int8_t colors);

// File writer

typedef void *FileWriterConfig;

FileWriterConfig file_writer_config_new(uint8_t level,
                                        const char *path,
                                        uint32_t size,
                                        uint32_t backlog,
                                        int32_t timeout,
                                        int64_t time,
                                        CompressionMethodEnum compression);

// Client writer

typedef void *ClientWriterConfig;

ClientWriterConfig client_writer_config_new(uint8_t level,
                                            const char *address,
                                            EncryptionMethod encryption,
                                            const char *key);

// Server

typedef void *ServerConfig;

ServerConfig server_config_new(uint8_t level,
                               const char *address,
                               EncryptionMethod encryption,
                               const char *key);

// Syslog writer

typedef void *SyslogWriterConfig;

SyslogWriterConfig syslog_writer_config_new(uint8_t level,
                                            const char *hostname,
                                            const char *pname,
                                            uint32_t pid);

// Logger module

typedef void *Logger;

Logger logger_new(uint8_t level, const char *domain);

Logger logger_new_ext(uint8_t level, const char *domain, int8_t tname, int8_t tid);

void logger_set_level(Logger logger, uint8_t level);

void logger_set_domain(Logger logger, const char *domain);

// Logger calls

int logger_trace(Logger logger, const char *message);

int logger_debug(Logger logger, const char *message);

int logger_info(Logger logger, const char *message);

int logger_success(Logger logger, const char *message);

int logger_warning(Logger logger, const char *message);

int logger_error(Logger logger, const char *message);

int logger_critical(Logger logger, const char *message);

int logger_fatal(Logger logger, const char *message);

int logger_exception(Logger logger, const char *message);

// Logging module

typedef void *Logging;

Logging logging_init();

Logging logging_new(uint8_t level,
                    const char *domain,
                    ExtConfig *ext_config,
                    ConsoleWriterConfig *console,
                    FileWriterConfig *file,
                    ServerConfig *server,
                    ClientWriterConfig *connect,
                    int8_t syslog,
                    const char *config);

int logging_shutdown(Logging logging, int8_t now);

void logging_add_logger(Logging logging, Logger logger);

void logging_remove_logger(Logging logging, Logger logger);

int logging_set_level(Logging logging, WriterTypeEnum writer, uint8_t level);

void logging_set_domain(Logging logging, const char *domain);

void logging_set_level2sym(Logging logging, uint8_t level2sym);

void logging_set_ext_config(Logging logging, ExtConfig ext_config);

int logging_add_writer(Logging logging, WriterConfigEnum writer);

int logging_remove_writer(Logging logging, WriterTypeEnum writer);

int logging_sync(Logging logging, int8_t console, int8_t file, int8_t client, int8_t syslog, double timeout);

int logging_sync_all(Logging logging, double timeout);

// File writer

int logging_rotate(Logging logging, const char *path);

// Network

int logging_set_encryption(Logging logging, WriterTypeEnum writer, EncryptionMethod encryption, char *key);

// Config

WriterConfigEnum logging_get_config(Logging logging, WriterTypeEnum writer);

ServerConfig logging_get_server_config(Logging logging);

const char *logging_get_server_auth_key(Logging logging);

const char *logging_get_config_string(Logging logging);

int logging_save_config(Logging logging, const char *path);

// Logging calls

int logging_trace(Logging logging, const char *message);

int logging_debug(Logging logging, const char *message);

int logging_info(Logging logging, const char *message);

int logging_success(Logging logging, const char *message);

int logging_warning(Logging logging, const char *message);

int logging_error(Logging logging, const char *message);

int logging_critical(Logging logging, const char *message);

int logging_fatal(Logging logging, const char *message);

int logging_exception(Logging logging, const char *message);

#endif
