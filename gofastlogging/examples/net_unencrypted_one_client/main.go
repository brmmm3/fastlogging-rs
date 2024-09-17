package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../lib/cfastlogging.h"
*/
import "C"
import "examples/logging"

func main() {
	var encryption logging.EncryptionMethod = logging.NONE
	server := logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", encryption, nil)
	server_domain := "LOGSRV"
	console := logging.ConsoleWriterConfigNew(logging.DEBUG, true)
	logger := logging.New(logging.DEBUG, &server_domain, nil, &console, nil, &server, nil, -1, nil)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
