package logger

/*
#include <stdlib.h>
#cgo CFLAGS: -I../../h
#cgo LDFLAGS: -L../../lib -lcfastlogging
#include "../../h/logger.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

type Logger struct {
	Logger C.Logger
}

func New(level uint8, domain *string) *Logger {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
		defer C.free(unsafe.Pointer(c_domain))
	}
	logger := C.logger_new(C.uint8_t(level), c_domain)
	if logger == nil {
		return nil
	}
	return &Logger{Logger: logger}
}

func NewExt(level uint8, domain *string, tname int8, tid int8) *Logger {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
		defer C.free(unsafe.Pointer(c_domain))
	}
	logger := C.logger_new_ext(C.uint8_t(level), c_domain, C.int8_t(tname), C.int8_t(tid))
	if logger == nil {
		return nil
	}
	return &Logger{Logger: logger}
}

func (l *Logger) SetLevel(level uint8) error {
	C.logger_set_level(l.Logger, C.uchar(level))
	return nil
}

func (l *Logger) SetDomain(domain *string) error {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
		defer C.free(unsafe.Pointer(c_domain))
	}
	C.logger_set_domain(l.Logger, c_domain)
	return nil
}

func (l *Logger) Trace(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_trace(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Debug(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_debug(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Info(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_info(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Success(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_success(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Warning(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_warning(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Error(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_error(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Critical(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_critical(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Fatal(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_fatal(l.Logger, cmsg))
	return wrapCError(code)
}

func (l *Logger) Exception(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logger_exception(l.Logger, cmsg))
	return wrapCError(code)
}

// wrapCError converts a C int error code to a Go error
func wrapCError(code int) error {
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging error: code %d", code)
}
