#ifndef CFASTLOGGING_H
#define CFASTLOGGING_H

#include "def.h"
#include "writer.h"
#include "root.h"
#include "logging.h"
#include "logger.h"

CKeyStruct *create_key(CEncryptionMethodEnum typ, uint32_t len, const uint8_t *key);

CKeyStruct *create_random_key(CEncryptionMethodEnum typ);

#endif
