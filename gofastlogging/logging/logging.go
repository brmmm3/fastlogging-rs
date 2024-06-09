package logging

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../lib/cfastlogging.h"
*/
import "C"

// Log-Levels
const NOLOG = C.NOLOG
const EXCEPTION = C.EXCEPTION
const CRITICAL = C.CRITICAL
const FATAL = C.FATAL
const ERROR = C.ERROR
const WARNING = C.WARNING
const WARN = C.WARN
const SUCCESS = C.SUCCESS
const INFO = C.INFO
const DEBUG = C.DEBUG
const TRACE = C.TRACE
const NOTSET = C.NOTSET

type Logger struct {
	logger C.Logger
}

type Logging struct {
	logging C.Logging
}

func ExtConfigNew(structured C.MessageStructEnum, hostname bool, pname bool, pid bool, tname bool, tid bool) C.ExtConfig {
	var c_hostname C.int8_t
	if hostname {
		c_hostname = 1
	}
	var c_pname C.int8_t
	if pname {
		c_pname = 1
	}
	var c_pid C.int8_t
	if pid {
		c_pid = 1
	}
	var c_tname C.int8_t
	if tname {
		c_tname = 1
	}
	var c_tid C.int8_t
	if tid {
		c_tid = 1
	}
	return C.ext_config_new(structured, c_hostname, c_pname, c_pid, c_tname, c_tid)
}

// Console writer

func ConsoleWriterConfigNew(level uint8, colors bool) C.ConsoleWriterConfig {
	var colors_int int8
	if colors {
		colors_int = 1
	}
	return C.console_writer_config_new(C.uint8_t(level), C.int8_t(colors_int))
}

// File writer

func FileWriterConfigNew(level uint8, path string, size uint32, backlog uint32, timeout int32, time int64, compression C.CompressionMethodEnum) C.FileWriterConfig {
	return C.file_writer_config_new(C.uint8_t(level), C.CString(path), C.uint32_t(size), C.uint32_t(backlog), C.int32_t(timeout), C.int64_t(time), compression)
}

// Client writer

func ClientWriterConfigNew(level uint8, address string, encryption C.EncryptionMethod, key *string) C.ClientWriterConfig {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	return C.client_writer_config_new(C.uint8_t(level), C.CString(address), encryption, c_key)
}

// Server

func ServerConfigNew(level uint8, address string, encryption C.EncryptionMethod, key *string) C.ServerConfig {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	return C.server_config_new(C.uint8_t(level), C.CString(address), encryption, c_key)
}

// Syslog writer

func SyslogWriterConfigNew(level uint8, hostname *string, pname *string, pid uint32) C.SyslogWriterConfig {
	var c_hostname *C.char = nil
	if hostname != nil {
		c_hostname = C.CString(*hostname)
	}
	var c_pname *C.char = nil
	if pname != nil {
		c_pname = C.CString(*pname)
	}
	return C.syslog_writer_config_new(C.uint8_t(level), c_hostname, c_pname, C.uint32_t(pid))
}

// Logging module

func Init() Logging {
	logging := C.logging_init()
	instance := Logging{logging}
	return instance
}

func New(level uint8, domain *string, ext_config *C.ExtConfig,
	console *C.ConsoleWriterConfig,
	file *C.FileWriterConfig,
	server *C.ServerConfig,
	connect *C.ClientWriterConfig,
	syslog int8,
	config *string) Logging {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	var c_config *C.char = nil
	if config != nil {
		c_config = C.CString(*config)
	}
	logging := C.logging_new(C.uint8_t(level), c_domain, ext_config, console, file, server, connect, C.int8_t(syslog), c_config)
	instance := Logging{logging}
	return instance
}

func (instance Logging) Shutdown(now bool) int {
	if now {
		return int(C.logging_shutdown(instance.logging, 1))
	} else {
		return int(C.logging_shutdown(instance.logging, 0))
	}
}

func (instance Logging) SetLevel(writer C.WriterTypeEnum, level uint8) int {
	return int(C.logging_set_level(instance.logging, writer, C.uchar(level)))
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

func (instance Logging) SetExtConfig(ext_config C.ExtConfig) {
	C.logging_set_ext_config(instance.logging, ext_config)
}

func (instance Logging) AddWriter(writer C.WriterConfigEnum) int {
	return int(C.logging_add_writer(instance.logging, writer))
}

func (instance Logging) RemoveWriter(writer C.WriterTypeEnum) int {
	return int(C.logging_remove_writer(instance.logging, writer))
}

func (instance Logging) Sync(console bool, file bool, client bool, syslog bool, timeout float64) int {
	var c_console C.int8_t
	if console {
		c_console = 1
	}
	var c_file C.int8_t
	if file {
		c_file = 1
	}
	var c_client C.int8_t
	if client {
		c_client = 1
	}
	var c_syslog C.int8_t
	if syslog {
		c_syslog = 1
	}
	return int(C.logging_sync(instance.logging, c_console, c_file, c_client, c_syslog, C.double(timeout)))
}

func (instance Logging) SyncAll(timeout float64) int {
	return int(C.logging_sync_all(instance.logging, C.double(timeout)))
}

// File writer

func (instance Logging) Rotate(path string) int {
	return int(C.logging_rotate(instance.logging, C.CString(path)))
}

// Network

func (instance Logging) SetEncryption(address string, writer C.WriterTypeEnum, encryption C.EncryptionMethod, key *string) int {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	return int(C.logging_set_encryption(instance.logging, writer, encryption, c_key))
}

// Config

func (instance Logging) GetConfig(writer C.WriterTypeEnum) C.WriterConfigEnum {
	return C.logging_get_config(instance.logging, writer)
}

func (instance Logging) GetServerConfig() C.ServerConfig {
	return C.logging_get_server_config(instance.logging)
}

func (instance Logging) GetServerAuthKey() string {
	return C.GoString(C.logging_get_server_auth_key(instance.logging))
}

func (instance Logging) GetConfigString() string {
	return C.GoString(C.logging_get_config_string(instance.logging))
}

func (instance Logging) SaveConfig(path string) int {
	return int(C.logging_save_config(instance.logging, C.CString(path)))
}

// Logging calls

func (instance Logging) Trace(message string) int {
	return int(C.logging_trace(instance.logging, C.CString(message)))
}

func (instance Logging) Debug(message string) int {
	return int(C.logging_debug(instance.logging, C.CString(message)))
}

func (instance Logging) Info(message string) int {
	return int(C.logging_info(instance.logging, C.CString(message)))
}

func (instance Logging) Success(message string) int {
	return int(C.logging_success(instance.logging, C.CString(message)))
}

func (instance Logging) Warning(message string) int {
	return int(C.logging_warning(instance.logging, C.CString(message)))
}

func (instance Logging) Error(message string) int {
	return int(C.logging_error(instance.logging, C.CString(message)))
}

func (instance Logging) Critical(message string) int {
	return int(C.logging_critical(instance.logging, C.CString(message)))
}

func (instance Logging) Fatal(message string) int {
	return int(C.logging_fatal(instance.logging, C.CString(message)))
}

func (instance Logging) Exception(message string) int {
	return int(C.logging_exception(instance.logging, C.CString(message)))
}
