// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00622c4c: i32;
    static mut DAT_00622bc4: i32;
    static mut DAT_00622bcc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a5340(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table entry is read
    // at the same scaled offset the original uses.
    unsafe {
        if DAT_00622c4c != 0 {
            return 0;
        }
        if 0 < param_1 && param_1 <= DAT_00622bc4 {
            let u_var1: u32 = (&raw mut DAT_00622bcc)
                .cast_const()
                .offset(param_1.wrapping_mul(4) as isize)
                .cast::<u32>()
                .read_unaligned();
            let a: u32 = (((u_var1 >> 0x10) & 0xff).wrapping_mul(1000)) / 0x4b;
            let b: u32 = ((u_var1 & 0xff)
                .wrapping_mul(0x3c)
                .wrapping_add((u_var1 >> 8) & 0xff))
            .wrapping_mul(1000);
            return a.wrapping_add(b) as i32;
        }
        0
    }
}
