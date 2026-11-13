// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0052c05c: [i8; 1];
    static mut DAT_0052c050: [i8; 1];
    static mut DAT_0052c044: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004daba0(param_1: *mut i32) {
    let mut local_20: [i32; 8] = [0; 8];
    // SAFETY: param_1 points at eight caller ints; the globals, format
    // strings and callees are the game's own.
    unsafe {
        let mut k = 0usize;
        while k < 8 {
            local_20[k] = param_1.add(k).read_unaligned();
            k += 1;
        }
        let mut i_var2: i32 = 0;
        let mut i_var1: i32;
        loop {
            if local_20[i_var2 as usize] == 1 {
                sprintf(
                    &raw mut DAT_00b69d20 as *mut i8,
                    &raw const DAT_0052c05c as *const i8,
                    i_var2.wrapping_add(1),
                );
                FUN_004b5570(&raw mut DAT_00b69d20 as *mut i8, 0, 0);
            }
            i_var1 = local_20[7];
            i_var2 = i_var2.wrapping_add(1);
            if i_var2 >= 8 {
                break;
            }
        }
        local_20[7] = local_20[6];
        local_20[6] = local_20[5];
        local_20[5] = i_var1;
        let mut i: usize = 0;
        loop {
            if local_20[i] == 2 {
                if i != 0 && i != 6 {
                    sprintf(
                        &raw mut DAT_00b69d20 as *mut i8,
                        &raw const DAT_0052c050 as *const i8,
                        (i as i32).wrapping_add(1),
                        local_20[i - 1],
                    );
                    FUN_004b5570(&raw mut DAT_00b69d20 as *mut i8, 0, 0);
                }
                if i != 7 && i != 5 {
                    sprintf(
                        &raw mut DAT_00b69d20 as *mut i8,
                        &raw const DAT_0052c044 as *const i8,
                        (i as i32).wrapping_add(1),
                        local_20[i + 1],
                    );
                    FUN_004b5570(&raw mut DAT_00b69d20 as *mut i8, 0, 0);
                }
            }
            i += 1;
            if i >= 8 {
                break;
            }
        }
    }
}
