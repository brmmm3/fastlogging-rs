#pragma once

#include "def.hpp"
#include "writer.hpp"
#include "root.hpp"
#include "logging.hpp"
#include "logger.hpp"

extern "C" {
rust::KeyStruct *create_key(rust::EncryptionMethodEnum typ, uint32_t len,
                             const uint8_t *key);
rust::KeyStruct *create_random_key(rust::EncryptionMethodEnum typ);
} // extern "C"
