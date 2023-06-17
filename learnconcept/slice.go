package main

import "fmt"

func slice() {

	var slice []int
	fmt.Println(slice)

	var slice1 = []int{1, 2, 3, 4}
	fmt.Println(slice1)

	var array = [4]int{1, 2, 3, 4}
	slice2 := array[1:3]
	fmt.Println(slice2)

	slice3 := array[:]
	fmt.Println(slice3)

	slice4 := array[2:]
	fmt.Println(slice4)

	slice5 := array[:3]
	fmt.Println(slice5)

	// slice ==> reference type
	var array1 = [5]int{1, 2, 3, 4, 5}
	slice9 := array1
	slice9[0] = 777
	fmt.Println(slice9)

	//lengt and capacity of slice
	// len: so luong phan tu cua slice
	// cap: so luong phan tu underlying array bat dau tu vi tri start khi mà slice đươc tạo
	countries := [...]string{"vn", "us", "ca", "fr", "cn"}
	slice10 := countries[1:4]
	fmt.Println(slice10, len(slice10), cap(slice10))

	// make, copy, append

	slice11 := make([]int, 2, 5)
	fmt.Println(slice11)

	slice13 := make([]int, 2)
	slice13 = append(slice13, 100)
	fmt.Println(slice13, len(slice13), cap(slice13))

	slice13 = append(slice13, 100)
	fmt.Println(slice13, len(slice13), cap(slice13))

	slice13 = append(slice13, 100)
	fmt.Println(slice13, len(slice13), cap(slice13))

	// src := []string {"A", "B", "C"}
	// dest := make([]string, 2)

	// number := copy(dest, src)
	// fmt.Println(src, dest, number)

	// //delete item with index = 1
	// src = append(src[:1],src[2:]...)
	// fmt.Println(src)

}
