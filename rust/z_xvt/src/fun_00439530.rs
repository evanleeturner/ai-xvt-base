// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005ef2cc: i32;
    static mut DAT_005ef244: i32;
    static mut DAT_005ef24c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00439530(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table entry read is
    // at the offset the original computes from param_1.
    unsafe {
        if DAT_005ef2cc == 0 {
            return 0;
        }
        if 0 < param_1 && param_1 <= DAT_005ef244 {
            let u_var1: u32 = (&raw const DAT_005ef24c)
                .cast::<u8>()
                .byte_offset(param_1.wrapping_mul(4) as isize)
                .cast::<u32>()
                .read_unaligned();
            let a = ((u_var1 >> 0x10) & 0xff).wrapping_mul(1000) / 0x4b;
            let b = (((u_var1 & 0xff).wrapping_mul(0x3c)).wrapping_add((u_var1 >> 8) & 0xff))
                .wrapping_mul(1000);
            return a.wrapping_add(b) as i32;
        }
        0
    }
}
