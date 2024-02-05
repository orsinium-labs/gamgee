package main

import "machine"

type Button uint8

const (
	ButtonLeft   Button = machine.BUTTON_LEFT_MASK
	ButtonUp     Button = machine.BUTTON_UP_MASK
	ButtonDown   Button = machine.BUTTON_DOWN_MASK
	ButtonRight  Button = machine.BUTTON_RIGHT_MASK
	ButtonSelect Button = machine.BUTTON_SELECT_MASK
	ButtonStart  Button = machine.BUTTON_START_MASK
	ButtonA      Button = machine.BUTTON_A_MASK
	ButtonB      Button = machine.BUTTON_B_MASK
)

// Buttons is the state of buttons: which are pressed and which are released.
//
// Use `Device.ReadButtons` to get the current state.
type Buttons struct {
	s uint8
}

// Pressed checks if the given button is currently pressed.
func (s Buttons) Pressed(b Button) bool {
	return Button(s.s)&b > 0
}

// Pressed checks if all of the given buttons are currently pressed.
func (s Buttons) PressedAll(bs ...Button) bool {
	for _, b := range bs {
		if Button(s.s)&b == 0 {
			return false
		}
	}
	return true
}
