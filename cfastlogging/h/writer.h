#ifndef CFASTLOGGING_WRITER_H
#define CFASTLOGGING_WRITER_H

// Lets use some types which we can easily pair with rust types.
#include "def.h"

// Console writer

CWriterConfigEnum console_writer_config_new(uint8_t level,
                                            int8_t colors);

// File writer

CWriterConfigEnum file_writer_config_new(uint8_t level,
                                         const char *path,
                                         uint32_t size,
                                         uint32_t backlog,
                                         int32_t timeout,
                                         int64_t time,
                                         CCompressionMethodEnum compression);

// Client writer

CWriterConfigEnum client_writer_config_new(uint8_t level,
                                           const char *address,
                                           const CKeyStruct *key);

// Server

CWriterConfigEnum server_config_new(uint8_t level,
                                    const char *address,
                                    const CKeyStruct *key);

// Syslog writer

CWriterConfigEnum syslog_writer_config_new(uint8_t level,
                                           const char *hostname,
                                           const char *pname,
                                           uint32_t pid);

// Callback writer

CWriterConfigEnum callback_writer_config_new(uint8_t level,
                                             void (*callback)(uint8_t, const char *, const char *));

#endif
