#pragma once

// TODO: Implement solution for complex enums.

// Lets use some types which we can easily pair with rust types.

#include <cstdint>

// Log-Levels
extern "C"
{
    static const uint8_t NOLOG = 100;
    static const uint8_t EXCEPTION = 60;
    static const uint8_t CRITICAL = 50;
    static const uint8_t FATAL = CRITICAL;
    static const uint8_t ERROR = 40;
    static const uint8_t WARNING = 30;
    static const uint8_t WARN = WARNING;
    static const uint8_t SUCCESS = 25;
    static const uint8_t INFO = 20;
    static const uint8_t DEBUG = 10;
    static const uint8_t TRACE = 5;
    static const uint8_t NOTSET = 0;

    /// We take ownership as we are passing by value, so when function
    /// exits the drop gets run.  Handles being passed null.
    void error_free(const void *e);

    /// Our example "getter" methods which work on the Error type. The value
    /// returned is only valid as long as the Error has not been freed. If C
    /// caller needs a longer lifetime they need to copy the value.
    const char *error_msg(const void *e);

    intptr_t error_code(const void *e);
}

namespace rust {
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

    typedef struct Cu32StringVec {
        uint32_t cnt;
        uint32_t *keys;
        char **values;
    } Cu32StringVec;

    typedef struct Cu32u16Vec {
        uint32_t cnt;
        uint32_t *keys;
        uint16_t *values;
    } Cu32u16Vec;

    // Simple enum
    enum class LevelSyms: uint8_t
    {
        Sym = 0,
        Short = 1,
        Str = 2
    };

    enum class FileTypeEnum: uint8_t
    {
        Message = 0,
        Sync = 1,
        Rotate = 2,
        Stop = 3
    };

    enum class CompressionMethodEnum: uint8_t
    {
        Store = 0,
        Deflate = 1,
        Zstd = 2,
        Lzma = 3
    };

    /* Complex enum
    typedef enum WriterTypeEnum: uint8_t
    {
        Root = 0,
        Console = 1,
        File = 2,
        Files = 3,
        Client = 4,
        Clients = 5,
        Server = 6,
        Servers = 7,
        Syslog = 8
    } WriterTypeEnum;*/

    // typedef void* WriterTypeEnum;  --> Defined in writer.hpp

    /* Complex enum
    typedef enum WriterConfigEnum: uint8_t
    {
        Root = 0,
        Console = 1,
        File = 2,
        Client = 3,
        Server = 4,
        Callback = 5,
        Syslog = 6
    } WriterConfigEnum;

    typedef struct CWriterConfig
    {
        WriterConfigEnum typ;
        void *config;
    } CWriterConfig;*/

    // typedef void* WriterConfigEnum;  --> Defined in writer.hpp

    /* Complex enum
    typedef enum WriterEnum: uint8_t
    {
        Root = 0,
        Console = 1,
        File = 2,
        Client = 3,
        Server = 4,
        Callback = 5,
        Syslog = 6
    } CWriterEnum_t;

    typedef struct CWriter
    {
        CWriterEnum_t typ;
        void *writer;
    } CWriter_t;*/

    typedef void* WriterEnum;

    typedef struct WriterEnums {
        uint32_t cnt;
        WriterEnum *values;
    } WriterEnums;

    // Simple enum
    enum class MessageStructEnum: uint8_t
    {
        String = 0,
        Json = 1,
        Xml = 2
    };

    enum class EncryptionMethodEnum: uint8_t
    {
        NONE = 0,
        AuthKey = 1,
        AES = 2
    };

    typedef struct ExtConfig {
        MessageStructEnum structured;  // enum MessageStructEnum
        int8_t hostname;
        int8_t pname;
        int8_t pid;
        int8_t tname;
        int8_t tid;
    } ExtConfig;

    typedef struct KeyStruct
    {
        EncryptionMethodEnum typ;
        uint32_t len;
        const char *key;
    } KeyStruct;

    typedef struct ClientWriterConfig {
        int8_t enabled;
        uint8_t level;
        const char *domain_filter;
        const char *message_filter;
        const char *address;
        uint16_t port;
        KeyStruct *key;  // EncryptionMethod,
        uint8_t debug;
    } ClientWriterConfig;

    typedef struct ServerConfig
    {
        uint8_t level;
        const char *address;
        uint16_t port;
        KeyStruct *key;
        const char *port_File;
    } ServerConfig;

    typedef struct ServerConfigs
    {
        uint32_t cnt;
        uint32_t *keys;
        ServerConfig *values;
    } ServerConfigs;
}
