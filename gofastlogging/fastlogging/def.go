package fastlogging

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
*/
import "C"
import "unsafe"

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

type Cu32StringVec struct {
	Cnt    uint32
	Keys   []uint32
	Values []string
}

type Cu32u16Vec struct {
	Cnt    uint32
	Keys   []uint32
	Values []uint16
}

type LevelSyms int

const (
	Sym LevelSyms = iota
	Short
	Str
)

func (s LevelSyms) Into() C.CLevelSyms {
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

func (s FileTypeEnum) Into() C.CFileTypeEnum {
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

func (s CompressionMethodEnum) Into() C.CCompressionMethodEnum {
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

type ServerConfig struct {
	Config *C.CServerConfig
}

type ServerConfigs struct {
	Config *C.CServerConfigs
}

type WriterConfigEnum struct {
	Config C.CWriterConfigEnum
}

type WriterConfigEnums struct {
	Configs *C.CWriterConfigEnums
}

type WriterTypeEnum struct {
	Typ C.CWriterTypeEnum
}

type WriterEnum struct {
	Writer C.CWriterEnum
}

type WriterEnums struct {
	Writers *C.CWriterEnums
}

func WriterEnumsNew(writers *C.CWriterEnums) WriterEnums {
	return WriterEnums{Writers: writers}
}

type MessageStructEnum int

const (
	String MessageStructEnum = iota
	Json
	Xml
)

func (s MessageStructEnum) Into() C.CMessageStructEnum {
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

type EncryptionMethodEnum int

const (
	NONE EncryptionMethodEnum = iota
	AuthKey
	AES
)

func (s EncryptionMethodEnum) Into() C.CEncryptionMethodEnum {
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

type KeyStruct struct {
	Key *C.CKeyStruct
}

type ExtConfig struct {
	Config *C.CExtConfig
}

func ExtConfigNew(
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
	return ExtConfig{C.ext_config_new(structured.Into(), c_hostname, c_pname, c_pid, c_tname, c_tid)}
}

func RemoveWriters(wids []uint32, wid_cnt uint32) WriterEnums {
	writers := C.root_remove_writers((*C.uint32_t)(unsafe.Pointer(&wids[0])), C.uint32_t(wid_cnt))
	return WriterEnumsNew(writers)
}

func GetServerConfig() ServerConfig {
	return ServerConfig{Config: (*C.CServerConfig)(unsafe.Pointer(C.root_get_server_config()))}
}

func GetServerAuthKey() KeyStruct {
	return KeyStruct{Key: (*C.CKeyStruct)(unsafe.Pointer(C.root_get_server_auth_key()))}
}
