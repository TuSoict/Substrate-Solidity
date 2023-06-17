package main

import (
	"fmt"
	"sync/atomic"
)

var counter int32 = 10

func main() {

	oldValue := atomic.SwapInt32(&counter, 10)
	fmt.Println("Giá trị cũ:", oldValue)
	fmt.Println("Giá trị mới:", counter)
	fmt.Println("---")
	oldValue = atomic.SwapInt32(&counter, 20)
	fmt.Println("Giá trị cũ:", oldValue)
	fmt.Println("Giá trị mới:", counter)
	fmt.Println("---")
	oldValue = atomic.SwapInt32(&counter, 10)
	fmt.Println("Giá trị cũ:", oldValue)
	fmt.Println("Giá trị mới:", counter)
	fmt.Println("#########################################")
	fmt.Println("Giá trị cũ:", counter)
	swapped := atomic.CompareAndSwapInt32(&counter, 10, 20)
	fmt.Println("Đã thay đổi:", swapped)
	fmt.Println("Giá trị mới:", counter)
	fmt.Println("---")
	fmt.Println("Giá trị cũ:", counter)
	swapped = atomic.CompareAndSwapInt32(&counter, 20, 10)
	fmt.Println("Đã thay đổi:", swapped)
	fmt.Println("Giá trị mới:", counter)
	fmt.Println("---")
	fmt.Println("Giá trị cũ:", counter)
	swapped = atomic.CompareAndSwapInt32(&counter, 20, 20)
	fmt.Println("Đã thay đổi:", swapped)
	fmt.Println("Giá trị mới:", counter)
	fmt.Println("---")
	fmt.Println("Giá trị cũ:", counter)
	swapped = atomic.CompareAndSwapInt32(&counter, 10, 10)
	fmt.Println("Đã thay đổi:", swapped)
	fmt.Println("Giá trị mới:", counter)

}
