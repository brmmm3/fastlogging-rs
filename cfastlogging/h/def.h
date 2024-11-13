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

typedef struct CusizeVec
{
    uint32_t cnt;
    size_t **values;
} CusizeVec;

typedef struct Cu32StringVec
{
    uint32_t cnt;
    uint *keys;
    char **values;
} Cu32StringVec;

typedef struct Cu32u16Vec
{
    uint32_t cnt;
    uint32_t *keys;
    uint16_t *values;
} Cu32u16Vec;

// Simple enum
typedef enum LevelSyms: uint8_t
{
    LevelSyms_Sym = 0,
    LevelSyms_Short = 1,
    LevelSyms_Str = 2
} LevelSyms;

typedef enum FileTypeEnumTyp: uint8_t
{
    FileTypeEnum_Message = 0,
    FileTypeEnum_Sync = 1,
    FileTypeEnum_Rotate = 2,
    FileTypeEnum_Stop = 3
} FileTypeEnumTyp;

typedef void* FileTypeEnum;

// Simple enum
typedef enum CompressionMethodEnum: uint8_t
{
    CompressionMethodEnum_Store = 0,
    CompressionMethodEnum_Deflate = 1,
    CompressionMethodEnum_Zstd = 2,
    CompressionMethodEnum_Lzma = 3
} CompressionMethodEnum;

typedef enum WriterTypeEnumTyp: uint8_t
{
    WriterConfigEnum_Root = 0,
    WriterConfigEnum_Console = 1,
    WriterConfigEnum_File = 2,
    WriterConfigEnum_Files = 3,
    WriterConfigEnum_Client = 4,
    WriterConfigEnum_Clients = 5,
    WriterConfigEnum_Server = 6,
    WriterConfigEnum_Servers = 7,
    WriterConfigEnum_Callback = 8,
    WriterConfigEnum_Syslog = 9
} WriterTypeEnumTyp;

typedef struct WriterTypeEnum
{
    WriterTypeEnumTyp typ;
    const char *value;
} WriterTypeEnum;

typedef struct WriterTypeEnums
{
    uint32_t cnt;
    WriterTypeEnum **types;
} WriterTypeEnums;

typedef enum WriterEnumTyp: uint8_t
{
    WriterEnumTyp_Root = 0,
    WriterEnumTyp_Console = 1,
    WriterEnumTyp_File = 2,
    WriterEnumTyp_Client = 3,
    WriterEnumTyp_Server = 4,
    WriterEnumTyp_Callback = 5,
    WriterEnumTyp_Syslog = 6
} WriterEnumTyp;

typedef struct WriterConfigEnum
{
    WriterEnumTyp typ;
    void *config;
} WriterConfigEnum;

typedef struct WriterConfigEnums
{
    uint32_t cnt;
    uint32_t *wids;
    WriterConfigEnum **configs;
} WriterConfigEnums;

/*typedef struct WriterEnum
{
    WriterEnumTyp typ;
    void *writer;
} WriterEnum;*/

typedef void* WriterEnum;

typedef struct WriterEnums
{
    uint32_t cnt;
    const WriterEnum **writers;
} WriterEnums;

// Simple enum
typedef enum MessageStructEnum: uint8_t
{
    MessageStructEnum_String = 0,
    MessageStructEnum_Json = 1,
    MessageStructEnum_Xml = 2
} MessageStructEnum;

typedef enum EncryptionMethodEnum: uint8_t
{
    EncryptionMethod_NONE = 0,
    EncryptionMethod_AuthKey = 1,
    EncryptionMethod_AES = 2
} EncryptionMethodEnum;

typedef struct ExtConfig
{
    MessageStructEnum structured;  // enum MessageStructEnum
    int8_t hostname;
    int8_t pname;
    int8_t pid;
    int8_t tname;
    int8_t tid;
} ExtConfig;

// EncryptionMethod
typedef struct KeyStruct
{
    uint typ;
    uint len;
    const char *key;
} KeyStruct;

typedef struct ClientWriterConfig
{
    int8_t enabled;
    uint8_t level;
    const char *domain_filter;
    const char *message_filter;
    const char *address;
    uint16_t port;
    KeyStruct *key;  // EncryptionMethod
    uint8_t debug;
} ClientWriterConfig;

typedef struct ServerConfig
{
    uint8_t level;
    const char *address;
    uint16_t port;
    KeyStruct *key;  // EncryptionMethod
} ServerConfig;

typedef struct ServerConfigs
{
    uint32_t cnt;
    size_t *keys;
    ServerConfig *values;
} ServerConfigs;

typedef void *Logging;

typedef void *Logger;

#endif
