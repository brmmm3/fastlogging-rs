package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../lib/cfastlogging.h"
*/
import "C"
import "examples/logging"

func main() {
	logging := logging.Init()
	logging.Trace("Trace message")
	logging.Debug("Debug message")
	logging.Info("Info Message")
	logging.Warning("Warning Message")
	logging.Error("Error Message")
	logging.Fatal("Fatal Message")
	logging.Shutdown(false)
}
