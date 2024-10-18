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
    struct WriterEnum;
    struct WriterConfigEnum;
    struct ConsoleWriterConfig;
    struct FileWriterConfig;
    struct ServerConfig;
    struct ClientWriterConfig;
    struct SyslogWriterConfig;
    struct CallbackWriterConfig;
} // namespace logging::rust

extern "C"
{
    // Console writer

    rust::WriterConfigEnum *console_writer_config_new(uint8_t level,
                                                         int8_t colors);

    // File writer

    rust::WriterConfigEnum *file_writer_config_new(uint8_t level,
                                                   const char *path,
                                                   uint32_t size,
                                                   uint32_t backlog,
                                                   int32_t timeout,
                                                   int64_t time,
                                                   CCompressionMethodEnum_t compression);

    // Client writer

    rust::WriterConfigEnum *client_writer_config_new(uint8_t level,
                                                     const char *address,
                                                     CEncryptionMethodEnum_t encryption,
                                                     const char *key);

    // Server

    rust::WriterConfigEnum *server_config_new(uint8_t level,
                                              const char *address,
                                              CEncryptionMethodEnum_t encryption,
                                              const char *key);

    // Syslog writer

    rust::WriterConfigEnum *syslog_writer_config_new(uint8_t level,
                                                     const char *hostname,
                                                     const char *pname,
                                                     uint32_t pid);

    // Callback writer

    rust::WriterConfigEnum *callback_writer_config_new(uint8_t level,
                                                       void (*callback)(uint8_t, const char *, const char *));

    // Classes

    class WriterConfig
    {
    public:
        rust::WriterConfigEnum *config = NULL;
    };

    class ConsoleWriterConfig : public WriterConfig
    {
    public:
        ConsoleWriterConfig(uint8_t level, bool colors)
        {
            config = console_writer_config_new(level, (int8_t)colors);
        }

        ~ConsoleWriterConfig()
        {
            config = NULL;
        }
    };

    class FileWriterConfig : public WriterConfig
    {
    public:
        FileWriterConfig(uint8_t level,
                         const char *path,
                         uint32_t size,
                         uint32_t backlog,
                         int32_t timeout,
                         int64_t time,
                         CCompressionMethodEnum_t compression)
        {
            config = file_writer_config_new(level, path, size, backlog, timeout, time, compression);
        }

        ~FileWriterConfig()
        {
            config = NULL;
        }
    };

    class ClientWriterConfig : public WriterConfig
    {
    public:
        ClientWriterConfig(uint8_t level,
                           const char *address,
                           CEncryptionMethodEnum_t encryption,
                           const char *key)
        {
            config = client_writer_config_new(level, address, encryption, key);
        }

        ~ClientWriterConfig()
        {
            config = NULL;
        }
    };

    class ServerConfig : public WriterConfig
    {
    public:
        ServerConfig(uint8_t level,
                     const char *address,
                     CEncryptionMethodEnum_t encryption,
                     const char *key)
        {
            config = server_config_new(level, address, encryption, key);
        }

        ~ServerConfig()
        {
            config = NULL;
        }
    };

    class SyslogWriterConfig : public WriterConfig
    {
    public:
        SyslogWriterConfig(uint8_t level,
                           const char *hostname,
                           const char *pname,
                           uint32_t pid)
        {
            config = syslog_writer_config_new(level, hostname, pname, pid);
        }

        ~SyslogWriterConfig()
        {
            config = NULL;
        }
    };

    class CallbackWriterConfig : public WriterConfig
    {
    public:
        CallbackWriterConfig(uint8_t level,
                             void (*callback)(uint8_t, const char *, const char *))
        {
            config = callback_writer_config_new(level, callback);
        }

        ~CallbackWriterConfig()
        {
            config = NULL;
        }
    };
}
