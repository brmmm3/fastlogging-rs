package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
	logging "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logger"
	"sync"
)

func loggerThread(logger_thread logger.Logger, wg *sync.WaitGroup) {
	logger_thread.Trace("Trace message")
	logger_thread.Debug("Debug message")
	logger_thread.Info("Info Message")
	logger_thread.Warning("Warning Message")
	logger_thread.Error("Error Message")
	logger_thread.Fatal("Fatal Message")
	wg.Done()
}

func main() {
	writers := []logging.WriterConfigEnum{logging.ConsoleWriterConfigNew(logging.DEBUG, true)}
	logger_main := logging.New(logging.DEBUG, nil, writers, nil, nil)
	logger_name := "LoggerThread"
	logger_thread := logger.NewExt(logging.DEBUG, &logger_name, 1, 1)
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
