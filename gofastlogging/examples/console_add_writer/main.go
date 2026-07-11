package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

func main() {
	logger := logging.New(fl.DEBUG, nil, nil, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
	if console == nil {
		panic("Failed to create writer")
	}
	logger.AddWriterConfig(*console)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
