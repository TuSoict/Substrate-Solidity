package main

import (
	"fmt"
	"runtime"
	"sync"
	"time"
)

var wg sync.WaitGroup

func createPizza(pizza int) {
	defer wg.Done()
	time.Sleep(time.Second)
	fmt.Printf("Create pizza %d \n", pizza)
}

func TimeTrack(start time.Time, functionName string) {
	elapesd := time.Since(start)
	fmt.Println(functionName, "took", elapesd)
}

func main3() {
	defer TimeTrack(time.Now(), "Build Pizzas")

	number_of_ovens := 10
	runtime.GOMAXPROCS(number_of_ovens)
	wg.Add(number_of_ovens)

	for i := 0; i < number_of_ovens; i++ {
		go createPizza(i)
	}

	wg.Wait()
}
