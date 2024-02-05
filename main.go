package main

import (
	"context"
	_ "embed"

	"github.com/tetratelabs/wazero"
)

//go:embed demo.wat
var binaryModule []byte

func run() error {
	device := NewDevice()
	_ = device
	ctx := context.Background()
	r := wazero.NewRuntimeWithConfig(ctx, wazero.NewRuntimeConfigInterpreter())
	defer r.Close(ctx)
	return nil
}

func main() {
	err := run()
	if err != nil {
		panic(err)
	}
}
