package fastlogging

/*
#include <stdlib.h>
#cgo CFLAGS: -I../h
#cgo LDFLAGS: -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

// Log level constants for fastlogging
const (
	NOLOG     = C.NOLOG     // No logging
	EXCEPTION = C.EXCEPTION // Exception level
	CRITICAL  = C.CRITICAL  // Critical level
	FATAL     = C.FATAL     // Fatal level
	ERROR     = C.ERROR     // Error level
	WARNING   = C.WARNING   // Warning level
	WARN      = C.WARN      // Warn level
	SUCCESS   = C.SUCCESS   // Success level
	INFO      = C.INFO      // Info level
	DEBUG     = C.DEBUG     // Debug level
	TRACE     = C.TRACE     // Trace level
	NOTSET    = C.NOTSET    // Not set
)

// Cu32StringVec represents a vector of uint32 keys and string values.
type Cu32StringVec struct {
	Cnt    uint32
	Keys   []uint32
	Values []string
}

// Cu32u16Vec represents a vector of uint32 keys and uint16 values.
type Cu32u16Vec struct {
	Cnt    uint32
	Keys   []uint32
	Values []uint16
}

// LevelSymbol controls the symbol format for log levels.
type LevelSymbol int

const (
	// Sym uses 1 character symbol (!, F, E, W, ...)
	Sym LevelSymbol = iota
	// Short uses 3 character text (EXC, FTL, ERR, WRN, ...)
	Short
	// Str uses long text (EXCEPTION, FATAL, ERROR, WARNING, ...). This is the default.
	Str
)

// Into converts LevelSymbol to the underlying C enum value.
//
// The result is a plain uint8 rather than a cgo-generated type: cgo creates a
// distinct, non-interchangeable Go type per package for every C type, even
// when two "import C" blocks include the exact same header. Returning a
// package-local C type here would make this method unusable from the other
// gofastlogging packages (logging/logger/writer).
func (s LevelSymbol) Into() uint8 {
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

// FileType represents the type of file operation for logging.
type FileType int

const (
	MessageOp FileType = iota
	SyncOp
	RotateOp
	StopOp
)

// Into converts FileType to the underlying C enum value. See [LevelSymbol.Into].
func (s FileType) Into() uint8 {
	switch s {
	case MessageOp:
		return 0
	case SyncOp:
		return 1
	case RotateOp:
		return 2
	case StopOp:
		return 3
	}
	return 0
}

// MessageStruct specifies the message structure format.
type MessageStruct int

const (
	String MessageStruct = iota
	Json
	Xml
)

// Into converts MessageStruct to the underlying C enum value. See [LevelSymbol.Into].
func (s MessageStruct) Into() uint8 {
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

// EncryptionMethod specifies the encryption algorithm.
type EncryptionMethod int

const (
	NONE EncryptionMethod = iota
	AuthKey
	AES
)

// Into converts EncryptionMethod to the underlying C enum value. See [LevelSymbol.Into].
func (s EncryptionMethod) Into() uint8 {
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

// CompressionMethod specifies the compression algorithm for log files.
type CompressionMethod int

const (
	Store   CompressionMethod = iota // Do not compress the log files
	Deflate                          // Compress with Deflate
	Zstd                             // Compress with Zstandard
	Lzma                             // Compress with Lzma
)

// Into converts CompressionMethod to the underlying C enum value. See [LevelSymbol.Into].
func (s CompressionMethod) Into() uint8 {
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

// ServerConfig wraps the C server config pointer.
//
// The field is unsafe.Pointer (rather than a cgo-generated pointer type) so
// that values can be constructed and read from any gofastlogging package.
// See the comment on LevelSymbol.Into for why.
type ServerConfig struct {
	Config unsafe.Pointer
}

// ServerConfigs wraps the C server configs pointer.
type ServerConfigs struct {
	Config unsafe.Pointer
}

// WriterConfigEnum wraps a C WriterConfigEnum handle (a `void*` in C).
type WriterConfigEnum struct {
	Config unsafe.Pointer
}

// WriterConfigs wraps the C writer config enums pointer.
type WriterConfigs struct {
	Configs unsafe.Pointer
}

// WriterConfigEnums is an alias for WriterConfigs.
type WriterConfigEnums = WriterConfigs

// WriterType wraps the C writer type enum value.
type WriterType struct {
	Typ uint8
}

// WriterTypeEnum is an alias for WriterType.
type WriterTypeEnum = WriterType

// Writer wraps a C CWriterEnum handle (a `void*` in C).
type Writer struct {
	Writer unsafe.Pointer
}

// WriterEnum is an alias for Writer.
type WriterEnum = Writer

// Writers wraps the C writer enums pointer.
type Writers struct {
	Writers unsafe.Pointer
}

// WriterEnums is an alias for Writers.
type WriterEnums = Writers

// NewWriters creates a Writers struct from a C pointer.
func NewWriters(writers unsafe.Pointer) Writers {
	return Writers{Writers: writers}
}

// WriterEnumsNew is an alias for NewWriters.
func WriterEnumsNew(writers unsafe.Pointer) WriterEnums {
	return NewWriters(writers)
}

// Key wraps the C key struct pointer.
type Key struct {
	Key unsafe.Pointer
}

// KeyStruct is an alias for Key.
type KeyStruct = Key

// ExtConfig wraps the C extended config pointer.
type ExtConfig struct {
	Config unsafe.Pointer
}

// NewExtConfig creates a new ExtConfig.
func NewExtConfig(
	structured MessageStruct,
	hostname, pname, pid, tname, tid bool,
) ExtConfig {
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
	cfg := C.ext_config_new(C.CMessageStructEnum(structured.Into()), c_hostname, c_pname, c_pid, c_tname, c_tid)
	return ExtConfig{Config: unsafe.Pointer(cfg)}
}

// wrapCError converts a C int error code to a Go error
func wrapCError(code int) error {
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging error: code %d", code)
}
