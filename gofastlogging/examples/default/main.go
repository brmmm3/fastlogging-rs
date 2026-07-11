package main

import (
	"gofastlogging/fastlogging/logging"
	"log"
)

func main() {
	logger, err := logging.Default()
	if err != nil {
		log.Fatal(err)
	}
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
