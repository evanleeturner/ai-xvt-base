// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0adf0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042d600(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9380(param_1: i32) -> u8 {
    if 3 < param_1 && param_1 < 0x346 {
        // SAFETY: the global table and the callee are the game's own; the
        // index is bounded above, so the offset stays inside the table.
        unsafe {
            let base = (&raw mut DAT_00a0adf0).cast::<i8>();
            let p = base.byte_add(param_1.wrapping_mul(0x18) as usize);
            return FUN_0042d600(p.cast::<core::ffi::c_void>());
        }
    }
    0
}
