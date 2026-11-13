// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a604cc: i32;
    static mut DAT_00a1051c: i8;
    static mut DAT_00a6052c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042d600(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042d6d0() -> u8 {
    let mut b_var3: u8 = 1;
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the table is walked in 12-byte steps and read where the original does.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_00a604cc;
        let end: *mut i32 = &raw mut DAT_00a6052c;
        loop {
            let v = pi_var2.read_unaligned();
            if v != -1 {
                let base: *mut i8 = &raw mut DAT_00a1051c;
                let p = base.wrapping_offset(v.wrapping_mul(0x145) as isize);
                let b_var1 = FUN_0042d600(p.cast::<core::ffi::c_void>());
                b_var3 &= b_var1;
            }
            pi_var2 = pi_var2.wrapping_add(3);
            if pi_var2 >= end {
                break;
            }
        }
    }
    b_var3
}
