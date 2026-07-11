#ifndef CFASTLOGGING_H
#define CFASTLOGGING_H

#include "def.h"
#include "logger.h"
#include "logging.h"
#include "root.h"
#include "writer.h"

KeyStruct *create_key(EncryptionMethodEnum typ, uint32_t len,
                      const uint8_t *key);

KeyStruct *create_random_key(EncryptionMethodEnum typ);

#endif
