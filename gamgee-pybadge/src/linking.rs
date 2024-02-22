use crate::bridge::Bridge;
extern crate alloc;

type C<'a> = wasmi::Caller<'a, Bridge>;

pub fn link(
    linker: &mut wasmi::Linker<Bridge>,
    memory: wasmi::Memory,
) -> Result<(), wasmi::errors::LinkerError> {
    gamgee_core::link(linker, memory)?;
    linker.func_wrap(
        "env",
        "tone",
        move |mut caller: C, frequency: u32, duration: u32, volume: u32, flags: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_tone(data, frequency, duration, volume, flags)
        },
    )?;
    linker.func_wrap(
        "env",
        "diskr",
        move |mut caller: C, dest: i32, size: u32| -> u32 {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_diskr(data, dest, size)
        },
    )?;
    linker.func_wrap(
        "env",
        "diskw",
        move |mut caller: C, src: i32, size: u32| -> u32 {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_diskw(data, src, size)
        },
    )?;
    linker.func_wrap("env", "trace", move |mut caller: C, str: i32| {
        let (data, bridge) = memory.data_and_store_mut(&mut caller);
        bridge.wasm4_trace(data, str)
    })?;
    linker.func_wrap(
        "env",
        "traceUtf8",
        move |mut caller: C, str: i32, byte_len: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_trace_utf8(data, str, byte_len)
        },
    )?;
    linker.func_wrap(
        "env",
        "traceUtf16",
        move |mut caller: C, str: i32, byte_len: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_trace_utf16(data, str, byte_len)
        },
    )?;

    Ok(())
}
