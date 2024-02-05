package main

import (
	"image/color"
	"strconv"

	"tinygo.org/x/tinyfont"
	"tinygo.org/x/tinyfont/freesans"
)

type Bridge struct {
	d *Device
}

func NewBridge(d *Device) *Bridge {
	return &Bridge{d}
}

func (b *Bridge) EchoI32(x int32) {
	text := strconv.FormatInt(int64(x), 10)
	b.EchoText(text)
}

func (b *Bridge) EchoText(text string) {
	b.d.Display.FillScreen(white)
	font := freesans.Regular9pt7b
	w32, _ := tinyfont.LineWidth(&font, text)
	tinyfont.WriteLineColors(
		b.d.Display,
		&font,
		(width-int16(w32))/2,
		72,
		text,
		[]color.RGBA{red},
	)
}
