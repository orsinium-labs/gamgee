package main

import (
	_ "embed"
	"errors"
	"fmt"

	"github.com/perlin-network/life/exec"
)

//go:embed demo.wat
var binaryModule []byte

func run() error {
	device := NewDevice()
	_ = device
	// Leds(&device)
	vm, err := exec.NewVirtualMachine(binaryModule, exec.VMConfig{}, &exec.NopResolver{}, nil)
	if err != nil { // if the wasm bytecode is invalid
		return err
	}
	entryID, ok := vm.GetFunctionExport("update")
	if !ok {
		return errors.New("cannot find function `update`")
	}
	ret, err := vm.Run(entryID)
	if err != nil {
		vm.PrintStackTrace()
		return err
	}
	fmt.Printf("return value = %d\n", ret)
	return nil
}

func main() {
	err := run()
	if err != nil {
		panic(err)
	}
}
