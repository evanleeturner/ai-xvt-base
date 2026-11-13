// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn _fread(buf: *mut core::ffi::c_void, size: u32, count: u32, file: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046adf0(param_1: *mut i8, param_2: i32) -> i32 {
    let mut local_1: i8 = 0;
    // SAFETY: the game's _fread and the caller's buffer are used exactly as the original does.
    unsafe {
        *param_1 = 0;
        loop {
            loop {
                let s = _fread((&raw mut local_1).cast(), 1, 1, param_2);
                if s != 1 {
                    return 1;
                }
                if !(local_1 == 0x20 || local_1 == 9 || local_1 == 10 || local_1 == 0x2c) {
                    break;
                }
            }
            if local_1 != 0x3b {
                break;
            }
            loop {
                let s = _fread((&raw mut local_1).cast(), 1, 1, param_2);
                if s != 1 {
                    return 1;
                }
                if local_1 == 10 {
                    break;
                }
            }
        }
        let mut i_var2: i32 = 1;
        *param_1 = local_1;
        loop {
            let s = _fread((&raw mut local_1).cast(), 1, 1, param_2);
            if !(s == 1 && local_1 != 0x2c && local_1 != 10 && local_1 != 0x20 && local_1 != 9) {
                break;
            }
            *param_1.offset(i_var2 as isize) = local_1;
            i_var2 = i_var2.wrapping_add(1);
        }
        *param_1.offset(i_var2 as isize) = 0;
        1
    }
}
