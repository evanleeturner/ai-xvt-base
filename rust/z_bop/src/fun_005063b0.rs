// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b8a: i32;
    static mut DAT_00aa7b8e: i32;
    static mut DAT_00aa7b82: i32;
    static mut DAT_00aa7b86: i32;
    static mut DAT_00aa6cfc: u32;
}

unsafe fn fill_span(mut p: *mut u8, start: i32, end: i32, mode: i32, dw: u32, b: u8, w: u16) {
    let count = (end as u32).wrapping_sub(start as u32).wrapping_add(1);
    if mode == 0 {
        let mut n = count >> 2;
        // SAFETY: the destination is the game's own surface memory.
        unsafe {
            while n != 0 {
                p.cast::<u32>().write_unaligned(dw);
                p = p.add(4);
                n -= 1;
            }
            let mut r = count & 3;
            while r != 0 {
                *p = b;
                p = p.add(1);
                r -= 1;
            }
        }
        return;
    }
    if mode != 1 {
        return;
    }
    let mut c = count as i32;
    if 0 < c {
        // SAFETY: the destination is the game's own surface memory.
        unsafe {
            while c != 0 {
                p.cast::<u16>().write_unaligned(w);
                p = p.add(2);
                c = c.wrapping_sub(1);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_005063b0(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals are the game's own memory.
    let (g1f, g13, g8a, g8e, g82, g86, base) = unsafe {
        (
            DAT_00aa7b1f,
            DAT_00aa7b13,
            DAT_00aa7b8a,
            DAT_00aa7b8e,
            DAT_00aa7b82,
            DAT_00aa7b86,
            DAT_00aa6cfc,
        )
    };
    let i_var2 = g1f >> 4;
    if g8a <= param_3 && param_3 <= g8e {
        let u_var1 = param_4 as u8;
        let mut dw = u_var1 as u32;
        dw = (dw << 8) | dw;
        dw = (dw << 16) | dw;
        let ok;
        let start;
        let end;
        if param_2 < param_1 {
            let a = if g82 <= param_2 {
                true
            } else {
                param_2 = g82;
                g82 < param_1
            };
            ok = a
                && (if param_1 <= g86 {
                    true
                } else {
                    param_1 = g86;
                    param_2 < g86
                });
            start = param_2;
            end = param_1;
        } else {
            let a = if param_2 <= g86 {
                true
            } else {
                param_2 = g86;
                param_1 <= g86
            };
            ok = a
                && (if g82 <= param_1 {
                    true
                } else {
                    param_1 = g82;
                    g82 <= param_2
                });
            start = param_1;
            end = param_2;
        }
        if ok {
            let addr = (start as u32)
                .wrapping_shl((i_var2 as u8 & 0x1f) as u32)
                .wrapping_add((g13 as u32).wrapping_mul(param_3 as u32))
                .wrapping_add(base);
            // SAFETY: the address points into the game's surface memory.
            unsafe {
                fill_span(addr as *mut u8, start, end, i_var2, dw, u_var1, param_4);
            }
        }
    }
}
