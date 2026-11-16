// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a860c: i32;
    static mut DAT_0065865c: i32;
    static mut DAT_006a866c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00491010(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00491150() -> u8 {
    // SAFETY: the globals and callee are the game's own; the pointer walks the
    // game's table of 12-byte entries from DAT_006a860c up to DAT_006a866c.
    unsafe {
        let mut b_var2: u8 = 1;
        let mut pi_var3: *mut i32 = &raw mut DAT_006a860c;
        let end: *mut i32 = &raw mut DAT_006a866c;
        loop {
            let v = pi_var3.read_unaligned();
            if v != -1 {
                let addr =
                    (&raw mut DAT_0065865c as usize).wrapping_add(v.wrapping_mul(0x145) as usize);
                let b_var1 = FUN_00491010(addr as *mut i8);
                b_var2 &= b_var1;
            }
            pi_var3 = pi_var3.wrapping_add(3);
            if pi_var3 >= end {
                break;
            }
        }
        b_var2
    }
}
