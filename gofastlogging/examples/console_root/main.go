package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
	"gofastlogging/fastlogging/root"
)

func main() {
	root.Init()
	root.Trace("Trace message")
	root.Debug("Debug message")
	root.Info("Info Message")
	root.Warning("Warning Message")
	root.Error("Error Message")
	root.Fatal("Fatal Message")
	root.Shutdown(false)
}
