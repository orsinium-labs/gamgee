package main

import (
	"bytes"
	_ "embed"
	"fmt"

	"github.com/c0mm4nd/wasman"
	"github.com/c0mm4nd/wasman/config"
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
	bridge.EchoText("init")
	linker := wasman.NewLinker(config.LinkerConfig{})
	f := bytes.NewReader(binaryModule)
	module, err := wasman.NewModule(config.ModuleConfig{}, f)
	if err != nil {
		return err
	}
	bridge.EchoText("instantiate")
	ins, err := linker.Instantiate(module)
	bridge.EchoText("define funcs")
	linker.DefineFunc("pybadge", "echo_i32", bridge.EchoI32)
	if err != nil {
		return err
	}
	_ = ins
	bridge.EchoText("start")
	// _, _, err = ins.CallExportedFunc("update")
	if err != nil {
		return err
	}
	return nil
}

func main() {
	device := NewDevice()
	bridge := NewBridge(&device)
	err := run(bridge)
	if err != nil {
		bridge.EchoText(err.Error())
	}
	for {
	}
}
