package main

import (
	"fmt"
	"time"
)

func createPizza1(pizza int) {
	time.Sleep(time.Second)
	fmt.Printf("Create Pizza %d \n", pizza)
}

func timeTrack1(start time.Time, functionName string) {
	elapesd := time.Since(start)
	fmt.Println(functionName, "took", elapesd)
}

func main2() {
	defer timeTrack1(time.Now(), "Build Pizzas")
	for i := 0; i < 10; i++ {
		createPizza1(i)
	}
}
