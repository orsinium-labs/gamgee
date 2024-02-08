# pybadge wasm

| runtime       | language  | archived | stale | micro | test results |
| ------------- | --------- | -------- | ----- | ----- | ------------ |
| [wazero]      | 🐹 Go     |    |    |    | ❌ |
| [wasman]      | 🐹 Go     |    | 2y |    | ✅ |
| [wagon]       | 🐹 Go     | 🗃 | 4y |    |    |
| [life]        | 🐹 Go     | 💀 | 5y |    | ❌ |
| [wamr]        | C+Go      |    |    | 🔬 |    |
| [go-wasm3]    | C+Go      |    | 2y | 🔬 |    |
| [wasmer-go]   | Rust+Go   | 💀 | 2y |    |    |
| [wasmtime-go] | Rust+Go   |    |    |    |    |
| [wasmi]       | Rust      |    |    |    |    |

[wagon]:        https://github.com/go-interpreter/wagon
[wasmer-go]:    https://github.com/wasmerio/wasmer-go
[go-wasm3]:     https://github.com/matiasinsaurralde/go-wasm3
[wamr]:         https://github.com/bytecodealliance/wasm-micro-runtime
[wasmi]:        https://github.com/paritytech/wasmi
[life]:         https://github.com/perlin-network/life
[wazero]:       https://github.com/tetratelabs/wazero
[wasman]:       https://github.com/c0mm4nd/wasman
[wasmtime-go]:  https://github.com/bytecodealliance/wasmtime-go

Test results:

1. [wasman] is a very simple pure Go interpreter written by someone just for fun. it's not popular, it's not optimized, and it's not well tested. To make it work, I had to fork it, get rid of reflection, fix a memory leak, make my code use memory (the interpreter explodes with nil pointer dereference without memory), and change the memory page size to 16 kB to fit it into the available RAM.
1. [life] fails in `NewVirtualMachine` for a mysterious reason. Reason critical enough for the code to explode without even showing the panic. Probably, something like OOM. I reaches the `m.CompileForInterpreter(gasPolicy)` call but never actually enters the method.
1. [wazero] for tinygo is in progress, see [wazero#1854](https://github.com/tetratelabs/wazero/issues/1854). I tried running the patch linked there with dev version of tinygo and reduced page size but it still dies in `DecodeModule`, at calling `newMemorySizer`. And even with the memory sizer replaced by `nil`, it gets a but farther but not too far.
