package main

import "time"

type FrameRate struct {
	rate   uint16
	synced time.Time
	buffer uint16
}

func NewFrameRate() FrameRate {
	return FrameRate{synced: time.Now()}
}

// Update and return the current frame rate
func (fr *FrameRate) Update() uint16 {
	now := time.Now()
	if now.Sub(fr.synced) >= time.Second {
		fr.synced = now
		fr.rate = fr.buffer
		fr.buffer = 0
	}
	fr.buffer += 1
	return fr.rate
}
