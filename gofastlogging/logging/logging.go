package logging

/*
#include <stdint.h>

extern void go_logging_callback_writer(uintptr_t h, char level, char *domain, char *message);

static inline void CallCallbackWriter(uintptr_t h, char level, const char *domain, const char *message) {
    go_logging_callback_writer(h, level, domain, message);
}

#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../lib/cfastlogging.h"
*/
import "C"
import (
	"runtime/cgo"
	"unsafe"
)

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

type LevelSyms int

const (
	Sym LevelSyms = iota
	Short
	Str
)

func (s LevelSyms) into() C.LevelSyms {
	switch s {
	case Sym:
		return 0
	case Short:
		return 1
	case Str:
		return 2
	}
	return 0
}

type FileTypeEnum int

const (
	Message FileTypeEnum = iota
	Sync
	Rotate
	Stop
)

func (s FileTypeEnum) into() C.FileTypeEnum {
	switch s {
	case Message:
		return 0
	case Sync:
		return 1
	case Rotate:
		return 2
	case Stop:
		return 3
	}
	return 0
}

type CompressionMethodEnum int

const (
	Store CompressionMethodEnum = iota
	Deflate
	Zstd
	Lzma
)

func (s CompressionMethodEnum) into() C.CompressionMethodEnum {
	switch s {
	case Store:
		return 0
	case Deflate:
		return 1
	case Zstd:
		return 2
	case Lzma:
		return 3
	}
	return 0
}

type WriterConfigEnum C.WriterConfigEnum

type WriterTypeEnum int

const (
	Root WriterTypeEnum = iota
	Console
	File
	Client
	Server
	Syslog
)

func (s WriterTypeEnum) into() C.WriterTypeEnum {
	switch s {
	case Root:
		return 0
	case File:
		return 1
	case Client:
		return 2
	case Server:
		return 3
	case Syslog:
		return 4
	}
	return 0
}

type MessageStructEnum int

const (
	String MessageStructEnum = iota
	Json
	Xml
)

func (s MessageStructEnum) into() C.MessageStructEnum {
	switch s {
	case String:
		return 0
	case Json:
		return 1
	case Xml:
		return 2
	}
	return 0
}

type EncryptionMethod int

const (
	NONE EncryptionMethod = iota
	AuthKey
	AES
)

func (s EncryptionMethod) into() C.EncryptionMethod {
	switch s {
	case NONE:
		return 0
	case AuthKey:
		return 1
	case AES:
		return 2
	}
	return 0
}

type ExtConfig struct {
	config C.ExtConfig
}

func (s ExtConfig) New(
	structured MessageStructEnum,
	hostname bool,
	pname bool,
	pid bool,
	tname bool,
	tid bool) ExtConfig {
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
	return ExtConfig{C.ext_config_new(structured.into(), c_hostname, c_pname, c_pid, c_tname, c_tid)}
}

type ConsoleWriterConfig struct {
	config C.ConsoleWriterConfig
}

type FileWriterConfig struct {
	config C.FileWriterConfig
}

type ClientWriterConfig struct {
	config C.ClientWriterConfig
}

type ServerConfig struct {
	config C.ServerConfig
}

type SyslogWriterConfig struct {
	config C.SyslogWriterConfig
}

type CallbackWriterConfig struct {
	config C.CallbackWriterConfig
}

type Logger struct {
	logger C.Logger
}

type Logging struct {
	logging C.Logging
}

// Console writer

func ConsoleWriterConfigNew(
	level uint8,
	colors bool) ConsoleWriterConfig {
	var colors_int int8
	if colors {
		colors_int = 1
	}
	return ConsoleWriterConfig{C.console_writer_config_new(C.uint8_t(level), C.int8_t(colors_int))}
}

// File writer

func FileWriterConfigNew(
	level uint8,
	path string,
	size uint32,
	backlog uint32,
	timeout int32,
	time int64,
	compression CompressionMethodEnum) FileWriterConfig {
	return FileWriterConfig{C.file_writer_config_new(
		C.uint8_t(level),
		C.CString(path),
		C.uint32_t(size),
		C.uint32_t(backlog),
		C.int32_t(timeout),
		C.int64_t(time),
		compression.into())}
}

// Client writer

func ClientWriterConfigNew(
	level uint8,
	address string,
	encryption EncryptionMethod,
	key *string) ClientWriterConfig {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	return ClientWriterConfig{C.client_writer_config_new(C.uint8_t(level), C.CString(address), encryption.into(), c_key)}
}

// Server

func ServerConfigNew(
	level uint8,
	address string,
	encryption EncryptionMethod,
	key *string) ServerConfig {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	return ServerConfig{C.server_config_new(C.uint8_t(level), C.CString(address), encryption.into(), c_key)}
}

// Syslog writer

func SyslogWriterConfigNew(
	level uint8,
	hostname *string,
	pname *string,
	pid uint32) SyslogWriterConfig {
	var c_hostname *C.char = nil
	if hostname != nil {
		c_hostname = C.CString(*hostname)
	}
	var c_pname *C.char = nil
	if pname != nil {
		c_pname = C.CString(*pname)
	}
	return SyslogWriterConfig{C.syslog_writer_config_new(C.uint8_t(level), c_hostname, c_pname, C.uint32_t(pid))}
}

// Callback writer

//export go_logging_callback_writer
func go_logging_callback_writer(h C.uintptr_t, level C.char, domain *C.char, message *C.char) {
	fn := cgo.Handle(h).Value().(func(C.char, *C.char, *C.char))
	fn(level, domain, message)
}

func CallbackWriterConfigNew(
	level uint8,
	callback uintptr) CallbackWriterConfig {
	fn := go_logging_callback_writer
	// TODO
	return CallbackWriterConfig{C.callback_writer_config_new(C.uint8_t(level), (*[0]byte)(unsafe.Pointer(&fn)))}
}

// Logging module

func Init() Logging {
	logging := C.logging_init()
	instance := Logging{logging}
	return instance
}

func New(
	level uint8,
	domain *string,
	ext_config *ExtConfig,
	console *ConsoleWriterConfig,
	file *FileWriterConfig,
	server *ServerConfig,
	connect *ClientWriterConfig,
	syslog int8,
	config *string) Logging {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	var c_ext_config C.ExtConfig = nil
	if ext_config != nil {
		c_ext_config = ext_config.config
	}
	var c_console_config C.ConsoleWriterConfig = nil
	if console != nil {
		c_console_config = console.config
	}
	var c_file_config C.FileWriterConfig = nil
	if file != nil {
		c_file_config = file.config
	}
	var c_server_config C.ServerConfig = nil
	if server != nil {
		c_server_config = server.config
	}
	var c_connect_config C.ClientWriterConfig = nil
	if connect != nil {
		c_connect_config = connect.config
	}
	var c_config *C.char = nil
	if config != nil {
		c_config = C.CString(*config)
	}
	return Logging{C.logging_new(
		C.uint8_t(level),
		c_domain,
		&c_ext_config,
		&c_console_config,
		&c_file_config,
		&c_server_config,
		&c_connect_config,
		C.int8_t(syslog),
		c_config)}
}

func (instance Logging) Shutdown(now bool) int {
	var c_now C.schar = 0
	if now {
		c_now = 1
	}
	return int(C.logging_shutdown(instance.logging, c_now))
}

func (instance Logging) SetLevel(writer WriterTypeEnum, level uint8) int {
	return int(C.logging_set_level(instance.logging, writer.into(), C.uchar(level)))
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

func (instance Logging) SetExtConfig(ext_config ExtConfig) {
	C.logging_set_ext_config(instance.logging, ext_config.config)
}

func (instance Logging) AddLogger(logger Logger) {
	C.logging_add_logger(instance.logging, logger.logger)
}

func (instance Logging) RemoveLogger(logger Logger) {
	C.logging_remove_logger(instance.logging, logger.logger)
}

func (instance Logging) AddWriter(writer WriterConfigEnum) int {
	return int(C.logging_add_writer(instance.logging, C.WriterConfigEnum(writer)))
}

func (instance Logging) RemoveWriter(writer WriterTypeEnum) int {
	return int(C.logging_remove_writer(instance.logging, writer.into()))
}

func (instance Logging) Sync(console bool, file bool, client bool, syslog bool, callback bool, timeout float64) int {
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
	var c_callback C.int8_t
	if callback {
		c_callback = 1
	}
	return int(C.logging_sync(instance.logging, c_console, c_file, c_client, c_syslog, c_callback, C.double(timeout)))
}

func (instance Logging) SyncAll(timeout float64) int {
	return int(C.logging_sync_all(instance.logging, C.double(timeout)))
}

// File writer

func (instance Logging) Rotate(path string) int {
	return int(C.logging_rotate(instance.logging, C.CString(path)))
}

// Network

func (instance Logging) SetEncryption(address string, writer WriterTypeEnum, encryption EncryptionMethod, key *string) int {
	var c_key *C.char = nil
	if key != nil {
		c_key = C.CString(*key)
	}
	return int(C.logging_set_encryption(instance.logging, writer.into(), encryption.into(), c_key))
}

// Config

func (instance Logging) GetConfig(writer WriterTypeEnum) C.WriterConfigEnum {
	return C.logging_get_config(instance.logging, writer.into())
}

func (instance Logging) GetServerConfig() ServerConfig {
	var config *C.CServerConfig = C.logging_get_server_config(instance.logging)
	return ServerConfig{C.server_config_new(C.uint8_t(config.level), config.address, config.encryption, config.key)}
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
