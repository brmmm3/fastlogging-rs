package root

/*
#cgo LDFLAGS: -L. -L../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
	logging "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logger"
	"unsafe"
)

func Init() {
	C.root_init()
}

func Shutdown(now bool) int {
	var c_now C.schar = 0
	if now {
		c_now = 1
	}
	return int(C.root_shutdown(c_now))
}

func SetLevel(wid uint, level uint8) int {
	return int(C.root_set_level(C.uint32_t(wid), C.uchar(level)))
}

func SetDomain(domain *string) {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
	}
	C.root_set_domain(c_domain)
}

func SetLevel2Sym(level2sym uint8) {
	C.root_set_level2sym(C.uchar(level2sym))
}

func SetExtConfig(ext_config logging.ExtConfig) {
	C.root_set_ext_config((*C.CExtConfig)(unsafe.Pointer(ext_config.Config)))
}

func AddLogger(logger logger.Logger) {
	C.root_add_logger(C.Logger(logger.Logger))
}

func RemoveLogger(logger logger.Logger) {
	C.root_remove_logger(C.Logger(logger.Logger))
}

func SetRootWriterConfig(config logging.WriterConfigEnum) {
	C.root_set_root_writer_config((C.CWriterConfigEnum)(unsafe.Pointer(config.Config)))
}

func SetRootWriter(writer logging.WriterEnum) {
	C.root_set_root_writer((C.CWriterEnum)(unsafe.Pointer(writer.Writer)))
}

func AddWriterConfig(config logging.WriterConfigEnum) int {
	return int(C.root_add_writer_config((C.CWriterConfigEnum)(unsafe.Pointer(config.Config))))
}

func AddWriter(writer logging.WriterEnum) int {
	return int(C.root_add_writer((C.CWriterEnum)(unsafe.Pointer(writer.Writer))))
}

func RemoveWriter(wid uint) int {
	return int(C.root_remove_writer(C.uint32_t(wid)))
}

func AddWriterConfigs(configs logging.WriterConfigEnums, config_cnt uint32) int {
	return int(C.root_add_writer_configs((*C.CWriterConfigEnums)(unsafe.Pointer(configs.Configs)), C.uint32_t(config_cnt)))
}

func AddWriters(writers logging.WriterEnums, writer_cnt uint32) int {
	return int(C.root_add_writers((*C.CWriterEnums)(unsafe.Pointer(writers.Writers)), C.uint32_t(writer_cnt)))
}

func RemoveWriters(wids []uint32, wid_cnt uint32) logging.WriterEnums {
	writers := C.root_remove_writers((*C.uint32_t)(unsafe.Pointer(&wids[0])), C.uint32_t(wid_cnt))
	return logging.WriterEnums{Writers: (*C.CWriterEnums)(unsafe.Pointer(writers))}
}

func Enable(wid uint32) int {
	return int(C.root_enable(C.uint32_t(wid)))
}

func Disable(wid uint32) int {
	return int(C.root_disable(C.uint32_t(wid)))
}

func EnableType(typ logging.WriterTypeEnum) int {
	return int(C.root_enable_type((C.CWriterTypeEnum)(typ.Typ)))
}

func DisableType(typ logging.WriterTypeEnum) int {
	return int(C.root_disable_type((C.CWriterTypeEnum)(typ.Typ)))
}

func Sync(types *logging.WriterTypeEnum, type_cnt uint32, timeout float64) int {
	var c_types C.CWriterTypeEnum
	if types != nil {
		c_types = (C.CWriterTypeEnum)(types.Typ)
	}
	return int(C.root_sync(&c_types, C.uint32_t(type_cnt), C.double(timeout)))
}

func SyncAll(timeout float64) int {
	return int(C.root_sync_all(C.double(timeout)))
}

// File writer

func Rotate(path string) int {
	return int(C.root_rotate(C.CString(path)))
}

// Network

func SetEncryption(wid uint32, key logging.KeyStruct) int {
	return int(C.root_set_encryption(C.uint32_t(wid), (*C.CKeyStruct)(unsafe.Pointer(key.Key))))
}

// Config

func GetServerConfig() logging.ServerConfig {
	return logging.ServerConfig{Config: (*C.CServerConfig)(unsafe.Pointer(C.root_get_server_config()))}
}

func GetServerAuthKey() logging.KeyStruct {
	return logging.KeyStruct{Key: (*C.CKeyStruct)(unsafe.Pointer(C.root_get_server_auth_key()))}
}

func GetConfigString() string {
	return C.GoString(C.root_get_config_string())
}

func SaveConfig(path string) int {
	return int(C.root_save_config(C.CString(path)))
}

// Logging calls

func Trace(message string) int {
	return int(C.root_trace(C.CString(message)))
}

func Debug(message string) int {
	return int(C.root_debug(C.CString(message)))
}

func Info(message string) int {
	return int(C.root_info(C.CString(message)))
}

func Success(message string) int {
	return int(C.root_success(C.CString(message)))
}

func Warning(message string) int {
	return int(C.root_warning(C.CString(message)))
}

func Error(message string) int {
	return int(C.root_error(C.CString(message)))
}

func Critical(message string) int {
	return int(C.root_critical(C.CString(message)))
}

func Fatal(message string) int {
	return int(C.root_fatal(C.CString(message)))
}

func Exception(message string) int {
	return int(C.root_exception(C.CString(message)))
}
