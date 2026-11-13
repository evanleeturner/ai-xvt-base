// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081f7de: u32;
    static mut DAT_0081fa9e: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cdc20(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fe2b0() -> u32 {
    // SAFETY: the globals and the callee are the game's own; the walk steps
    // through the game's table in 0x58-byte strides, reading each entry's
    // first dword unaligned, as the original does.
    unsafe {
        let mut p_var1: *mut u32 = &raw mut DAT_0081f7de;
        loop {
            let v = p_var1.read_unaligned();
            if v != 0 {
                FUN_004cdc20(v);
            }
            p_var1 = p_var1.wrapping_add(0x16);
            if !(p_var1 < &raw mut DAT_0081fa9e) {
                break;
            }
        }
    }
    1
}
