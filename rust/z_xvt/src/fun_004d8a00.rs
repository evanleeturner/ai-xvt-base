// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532a34: i32;
    static mut DAT_00758820: i32;
    static mut DAT_00758618: i32;
    static mut DAT_0064f658: i32;
    static mut DAT_00757cc4: *mut i32;
    static mut DAT_00758824: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ca200();
    fn FUN_004d8520();
    fn FUN_004d8900();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d8a70(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d8a00() -> i32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointer writes cover the 8-int block at DAT_0064f658 and the
    // 0x100-int block at DAT_00757cc4, as the original does.
    unsafe {
        let mut i_var2: i32 = 0;
        DAT_00532a34 = -1;
        DAT_00758820 = 0;
        DAT_00758618 = -1;
        let mut p: *mut i32 = &raw mut DAT_0064f658;
        let mut i_var1: i32 = 8;
        while i_var1 != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i_var1 -= 1;
        }
        p = DAT_00757cc4;
        i_var1 = 0x100;
        while i_var1 != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i_var1 -= 1;
        }
        FUN_004ca200();
        FUN_004d8520();
        if DAT_00758824 > 0 {
            loop {
                FUN_004d8a70(i_var2);
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= DAT_00758824 {
                    break;
                }
            }
        }
        FUN_004ca200();
        FUN_004d8900();
        DAT_00758824
    }
}
