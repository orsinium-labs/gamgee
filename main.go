package main

import (
	"bytes"
	_ "embed"
	"fmt"

	"github.com/c0mm4nd/wasman"
	"github.com/c0mm4nd/wasman/config"
	"github.com/c0mm4nd/wasman/tollstation"
)

//go:embed demo.wasm
var binaryModule []byte

func run(bridge *Bridge) (err error) {
	defer func() {
		p := recover()
		if p != nil {
			err = fmt.Errorf("panic: %v", p)
		}
	}()
	linker := wasman.NewLinker(config.LinkerConfig{})
	wasman.DefineFunc10(linker, "pybadge", "echo_i32", bridge.EchoI32)

	f := bytes.NewReader(binaryModule)
	// fuel consumption to interrupt infinite loops
	ts := tollstation.NewSimpleTollStation(10_000)
	mconf := config.ModuleConfig{
		Recover:     true,
		Logger:      bridge.EchoText,
		TollStation: ts,
	}
	module, err := wasman.NewModule(mconf, f)
	if err != nil {
		return err
	}
	ins, err := linker.Instantiate(module)
	if err != nil {
		return err
	}
	bridge.EchoText("start")

	// frameRate := NewFrameRate()
	for {
		_, _, err = ins.CallExportedFunc("update")
		if err != nil {
			return err
		}
		// reset fuel counter after each update
		ts.AddToll(-ts.GetToll())
		// bridge.EchoText(fmt.Sprintf("%d", frameRate.Update()))
	}
	// return nil
}

func main() {
	device := NewDevice()
	bridge := NewBridge(&device)
	err := run(bridge)
	if err != nil {
		bridge.EchoText(err.Error())
	}
	select {}
}
