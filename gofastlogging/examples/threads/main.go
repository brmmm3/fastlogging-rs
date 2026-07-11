package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logger"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
	"sync"
)

func loggerThread(logger_thread *logger.Logger, wg *sync.WaitGroup) {
	logger_thread.Trace("Trace message")
	logger_thread.Debug("Debug message")
	logger_thread.Info("Info Message")
	logger_thread.Warning("Warning Message")
	logger_thread.Error("Error Message")
	logger_thread.Fatal("Fatal Message")
	wg.Done()
}

func main() {
	console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
	if console == nil {
		panic("Failed to create console writer")
	}
	writers := []fl.WriterConfigEnum{*console}
	logger_main := logging.New(fl.DEBUG, nil, writers, nil, nil)
	if logger_main == nil {
		panic("Failed to create logger")
	}
	logger_name := "LoggerThread"
	logger_thread := logger.NewExt(fl.DEBUG, &logger_name, 1, 1)
	if logger_thread == nil {
		panic("Failed to create thread logger")
	}
	var wg sync.WaitGroup
	wg.Add(1)
	go loggerThread(logger_thread, &wg)
	logger_main.Trace("Trace message")
	logger_main.Debug("Debug message")
	logger_main.Info("Info Message")
	logger_main.Warning("Warning Message")
	logger_main.Error("Error Message")
	logger_main.Fatal("Fatal Message")
	wg.Wait()
	logger_main.Shutdown(false)
}
