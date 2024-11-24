package logger

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"

type Logger struct {
	Logger C.Logger
}

func New(
	level uint8,
	domain *string,
) Logger {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	return Logger{Logger: C.logger_new(
		C.uint8_t(level),
		c_domain,
	)}
}

func NewExt(
	level uint8,
	domain *string,
	tname int8,
	tid int8,
) Logger {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	return Logger{Logger: C.logger_new_ext(
		C.uint8_t(level),
		c_domain,
		C.int8_t(tname),
		C.int8_t(tid),
	)}
}

func (instance Logger) SetLevel(wid uint, level uint8) {
	C.logger_set_level(instance.Logger, C.uchar(level))
}

func (instance Logger) SetDomain(domain *string) {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	C.logger_set_domain(instance.Logger, c_domain)
}

// Logger calls

func (instance Logger) Trace(message string) int {
	return int(C.logger_trace(instance.Logger, C.CString(message)))
}

func (instance Logger) Debug(message string) int {
	return int(C.logger_debug(instance.Logger, C.CString(message)))
}

func (instance Logger) Info(message string) int {
	return int(C.logger_info(instance.Logger, C.CString(message)))
}

func (instance Logger) Success(message string) int {
	return int(C.logger_success(instance.Logger, C.CString(message)))
}

func (instance Logger) Warning(message string) int {
	return int(C.logger_warning(instance.Logger, C.CString(message)))
}

func (instance Logger) Error(message string) int {
	return int(C.logger_error(instance.Logger, C.CString(message)))
}

func (instance Logger) Critical(message string) int {
	return int(C.logger_critical(instance.Logger, C.CString(message)))
}

func (instance Logger) Fatal(message string) int {
	return int(C.logger_fatal(instance.Logger, C.CString(message)))
}

func (instance Logger) Exception(message string) int {
	return int(C.logger_exception(instance.Logger, C.CString(message)))
}
