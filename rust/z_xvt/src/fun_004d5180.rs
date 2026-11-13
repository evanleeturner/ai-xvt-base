// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007596b7: u32;
    static mut DAT_007596bb: u32;
    static mut DAT_007596bf: u32;
    static mut DAT_007596c3: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut u32, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d5180(param_1: *mut u32) {
    // SAFETY: the globals and the callee are the game's own; the globals are
    // read by value in the original's argument order.
    unsafe {
        let a = DAT_007596b7;
        let b = DAT_007596bf;
        let c = DAT_007596bb;
        let d = DAT_007596c3;
        FUN_004dcfd0(param_1, a, b, c, d);
    }
}
