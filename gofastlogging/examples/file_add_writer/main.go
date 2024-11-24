package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import logging "gofastlogging/fastlogging"

func main() {
	logger := logging.Default()
	file := logging.FileWriterConfigNew(logging.DEBUG,
		"/tmp/gofastlogging.log",
		1024,
		3,
		-1,
		-1,
		logging.Store)
	logger.AddWriterConfig(file)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
