package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

func main() {
	console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
	if console == nil {
		panic("Failed to create writer")
	}
	writers := []fl.WriterConfigEnum{*console}
	logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	ext_config := fl.NewExtConfig(fl.Xml, true, false, true, false, true)
	logger.SetExtConfig(ext_config)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
