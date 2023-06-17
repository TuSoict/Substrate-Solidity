package main

import (
	"fmt"
	"time"
)

func startSender1(name string) <-chan string {
	c := make(chan string)
	go func() {
		for i := 1; i <= 5; i++ {
			c <- (name + " hello")
			time.Sleep(time.Second)
		}
	}()
	return c
}
func main_channel_1() {
	sender := startSender1("Ti")
	for i := 1; i <= 5; i++ {
		fmt.Println(<-sender)
	}
}
