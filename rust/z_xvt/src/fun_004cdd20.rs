// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_007806fa: i32;
    static mut DAT_00780732: i32;
    static mut DAT_0078077a: i32;
    static mut DAT_0078077e: i32;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdd20(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table entries are read and written at the original's offsets.
    unsafe {
        let c_var1 = DAT_00759645;
        if DAT_007806fa == 0 {
            return 0;
        }
        FUN_004d4d90();
        let mut i_var2: i32 = 0;
        let mut b_var4 = DAT_00780732 == 0;
        if 0 < DAT_00780732 {
            let mut p = (&raw mut DAT_0078077a) as *mut u8;
            loop {
                let local_val = p.cast::<i32>().read_unaligned();
                if local_val == param_1 {
                    break;
                }
                p = p.byte_add(0x28);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_00780732) {
                    break;
                }
            }
            b_var4 = i_var2 == DAT_00780732;
        }
        if b_var4 {
            if c_var1 != 0 {
                DAT_00758830 = FUN_004d4d00();
            }
            return 0;
        }
        let one: i32 = 1;
        ((&raw mut DAT_0078077e) as *mut u8)
            .byte_add(i_var2.wrapping_mul(0x28) as usize)
            .cast::<i32>()
            .write_unaligned(one);
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        1
    }
}
