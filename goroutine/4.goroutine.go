package main

import (
	"fmt"
	"sync"
	"time"
)

func gr1() {
	time.Sleep(time.Second)
	fmt.Println("g1")
	wg2.Done()
}
func rg2() {
	fmt.Println("g2")
	wg2.Done()
}

var wg2 sync.WaitGroup

func main4() {
	// go g1()
	// go g2()
	// ng:= runtime.NumGoroutine()
	// fmt.Println(ng)
	// time.Sleep(time.Second)

	// Synchronied
	/*
		logic 1

		go g1()
		go g2()

		logic 2
	*/
	fmt.Println("Begin...")
	wg2.Add(2)

	go gr1()
	go rg2()

	wg2.Wait()

	fmt.Println("End...")

}
