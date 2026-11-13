// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532adc: i32;
    static mut DAT_0064f688: i32;
}

unsafe extern "C" {
    fn memcpy(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d8e80(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the pointer walks the
    // game's table exactly as the original does, and memcpy is the C runtime's.
    unsafe {
        let mut u_var2: u32 = 0;
        if DAT_00532adc != 0 {
            let mut pi_var1: *mut i32 = &raw mut DAT_0064f688;
            loop {
                if pi_var1.read_unaligned() == param_1 {
                    let n = (DAT_00532adc as u32)
                        .wrapping_sub(u_var2)
                        .wrapping_mul(4)
                        .wrapping_sub(4);
                    memcpy(
                        pi_var1 as *mut core::ffi::c_void,
                        pi_var1.add(1) as *const core::ffi::c_void,
                        n,
                    );
                    DAT_00532adc = DAT_00532adc.wrapping_sub(1);
                    return 1;
                }
                pi_var1 = pi_var1.add(1);
                u_var2 = u_var2.wrapping_add(1);
                if !(u_var2 < DAT_00532adc as u32) {
                    break;
                }
            }
        }
        0
    }
}
