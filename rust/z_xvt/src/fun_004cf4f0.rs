// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b5be: i32;
    static mut DAT_00815cf6: i32;
    static mut DAT_0064f070: i32;
    static mut DAT_0064f080: i32;
    static mut DAT_0064f430: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cee90(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf4f0(param_1: i32) -> i32 {
    // SAFETY: the callee and the globals are the game's own memory; the
    // table reads stay where the original reads them.
    unsafe {
        let u_var1 = FUN_004cee90(param_1) as u32;
        let mut i_var4: i32;
        if u_var1 < (DAT_0081b5be as u32) && u_var1 < 0x28 {
            i_var4 = (&raw mut DAT_00815cf6)
                .wrapping_add(u_var1.wrapping_mul(0x8e) as usize)
                .read_unaligned();
        } else {
            i_var4 = 0;
        }
        let mut i_var2: i32 = 0;
        let mut pi_var3: *mut i32 = &raw mut DAT_0064f070;
        let end: *mut i32 = &raw mut DAT_0064f430;
        loop {
            if pi_var3.read_unaligned() == param_1 {
                let v = (&raw mut DAT_0064f080)
                    .wrapping_offset(i_var2.wrapping_mul(6) as isize)
                    .read_unaligned();
                return v.wrapping_add(i_var4);
            }
            pi_var3 = pi_var3.wrapping_add(6);
            i_var2 = i_var2.wrapping_add(1);
            if !(pi_var3 < end) {
                break;
            }
        }
        i_var4
    }
}
