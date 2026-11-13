// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075dc46: u32;
    static mut DAT_0075dc4a: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db4c0() -> u32 {
    // SAFETY: the globals are the game's own memory; the addresses may be
    // unaligned, so the writes are done unaligned through raw pointers.
    unsafe {
        (&raw mut DAT_0075dc46).write_unaligned(0);
        (&raw mut DAT_0075dc4a).write_unaligned(0);
    }
    1
}
