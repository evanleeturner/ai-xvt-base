// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075dc46: u32;
    static mut DAT_0064f89c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db500() -> u32 {
    // SAFETY: the globals are the game's own memory; DAT_0075dc46 is
    // unaligned-safe to write via write_unaligned on its address.
    unsafe {
        let v = DAT_0064f89c;
        (&raw mut DAT_0075dc46).write_unaligned(v);
    }
    1
}
