package fastlogging

/*
#include <stdint.h>

extern void go_logging_callback_writer(uintptr_t h, char level, char *domain, char *message);

static inline void CallCallbackWriter(uintptr_t h, char level, char *domain, char *message) {
    go_logging_callback_writer(h, level, domain, message);
}

#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
*/
import "C"
import (
	"runtime/cgo"
	"unsafe"
)

// Console writer

func ConsoleWriterConfigNew(
	level uint8,
	colors bool) WriterConfigEnum {
	var colors_int int8
	if colors {
		colors_int = 1
	}
	return WriterConfigEnum{Config: C.console_writer_config_new(C.uint8_t(level), C.int8_t(colors_int))}
}

// File writer

func FileWriterConfigNew(
	level uint8,
	path string,
	size uint32,
	backlog uint32,
	timeout int32,
	time int64,
	compression CompressionMethodEnum) WriterConfigEnum {
	return WriterConfigEnum{Config: C.file_writer_config_new(
		C.uint8_t(level),
		C.CString(path),
		C.uint32_t(size),
		C.uint32_t(backlog),
		C.int32_t(timeout),
		C.int64_t(time),
		compression.Into())}
}

// Client writer

func ClientWriterConfigNew(
	level uint8,
	address string,
	key *KeyStruct) WriterConfigEnum {
	var c_key *C.CKeyStruct = nil
	if key != nil {
		c_key = key.Key
	}
	return WriterConfigEnum{Config: C.client_writer_config_new(C.uint8_t(level), C.CString(address), c_key)}
}

// Server

func ServerConfigNew(
	level uint8,
	address string,
	key *KeyStruct) WriterConfigEnum {
	var c_key *C.CKeyStruct = nil
	if key != nil {
		c_key = key.Key
	}
	return WriterConfigEnum{Config: C.server_config_new(C.uint8_t(level), C.CString(address), c_key)}
}

// Syslog writer

func SyslogWriterConfigNew(
	level uint8,
	hostname *string,
	pname *string,
	pid uint32) WriterConfigEnum {
	var c_hostname *C.char = nil
	if hostname != nil {
		c_hostname = C.CString(*hostname)
	}
	var c_pname *C.char = nil
	if pname != nil {
		c_pname = C.CString(*pname)
	}
	return WriterConfigEnum{Config: C.syslog_writer_config_new(C.uint8_t(level), c_hostname, c_pname, C.uint32_t(pid))}
}

// Callback writer

//export go_logging_callback_writer
func go_logging_callback_writer(h C.uintptr_t, level C.char, domain *C.char, message *C.char) {
	fn := cgo.Handle(h).Value().(func(C.char, *C.char, *C.char))
	fn(level, domain, message)
}

func CallbackWriterConfigNew(
	level uint8,
	callback uintptr) WriterConfigEnum {
	fn := go_logging_callback_writer
	// TODO
	return WriterConfigEnum{Config: C.callback_writer_config_new(C.uint8_t(level), (*[0]byte)(unsafe.Pointer(&fn)))}
}
