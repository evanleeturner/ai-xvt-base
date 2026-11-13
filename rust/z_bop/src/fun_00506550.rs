// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b82: i32;
    static mut DAT_00aa7b86: i32;
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b8a: i32;
    static mut DAT_00aa7b8e: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa6cfc: i32;
}

unsafe fn fill(lo: i32, hi: i32, param_3: i32, param_4: u16, mode: i32) {
    // SAFETY: the globals are the game's own; the destination is the game's
    // surface memory, written where the original writes.
    unsafe {
        let pitch = DAT_00aa7b13;
        let mut p = (pitch
            .wrapping_mul(lo)
            .wrapping_add(param_3.wrapping_shl(mode as u32 & 0x1f))
            .wrapping_add(DAT_00aa6cfc)) as usize as *mut u8;
        if mode == 0 {
            if lo <= hi {
                let mut n = hi.wrapping_sub(lo).wrapping_add(1);
                loop {
                    p.write(param_4 as u8);
                    p = p.byte_add(DAT_00aa7b13 as isize as usize);
                    n = n.wrapping_sub(1);
                    if n == 0 {
                        break;
                    }
                }
            }
        } else if mode == 1 && lo <= hi {
            let mut n = hi.wrapping_sub(lo).wrapping_add(1);
            loop {
                p.cast::<u16>().write_unaligned(param_4);
                p = p.byte_add((DAT_00aa7b13 & 0xffff_fffeu32 as i32) as isize as usize);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00506550(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: reads of the game's own globals.
    unsafe {
        if !(DAT_00aa7b82 <= param_3 && param_3 <= DAT_00aa7b86) {
            return;
        }
        let mode = DAT_00aa7b1f >> 4;
        if param_2 < param_1 {
            let a = if DAT_00aa7b8a <= param_2 {
                true
            } else {
                param_2 = DAT_00aa7b8a;
                DAT_00aa7b8a < param_1
            };
            if !a {
                return;
            }
            let b = if param_1 <= DAT_00aa7b8e {
                true
            } else {
                param_1 = DAT_00aa7b8e;
                param_2 < DAT_00aa7b8e
            };
            if !b {
                return;
            }
            fill(param_2, param_1, param_3, param_4, mode);
        } else {
            let a = if param_2 <= DAT_00aa7b8e {
                true
            } else {
                param_2 = DAT_00aa7b8e;
                param_1 <= DAT_00aa7b8e
            };
            if !a {
                return;
            }
            let b = if DAT_00aa7b8a <= param_1 {
                true
            } else {
                param_1 = DAT_00aa7b8a;
                DAT_00aa7b8a <= param_2
            };
            if !b {
                return;
            }
            fill(param_1, param_2, param_3, param_4, mode);
        }
    }
}
