module github.com/orsinium-labs/pybadge-wasm

go 1.21.3

replace (
	github.com/c0mm4nd/wasman => ../wasman
)

require (
	github.com/c0mm4nd/wasman v0.0.0-20220422074058-87e38ef26abd
	tinygo.org/x/drivers v0.26.0
	tinygo.org/x/tinyfont v0.4.0
)

require github.com/google/shlex v0.0.0-20191202100458-e7afc7fbc510 // indirect
