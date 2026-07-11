package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
	"log"
)

func main() {
	logger, err := logging.Default()
	if err != nil {
		log.Fatal(err)
	}
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
	logger.AddWriterConfig(*file)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
