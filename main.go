package main

import (
	"fmt"

	"github.com/perlin-network/life/exec"
)

func run() error {
	device := NewDevice()
	// Leds(&device)
	vm, err := exec.NewVirtualMachine(input, exec.VMConfig{}, &exec.NopResolver{}, nil)
	if err != nil { // if the wasm bytecode is invalid
		return err
	}
	entryID, ok := vm.GetFunctionExport("app_main") // can be changed to your own exported function
	if !ok {
		panic("entry function not found")
	}
	ret, err := vm.Run(entryID)
	if err != nil {
		vm.PrintStackTrace()
		panic(err)
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
