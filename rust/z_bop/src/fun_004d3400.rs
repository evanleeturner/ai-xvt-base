// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7ff7: [u8; 4];
    static mut DAT_00aa7ff3: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d3400() -> u32 {
    // SAFETY: the globals are the game's own memory, accessed by address
    // with unaligned reads/writes as the original memcpy does.
    unsafe {
        let p7 = (&raw mut DAT_00aa7ff7).cast::<u32>();
        let v1 = p7.read_unaligned();
        if v1 == 1 {
            p7.write_unaligned(2);
            let tick = GetTickCount();
            let v2 = tick.wrapping_add(1000);
            (&raw mut DAT_00aa7ff3).cast::<u32>().write_unaligned(v2);
        }
    }
    1
}
