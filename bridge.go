package main

import "github.com/perlin-network/life/exec"

type Bridge struct {
	Device *Device
}

func (b *Bridge) ResolveFunc(module, field string) exec.FunctionImport {
	// exec.FunctionImport{}
	return nil
}

func (b *Bridge) ResolveGlobal(module, field string) int64 {
	//...
	return 0
}
