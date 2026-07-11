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
	file := writer.FileWriterConfigNew(
		fl.DEBUG,
		"/tmp/cfastlogging.log",
		1024,
		3,
		-1,
		-1,
		fl.Store)
	if file == nil {
		panic("Failed to create file writer")
	}
	server_writers := []fl.WriterConfigEnum{*console, *file}
	server_domain := "LOGSRV"
	logging_server := logging.New(fl.DEBUG, &server_domain, server_writers, nil, nil)
	if logging_server == nil {
		panic("Failed to create server logger")
	}
	// Set root writer, encrypted with a random AES key.
	// IMPORTANT: server_key is consumed by Rust when the writer config is
	// created below. Using server_key again afterwards leads to errors!
	server_key := fl.CreateRandomKey(fl.AES)
	server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", &server_key)
	if server == nil {
		panic("Failed to create server writer")
	}
	logging_server.SetRootWriterConfig(*server)
	logging_server.SyncAll(5.0)
	// Client
	address_port := logging_server.GetRootServerAddressPort()
	fmt.Printf("address_port=%s\n", address_port)
	auth_key := logging_server.GetServerAuthKey()
	client := writer.ClientWriterConfigNew(fl.DEBUG, address_port, &auth_key)
	if client == nil {
		panic("Failed to create client writer")
	}
	client_writers := []fl.WriterConfigEnum{*client}
	client_domain := "LOGCLIENT"
	logging_client := logging.New(fl.DEBUG, &client_domain, client_writers, nil, nil)
	if logging_client == nil {
		panic("Failed to create client logger")
	}
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
