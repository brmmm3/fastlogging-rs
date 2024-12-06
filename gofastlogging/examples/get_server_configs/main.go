package main

/*
#cgo LDFLAGS: -L. -L../../lib -lcfastlogging
#include "../../h/cfastlogging.h"
*/
import "C"
import (
	"fmt"
	logging "gofastlogging/fastlogging"
)

func main() {
	server_writers := []logging.WriterConfigEnum{
		logging.ConsoleWriterConfigNew(logging.DEBUG, true),
		logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", nil),
	}
	domain := "LOGSRV"
	logger := logging.New(logging.DEBUG, &domain, server_writers, nil, nil)
	server := logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", nil)
	fmt.Printf("server_config=%p\n", server)
	logger.SetRootWriterConfig(server)
	logger.SyncAll(5.0)
	// Show configs
	configs := logger.GetServerConfigs()
	fmt.Printf("configs=%v\n", configs)
	fmt.Print("Shutdown Logger\n")
	logger.Shutdown(false)
	fmt.Print("-------- Finished --------\n")
}
