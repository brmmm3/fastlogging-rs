package main

import "examples/logging"

func main() {
	//logger := logging.Init()
	logger := logging.New(logging.DEBUG, nil, 1, nil, nil, nil, 0, 0)
	logger.Debug("Debug message")
	logger.Info("Info message")
	logger.Warning("Warning message")
	logger.Error("Error message")
	logger.Fatal("Fatal message")
	logger.Shutdown(false)
}
