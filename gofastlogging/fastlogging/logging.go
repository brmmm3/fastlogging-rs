package fastlogging

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../h/cfastlogging.h"
*/
import "C"
import (
	"gofastlogging/fastlogging/logger"
	"unsafe"
)

type Logging struct {
	Logging C.Logging
}

func Default() Logging {
	logging := C.logging_new_default()
	instance := Logging{Logging: logging}
	return instance
}

func New(
	level uint8,
	domain *string,
	configs_ptr []WriterConfigEnum,
	ext_config *ExtConfig,
	config_path *string) Logging {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	var c_configs_ptr *C.CWriterConfigEnum = nil
	if configs_ptr != nil {
		var c_configs_arr = []C.CWriterConfigEnum{}
		s := c_configs_arr[:0]
		for _, value := range configs_ptr {
			s = append(s, value.Config)
		}
		c_configs_ptr = unsafe.SliceData(c_configs_arr)
	}
	var c_ext_config *C.CExtConfig = nil
	if ext_config != nil {
		c_ext_config = ext_config.Config
	}
	var c_config_path *C.char = nil
	if config_path != nil {
		c_config_path = C.CString(*config_path)
	}
	return Logging{Logging: C.logging_new(
		C.uint8_t(level),
		c_domain,
		c_configs_ptr,
		C.uint32_t(len(configs_ptr)),
		c_ext_config,
		c_config_path)}
}

func (instance Logging) Shutdown(now bool) int {
	var c_now C.schar = 0
	if now {
		c_now = 1
	}
	return int(C.logging_shutdown(instance.Logging, c_now))
}

func (instance Logging) SetLevel(wid uint, level uint8) int {
	return int(C.logging_set_level(instance.Logging, C.uint32_t(wid), C.uchar(level)))
}

func (instance Logging) SetDomain(domain *string) {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	C.logging_set_domain(instance.Logging, c_domain)
}

func (instance Logging) SetLevel2Sym(level2sym uint8) {
	C.logging_set_level2sym(instance.Logging, C.uchar(level2sym))
}

func (instance Logging) SetExtConfig(ext_config ExtConfig) {
	C.logging_set_ext_config(instance.Logging, ext_config.Config)
}

func (instance Logging) AddLogger(logger logger.Logger) {
	C.logging_add_logger(instance.Logging, C.Logger(logger.Logger))
}

func (instance Logging) RemoveLogger(logger logger.Logger) {
	C.logging_remove_logger(instance.Logging, C.Logger(logger.Logger))
}

func (instance Logging) SetRootWriterConfig(config WriterConfigEnum) {
	C.logging_set_root_writer_config(instance.Logging, config.Config)
}

func (instance Logging) SetRootWriter(writer WriterEnum) {
	C.logging_set_root_writer(instance.Logging, writer.Writer)
}

func (instance Logging) AddWriterConfig(config WriterConfigEnum) int {
	return int(C.logging_add_writer_config(instance.Logging, config.Config))
}

func (instance Logging) AddWriter(writer WriterEnum) int {
	return int(C.logging_add_writer(instance.Logging, writer.Writer))
}

func (instance Logging) RemoveWriter(wid uint) int {
	return int(C.logging_remove_writer(instance.Logging, C.uint32_t(wid)))
}

func (instance Logging) AddWriterConfigs(configs WriterConfigEnums, config_cnt uint32) int {
	return int(C.logging_add_writer_configs(instance.Logging, configs.Configs, C.uint32_t(config_cnt)))
}

func (instance Logging) AddWriters(writers WriterEnums, writer_cnt uint32) int {
	return int(C.logging_add_writers(instance.Logging, writers.Writers, C.uint32_t(writer_cnt)))
}

func (instance Logging) RemoveWriters(wids []uint32, wid_cnt uint32) WriterEnums {
	return WriterEnums{C.logging_remove_writers(instance.Logging, (*C.uint32_t)(unsafe.Pointer(&wids[0])), C.uint32_t(wid_cnt))}
}

func (instance Logging) Enable(wid uint32) int {
	return int(C.logging_enable(instance.Logging, C.uint32_t(wid)))
}

func (instance Logging) Disable(wid uint32) int {
	return int(C.logging_disable(instance.Logging, C.uint32_t(wid)))
}

func (instance Logging) EnableType(typ WriterTypeEnum) int {
	return int(C.logging_enable_type(instance.Logging, typ.Typ))
}

func (instance Logging) DisableType(typ WriterTypeEnum) int {
	return int(C.logging_disable_type(instance.Logging, typ.Typ))
}

func (instance Logging) Sync(types *WriterTypeEnum, type_cnt uint32, timeout float64) int {
	var c_types C.CWriterTypeEnum
	if types != nil {
		c_types = types.Typ
	}
	return int(C.logging_sync(instance.Logging, &c_types, C.uint32_t(type_cnt), C.double(timeout)))
}

func (instance Logging) SyncAll(timeout float64) int {
	return int(C.logging_sync_all(instance.Logging, C.double(timeout)))
}

// File writer

func (instance Logging) Rotate(path string) int {
	return int(C.logging_rotate(instance.Logging, C.CString(path)))
}

// Network

func (instance Logging) SetEncryption(address string, typ WriterTypeEnum, key KeyStruct) int {
	return int(C.logging_set_encryption(instance.Logging, typ.Typ, key.Key))
}

// Config

func (instance Logging) GetServerConfig() ServerConfig {
	return ServerConfig{C.logging_get_server_config(instance.Logging)}
}

func (instance Logging) GetServerAuthKey() KeyStruct {
	return KeyStruct{C.logging_get_server_auth_key(instance.Logging)}
}

func (instance Logging) GetConfigString() string {
	return C.GoString(C.logging_get_config_string(instance.Logging))
}

func (instance Logging) SaveConfig(path string) int {
	return int(C.logging_save_config(instance.Logging, C.CString(path)))
}

// Logging calls

func (instance Logging) Trace(message string) int {
	return int(C.logging_trace(instance.Logging, C.CString(message)))
}

func (instance Logging) Debug(message string) int {
	return int(C.logging_debug(instance.Logging, C.CString(message)))
}

func (instance Logging) Info(message string) int {
	return int(C.logging_info(instance.Logging, C.CString(message)))
}

func (instance Logging) Success(message string) int {
	return int(C.logging_success(instance.Logging, C.CString(message)))
}

func (instance Logging) Warning(message string) int {
	return int(C.logging_warning(instance.Logging, C.CString(message)))
}

func (instance Logging) Error(message string) int {
	return int(C.logging_error(instance.Logging, C.CString(message)))
}

func (instance Logging) Critical(message string) int {
	return int(C.logging_critical(instance.Logging, C.CString(message)))
}

func (instance Logging) Fatal(message string) int {
	return int(C.logging_fatal(instance.Logging, C.CString(message)))
}

func (instance Logging) Exception(message string) int {
	return int(C.logging_exception(instance.Logging, C.CString(message)))
}
