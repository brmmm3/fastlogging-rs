package fastlogging

/*
#include <stdlib.h>
#cgo CFLAGS: -I../h
#cgo LDFLAGS: -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
*/
import "C"
import "unsafe"

// CreateKey creates an encryption key of the given type from raw key bytes.
// If key is empty, a random key is generated (see CreateRandomKey).
func CreateKey(typ EncryptionMethod, key []byte) KeyStruct {
	var c_key *C.uint8_t
	if len(key) > 0 {
		c_key = (*C.uint8_t)(unsafe.Pointer(&key[0]))
	}
	cKey := C.create_key(C.CEncryptionMethodEnum(typ.Into()), C.uint32_t(len(key)), c_key)
	return KeyStruct{Key: unsafe.Pointer(cKey)}
}

// CreateRandomKey creates a random encryption key of the given type.
func CreateRandomKey(typ EncryptionMethod) KeyStruct {
	cKey := C.create_random_key(C.CEncryptionMethodEnum(typ.Into()))
	return KeyStruct{Key: unsafe.Pointer(cKey)}
}
