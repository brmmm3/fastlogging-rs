package main

import "examples/logging"

func main() {
	logger := logging.Init()
	logger.Debug("Debug message")
	logger.Info("Info message")
	logger.Warning("Warning message")
	logger.Error("Error message")
	logger.Fatal("Fatal message")
	logger.Shutdown(false)
}
