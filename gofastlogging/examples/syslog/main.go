package main

import (
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

func main() {
	hostname := "hostname"
	pname := "pname"
	syslogWriter := writer.SyslogWriterConfigNew(fl.DEBUG, hostname, pname, 1234)
	if syslogWriter == nil {
		panic("Failed to create syslog writer")
	}
	writers := []fl.WriterConfigEnum{*syslogWriter}
	logger := logging.New(fl.DEBUG, nil, writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	logger.Trace("Trace message")
	logger.Debug("Debug message")
	logger.Info("Info Message")
	logger.Warning("Warning Message")
	logger.Error("Error Message")
	logger.Fatal("Fatal Message")
	logger.Shutdown(false)
}
