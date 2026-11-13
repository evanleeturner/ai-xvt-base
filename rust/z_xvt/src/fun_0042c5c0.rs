// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005373d0: i32;
    static mut _DAT_005373d4: i32;
    static mut DAT_005373d0: i32;
    static mut DAT_00732920: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004310c0(param_1: *mut u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042c5c0(param_1: i32) {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the indexed write lands where the original's array store does.
    unsafe {
        _DAT_005373d0 = 3;
        _DAT_005373d4 = param_1;
        FUN_004310c0((&raw mut DAT_005373d0).cast::<u32>(), 8);
        (&raw mut DAT_00732920)
            .wrapping_offset(param_1 as isize)
            .write(0);
    }
}
