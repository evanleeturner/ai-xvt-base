// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070b210: [u8; 0];
    static mut DAT_0070b216: [u8; 0];
    static mut DAT_0070ad43: [u8; 0];
    static mut DAT_0070ad51: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ac6f0(param_1: i32, param_2: *mut i32) -> i32 {
    let idx = param_1.wrapping_mul(0x566) as isize;
    // SAFETY: the globals are the game's own tables, indexed as the original
    // does; param_2 points at the caller's int, read only where the original reads.
    unsafe {
        if (&raw mut DAT_0070b210).cast::<u8>().offset(idx).read() == 1 {
            return 0;
        }
        if (&raw mut DAT_0070b216).cast::<u8>().offset(idx).read() == 0 {
            return param_2.read_unaligned();
        }
        if (&raw mut DAT_0070ad43).cast::<u8>().offset(idx).read() == 1 {
            return param_2.read_unaligned();
        }
        if (&raw mut DAT_0070ad51).cast::<u8>().offset(idx).read() == 0 {
            return 0;
        }
        param_2.read_unaligned()
    }
}
