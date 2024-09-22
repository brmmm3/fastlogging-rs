package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../lib/cfastlogging.h"
*/
import "C"
import "examples/logging"

func main() {
	file := logging.FileWriterConfigNew(logging.DEBUG,
		"/tmp/gofastlogging.log",
		1024,
		3,
		-1,
		-1,
		logging.Store)
	logger := logging.New(logging.DEBUG, nil, nil, nil, &file, nil, nil, -1, nil)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
