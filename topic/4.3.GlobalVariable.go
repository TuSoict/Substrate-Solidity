package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

var globalData atomic.Value
var mutex sync.Mutex

func readData() {
	// Đọc dữ liệu từ biến toàn cục
	data := globalData.Load().(int)
	// Xử lý dữ liệu
	fmt.Println(data)

}

func writeData(newData int) {
	// Ghi dữ liệu vào biến toàn cục
	globalData.Store(newData)
}

func main() {
	// Các goroutine có thể gọi các hàm readData() và writeData() để truy cập vào biến toàn cục globalData
}
