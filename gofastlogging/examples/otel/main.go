package main
package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

func main() {
	// Create an OpenTelemetry writer pointing to a local collector.
	otelWriter := writer.OtelWriterConfigNew(fl.DEBUG, "http://localhost:4318", "my-go-app")
	if otelWriter == nil {
		panic("Failed to create otel writer")
	}
	writers := []fl.WriterConfigEnum{*otelWriter}
	logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Success("Success Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
