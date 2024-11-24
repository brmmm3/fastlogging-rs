package fastlogging

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
*/
import "C"
import "unsafe"

func CreateKey(
	typ C.CEncryptionMethodEnum,
	len uint,
	key *uint8) *C.CKeyStruct {
	return C.create_key(typ, C.uint32_t(len), (*C.uint8_t)(unsafe.Pointer(key)))
}

func CreateRandomKey(typ C.CEncryptionMethodEnum) *C.CKeyStruct {
	return C.create_random_key(typ)
}
