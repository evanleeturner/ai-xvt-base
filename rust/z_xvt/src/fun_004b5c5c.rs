// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1c4: *mut core::ffi::c_void;
    static mut DAT_0052f204: i32;
    static mut DAT_0064ee60: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bbe0();
    fn FID_conflict__memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5c5c(param_1: *mut core::ffi::c_void, param_2: i32) -> u32 {
    // SAFETY: the globals and callees are the game's own; the pointers are
    // read and the buffers copied exactly where the original does.
    unsafe {
        let limit = DAT_0052f1c4.byte_add(0x18).cast::<u32>().read_unaligned();
        if limit < (DAT_0052f204.wrapping_add(param_2)) as u32 {
            FUN_0045bbe0();
            0
        } else {
            if param_1 != DAT_0064ee60 {
                FID_conflict__memcpy(DAT_0064ee60, param_1, (param_2 << 5) as u32);
            }
            DAT_0064ee60 = (DAT_0064ee60 as i32).wrapping_add(param_2.wrapping_mul(0x20))
                as *mut core::ffi::c_void;
            DAT_0052f204 = DAT_0052f204.wrapping_add(param_2);
            1
        }
    }
}
