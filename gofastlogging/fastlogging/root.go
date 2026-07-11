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
	"gofastlogging/fastlogging/logger"
	"unsafe"
)

func Init() error {
	C.root_init()
	return nil
}

func Shutdown(now bool) error {
	var c_now C.schar = 0
	if now {
		c_now = 1
	}
	code := int(C.root_shutdown(c_now))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root shutdown error: code %d", code)
}

func SetLevel(wid uint32, level uint8) error {
	code := int(C.root_set_level(C.uint32_t(wid), C.uchar(level)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root set level error: code %d", code)
}

func SetDomain(domain *string) error {
	var c_domain *C.char = nil
	if domain != nil {
		c_domain = C.CString(*domain)
		defer C.free(unsafe.Pointer(c_domain))
	}
	C.root_set_domain(c_domain)
	return nil
}

func SetLevel2Sym(level2sym uint8) error {
	C.root_set_level2sym(C.uchar(level2sym))
	return nil
}

func SetExtConfig(ext_config ExtConfig) error {
	C.root_set_ext_config((*C.ExtConfig)(unsafe.Pointer(ext_config.Config)))
	return nil
}

func AddLogger(log logger.Logger) error {
	C.root_add_logger(C.Logger(log.Logger))
	return nil
}

func RemoveLogger(log logger.Logger) error {
	C.root_remove_logger(C.Logger(log.Logger))
	return nil
}

func SetRootWriterConfig(config WriterConfigEnum) error {
	C.root_set_root_writer_config((C.WriterConfigEnum)(unsafe.Pointer(config.Config)))
	return nil
}

func SetRootWriter(writer WriterEnum) error {
	C.root_set_root_writer((C.CWriterEnum)(unsafe.Pointer(writer.Writer)))
	return nil
}

func AddWriterConfig(config WriterConfigEnum) error {
	code := int(C.root_add_writer_config((C.WriterConfigEnum)(unsafe.Pointer(config.Config))))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root add writer config error: code %d", code)
}

func AddWriter(writer WriterEnum) error {
	code := int(C.root_add_writer((C.CWriterEnum)(unsafe.Pointer(writer.Writer))))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root add writer error: code %d", code)
}

func RemoveWriter(wid uint32) error {
	code := int(C.root_remove_writer(C.uint32_t(wid)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root remove writer error: code %d", code)
}

func AddWriterConfigs(configs WriterConfigEnums, config_cnt uint32) int {
	return int(C.root_add_writer_configs((*C.WriterConfigEnums)(unsafe.Pointer(configs.Configs)), C.uint32_t(config_cnt)))
}

func AddWriters(writers WriterEnums, writer_cnt uint32) int {
	return int(C.root_add_writers((*C.CWriterEnums)(unsafe.Pointer(writers.Writers)), C.uint32_t(writer_cnt)))
}

func RemoveWriters(wids []uint32) (WriterEnums, error) {
	if len(wids) == 0 {
		return WriterEnums{}, nil
	}
	writers := C.root_remove_writers((*C.uint32_t)(unsafe.Pointer(&wids[0])), C.uint32_t(len(wids)))
	return WriterEnums{Writers: unsafe.Pointer(writers)}, nil
}

func Enable(wid uint32) error {
	code := int(C.root_enable(C.uint32_t(wid)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root enable error: code %d", code)
}

func Disable(wid uint32) error {
	code := int(C.root_disable(C.uint32_t(wid)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root disable error: code %d", code)
}

func EnableType(typ WriterTypeEnum) error {
	code := int(C.root_enable_type((C.CWriterTypeEnum)(typ.Typ)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root enable type error: code %d", code)
}

func DisableType(typ WriterTypeEnum) error {
	code := int(C.root_disable_type((C.CWriterTypeEnum)(typ.Typ)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root disable type error: code %d", code)
}

func Sync(types []WriterTypeEnum, timeout float64) error {
	if len(types) == 0 {
		return nil
	}
	c_types_arr := make([]C.CWriterTypeEnum, len(types))
	for i, value := range types {
		c_types_arr[i] = C.CWriterTypeEnum(value.Typ)
	}
	code := int(C.root_sync((*C.CWriterTypeEnum)(unsafe.Pointer(&c_types_arr[0])), C.uint32_t(len(types)), C.double(timeout)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root sync error: code %d", code)
}

func SyncAll(timeout float64) error {
	code := int(C.root_sync_all(C.double(timeout)))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root sync all error: code %d", code)
}

// File writer

func Rotate(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	code := int(C.root_rotate(cpath))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root rotate error: code %d", code)
}

// Network

func SetEncryption(wid uint32, key KeyStruct) error {
	code := int(C.root_set_encryption(C.uint32_t(wid), (*C.CKeyStruct)(unsafe.Pointer(key.Key))))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root set encryption error: code %d", code)
}

// Config

func GetServerConfig() (ServerConfig, error) {
	return ServerConfig{unsafe.Pointer(C.root_get_server_config())}, nil
}

func GetServerAuthKey() (KeyStruct, error) {
	return KeyStruct{unsafe.Pointer(C.root_get_server_auth_key())}, nil
}

func GetConfigString() (string, error) {
	return C.GoString(C.root_get_config_string()), nil
}

func SaveConfig(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	code := int(C.root_save_config(cpath))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root save config error: code %d", code)
}

// Logging calls

func Trace(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_trace(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root trace error: code %d", code)
}

func Debug(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_debug(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root debug error: code %d", code)
}

func Info(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_info(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root info error: code %d", code)
}

func Success(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_success(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root success error: code %d", code)
}

func Warning(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_warning(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root warning error: code %d", code)
}

func Error(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_error(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root error error: code %d", code)
}

func Critical(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_critical(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root critical error: code %d", code)
}

func Fatal(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_fatal(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root fatal error: code %d", code)
}

func Exception(message string) error {
	cmsg := C.CString(message)
	defer C.free(unsafe.Pointer(cmsg))
	code := int(C.root_exception(cmsg))
	if code == 0 {
		return nil
	}
	return fmt.Errorf("fastlogging root exception error: code %d", code)
}
