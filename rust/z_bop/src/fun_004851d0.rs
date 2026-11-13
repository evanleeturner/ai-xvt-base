// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: *mut c_void;
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009ec468: i8;
    static mut _DAT_00a07be0: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046f720(param_1: *mut c_void) -> *mut c_void;
    fn FUN_00451bf0(param_1: i32, param_2: *mut c_void);
    fn FUN_00450650(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004851d0(param_1: *mut c_void) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table walk reads bytes at the original's stride within its bounds.
    unsafe {
        if DAT_009ecc34 != param_1 {
            let p_var1 = FUN_0046f720(param_1);
            FUN_00451bf0(0, p_var1);
            FUN_00450650(0x17d, DAT_009ecc34 as usize as i32);
            let mut i_var3: i32 = 0;
            let mut pc_var2: *mut i8 = (&raw mut DAT_009e9680).cast::<i8>();
            let end = (&raw mut DAT_009ec468) as usize;
            loop {
                if pc_var2.read() == 1 {
                    i_var3 = i_var3.wrapping_add(1);
                }
                pc_var2 = pc_var2.wrapping_add(0x5bd);
                if (pc_var2 as usize) >= end {
                    break;
                }
            }
            let idx = (DAT_009ecc34 as usize as i32).wrapping_mul(0x5bd);
            let base = (&raw mut DAT_009e9680).cast::<i8>();
            if base.byte_offset(idx as isize).read() == 1 {
                if i_var3 == 1 {
                    FUN_00450650(0x17f, DAT_009ecc34 as usize as i32);
                    return;
                }
                _DAT_00a07be0 = i_var3 as u16;
                FUN_00450650(0x17e, DAT_009ecc34 as usize as i32);
            }
        }
    }
}
