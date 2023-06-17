package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

var counter uint64
var wg sync.WaitGroup

func performTask() {
	defer wg.Done()

	// Xử lý công việc
	atomic.AddUint64(&counter, 1)
}

func main() {
	numTasks := 100

	wg.Add(numTasks)
	for i := 0; i < numTasks; i++ {
		go performTask()
	}

	wg.Wait()

	// In ra số lần hàm performTask() được gọi
	fmt.Println("Số lần hàm performTask() được gọi:", counter)
}
