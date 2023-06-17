package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

var counter uint64

type Resource struct {
	accessCount int64
	mu          sync.Mutex
}

func (r *Resource) Access() {
	// r.mu.Lock()
	// defer r.mu.Unlock()
	atomic.AddInt64(&r.accessCount, 1)
	// r.accessCount++
	// Thực hiện các hoạt động truy cập vào tài nguyên chia sẻ ở đây
}

// đếm các lượt truy cập tài nguyên
func main() {
	resource := &Resource{}

	var wg sync.WaitGroup
	numAccesses := 10000

	wg.Add(numAccesses)

	for i := 0; i < numAccesses; i++ {
		go func() {
			resource.Access()
			wg.Done()
		}()
	}

	wg.Wait()

	fmt.Println("Số lượt truy cập:", atomic.LoadInt64(&resource.accessCount))
}
