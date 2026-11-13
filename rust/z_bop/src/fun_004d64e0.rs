// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b82: i32;
    static mut DAT_00aa7b86: i32;
    static mut DAT_00aa7b8a: i32;
    static mut DAT_00aa7b8e: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d64e0(param_1: *mut i32) -> u32 {
    // SAFETY: the globals are the game's own; param_1 points at the caller's
    // four-int rectangle, read and written where the original does.
    unsafe {
        let mut i_var1 = DAT_00aa7b82;
        let mut u_var2: u32 = 0;
        if param_1.read_unaligned() < DAT_00aa7b82 {
            param_1.write_unaligned(DAT_00aa7b82);
            if param_1.byte_add(8).read_unaligned() < i_var1 {
                param_1.byte_add(8).write_unaligned(i_var1.wrapping_sub(1));
            }
            u_var2 = 1;
        }
        i_var1 = DAT_00aa7b86;
        if DAT_00aa7b86 < param_1.byte_add(8).read_unaligned() {
            param_1.byte_add(8).write_unaligned(DAT_00aa7b86);
            if i_var1 < param_1.read_unaligned() {
                param_1.write_unaligned(i_var1.wrapping_add(1));
            }
            u_var2 |= 4;
        }
        i_var1 = DAT_00aa7b8a;
        if param_1.byte_add(4).read_unaligned() < DAT_00aa7b8a {
            param_1.byte_add(4).write_unaligned(DAT_00aa7b8a);
            if param_1.byte_add(12).read_unaligned() < i_var1 {
                param_1.byte_add(12).write_unaligned(i_var1.wrapping_sub(1));
            }
            u_var2 |= 2;
        }
        i_var1 = DAT_00aa7b8e;
        if DAT_00aa7b8e < param_1.byte_add(12).read_unaligned() {
            param_1.byte_add(12).write_unaligned(DAT_00aa7b8e);
            if i_var1 < param_1.byte_add(4).read_unaligned() {
                param_1.byte_add(4).write_unaligned(i_var1.wrapping_add(1));
            }
            u_var2 |= 8;
        }
        u_var2
    }
}
