// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a1051c: i8;
    static mut DAT_00a5faa4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042d040(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042d020() {
    // SAFETY: the globals and the callee are the game's own; the pointer
    // walks the game's record array in 0x145-byte steps, as the original does.
    unsafe {
        let end = &raw mut DAT_00a5faa4;
        let mut p_var1 = &raw mut DAT_00a1051c;
        loop {
            FUN_0042d040(p_var1);
            p_var1 = p_var1.wrapping_byte_add(0x145);
            if p_var1 >= end {
                break;
            }
        }
    }
}
