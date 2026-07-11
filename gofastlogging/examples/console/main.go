package main

import (
	"fmt"
	"gofastlogging/fastlogging/logging"
	"log"
)

func main() {
	// Try default logger first
	logger, err := logging.Default()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Default logger created: %+v\n", logger)
	fmt.Printf("Logger.Logging: %+v\n", logger.Logging)

	err = logger.Trace("Trace message")
	if err != nil {
		log.Fatal(err)
	}
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Critical("Critical Message")
	logger.Shutdown(false)
}
