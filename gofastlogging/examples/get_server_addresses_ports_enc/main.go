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
	// IMPORTANT: server_key is consumed by Rust when the server writer config
	// is created below. Using server_key again afterwards leads to errors!
	server_key := fl.CreateRandomKey(fl.AES)
	server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", &server_key)
	if server == nil {
		panic("Failed to create server writer")
	}
	server_writers := []fl.WriterConfigEnum{*console, *server}
	domain := "LOGSRV"
	logger := logging.New(fl.DEBUG, &domain, server_writers, nil, nil)
	if logger == nil {
		panic("Failed to create logger")
	}
	// IMPORTANT: We have to create another instance of server_key, because it was consumed above.
	server_key2 := fl.CreateRandomKey(fl.AES)
	root_server := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", &server_key2)
	if root_server == nil {
		panic("Failed to create root server writer")
	}
	fmt.Printf("server_config=%p\n", root_server)
	logger.SetRootWriterConfig(*root_server)
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
