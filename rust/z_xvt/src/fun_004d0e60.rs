// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d41: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3570(param_1: *mut i8) -> i32;
    fn FUN_004d0ea0(param_1: *mut core::ffi::c_void, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: the callees and the global are the game's own; the table
    // address is computed from the global and index as the original does.
    unsafe {
        let i_var1 = FUN_004d3570(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let addr = DAT_00758d41
            .wrapping_add(0x40)
            .wrapping_add(i_var1.wrapping_mul(0x44));
        let p = (addr as usize as *const *mut core::ffi::c_void).read_unaligned();
        FUN_004d0ea0(p, param_2, param_3)
    }
}
