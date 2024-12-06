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
		logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", nil),
	}
	domain := "LOGSRV"
	logger := logging.New(logging.DEBUG, &domain, server_writers, nil, nil)
	server := logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", nil)
	fmt.Printf("server_config=%p\n", server)
	logger.SetRootWriterConfig(server)
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