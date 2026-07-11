package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

func main() {
	file := writer.FileWriterConfigNew(fl.DEBUG,
		"/tmp/gofastlogging.log",
		1024,
		3,
		-1,
		-1,
		fl.Store)
	if file == nil {
		panic("Failed to create file writer")
	}
	writers := []fl.WriterConfigEnum{*file}
	logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
