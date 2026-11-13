// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d9150: [u8; 0];
    static mut DAT_009d9156: [u8; 0];
    static mut DAT_009d8c83: [u8; 0];
    static mut DAT_009d8c91: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043c740(param_1: i32, param_2: *mut i32) -> i32 {
    let idx = param_1.wrapping_mul(0x566) as isize;
    // SAFETY: the globals are the game's own byte tables, indexed as the
    // original does; param_2 points at the caller's int.
    unsafe {
        if (&raw mut DAT_009d9150).cast::<u8>().offset(idx).read() == 1 {
            return 0;
        }
        if (&raw mut DAT_009d9156).cast::<u8>().offset(idx).read() == 0 {
            return param_2.read_unaligned();
        }
        if (&raw mut DAT_009d8c83).cast::<u8>().offset(idx).read() == 1 {
            return param_2.read_unaligned();
        }
        if (&raw mut DAT_009d8c91).cast::<u8>().offset(idx).read() == 0 {
            return 0;
        }
        param_2.read_unaligned()
    }
}
