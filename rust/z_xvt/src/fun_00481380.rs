// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703ca0: i8;
    static mut DAT_007091a0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "strncmp"]
    fn _strncmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481380(param_1: *mut i8) -> i32 {
    // SAFETY: the globals and strncmp are the game's own; param_1 points at
    // the caller's string, compared as the original does.
    unsafe {
        let mut str1: *const i8 = &raw mut DAT_00703ca0 as *const i8;
        let end: *const i8 = &raw mut DAT_007091a0 as *const i8;
        let mut i_var2: i32 = 0;
        loop {
            let i_var1 = _strncmp(str1, param_1 as *const i8, 0x50);
            if i_var1 == 0 {
                return i_var2;
            }
            str1 = str1.byte_add(0x55);
            i_var2 = i_var2.wrapping_add(1);
            if !(str1 < end) {
                break;
            }
        }
        0
    }
}
