package main

import (
	"fmt"
	api "mangapplizer-backend/pkg/api"
	"os"
	"os/signal"
)

func main() {

	if len(os.Args) < 2 {
		fmt.Println("Please tell me the port number with `:` in front of it!")
		os.Exit(255)
	}
	go api.RunServer(os.Args[1], "admin", "123")

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, os.Kill)
	<-c
}
