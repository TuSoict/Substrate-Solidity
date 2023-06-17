package main

import "fmt"

func array() {
	// chao1()
	array1 := [3]int{1, 2, 3}
	fmt.Println(array1)

	array2 := [3]int{100}
	fmt.Println(array2)

	array3 := [...]string{"100", "100", "100", "1000"}
	fmt.Println(array3)

	//array is value type not reference type

	contries := [...]string{"vn", "us", "fr"}
	copycontries := contries
	copycontries[0] = "ja"
	fmt.Println(contries)

	fmt.Println(copycontries)

}
