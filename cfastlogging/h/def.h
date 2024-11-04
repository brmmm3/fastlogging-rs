#ifndef CFASTLOGGING_DEF_H
#define CFASTLOGGING_DEF_H

// TODO: Implement solution for complex enums.

// Lets use some types which we can easily pair with rust types.

#include <stdint.h>

// Log-Levels
#define NOLOG 100
#define EXCEPTION 60
#define CRITICAL 50
#define FATAL CRITICAL
#define ERROR 40
#define WARNING 30
#define WARN WARNING
#define SUCCESS 25
#define INFO 20
#define DEBUG 10
#define TRACE 5
#define NOTSET 0

// Simple enum
typedef enum CLevelSyms: uint8_t
{
    LevelSyms_Sym = 0,
    LevelSyms_Short = 1,
    LevelSyms_Str = 2
} CLevelSyms;

/* Complex enum
typedef enum CFileTypeEnum: uint8_t
{
    FileTypeEnum_Message = 0,
    FileTypeEnum_Sync = 1,
    FileTypeEnum_Rotate = 2,
    FileTypeEnum_Stop = 3
} CFileTypeEnum;*/

typedef void* CFileTypeEnum;

// Simple enum
typedef enum CCompressionMethodEnum: uint8_t
{
    CompressionMethodEnum_Store = 0,
    CompressionMethodEnum_Deflate = 1,
    CompressionMethodEnum_Zstd = 2,
    CompressionMethodEnum_Lzma = 3
} CCompressionMethodEnum;

/* Complex enum
typedef enum CWriterTypeEnum: uint8_t
{
    WriterTypeEnum_Root = 0,
    WriterTypeEnum_Console = 1,
    WriterTypeEnum_File = 2,
    WriterTypeEnum_Files = 3,
    WriterTypeEnum_Client = 4,
    WriterTypeEnum_Clients = 5,
    WriterTypeEnum_Server = 6,
    WriterTypeEnum_Servers = 7,
    WriterTypeEnum_Syslog = 8
} CWriterTypeEnum;*/

typedef void* CWriterTypeEnum;

/* Complex enum
typedef enum CWriterConfigEnum: uint8_t
{
    WriterConfigEnum_Root = 0,
    WriterConfigEnum_Console = 1,
    WriterConfigEnum_File = 2,
    WriterConfigEnum_Client = 3,
    WriterConfigEnum_Server = 4,
    WriterConfigEnum_Callback = 5,
    WriterConfigEnum_Syslog = 6
} CWriterConfigEnum;

typedef struct CWriterConfig
{
    CWriterConfigEnum typ;
    void *config;
} CWriterConfig;*/

typedef void* CWriterConfigEnum;

/* Complex enum
typedef enum CWriterEnum: uint8_t
{
    WriterEnum_Root = 0,
    WriterEnum_Console = 1,
    WriterEnum_File = 2,
    WriterEnum_Client = 3,
    WriterEnum_Server = 4,
    WriterEnum_Callback = 5,
    WriterEnum_Syslog = 6
} CWriterEnum;

typedef struct CWriter
{
    CWriterEnum typ;
    void *writer;
} CWriter;*/

typedef void* CWriterEnum;

// Simple enum
typedef enum CMessageStructEnum: uint8_t
{
    MessageStructEnum_String = 0,
    MessageStructEnum_Json = 1,
    MessageStructEnum_Xml = 2
} CMessageStructEnum;

typedef enum CEncryptionMethodEnum: uint8_t
{
    EncryptionMethod_NONE = 0,
    EncryptionMethod_AuthKey = 1,
    EncryptionMethod_AES = 2
} CEncryptionMethodEnum;

typedef struct CExtConfig {
    CMessageStructEnum structured;  // enum MessageStructEnum
    int8_t hostname;
    int8_t pname;
    int8_t pid;
    int8_t tname;
    int8_t tid;
} CExtConfig;

typedef struct CClientWriterConfig {
    int8_t enabled;
    uint8_t level;
    const char *domain_filter;
    const char *message_filter;
    const char *address;
    uint16_t port;
    int8_t key;  // EncryptionMethod,
    uint8_t debug;
} CClientWriterConfig;

typedef struct CServerConfig
{
    uint8_t level;
    const char *address;
    uint16_t port;
    CEncryptionMethodEnum encryption;
    const char *key;
} CServerConfig;

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

typedef struct CKeyStruct {
    uint typ;
    uint len;
    const char *key;
} CKeyStruct;

typedef void *Logging;

typedef void *Logger;

#endif
