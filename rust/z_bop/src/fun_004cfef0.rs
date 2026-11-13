// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00acebc5: i32;
    static mut DAT_00b53575: i32;
    static mut DAT_00b53579: i32;
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00acf571: i32;
    static mut DAT_00acf581: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970() -> i32;
    fn FUN_004ce130() -> i32;
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cfef0(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table is indexed with the same stride as the original.
    unsafe {
        let c_var2 = DAT_00aa7b0d;
        if DAT_00acebc5 == 0 {
            return 0;
        }
        FUN_004d4970();
        FUN_004ce130();
        let mut i_var3 = DAT_00b53575;
        let mut i_var1 = DAT_00b53579;
        if i_var1 < 0x201 {
            loop {
                if i_var1 < 1 {
                    if c_var2 != 0 {
                        DAT_00aa6cfc = FUN_004d48e0();
                    }
                    return 0;
                }
                let off = i_var3.wrapping_mul(0x84) as isize;
                let a = (&raw mut DAT_00acf571).wrapping_offset(off);
                let b = (&raw mut DAT_00acf581).wrapping_offset(off);
                if a.read_unaligned() != 0 && b.read_unaligned() == param_1 {
                    break;
                }
                i_var3 = i_var3.wrapping_add(1);
                if 0x3ff < i_var3 {
                    i_var3 = 0;
                }
                i_var1 = i_var1.wrapping_sub(1);
            }
            if c_var2 != 0 {
                DAT_00aa6cfc = FUN_004d48e0();
            }
            return 1;
        }
        if c_var2 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        1
    }
}
