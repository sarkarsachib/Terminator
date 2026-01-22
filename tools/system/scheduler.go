package system

import "time"

type Job func()

type Scheduler struct{}

func (s *Scheduler) Every(d time.Duration, job Job) {
	ticker := time.NewTicker(d)
	go func() {
		for range ticker.C {
			job()
		}
	}()
}
