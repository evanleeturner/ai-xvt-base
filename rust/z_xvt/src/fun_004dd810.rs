// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758e8e: i32;
    static mut DAT_00758e92: i32;
    static mut DAT_00758e86: *mut i32;
    static mut DAT_00758e8a: *mut i32;
    static mut DAT_00758d5a: i32;
    static mut DAT_00758dbe: i32;
    static mut DAT_00532bd8: i32;
    static mut DAT_00758e9e: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dd810() {
    // SAFETY: all globals are the game's own memory; the copy and fill
    // ranges are the fixed-size tables the original touches.
    unsafe {
        DAT_00758e8e = 10;
        DAT_00758e92 = 10;
        DAT_00758e86 = &raw mut DAT_00758d5a;
        DAT_00758e8a = &raw mut DAT_00758dbe;
        let mut p_var2: *const i32 = &raw mut DAT_00532bd8;
        let mut p_var3: *mut i32 = &raw mut DAT_00758d5a;
        let mut i_var1: i32 = 0x19;
        while i_var1 != 0 {
            p_var3.write_unaligned(p_var2.read_unaligned());
            p_var2 = p_var2.wrapping_add(1);
            p_var3 = p_var3.wrapping_add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
        let mut p_var2: *mut i32 = &raw mut DAT_00758e9e;
        let mut i_var1: i32 = 0x10;
        while i_var1 != 0 {
            p_var2.write_unaligned(0);
            p_var2 = p_var2.wrapping_add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
    }
}
