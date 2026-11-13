// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5cb0: u32;
    static mut DAT_00aa5df0: u32;
    static mut DAT_00b69e30: u32;
    static mut DAT_00aa6af0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cef70(param_1: u32, param_2: *mut u32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f46c0() -> u32 {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the loops clear the same word counts from the same start addresses.
    unsafe {
        let mut p: *mut u32 = &raw mut DAT_00aa5cb0;
        let mut i: i32 = 0x50;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        p = &raw mut DAT_00aa5df0;
        i = 8;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        if DAT_00b69e30 != 2 {
            DAT_00aa6af0 = 0x54;
            FUN_004cef70(0, &raw mut DAT_00aa6af0, 4);
        }
        1
    }
}
