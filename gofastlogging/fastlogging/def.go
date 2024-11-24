package fastlogging

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
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

type LevelSyms int

const (
	Sym LevelSyms = iota
	Short
	Str
)

func (s LevelSyms) into() C.CLevelSyms {
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

func (s FileTypeEnum) into() C.CFileTypeEnum {
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

func (s CompressionMethodEnum) into() C.CCompressionMethodEnum {
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

type MessageStructEnum int

const (
	String MessageStructEnum = iota
	Json
	Xml
)

func (s MessageStructEnum) into() C.CMessageStructEnum {
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
