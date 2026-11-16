// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebc5: u32;
    static mut DAT_00acebd1: u32;
    static mut DAT_00acebd5: u32;
    static mut DAT_00acebd9: u32;
    static mut DAT_00acebdd: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn LAB_004cdfd0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdf60(param_1: u32) -> u32 {
    let mut local_50: [u32; 0x14] = [0; 0x14];
    // SAFETY: the globals are the game's own memory; DAT_00acebc5 holds a
    // pointer to an object whose slot at +0x34 is a stdcall function pointer,
    // called as the original does with a pointer to our local buffer.
    unsafe {
        local_50[0] = 0x50;
        local_50[6] = DAT_00acebd1;
        local_50[7] = DAT_00acebd5;
        local_50[8] = DAT_00acebd9;
        local_50[9] = DAT_00acebdd;
        let obj = DAT_00acebc5;
        let slot = (obj.wrapping_add(0x34) as usize) as *const u32;
        let target = slot.read_unaligned();
        let f: unsafe extern "stdcall" fn(
            u32,
            *mut u32,
            u32,
            *const core::ffi::c_void,
            u32,
            u32,
        ) -> i32 = core::mem::transmute((target as usize) as *const core::ffi::c_void);
        let i_var2 = f(
            obj,
            local_50.as_mut_ptr(),
            0,
            LAB_004cdfd0 as *const core::ffi::c_void,
            param_1,
            1,
        );
        (if i_var2 == 0 { 0xFFFF_FFFFu32 } else { 0 }) & 0x665028
    }
}
