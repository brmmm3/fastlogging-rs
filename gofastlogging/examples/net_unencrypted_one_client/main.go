package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import logging "gofastlogging/fastlogging"

func main() {
	var encryption logging.EncryptionMethodEnum = logging.NONE
	key := logging.CreateRandomKey(encryption.Into())
	writers := []logging.WriterConfigEnum{
		logging.ConsoleWriterConfigNew(logging.DEBUG, true),
		logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", logging.KeyStruct{Key: key}),
	}
	logger := logging.New(logging.DEBUG, nil, writers, nil, nil)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
