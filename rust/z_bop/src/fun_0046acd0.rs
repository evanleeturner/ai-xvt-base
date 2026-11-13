// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525280: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046acd0(param_1: *mut u8) -> i32 {
    let mut i_var3: i32 = 0;
    // SAFETY: the table is the game's own memory, made of 0x52-byte entries
    // ending at an entry whose first byte is zero; param_1 is a
    // NUL-terminated string supplied by the caller.
    unsafe {
        let base: *mut u8 = &raw mut DAT_00525280;
        if base.read() != 0 {
            let mut entry: *mut u8 = base;
            loop {
                let mut a = entry;
                let mut b = param_1;
                let equal;
                loop {
                    let x = a.read();
                    let y = b.read();
                    if x != y {
                        equal = false;
                        break;
                    }
                    if x == 0 {
                        equal = true;
                        break;
                    }
                    let x2 = a.add(1).read();
                    let y2 = b.add(1).read();
                    if x2 != y2 {
                        equal = false;
                        break;
                    }
                    a = a.add(2);
                    b = b.add(2);
                    if x2 == 0 {
                        equal = true;
                        break;
                    }
                }
                if equal {
                    return i_var3;
                }
                entry = entry.add(0x52);
                i_var3 = i_var3.wrapping_add(1);
                if entry.read() == 0 {
                    break;
                }
            }
        }
    }
    i_var3
}
