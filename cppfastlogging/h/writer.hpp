#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <new>
#include <variant>

using namespace std;

#include "def.hpp"
#include "logger.hpp"

namespace rust
{
    struct Root {};
    struct Console {};
    struct File { const char *path; };
    struct Files {};
    struct Client { const char *address_port; };
    struct Clients {};
    struct Server { const char *address_port; };
    struct Servers {};
    struct Callback {};
    struct Syslog {};
    using WriterTypeEnum = std::variant<Root, Console, File, Files, Client, Clients, Server, Servers, Callback, Syslog>;

    struct RootConfig {
        uint8_t level;
        const char *domain;
        const char *hostname;
        const char *pname;
        uint32_t pid;
        bool tname;
        bool tid;
        MessageStructEnum structured;
        LevelSyms level2sym;
    };

    enum class ConsoleTargetEnum {
        StdOut = 0,
        StdErr = 1,
        Both = 2,
    };

    struct ConsoleWriterConfig {
        bool enabled;
        uint8_t level;
        const char *domain_filter;
        const char *message_filter;
        bool colors;
        ConsoleTargetEnum target;
        uint8_t debug;
    };

    struct FileWriterConfig {};
    struct CallbackWriterConfig {};
    struct SyslogWriterConfig {};
    using WriterConfigEnum = std::variant<RootConfig, ConsoleWriterConfig, FileWriterConfig, ClientWriterConfig, ServerConfig, CallbackWriterConfig, SyslogWriterConfig>;

    template<typename T, typename... Ts>
    std::ostream& operator<<(std::ostream& os, const std::variant<T, Ts...>& v)
    {
        std::visit([&os](auto&& arg) {
            os << arg;
        }, v);
        return os;
    }

    //struct WriterEnum;

    /*typedef struct KeyStruct {
        unsigned int typ;
        unsigned int len;
        const char *key;
    } KeyStruct;*/
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
                                                   rust::CompressionMethodEnum compression);

    // Client writer

    rust::WriterConfigEnum *client_writer_config_new(uint8_t level,
                                                     const char *address,
                                                     const rust::KeyStruct *key);

    // Server

    rust::WriterConfigEnum *server_config_new(uint8_t level,
                                              const char *address,
                                              const rust::KeyStruct *key);

    // Syslog writer

    rust::WriterConfigEnum *syslog_writer_config_new(uint8_t level,
                                                     const char *hostname,
                                                     const char *pname,
                                                     uint32_t pid);

    // Callback writer

    rust::WriterConfigEnum *callback_writer_config_new(uint8_t level,
                                                       void (*callback)(uint8_t, const char *, const char *));

    // Classes

    enum class CompressionMethod: uint8_t
    {
        Store = 0,
        Deflate = 1,
        Zstd = 2,
        Lzma = 3
    };

    enum class EncryptionMethod: uint8_t
    {
        NONE = 0,
        AuthKey = 1,
        AES = 2
    };

    class WriterConfig
    {
    public:
        rust::WriterConfigEnum *config = NULL;
    };

    class ConsoleWriterConfig : public WriterConfig
    {
    public:
        ConsoleWriterConfig(uint8_t level, bool colors = false)
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
                         uint32_t size = 0,
                         uint32_t backlog = 0,
                         int32_t timeout = -1,
                         int64_t time = -1,
                         CompressionMethod compression = CompressionMethod::Store)
        {
            rust::CompressionMethodEnum compression_enum = rust::CompressionMethodEnum::Store;
            switch (compression) {
                case CompressionMethod::Store:
                    // Already handled above
                    break;
                case CompressionMethod::Deflate:
                    compression_enum = rust::CompressionMethodEnum::Deflate;
                    break;
                case CompressionMethod::Zstd:
                    compression_enum = rust::CompressionMethodEnum::Zstd;
                    break;
                case CompressionMethod::Lzma:
                    compression_enum = rust::CompressionMethodEnum::Lzma;
            }
            config = file_writer_config_new(level, path, size, backlog, timeout, time, compression_enum);
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
                           const rust::KeyStruct *key = NULL)
        {
            config = client_writer_config_new(level, address, key);
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
                     const rust::KeyStruct *key = NULL)
        {
            config = server_config_new(level, address, key);
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
                           const char *hostname = NULL,
                           const char *pname = NULL,
                           uint32_t pid = 0)
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
