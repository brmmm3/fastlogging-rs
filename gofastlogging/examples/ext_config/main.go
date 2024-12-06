package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import logging "gofastlogging/fastlogging"

func main() {
	writers := []logging.WriterConfigEnum{logging.ConsoleWriterConfigNew(logging.DEBUG, true)}
	logger := logging.New(logging.DEBUG, nil, writers, nil, nil)
	ext_config := logging.ExtConfigNew(logging.Xml, true, false, true, false, true)
	logger.SetExtConfig(ext_config)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
