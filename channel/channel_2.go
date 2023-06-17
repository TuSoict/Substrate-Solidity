package main

import (
	"fmt"
	"math/rand"
	"time"
)

func startSender2(name string) <-chan string {
	c := make(chan string)
	go func() {
		for i := 1; i <= 5; i++ {
			c <- (name + " hello")
			time.Sleep(time.Duration(rand.Intn(1e3)) * time.Millisecond)
		}
	}()
	return c
}

func main_channel_2() {
	ti := startSender2("Ti")
	teo := startSender2("Teo")

	for {
		select {
		case msgTi := <-ti:
			fmt.Println(msgTi)
		case msgTeo := <-teo:
			fmt.Println(msgTeo)
		}
	}
}
