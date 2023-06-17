package main

import (
	"fmt"
	"math/rand"
	"time"
)

func fetchAPI(model string) string {
	time.Sleep(time.Duration(rand.Intn(1e3)) * time.Millisecond)
	return model
}

func main_channel_3() {
	responseChan := make(chan string)
	var results []string

	go func() { responseChan <- fetchAPI("users") }()
	go func() { responseChan <- fetchAPI("categories") }()
	go func() { responseChan <- fetchAPI("products") }()

	for i := 1; i <= 3; i++ {
		results = append(results, <-responseChan)
	}
	fmt.Println(results)
}
