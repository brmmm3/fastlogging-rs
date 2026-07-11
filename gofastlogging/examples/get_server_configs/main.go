package main

import (
	"fmt"
	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

func main() {
	console := writer.ConsoleWriterConfigNew(fl.DEBUG, true)
	if console == nil {
		panic("Failed to create console writer")
	}
	server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", nil)
	if server == nil {
		panic("Failed to create server writer")
	}
	server_writers := []fl.WriterConfigEnum{*console, *server}
	domain := "LOGSRV"
	logger := logging.New(fl.DEBUG, &domain, server_writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	logger.SetRootWriterConfig(*server)
	logger.SyncAll(5.0)
	// Show configs
	configs := logger.GetServerConfigs()
	fmt.Printf("configs=%v\n", configs)
	fmt.Print("Shutdown Logger\n")
	logger.Shutdown(false)
	fmt.Print("-------- Finished --------\n")
}
