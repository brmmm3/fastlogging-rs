package logging

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../lib/gofastlogging.h"
*/
import "C"

const DEBUG = 10

type Logging struct {
	logging C.Logging
}

func Init() Logging {
	logging := C.logging_init()
	instance := Logging{logging}
	return instance
}

func New(level uint8, domain *string, console int, file *string, server *string, connect *string, max_size uint32, backlog uint32) Logging {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	var c_file *C.char = nil
	if file != nil {
		c_file = C.CString(*file)
	}
	var c_server *C.char = nil
	if file != nil {
		c_server = C.CString(*server)
	}
	var c_connect *C.char = nil
	if file != nil {
		c_connect = C.CString(*connect)
	}
	logging := C.logging_new(C.uchar(level), c_domain, C.int(console), c_file, c_server, c_connect, C.uint(max_size), C.uint(backlog))
	instance := Logging{logging}
	return instance
}

func (instance Logging) Shutdown(now bool) {
	if now {
		C.logging_shutdown(instance.logging, 1)
	} else {
		C.logging_shutdown(instance.logging, 0)
	}
}

func (instance Logging) SetLevel(level uint8) {
	C.logging_set_level(instance.logging, C.uchar(level))
}

func (instance Logging) SetDomain(domain *string) {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	C.logging_set_domain(instance.logging, c_domain)
}

func (instance Logging) SetLevel2Sym(level2sym uint8) {
	C.logging_set_level2sym(instance.logging, C.uchar(level2sym))
}

// Console writer

func (instance Logging) SetConsoleWriter(level int8) {
	C.logging_set_console_writer(instance.logging, C.schar(level))
}

func (instance Logging) SetConsoleColors(colors uint8) {
	C.logging_set_console_colors(instance.logging, C.uchar(colors))
}

// File writer

func (instance Logging) SetFileWriter(level int8, path string, max_size int, backlog int) {
	C.logging_set_file_writer(instance.logging, C.schar(level), C.CString(path), C.int(max_size), C.int(backlog))
}

func (instance Logging) Rotate() {
	C.logging_rotate(instance.logging)
}

func (instance Logging) Sync(timeout float64) {
	C.logging_sync(instance.logging, C.double(timeout))
}

// Network client

func (instance Logging) Connect(address string, level uint8, key *string) {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	C.logging_connect(instance.logging, C.CString(address), C.uchar(level), c_key)
}

func (instance Logging) Disonnect(address string) {
	C.logging_disconnect(instance.logging, C.CString(address))
}

func (instance Logging) SetClientLevel(address string, level uint8) {
	C.logging_set_client_level(instance.logging, C.CString(address), C.uchar(level))
}

func (instance Logging) SetClientEncryption(address string, key *string) {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	C.logging_set_client_encryption(instance.logging, C.CString(address), c_key)
}

// Network server

func (instance Logging) ServerStart(address string, level uint8, key *string) {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	C.logging_server_start(instance.logging, C.CString(address), C.uchar(level), c_key)
}

func (instance Logging) ServerShutdown() {
	C.logging_server_shutdown(instance.logging)
}

func (instance Logging) SetServerLevel(level uint8) {
	C.logging_set_server_level(instance.logging, C.uchar(level))
}

func (instance Logging) SetServerEncryption(key *string) {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	C.logging_set_server_encryption(instance.logging, c_key)
}

// Logging calls

func (instance Logging) Debug(message string) {
	C.logging_debug(instance.logging, C.CString(message))
}

func (instance Logging) Info(message string) {
	C.logging_info(instance.logging, C.CString(message))
}

func (instance Logging) Warning(message string) {
	C.logging_warning(instance.logging, C.CString(message))
}

func (instance Logging) Error(message string) {
	C.logging_error(instance.logging, C.CString(message))
}

func (instance Logging) Critical(message string) {
	C.logging_critical(instance.logging, C.CString(message))
}

func (instance Logging) Fatal(message string) {
	C.logging_fatal(instance.logging, C.CString(message))
}

func (instance Logging) Exception(message string) {
	C.logging_exception(instance.logging, C.CString(message))
}
