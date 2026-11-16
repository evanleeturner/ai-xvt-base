// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b6f4: *mut core::ffi::c_void;
    static mut DAT_0081bcf0: i8;
    static mut DAT_0081b848: *mut *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(param_1: *const i8, param_2: *const i8) -> i32;
    fn FID_conflict__remove(param_1: *mut i8);
    fn FUN_004c0090(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c01d0() -> i32 {
    // SAFETY: all globals and callees belong to the game; the pointers
    // are read and written exactly where the original does.
    unsafe {
        if !DAT_0081b6f4.is_null() && DAT_0081bcf0 != 0 && !DAT_0081b848.is_null() {
            let mut pu_var2: *mut *mut core::ffi::c_void =
                DAT_0081b848.read_unaligned() as *mut *mut core::ffi::c_void;
            let mut local_4: i32 = 0;
            let mut i_var1: i32 = DAT_0081b6f4 as usize as i32;
            let mut found = false;
            loop {
                if i_var1 == 0 || (DAT_0081b848.add(1).read_unaligned() as usize as i32) <= local_4
                {
                    break;
                }
                i_var1 = __strcmpi(
                    (DAT_0081b6f4 as usize as i32).wrapping_add(local_4.wrapping_mul(0xe)) as usize
                        as *const i8,
                    &raw mut DAT_0081bcf0 as *const i8,
                );
                if i_var1 == 0 {
                    found = true;
                    break;
                }
                pu_var2 = pu_var2.add(1).read_unaligned() as *mut *mut core::ffi::c_void;
                local_4 = local_4.wrapping_add(1);
                i_var1 = local_4
                    .wrapping_mul(0xe)
                    .wrapping_add(DAT_0081b6f4 as usize as i32);
            }
            if found {
                FID_conflict__remove(pu_var2.read_unaligned() as *mut i8);
            }
        }
        let mut p = (&raw mut DAT_0081bcf0) as *mut u8;
        let mut i_var1: i32 = 0xf7ce;
        while i_var1 != 0 {
            p.cast::<u32>().write_unaligned(0);
            p = p.add(4);
            i_var1 = i_var1.wrapping_sub(1);
        }
        p.cast::<u16>().write_unaligned(0);
        let mut local_4: i32 = 0;
        FUN_004c0090(&raw mut local_4);
        1
    }
}
