package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import logging "gofastlogging/fastlogging"

func main() {
	hostname := "hostname"
	pname := "pname"
	writers := []logging.WriterConfigEnum{logging.SyslogWriterConfigNew(logging.DEBUG, &hostname, &pname, 1234)}
	logger := logging.New(logging.DEBUG, nil, writers, nil, nil)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
