package main

import (
	"fmt"
	"time"
)

func sum(s []int, c chan int) {
	sum := 0
	for _, v := range s {
		sum += v
	}
	c <- sum // send sum to c
}
func main22() {

	c := make(chan int)

	go func() {
		for i := 1; i <= 10; i++ {
			time.Sleep(time.Second)
			c <- i
		}
	}()
	// c <- 1

	for {
		fmt.Println("blocking...")
		data := <-c
		fmt.Println(data)
	}
	//###############################

	// fmt.Println("Hello")
	// c:= make(chan int)

	// go func ()  {
	// 	time.Sleep(time.Second)
	// 	c <- 1
	// }()
	// // c <- 1

	// data:= <-c
	// fmt.Println(data)
	//###############################

	// s := []int{7, 2, 8, -9, 4, 0}

	// c := make(chan int)
	// go sum(s[len(s)/2:], c)
	// go sum(s[:len(s)/2], c)
	// x, y := <-c, <-c // receive from c

	// fmt.Println(x, y, x+y)

}
