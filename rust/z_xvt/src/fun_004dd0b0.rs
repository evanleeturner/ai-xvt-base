// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007596b7: i32;
    static mut DAT_007596bb: i32;
    static mut DAT_007596bf: i32;
    static mut DAT_007596c3: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd0b0(param_1: *mut i32) -> u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's four-int rectangle, read and written where the original does.
    unsafe {
        let mut i_var1 = DAT_007596b7;
        let mut u_var2: u32 = 0;
        if param_1.read_unaligned() < i_var1 {
            param_1.write_unaligned(i_var1);
            if param_1.byte_add(8).read_unaligned() < i_var1 {
                param_1
                    .byte_add(8)
                    .write_unaligned((i_var1 as u32).wrapping_sub(1) as i32);
            }
            u_var2 = 1;
        }
        i_var1 = DAT_007596bb;
        if i_var1 < param_1.byte_add(8).read_unaligned() {
            param_1.byte_add(8).write_unaligned(i_var1);
            if i_var1 < param_1.read_unaligned() {
                param_1.write_unaligned((i_var1 as u32).wrapping_add(1) as i32);
            }
            u_var2 |= 4;
        }
        i_var1 = DAT_007596bf;
        if param_1.byte_add(4).read_unaligned() < i_var1 {
            param_1.byte_add(4).write_unaligned(i_var1);
            if param_1.byte_add(12).read_unaligned() < i_var1 {
                param_1
                    .byte_add(12)
                    .write_unaligned((i_var1 as u32).wrapping_sub(1) as i32);
            }
            u_var2 |= 2;
        }
        i_var1 = DAT_007596c3;
        if i_var1 < param_1.byte_add(12).read_unaligned() {
            param_1.byte_add(12).write_unaligned(i_var1);
            if i_var1 < param_1.byte_add(4).read_unaligned() {
                param_1
                    .byte_add(4)
                    .write_unaligned((i_var1 as u32).wrapping_add(1) as i32);
            }
            u_var2 |= 8;
        }
        u_var2
    }
}
