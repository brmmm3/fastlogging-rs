package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../lib/cfastlogging.h"
*/
import "C"
import "examples/logging"

func main() {
	logger := logging.New(logging.DEBUG, nil, nil, nil, nil, nil, nil, -1, nil)
	file := logging.FileWriterConfigNew(logging.DEBUG,
		"/tmp/gofastlogging.log",
		1024,
		3,
		-1,
		-1,
		logging.Store)
	logger.AddWriter(logging.WriterConfigEnum(&file))
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
