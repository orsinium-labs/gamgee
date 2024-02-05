package main

import (
	"bytes"
	_ "embed"

	"github.com/c0mm4nd/wasman"
	"github.com/c0mm4nd/wasman/config"
)

//go:embed demo.wat
var binaryModule []byte

func run() error {
	device := NewDevice()
	_ = device
	linker1 := wasman.NewLinker(config.LinkerConfig{})
	f := bytes.NewReader(binaryModule)
	module, err := wasman.NewModule(config.ModuleConfig{}, f)
	if err != nil {
		return err
	}
	_ = module
	modules := make(map[string]*wasman.Module)
	ins, err := wasman.NewInstance(module, modules)
	// ins, err := linker1.Instantiate(module)
	if err != nil {
		return err
	}
	_ = linker1
	_ = ins
	// _, _, err = ins.CallExportedFunc("update")
	// if err != nil {
	// 	return err
	// }

	return nil
}

func main() {
	err := run()
	if err != nil {
		panic(err)
	}
}
