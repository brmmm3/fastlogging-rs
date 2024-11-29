#pragma once

using namespace std;

#include "def.hpp"
#include "writer.hpp"
#include "root.hpp"
#include "logging.hpp"
#include "logger.hpp"

extern "C"
{
    CKeyStruct *create_key(CEncryptionMethodEnum typ, uint32_t len, const uint8_t *key);

    CKeyStruct *create_random_key(CEncryptionMethodEnum typ);
}
