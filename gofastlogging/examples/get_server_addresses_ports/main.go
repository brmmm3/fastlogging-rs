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
		panic("Failed to create writer")
	}
	server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", nil)
	if server == nil {
		panic("Failed to create server")
	}
	server_writers := []fl.WriterConfigEnum{*console, *server}
	domain := "LOGSRV"
	logger := logging.New(fl.DEBUG, &domain, server_writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	logger.SetRootWriterConfig(*server)
	logger.SyncAll(5.0)
	// Show addresses and ports
	ports := logger.GetRootServerPorts()
	fmt.Print("GetRootServerPorts\n")
	fmt.Printf("  ports->cnt=%d\n", len(ports))
	for key, value := range ports {
		fmt.Printf("  addresses_ports->key=%d\n", key)
		fmt.Printf("  addresses_ports->value=%d\n", value)
	}
	addresses := logger.GetRootServerAddresses()
	fmt.Print("GetRootServerAddresses\n")
	fmt.Printf("  addresses->cnt=%d\n", len(addresses))
	for key, value := range addresses {
		fmt.Printf("  addresses_ports->key=%d\n", key)
		fmt.Printf("  addresses_ports->value=%s\n", value)
	}
	addresses_ports := logger.GetRootServerAddressesPorts()
	fmt.Print("GetRootServerAddressesPorts\n")
	fmt.Printf("addresses_ports->cnt=%d\n", len(addresses_ports))
	for key, value := range addresses_ports {
		fmt.Printf("  addresses_ports->key=%d\n", key)
		fmt.Printf("  addresses_ports->value=%s\n", value)
	}
	// Test logging
	logger.Info("Info Message")
	logger.SyncAll(1.0)
	fmt.Print("Shutdown Logger\n")
	logger.Shutdown(false)
	fmt.Print("-------- Finished --------\n")
}
