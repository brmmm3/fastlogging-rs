package main

import (
	"fmt"
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
	"os"
)

func main() {
	// Define a Go callback function
	callback := func(level uint8, domain, message string) {
		fmt.Fprintf(os.Stdout, "[CALLBACK] Level: %d, Domain: %s, Message: %s\n", level, domain, message)
	}

	// Register the callback writer
	// NOTE: callback writers are not yet implemented in gofastlogging, so this
	// will always return an error - see writer.CallbackWriterConfigNew.
	config, handle, err := writer.CallbackWriterConfigNew(fl.DEBUG, callback)
	if err != nil {
		panic(err)
	}
	defer handle.UnregisterCallback() // Clean up when done

	// Create a logger with the callback writer
	logger := logging.New(fl.DEBUG, nil, []fl.WriterConfigEnum{config}, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	logger.Info("Hello from callback writer!")
	logger.Error("This is an error message.")
	logger.Shutdown(false)
}
