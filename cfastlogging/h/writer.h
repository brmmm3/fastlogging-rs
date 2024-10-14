#ifndef CFASTLOGGING_WRITER_H
#define CFASTLOGGING_WRITER_H

// Lets use some types which we can easily pair with rust types.
#include "def.h"

// Console writer

CWriterConfigEnum_t console_writer_config_new(uint8_t level, int8_t colors);

// File writer

CWriterConfigEnum_t file_writer_config_new(uint8_t level,
                                           const char *path,
                                           uint32_t size,
                                           uint32_t backlog,
                                           int32_t timeout,
                                           int64_t time,
                                           CCompressionMethodEnum_t compression);

// Client writer

CWriterConfigEnum_t client_writer_config_new(uint8_t level,
                                             const char *address,
                                             CEncryptionMethodEnum_t encryption,
                                             const char *key);

// Server

CWriterConfigEnum_t server_config_new(uint8_t level,
                                      const char *address,
                                      CEncryptionMethodEnum_t encryption,
                                      const char *key);

// Syslog writer

CWriterConfigEnum_t syslog_writer_config_new(uint8_t level,
                                             const char *hostname,
                                             const char *pname,
                                             uint32_t pid);

// Callback writer

CWriterConfigEnum_t callback_writer_config_new(uint8_t level,
                                               void (*callback)(uint8_t, const char *, const char *));

#endif
