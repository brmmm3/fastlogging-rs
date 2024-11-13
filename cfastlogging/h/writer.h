#ifndef CFASTLOGGING_WRITER_H
#define CFASTLOGGING_WRITER_H

// Lets use some types which we can easily pair with rust types.
#include "def.h"

// Console writer

WriterConfigEnum *console_writer_config_new(uint8_t level,
                                            int8_t colors);

// File writer

WriterConfigEnum *file_writer_config_new(uint8_t level,
                                         const char *path,
                                         uint32_t size,
                                         uint32_t backlog,
                                         int32_t timeout,
                                         int64_t time,
                                         CompressionMethodEnum compression);

// Client writer

WriterConfigEnum *client_writer_config_new(uint8_t level,
                                           const char *address,
                                           const KeyStruct *key);

// Server

WriterConfigEnum *server_config_new(uint8_t level,
                                    const char *address,
                                    const KeyStruct *key);

// Syslog writer

WriterConfigEnum *syslog_writer_config_new(uint8_t level,
                                           const char *hostname,
                                           const char *pname,
                                           uint32_t pid);

// Callback writer

WriterConfigEnum *callback_writer_config_new(uint8_t level,
                                             void (*callback)(uint8_t, const char *, const char *));

#endif
