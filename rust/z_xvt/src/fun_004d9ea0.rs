// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b740: i32;
    static mut DAT_00532b34: [i8; 0];
    static mut DAT_00532b28: [i8; 0];
    static mut DAT_00532b1c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9ea0(param_1: *mut i32) {
    let mut local_1c: [i32; 7] = [0; 7];
    // SAFETY: param_1 points at seven ints supplied by the caller; the
    // globals, format strings and callees are the game's own.
    unsafe {
        let mut src = param_1;
        for i in 0..7 {
            local_1c[i] = src.read_unaligned();
            src = src.add(1);
        }
        let mut i_var1: i32 = 0;
        loop {
            if local_1c[i_var1 as usize] == 1 {
                _sprintf(
                    &raw mut DAT_0081b740 as *mut i8,
                    (&raw const DAT_00532b34) as *const i8,
                    i_var1.wrapping_add(1),
                );
                FUN_004d0e60(&raw mut DAT_0081b740 as *mut i8, 0, 0);
            }
            i_var1 = i_var1.wrapping_add(1);
            if i_var1 >= 7 {
                break;
            }
        }
        i_var1 = 0;
        let mut idx: usize = 0;
        loop {
            if local_1c[idx] == 2 {
                if idx != 0 && idx != 5 {
                    _sprintf(
                        &raw mut DAT_0081b740 as *mut i8,
                        (&raw const DAT_00532b28) as *const i8,
                        i_var1.wrapping_add(1),
                        local_1c[idx - 1],
                    );
                    FUN_004d0e60(&raw mut DAT_0081b740 as *mut i8, 0, 0);
                }
                if idx != 4 && idx != 6 {
                    _sprintf(
                        &raw mut DAT_0081b740 as *mut i8,
                        (&raw const DAT_00532b1c) as *const i8,
                        i_var1.wrapping_add(1),
                        local_1c[idx + 1],
                    );
                    FUN_004d0e60(&raw mut DAT_0081b740 as *mut i8, 0, 0);
                }
            }
            idx += 1;
            i_var1 = i_var1.wrapping_add(1);
            if idx >= 7 {
                break;
            }
        }
    }
}
