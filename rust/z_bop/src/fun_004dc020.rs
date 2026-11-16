// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527004: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004cc790(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: u32);
    fn FUN_004cc590(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dc020(param_1: *mut i8, param_2: *mut u32, param_3: u32) {
    if param_2.is_null() {
        return;
    }
    // SAFETY: the callees and the global are the game's own; param_2 points at
    // at least 0x60b readable bytes, copied into a local buffer as the original does.
    unsafe {
        let p_fvar1 = FUN_004cc4a0(param_1, &raw mut DAT_00527004);
        if !p_fvar1.is_null() {
            let mut local_60c: [u32; 0x183] = [0; 0x183];
            core::ptr::copy_nonoverlapping(
                param_2 as *const u8,
                local_60c.as_mut_ptr() as *mut u8,
                0x60b,
            );
            local_60c[0] = param_3;
            FUN_004cc790(
                p_fvar1,
                local_60c.as_mut_ptr() as *mut core::ffi::c_void,
                0x60b,
            );
            let inner = param_2.read_unaligned();
            FUN_004cc790(p_fvar1, inner as usize as *mut core::ffi::c_void, param_3);
            FUN_004cc590(p_fvar1);
        }
    }
}
