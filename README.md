# 👾 gamgee

Run WebAssembly (WASM-4) games on small devices.

Gamgee is a [WASM-4](https://wasm4.org/) games emulator written in [Rust](https://www.rust-lang.org/) and designed to be executed on devices with very little memory and space available. Currently, it supports only [Adafruit PyBadge](https://www.adafruit.com/product/4200) but we plan to add more in the future.

Features:

* 💾 **Small size**. The binary is just about 270 Kb, and it includes wasm interpreter, allocator, graphics library, a custom font, etc.
* 🐜 **Small memory requirements**. The runtime itself needs just a few kilobytes of RAM and the rest is fully available to the running game.
* 🕹 **WASM-4 compatible**. It can run any WASM-4 game ([and there are quite a few](https://wasm4.org/play)) as long as it fits into memory.

## 🔧 Installation and usage

1. Install Rust.
1. [Install task](https://taskfile.dev/installation/).
1. Connect PyBadge and turn it on.
1. Press the "reset" button twice on PyBadge to put it into the bootloader mode.
1. Flash a game onto the device: `task flash -- $PWD/watris.wasm`. The path must be absolute (hence `$PWD`).
1. Press the "reset" button on PyBadge once to refresh the screen.

## 🙅 Limitations

1. PyBadge screen size is 160x138. WASM-4 games expect 160x160. To fit the image, we skip every 5th line.
1. WASM-4 games expect 60 FPS but the PyBadge screen is limited to 20 FPS and then there is often just not enough CPU to keep up. The solution is to try calling the `update` callback as many times as possible but draw only every 5th frame. However, the games can still be slower than they should.
1. PyBadge screen uses 16 bits per pixel for colors. WASM-4 color palette is defined as 24 bits for each color. We translate the color palette to pybadge colors as close as possible but the colors might look a bit different from (not as vibrant as) what you see on your PC screen.
1. PyBadge has 192KB of RAM, and a few Kb are needed for the runtime itself. WebAssembly memory is allocated in pages of 64 Kb. So, we can allocate only 2 pages (128 Kb) of memory for the game to use before crashing with OOM. It's enough for most of the games but not all of them.
1. Unsupported: `tone` (playing sounds). PyBadge doesn't have a speaker by default. You can attach your own but I don't have one yet. PyBadge has a built-in buzzer but that's not enough for WASM-4 games.
1. Unsupported: `diskw` and `diskr` (persistent storage). PyBadge doesn't have a persistent sotrage. Luckily, I haven't seen a game yet that would use these functions.

## 🙏 Acknowledgments

I want to thank:

* [Ron Evans](https://github.com/deadprogram) for inspiting me to do the project and helping me to figure out some PyBadge quirks
* [Robin Freyler](https://github.com/Robbepop) for maintaining [wasmi](https://github.com/wasmi-labs/wasmi) and answering my questions about how to make Rust borrow checker happy.
* Creators of all other dependencies that the project uses, including [pybadge-high](https://github.com/LuckyTurtleDev/pybadge-high), [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics), [atsamd](https://github.com/atsamd-rs/atsamd), and [embedded-alloc](https://github.com/rust-embedded/embedded-alloc).

And thank you for using the project ♥️
