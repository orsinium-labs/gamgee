package main

import (
	"image/color"
	"strconv"
	"strings"

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
	text = hardWrap(text, 18)
	b.d.Display.FillScreen(white)
	font := freesans.Regular9pt7b
	// w32, _ := tinyfont.LineWidth(&font, text)
	tinyfont.WriteLineColors(
		b.d.Display,
		&font,
		1,
		12,
		text,
		[]color.RGBA{red},
	)
}

func hardWrap(text string, colBreak int) string {
	text = strings.TrimSpace(text)
	wrapped := ""
	var i int
	for i = 0; len(text[i:]) > colBreak; i += colBreak {
		wrapped += text[i:i+colBreak] + "\n"
	}
	wrapped += text[i:]
	return wrapped
}
