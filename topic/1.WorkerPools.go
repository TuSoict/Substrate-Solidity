// In this example we’ll look at how to implement a worker pool using goroutines and channels.
// Here’s the worker, of which we’ll run several concurrent instances. These workers will receive work on the jobs channel and send the corresponding results on results. We’ll sleep a second per job to simulate an expensive task.
// In order to use our pool of workers we need to send them work and collect their results. We make 2 channels for this.
// This starts up 3 workers, initially blocked because there are no jobs yet.
// Here we send 5 jobs and then close that channel to indicate that’s all the work we have.
// Finally we collect all the results of the work. This also ensures that the worker goroutines have finished. An alternative way to wait for multiple goroutines is to use a WaitGroup.
// Our running program shows the 5 jobs being executed by various workers. The program only takes about 2 seconds despite doing about 5 seconds of total work because there are 3 workers operating concurrently.
package main

import (
	"fmt"
	"time"
)

func worker(id int, jobs <-chan int, results chan<- int) {
	for j := range jobs {
		fmt.Println("worker", id, "started  job", j)
		time.Sleep(time.Second)
		fmt.Println("worker", id, "finished job", j)
		results <- j * 2
	}
}

func main() {

	const numJobs = 5
	jobs := make(chan int, numJobs)
	results := make(chan int, numJobs)

	for w := 1; w <= 3; w++ {
		go worker(w, jobs, results)
	}

	for j := 1; j <= numJobs; j++ {
		// khi bắt đầu có job (bắt đầu có giá) => worker pool chọn worker random -> worker không theo thứ tự
		// 3 worker song song
		jobs <- j 
	}
	close(jobs)

	for a := 1; a <= numJobs; a++ {
		<-results
	}
}