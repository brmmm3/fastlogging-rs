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
	var encryption logging.EncryptionMethodEnum = logging.NONE
	server_key := logging.CreateRandomKey(encryption.Into())
	server_writers := []logging.WriterConfigEnum{
		logging.ConsoleWriterConfigNew(logging.DEBUG, true),
		logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", &logging.KeyStruct{Key: server_key}),
	}
	domain := "LOGSRV"
	logger := logging.New(logging.DEBUG, &domain, server_writers, nil, nil)
	server := logging.ServerConfigNew(logging.DEBUG, "127.0.0.1", &logging.KeyStruct{Key: server_key})
	fmt.Printf("server_config=%p\n", server)
	logger.SetRootWriterConfig(server)
	logger.SyncAll(5.0)
	// Show addresses and ports
	ports := logger.GetRootServerPorts()
	fmt.Printf("ports->cnt=%d\n", ports.Cnt)
	for i := 0; i < (int)(ports.Cnt); i++ {
		fmt.Printf("ports->key[%d]=%d\n", i, ports.Keys[i])
		fmt.Printf("ports->value[%d]=%d\n", i, ports.Values[i])
	}
	addresses := logger.GetRootServerAddresses()
	fmt.Printf("addresses->cnt=%d\n", addresses.Cnt)
	for i := 0; i < (int)(addresses.Cnt); i++ {
		fmt.Printf("addresses->key[%d]=%d\n", i, addresses.Keys[i])
		fmt.Printf("addresses->value[%d]=%d\n", i, addresses.Values[i])
	}
	addresses_ports := logger.GetRootServerAddressesPorts()
	fmt.Printf("addresses_ports->cnt=%d\n", addresses_ports.Cnt)
	for i := 0; i < (int)(addresses_ports.Cnt); i++ {
		fmt.Printf("addresses_ports->key[%d]=%d\n", i, addresses_ports.Keys[i])
		fmt.Printf("addresses_ports->value[%d]=%d\n", i, addresses_ports.Values[i])
	}
	// Test logging
	logger.Info("Info Message")
	logger.SyncAll(1.0)
	fmt.Print("Shutdown Logger\n")
	logger.Shutdown(false)
	fmt.Print("-------- Finished --------\n")
}
