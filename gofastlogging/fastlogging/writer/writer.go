package writer

/*
#include <stdint.h>
#include <stdlib.h>

extern void go_logging_callback_writer(uintptr_t h, char level, char *domain, char *message);

static uintptr_t go_callback_handle;

static inline void set_go_callback_handle(uintptr_t h) {
	go_callback_handle = h;
}

static void go_callback_trampoline(uint8_t level, const char *domain, const char *message) {
	go_logging_callback_writer(go_callback_handle, (char)level, (char *)domain, (char *)message);
}

#cgo CFLAGS: -I../../h
#cgo LDFLAGS: -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"

static inline WriterConfigEnum go_callback_writer_config_new(uint8_t level) {
	return callback_writer_config_new(level, go_callback_trampoline);
}
*/
import "C"
import (
	"fmt"
	fl "gofastlogging/fastlogging"
	"runtime/cgo"
	"unsafe"
)

// Console writer

func ConsoleWriterConfigNew(level uint8, colors bool) *fl.WriterConfigEnum {
	var colors_int int8
	if colors {
		colors_int = 1
	}
	config := C.console_writer_config_new(C.uint8_t(level), C.int8_t(colors_int))
	if config == nil {
		return nil
	}
	return &fl.WriterConfigEnum{Config: unsafe.Pointer(config)}
}

// File writer

func FileWriterConfigNew(level uint8, path string, size uint32, backlog uint32, timeout int32, time int64, compression fl.CompressionMethod) *fl.WriterConfigEnum {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	// Pass nil for compression when Store (0) is selected — the Rust side
	// defaults to Store.  cgo does not reliably handle the C23 enum-with-
	// fixed-underlying-type syntax (enum : uint8_t) used in def.h, so taking
	// the address of a C.CCompressionMethodEnum value can cause heap corruption.
	var cCompression *C.CCompressionMethodEnum = nil
	if compression != fl.Store {
		tmp := C.CCompressionMethodEnum(compression.Into())
		cCompression = &tmp
	}
	config := C.file_writer_config_new(
		C.uint8_t(level),
		cpath,
		C.uint32_t(size),
		C.uint32_t(backlog),
		C.int32_t(timeout),
		C.int64_t(time),
		cCompression)
	if config == nil {
		return nil
	}
	return &fl.WriterConfigEnum{Config: unsafe.Pointer(config)}
}

// Client writer

func ClientWriterConfigNew(level uint8, address string, key *fl.KeyStruct) *fl.WriterConfigEnum {
	caddr := C.CString(address)
	defer C.free(unsafe.Pointer(caddr))
	var c_key *C.CKeyStruct = nil
	if key != nil {
		c_key = (*C.CKeyStruct)(key.Key)
	}
	config := C.client_writer_config_new(C.uint8_t(level), caddr, c_key)
	if config == nil {
		return nil
	}
	return &fl.WriterConfigEnum{Config: unsafe.Pointer(config)}
}

// Server

func ServerConfigNew(level uint8, address string, key *fl.KeyStruct) *fl.WriterConfigEnum {
	caddr := C.CString(address)
	defer C.free(unsafe.Pointer(caddr))
	var c_key *C.CKeyStruct = nil
	if key != nil {
		c_key = (*C.CKeyStruct)(key.Key)
	}
	config := C.server_config_new(C.uint8_t(level), caddr, c_key)
	if config == nil {
		return nil
	}
	return &fl.WriterConfigEnum{Config: unsafe.Pointer(config)}
}

// Syslog writer

func SyslogWriterConfigNew(level uint8, hostname, pname string, pid uint32) *fl.WriterConfigEnum {
	chost := C.CString(hostname)
	defer C.free(unsafe.Pointer(chost))
	cpname := C.CString(pname)
	defer C.free(unsafe.Pointer(cpname))
	config := C.syslog_writer_config_new(C.uint8_t(level), chost, cpname, C.uint32_t(pid))
	if config == nil {
		return nil
	}
	return &fl.WriterConfigEnum{Config: unsafe.Pointer(config)}
}

// Callback writer

//export go_logging_callback_writer
func go_logging_callback_writer(h C.uintptr_t, level C.char, domain *C.char, message *C.char) {
	handle := cgo.Handle(h)
	if cb, ok := handle.Value().(func(level uint8, domain, message string)); ok {
		cb(uint8(level), C.GoString(domain), C.GoString(message))
	}
}

// CallbackHandle is a handle for a registered Go callback.
type CallbackHandle struct {
	Handle cgo.Handle
}

// CallbackWriterConfigNew registers a Go callback and returns a WriterConfigEnum.
// The callback receives (level uint8, domain string, message string).
// The returned CallbackHandle must be kept alive as long as the writer is in use.
func CallbackWriterConfigNew(level uint8, callback func(level uint8, domain, message string)) (fl.WriterConfigEnum, CallbackHandle, error) {
	if callback == nil {
		return fl.WriterConfigEnum{}, CallbackHandle{}, fmt.Errorf("callback must not be nil")
	}
	handle := cgo.NewHandle(callback)
	C.set_go_callback_handle(C.uintptr_t(handle))
	config := C.go_callback_writer_config_new(C.uint8_t(level))
	if config == nil {
		handle.Delete()
		C.set_go_callback_handle(0)
		return fl.WriterConfigEnum{}, CallbackHandle{}, fmt.Errorf("failed to create callback writer")
	}
	return fl.WriterConfigEnum{Config: unsafe.Pointer(config)}, CallbackHandle{Handle: handle}, nil
}

// UnregisterCallback releases the Go callback handle. Call when the writer is no longer needed.
func (h CallbackHandle) UnregisterCallback() {
	C.set_go_callback_handle(0)
	h.Handle.Delete()
}
