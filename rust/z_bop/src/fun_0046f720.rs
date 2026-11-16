// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994fc: i32;
    static mut DAT_00999538: u8;
    static mut DAT_00999570: i32;
    static mut DAT_00999560: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046f660(param_1: i32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046f720(param_1: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    // SAFETY: the globals and the callee are the game's own memory; the
    // table pointer is only read at the strides the original uses.
    unsafe {
        if DAT_009994fc == 1 {
            return (&raw mut DAT_00999538) as *mut core::ffi::c_void;
        }
        let mut i_var2: i32 = 0;
        let mut p_var1: *mut core::ffi::c_void = DAT_009994fc as usize as *mut core::ffi::c_void;
        if DAT_009994fc > 0 {
            let mut pi_var3: *mut i32 = &raw mut DAT_00999570;
            loop {
                p_var1 = FUN_0046f660(pi_var3.read_unaligned());
                if p_var1 == param_1 {
                    break;
                }
                pi_var3 = pi_var3.wrapping_add(10);
                i_var2 = i_var2.wrapping_add(1);
                if DAT_009994fc <= i_var2 {
                    return p_var1;
                }
            }
            p_var1 = ((&raw mut DAT_00999560) as *mut u8)
                .wrapping_byte_offset(i_var2.wrapping_mul(0x28) as isize)
                as *mut core::ffi::c_void;
        }
        p_var1
    }
}
