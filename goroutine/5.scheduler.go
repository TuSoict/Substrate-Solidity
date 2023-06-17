package main

import (
	"fmt"
	"runtime"
)
func g1() {
	fmt.Println("g1")
}
func g2() {
	fmt.Println("g2")
}

func g3() {
	fmt.Println("g3")
}
func g4() {
	fmt.Println("g4")
}
func main5() {
	numberP := runtime.NumCPU()
	fmt.Println(numberP)

	runtime.GOMAXPROCS(2)
	numberP1 := runtime.GOMAXPROCS(0)
	fmt.Println(numberP1)

	go g1()
	go g2()
	go g3()
	go g4()

}

