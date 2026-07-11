package main

import (
	"fmt"
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
	"log"
)

func main() {
	logger := logging.New(fl.DEBUG, nil, nil, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	callback := func(level uint8, domain, message string) {
		fmt.Printf("[CALLBACK] Level: %d, Domain: %s, Message: %s\n", level, domain, message)
	}
	// NOTE: callback writers are not yet implemented in gofastlogging, so this
	// will always return an error - see writer.CallbackWriterConfigNew.
	config, handle, err := writer.CallbackWriterConfigNew(fl.DEBUG, callback)
	if err != nil {
		log.Fatal(err)
	}
	defer handle.UnregisterCallback()
	logger.AddWriterConfig(config)
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
