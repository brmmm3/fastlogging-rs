package logging

/*
#include <stdlib.h>
#cgo CFLAGS: -I../../h
#cgo LDFLAGS: -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
	"fmt"
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logger"
	"unsafe"
)

type Logging struct {
	Logging C.Logging
}

// Default returns a new Logging instance with default configuration.
func Default() (*Logging, error) {
	logging := C.logging_new_default()
	if logging == nil {
		return nil, fmt.Errorf("failed to create default logger")
	}
	return &Logging{Logging: logging}, nil
}

// New creates a new Logging instance with the given configuration.
func New(level uint8, domain *string, configs []fl.WriterConfigEnum, extConfig *fl.ExtConfig, configPath *string) *Logging {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
		defer C.free(unsafe.Pointer(c_domain))
	}
	var c_configs_ptr *C.WriterConfigEnum = nil
	var c_configs_heap_ptr unsafe.Pointer = nil
	if len(configs) > 0 {
		// Allocate using C.malloc to ensure the memory is valid during the C call
		c_configs_ptr = (*C.WriterConfigEnum)(C.malloc(C.size_t(len(configs)) * C.sizeof_WriterConfigEnum))
		c_configs_heap_ptr = unsafe.Pointer(c_configs_ptr)

		// Copy configs into the allocated memory
		for i, value := range configs {
			ptr := (*C.WriterConfigEnum)(unsafe.Pointer(uintptr(unsafe.Pointer(c_configs_ptr)) + uintptr(i)*unsafe.Sizeof(*c_configs_ptr)))
			*ptr = C.WriterConfigEnum(value.Config)
		}
	}
	var c_ext_config *C.ExtConfig = nil
	if extConfig != nil {
		c_ext_config = (*C.ExtConfig)(extConfig.Config)
	}
	var c_config_path *C.char = nil
	if configPath != nil {
		c_config_path = C.CString(*configPath)
		defer C.free(unsafe.Pointer(c_config_path))
	}
	logging := C.logging_new(
		C.uint8_t(level),
		c_domain,
		c_configs_ptr,
		C.uint32_t(len(configs)),
		c_ext_config,
		c_config_path)

	// Free the config array after logging_new returns
	// The C library has already copied the data into its own storage
	if c_configs_heap_ptr != nil {
		C.free(c_configs_heap_ptr)
	}

	if logging == nil {
		return nil
	}
	return &Logging{Logging: logging}
}

// Shutdown gracefully shuts down the logger. Honors context for cancellation.
func (l *Logging) Shutdown(now bool) error {
	var c_now C.schar = 0
	if now {
		c_now = 1
	}
	code := int(C.logging_shutdown(l.Logging, c_now))
	return wrapCError(code)
}

func (l *Logging) AddLogger(log logger.Logger) error {
	C.logging_add_logger(l.Logging, C.Logger(log.Logger))
	return nil
}

func (l *Logging) RemoveLogger(log logger.Logger) error {
	C.logging_remove_logger(l.Logging, C.Logger(log.Logger))
	return nil
}

func (l *Logging) SetLevel(wid uint32, level uint8) error {
	code := int(C.logging_set_level(l.Logging, C.uint32_t(wid), C.uchar(level)))
	return wrapCError(code)
}

func (l *Logging) SetDomain(domain *string) error {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
		defer C.free(unsafe.Pointer(c_domain))
	}
	C.logging_set_domain(l.Logging, c_domain)
	return nil
}

func (l *Logging) SetLevel2Sym(level2sym uint8) error {
	C.logging_set_level2sym(l.Logging, C.uchar(level2sym))
	return nil
}

func (l *Logging) SetExtConfig(extConfig fl.ExtConfig) error {
	C.logging_set_ext_config(l.Logging, (*C.ExtConfig)(extConfig.Config))
	return nil
}

func (l *Logging) SetRootWriterConfig(config fl.WriterConfigEnum) error {
	C.logging_set_root_writer_config(l.Logging, C.WriterConfigEnum(config.Config))
	return nil
}

func (l *Logging) SetRootWriter(writer fl.WriterEnum) error {
	C.logging_set_root_writer(l.Logging, C.CWriterEnum(writer.Writer))
	return nil
}

func (l *Logging) Enable(wid uint32) error {
	code := int(C.logging_enable(l.Logging, C.uint32_t(wid)))
	return wrapCError(code)
}

func (l *Logging) Disable(wid uint32) error {
	code := int(C.logging_disable(l.Logging, C.uint32_t(wid)))
	return wrapCError(code)
}

func (l *Logging) EnableType(typ fl.WriterTypeEnum) error {
	code := int(C.logging_enable_type(l.Logging, C.CWriterTypeEnum(typ.Typ)))
	return wrapCError(code)
}

func (l *Logging) DisableType(typ fl.WriterTypeEnum) error {
	code := int(C.logging_disable_type(l.Logging, C.CWriterTypeEnum(typ.Typ)))
	return wrapCError(code)
}

func (l *Logging) Sync(types []fl.WriterTypeEnum, timeout float64) error {
	if len(types) == 0 {
		return nil
	}
	c_types_arr := make([]C.CWriterTypeEnum, len(types))
	for i, value := range types {
		c_types_arr[i] = C.CWriterTypeEnum(value.Typ)
	}
	code := int(C.logging_sync(l.Logging, (*C.CWriterTypeEnum)(unsafe.Pointer(&c_types_arr[0])), C.uint32_t(len(types)), C.double(timeout)))
	return wrapCError(code)
}

func (l *Logging) SyncAll(timeout float64) error {
	code := int(C.logging_sync_all(l.Logging, C.double(timeout)))
	return wrapCError(code)
}

func (l *Logging) Rotate(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	code := int(C.logging_rotate(l.Logging, cpath))
	return wrapCError(code)
}

func (l *Logging) SetEncryption(typ fl.WriterTypeEnum, key fl.KeyStruct) error {
	code := int(C.logging_set_encryption(l.Logging, C.CWriterTypeEnum(typ.Typ), (*C.CKeyStruct)(key.Key)))
	return wrapCError(code)
}

func (l *Logging) SetDebug(debug uint32) error {
	C.logging_set_debug(l.Logging, C.uint32_t(debug))
	return nil
}

func (l *Logging) AddWriterConfig(config fl.WriterConfigEnum) error {
	code := int(C.logging_add_writer_config(l.Logging, C.WriterConfigEnum(config.Config)))
	return wrapCError(code)
}

func (l *Logging) RemoveWriter(wid uint32) error {
	code := int(C.logging_remove_writer(l.Logging, C.uint32_t(wid)))
	return wrapCError(code)
}

func (l *Logging) AddWriterConfigs(configs []fl.WriterConfigEnum) error {
	if len(configs) == 0 {
		return nil
	}
	c_configs_arr := make([]C.WriterConfigEnum, len(configs))
	for i, value := range configs {
		c_configs_arr[i] = C.WriterConfigEnum(value.Config)
	}
	c_configs := C.WriterConfigEnums{
		cnt:    C.uint32_t(len(configs)),
		values: (*C.WriterConfigEnum)(unsafe.Pointer(&c_configs_arr[0])),
	}
	code := int(C.logging_add_writer_configs(l.Logging, &c_configs, C.uint32_t(len(configs))))
	return wrapCError(code)
}

func (l *Logging) AddWriters(writers []fl.WriterEnum) error {
	if len(writers) == 0 {
		return nil
	}
	c_writers_arr := make([]C.CWriterEnum, len(writers))
	for i, value := range writers {
		c_writers_arr[i] = C.CWriterEnum(value.Writer)
	}
	c_writers := C.CWriterEnums{
		cnt:    C.uint32_t(len(writers)),
		values: (*C.CWriterEnum)(unsafe.Pointer(&c_writers_arr[0])),
	}
	code := int(C.logging_add_writers(l.Logging, &c_writers, C.uint32_t(len(writers))))
	return wrapCError(code)
}

func (l *Logging) RemoveWriters(wids []uint32) fl.WriterEnums {
	if len(wids) == 0 {
		return fl.WriterEnums{}
	}
	writers := C.logging_remove_writers(l.Logging, (*C.uint32_t)(unsafe.Pointer(&wids[0])), C.uint32_t(len(wids)))
	return fl.WriterEnums{Writers: unsafe.Pointer(writers)}
}

func (l *Logging) GetServerConfig() fl.ServerConfig {
	return fl.ServerConfig{Config: unsafe.Pointer(C.logging_get_server_config(l.Logging))}
}

func (l *Logging) GetServerConfigs() fl.ServerConfigs {
	return fl.ServerConfigs{Config: unsafe.Pointer(C.logging_get_server_configs(l.Logging))}
}

func (l *Logging) GetRootServerAddressPort() string {
	addr := C.logging_get_root_server_address_port(l.Logging)
	return C.GoString(addr)
}

func (l *Logging) GetRootServerAddressesPorts() map[uint32]string {
	s := C.logging_get_server_addresses_ports(l.Logging)
	cnt := int(s.cnt)
	c_keys := uintptr(unsafe.Pointer(s.keys))
	c_values := uintptr(unsafe.Pointer(s.values))
	m := make(map[uint32]string)
	for i := range cnt {
		key := uint32(*(*uint32)(unsafe.Pointer(c_keys + uintptr(i)*unsafe.Sizeof(*s.keys))))
		m[key] = C.GoString((*C.char)(unsafe.Pointer(c_values + uintptr(i)*unsafe.Sizeof(uintptr(0)))))
	}
	return m
}

func (l *Logging) GetRootServerAddresses() map[uint32]string {
	s := C.logging_get_server_addresses(l.Logging)
	cnt := int(s.cnt)
	c_keys := uintptr(unsafe.Pointer(s.keys))
	c_values := uintptr(unsafe.Pointer(s.values))
	m := make(map[uint32]string)
	for i := range cnt {
		key := uint32(*(*uint32)(unsafe.Pointer(c_keys + uintptr(i)*unsafe.Sizeof(*s.keys))))
		m[key] = C.GoString((*C.char)(unsafe.Pointer(c_values + uintptr(i)*unsafe.Sizeof(uintptr(0)))))
	}
	return m
}

func (l *Logging) GetRootServerPorts() map[uint32]uint16 {
	s := C.logging_get_server_ports(l.Logging)
	cnt := int(s.cnt)
	c_keys := uintptr(unsafe.Pointer(s.keys))
	c_values := uintptr(unsafe.Pointer(s.values))
	m := make(map[uint32]uint16)
	for i := range cnt {
		key := uint32(*(*uint32)(unsafe.Pointer(c_keys + uintptr(i)*unsafe.Sizeof(*s.keys))))
		m[key] = uint16(*(*uint16)(unsafe.Pointer(c_values + uintptr(i)*unsafe.Sizeof(*s.values))))
	}
	return m
}

func (l *Logging) GetServerAuthKey() fl.KeyStruct {
	return fl.KeyStruct{Key: unsafe.Pointer(C.logging_get_server_auth_key(l.Logging))}
}

func (l *Logging) GetConfigString() string {
	return C.GoString(C.logging_get_config_string(l.Logging))
}

func (l *Logging) SaveConfig(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	code := int(C.logging_save_config(l.Logging, cpath))
	return wrapCError(code)
}

// Logging calls

// Trace logs a trace message. Honors context for cancellation.
func (l *Logging) Trace(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_trace(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Debug(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_debug(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Info(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_info(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Success(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_success(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Warning(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_warning(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Error(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_error(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Critical(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_critical(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Fatal(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_fatal(l.Logging, cmsg))
	return wrapCError(code)
}

func (l *Logging) Exception(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.logging_exception(l.Logging, cmsg))
	return wrapCError(code)
}

// wrapCError converts a C int error code to a Go error
func wrapCError(code int) error {
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging error: code %d", code)
}

// ConsoleWriterConfigHelper creates a console writer configuration.
// This helper avoids exposing the writer package's types.
func ConsoleWriterConfigHelper(level uint8, color bool) fl.WriterConfigEnum {
	colorVal := C.int8_t(-1) // false
	if color {
		colorVal = C.int8_t(1) // true
	}
	cfg := C.console_writer_config_new(C.uint8_t(level), colorVal)
	return fl.WriterConfigEnum{Config: unsafe.Pointer(cfg)}
}

// FileWriterConfigHelper creates a file writer configuration.
func FileWriterConfigHelper(filepath string, compression uint32) fl.WriterConfigEnum {
	cpath := C.CString(filepath)
	defer C.free(unsafe.Pointer(cpath))
	cfg := C.file_writer_config_new(C.uint8_t(1), cpath, 1024*1024*10, 5, 0, 0, C.CCompressionMethodEnum(compression))
	return fl.WriterConfigEnum{Config: unsafe.Pointer(cfg)}
}

// ServerConfigHelper creates a client writer configuration pointing at host:port.
func ServerConfigHelper(host string, port uint16, key *fl.KeyStruct) fl.WriterConfigEnum {
	chost := C.CString(fmt.Sprintf("%s:%d", host, port))
	defer C.free(unsafe.Pointer(chost))
	var c_key *C.CKeyStruct = nil
	if key != nil {
		c_key = (*C.CKeyStruct)(key.Key)
	}
	cfg := C.client_writer_config_new(C.uint8_t(1), chost, c_key)
	return fl.WriterConfigEnum{Config: unsafe.Pointer(cfg)}
}
