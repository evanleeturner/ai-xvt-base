// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c004: i32;
    static mut DAT_00665470: i32;
    static mut DAT_00665474: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__memcpy(
        param_1: *mut core::ffi::c_void,
        param_2: *const core::ffi::c_void,
        param_3: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9bf0(param_1: i32) -> i32 {
    // SAFETY: the globals and memcpy are the game's own; the array at
    // DAT_00665470 is walked and compacted exactly as the original does.
    unsafe {
        let mut u_var2: u32 = 0;
        if DAT_0052c004 != 0 {
            let mut pi_var1: *mut i32 = &raw mut DAT_00665470;
            loop {
                if pi_var1.read() == param_1 {
                    let dst = (&raw mut DAT_00665470).wrapping_add(u_var2 as usize);
                    let src = (&raw mut DAT_00665474).wrapping_add(u_var2 as usize);
                    let n = (DAT_0052c004 as u32)
                        .wrapping_sub(u_var2)
                        .wrapping_mul(4)
                        .wrapping_sub(4);
                    FID_conflict__memcpy(
                        dst.cast::<core::ffi::c_void>(),
                        src.cast_const().cast::<core::ffi::c_void>(),
                        n,
                    );
                    DAT_0052c004 = DAT_0052c004.wrapping_sub(1);
                    return 1;
                }
                pi_var1 = pi_var1.wrapping_add(1);
                u_var2 = u_var2.wrapping_add(1);
                if !(u_var2 < DAT_0052c004 as u32) {
                    break;
                }
            }
        }
        0
    }
}
