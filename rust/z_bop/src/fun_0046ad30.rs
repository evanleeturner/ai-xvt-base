// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005255b8: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046ad30(param_1: *mut u8) -> i32 {
    let mut i_var3: i32 = 0;
    // SAFETY: the table is the game's own memory, made of 0x52-byte entries
    // that end with an empty-string entry; param_1 points at a
    // NUL-terminated string. Reads stop at the first difference or NUL.
    unsafe {
        let base = (&raw mut DAT_005255b8) as *const u8;
        if *base != 0 {
            let mut entry = base;
            loop {
                let mut j: usize = 0;
                let equal;
                loop {
                    let a = *entry.add(j);
                    let b = *param_1.add(j);
                    if a != b {
                        equal = false;
                        break;
                    }
                    if a == 0 {
                        equal = true;
                        break;
                    }
                    j += 1;
                }
                if equal {
                    return i_var3;
                }
                entry = entry.byte_add(0x52);
                i_var3 = i_var3.wrapping_add(1);
                if *entry == 0 {
                    break;
                }
            }
        }
    }
    i_var3
}
