package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

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
		logging.FileWriterConfigNew(
			logging.DEBUG,
			"/tmp/cfastlogging.log",
			1024,
			3,
			-1,
			-1,
			logging.Store),
	}
	server_domain := "LOGSRV"
	logging_server := logging.New(logging.DEBUG, &server_domain, server_writers, nil, nil)
	// Set root writer
	var encryption logging.EncryptionMethodEnum = logging.NONE
	server_key := logging.CreateRandomKey(encryption.Into())
	server := logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", &logging.KeyStruct{Key: server_key})
	logging_server.SetRootWriterConfig(server)
	//logging_server.SetDebug(3)
	logging_server.SyncAll(5.0)
	// Client
	address_port := logging_server.GetRootServerAddressPort()
	fmt.Printf("address_port=%s\n", address_port)
	auth_key := logging_server.GetServerAuthKey()
	client_writers := []logging.WriterConfigEnum{
		logging.ClientWriterConfigNew(logging.DEBUG, address_port, &auth_key),
	}
	client_domain := "LOGCLIENT"
	logging_client := logging.New(logging.DEBUG, &client_domain, client_writers, nil, nil)
	//logging_client.SetDebug(3)
	fmt.Print("Send logs\n")
	// Test logging
	logging_client.Trace("Trace message")
	logging_client.Debug("Debug message")
	logging_client.Info("Info Message")
	logging_client.Success("Success Message")
	logging_client.Warning("Warning Message")
	logging_client.Error("Error Message")
	logging_client.Fatal("Fatal Message")

	logging_server.Trace("Trace message")
	logging_server.Debug("Debug message")
	logging_server.Info("Info Message")
	logging_server.Success("Success Message")
	logging_server.Warning("Warning Message")
	logging_server.Error("Error Message")
	logging_server.Fatal("Fatal Message")

	logging_client.SyncAll(1.0)
	logging_server.SyncAll(1.0)
	fmt.Print("Shutdown Loggers\n")
	logging_client.Shutdown(false)
	logging_server.Shutdown(false)
	fmt.Print("-------- Finished --------\n")
}
