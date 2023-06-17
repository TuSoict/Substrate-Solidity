package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	var counter int64

	var wg sync.WaitGroup

	// Sử dụng Swap để tăng counter lên 10
	wg.Add(1)
	go func() {
		defer wg.Done()
		atomic.SwapInt64(&counter, 10)
	}()

	// Sử dụng CompareAndSwap để kiểm tra và tăng counter
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()

			// Kiểm tra giá trị hiện tại của counter
			current := atomic.LoadInt64(&counter)

			// Nếu counter đang là giá trị hiện tại, tăng counter lên 1
			// Sử dụng CompareAndSwap để kiểm tra và thay đổi giá trị
			// Nếu counter đã bị thay đổi bởi goroutine khác, không thay đổi giá trị và tiếp tục vòng lặp
			for !atomic.CompareAndSwapInt64(&counter, current, current+1) {
				current = atomic.LoadInt64(&counter)
			}
		}()
	}

	wg.Wait()

	fmt.Println("Counter:", counter)
}
