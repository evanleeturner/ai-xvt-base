// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004443d0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00444560() {
    // SAFETY: the globals and the callee are the game's own; the pointer walks
    // the game's record table in 0x5bd-byte steps, as the original does.
    unsafe {
        let mut i_var2: i32 = 0;
        let mut pc_var1: *const i8 = &raw mut DAT_0071b740;
        let end: *const i8 = &raw mut DAT_0071e528;
        loop {
            if pc_var1.read() != 0 {
                FUN_004443d0(i_var2);
            }
            pc_var1 = pc_var1.byte_add(0x5bd);
            i_var2 = i_var2.wrapping_add(1);
            if !(pc_var1 < end) {
                break;
            }
        }
    }
}
