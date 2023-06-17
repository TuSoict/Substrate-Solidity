package main

import "fmt"

func hello() {
	var str = "Hello world"
	fmt.Println(str)

	a := 10
	fmt.Println(a)

	var a1, b1 = 10, 11
	fmt.Println(a1)
	fmt.Println(b1)


	var (
		name string
		address string
		age int
	)

	name = "Quoc"
	address = "vietnam"
	age = 25
	fmt.Println(name)
	fmt.Println(address, age)
	println(name)
}
