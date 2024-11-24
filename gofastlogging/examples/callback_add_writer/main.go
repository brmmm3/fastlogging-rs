package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
	logging "gofastlogging/fastlogging"
	"unsafe"
)

func CallbackWriter(level C.char, domain *C.char, message *C.char) {
	println("%d %s %s", level, domain, message)
}

func main() {
	logger := logging.New(logging.DEBUG, nil, nil, nil, nil)
	fn := CallbackWriter
	callback := logging.CallbackWriterConfigNew(logging.DEBUG, uintptr(unsafe.Pointer(&fn)))
	logger.AddWriterConfig(callback)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
