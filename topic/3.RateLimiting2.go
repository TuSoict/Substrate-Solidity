// Rate limiting is an important mechanism for controlling resource utilization and maintaining quality of service. Go elegantly supports rate limiting with goroutines, channels, and tickers.
// First we’ll look at basic rate limiting. Suppose we want to limit our handling of incoming requests. We’ll serve these requests off a channel of the same name.
// This limiter channel will receive a value every 200 milliseconds. This is the regulator in our rate limiting scheme.
// By blocking on a receive from the limiter channel before serving each request, we limit ourselves to 1 request every 200 milliseconds.
// We may want to allow short bursts of requests in our rate limiting scheme while preserving the overall rate limit. We can accomplish this by buffering our limiter channel. This burstyLimiter channel will allow bursts of up to 3 events.
// Fill up the channel to represent allowed bursting.
// Every 200 milliseconds we’ll try to add a new value to burstyLimiter, up to its limit of 3.
// Now simulate 5 more incoming requests. The first 3 of these will benefit from the burst capability of burstyLimiter.
// Running our program we see the first batch of requests handled once every ~200 milliseconds as desired.
// For the second batch of requests we serve the first 3 immediately because of the burstable rate limiting, then serve the remaining 2 with ~200ms delays each
package main

import (
	"fmt"
	"time"
)

func main() {

	requests := make(chan int, 5)
	for i := 1; i <= 5; i++ {
		requests <- i
	}
	close(requests)

	burstyRequests := make(chan int, 20)
	for i := 1; i <= 20; i++ {
		burstyRequests <- i
	}
	close(burstyRequests)

	limiter := make(chan time.Time, 3) //time.Tick(2000 * time.Millisecond)
	burstyLimiter := make(chan time.Time, 3)

	go func() {
		for t := range time.Tick(1000 * time.Millisecond) {
			limiter <- t
		}
	}()

	go func() {
		for t := range time.Tick(200 * time.Millisecond) {
			burstyLimiter <- t
		}
	}()

	for i := 0; i < 3; i++ {
		burstyLimiter <- time.Now()
	}

	for i := 1; i <= 25; i++ {
		select {
		case v1 := <-limiter:
			fmt.Println("request", v1)
		case v2 := <-burstyLimiter:
			fmt.Println("bursty request", v2)
		}
	}

	// for req := range requests {
	// 	<-limiter
	// 	fmt.Println("request", req, time.Now())
	// }

	// for req := range burstyRequests {
	// 	<-burstyLimiter
	// 	fmt.Println("bursty request", req, time.Now())
	// }
}
