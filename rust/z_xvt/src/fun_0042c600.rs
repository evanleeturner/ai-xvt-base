// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT__005373d4: u32;
    static mut DAT__005373d0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004310c0(param_1: *mut u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042c600(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; the address of
    // the global is passed as the original does.
    unsafe {
        DAT__005373d4 = param_1;
        DAT__005373d0 = 0x15;
        FUN_004310c0(&raw mut DAT__005373d0, 8);
    }
}
