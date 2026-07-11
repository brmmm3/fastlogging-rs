package main

import (
	root "gofastlogging/fastlogging"
	"log"
)

func main() {
	err := root.Init()
	if err != nil {
		log.Fatal(err)
	}
	root.Trace("Trace message")
	root.Debug("Debug message")
	root.Info("Info Message")
	root.Warning("Warning Message")
	root.Error("Error Message")
	root.Fatal("Fatal Message")
	root.Shutdown(false)
}
