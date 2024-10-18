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

// Simple enum
typedef enum CLevelSyms: uint8_t
{
    Sym = 0,
    Short = 1,
    Str = 2
} CLevelSyms_t;

/* Complex enum
typedef enum CFileTypeEnum: uint8_t
{
    Message = 0,
    Sync = 1,
    Rotate = 2,
    Stop = 3
} CFileTypeEnum_t;*/

typedef void* CFileTypeEnum_t;

// Simple enum
typedef enum CCompressionMethodEnum: uint8_t
{
    Store = 0,
    Deflate = 1,
    Zstd = 2,
    Lzma = 3
} CCompressionMethodEnum_t;

/* Complex enum
typedef enum CWriterTypeEnum: uint8_t
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
} CWriterTypeEnum_t;*/

typedef void* CWriterTypeEnum_t;

/* Complex enum
typedef enum CWriterConfigEnum: uint8_t
{
    Root = 0,
    Console = 1,
    File = 2,
    Client = 3,
    Server = 4,
    Callback = 5,
    Syslog = 6
} CWriterConfigEnum_t;

typedef struct CWriterConfig
{
    CWriterConfigEnum_t typ;
    void *config;
} CWriterConfig_t;*/

typedef void* CWriterConfigEnum_t;

/* Complex enum
typedef enum CWriterEnum: uint8_t
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

typedef void* CWriterEnum_t;

// Simple enum
typedef enum CMessageStructEnum: uint8_t
{
    String = 0,
    Json = 1,
    Xml = 2
} CMessageStructEnum_t;

typedef enum CEncryptionMethodEnum: uint8_t
{
    NONE = 0,
    AuthKey = 1,
    AES = 2
} CEncryptionMethodEnum_t;

typedef struct CExtConfig {
    CMessageStructEnum_t structured;  // enum MessageStructEnum
    int8_t hostname;
    int8_t pname;
    int8_t pid;
    int8_t tname;
    int8_t tid;
} CExtConfig_t;

typedef struct CClientWriterConfig {
    int8_t enabled;
    uint8_t level;
    const char *domain_filter;
    const char *message_filter;
    const char *address;
    uint16_t port;
    int8_t key;  // EncryptionMethod,
    uint8_t debug;
} CClientWriterConfig_t;

typedef struct CServerConfig
{
    uint8_t level;
    const char *address;
    uint16_t port;
    CEncryptionMethodEnum_t encryption;
    const char *key;
} CServerConfig_t;

typedef struct Cu32StringVec {
    uint32_t cnt;
    uint32_t *keys;
    char **values;
} Cu32StringVec_t;

typedef struct Cu32u16Vec {
    uint32_t cnt;
    uint32_t *keys;
    uint16_t *values;
} Cu32u16Vec_t;
