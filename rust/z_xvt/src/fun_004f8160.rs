// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759657: i32;
    static mut DAT_007596bf: i32;
    static mut DAT_007596c3: i32;
    static mut DAT_007596b7: i32;
    static mut DAT_007596bb: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe fn fill(mut p: *mut u8, a: i32, b: i32, mode: i32, byte: u8, pat: u32, word: u16) {
    // SAFETY: p points into the game's own surface memory, written where the original does.
    unsafe {
        if mode == 0 {
            let cnt = (b as u32).wrapping_sub(a as u32).wrapping_add(1);
            let mut n = cnt >> 2;
            while n != 0 {
                p.cast::<u32>().write_unaligned(pat);
                p = p.byte_add(4);
                n -= 1;
            }
            let mut r = cnt & 3;
            while r != 0 {
                p.write(byte);
                p = p.byte_add(1);
                r -= 1;
            }
            return;
        }
        if mode != 1 {
            return;
        }
        let mut n = (b as u32).wrapping_sub(a as u32).wrapping_add(1) as i32;
        if 0 < n {
            while n != 0 {
                p.cast::<u16>().write_unaligned(word);
                p = p.byte_add(2);
                n = n.wrapping_sub(1);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f8160(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals are the game's own memory; reads are unaligned-safe
    // and the fill writes into the game's surface buffer.
    unsafe {
        let g_57 = (&raw const DAT_00759657).read_unaligned();
        let i_var2 = g_57 >> 4;
        let g_bf = (&raw const DAT_007596bf).read_unaligned();
        let g_c3 = (&raw const DAT_007596c3).read_unaligned();
        if g_bf <= param_3 && param_3 <= g_c3 {
            let u_var1 = param_4 as u8;
            let pat = (u_var1 as u32).wrapping_mul(0x01010101);
            let g_b7 = (&raw const DAT_007596b7).read_unaligned();
            let g_bb = (&raw const DAT_007596bb).read_unaligned();
            let g_4b = (&raw const DAT_0075964b).read_unaligned();
            let g_30 = DAT_00758830;
            let sh = (i_var2 & 0x1f) as u32;
            if param_2 < param_1 {
                let c1 = if g_b7 <= param_2 {
                    true
                } else {
                    param_2 = g_b7;
                    g_b7 < param_1
                };
                if c1 {
                    let c2 = if param_1 <= g_bb {
                        true
                    } else {
                        param_1 = g_bb;
                        param_2 < g_bb
                    };
                    if c2 {
                        let addr = (param_2 as u32)
                            .wrapping_shl(sh)
                            .wrapping_add((g_4b as u32).wrapping_mul(param_3 as u32))
                            .wrapping_add(g_30 as u32);
                        fill(
                            addr as usize as *mut u8,
                            param_2,
                            param_1,
                            i_var2,
                            u_var1,
                            pat,
                            param_4,
                        );
                    }
                }
            } else {
                let c1 = if param_2 <= g_bb {
                    true
                } else {
                    param_2 = g_bb;
                    param_1 <= g_bb
                };
                if c1 {
                    let c2 = if g_b7 <= param_1 {
                        true
                    } else {
                        param_1 = g_b7;
                        g_b7 <= param_2
                    };
                    if c2 {
                        let addr = (param_1 as u32)
                            .wrapping_shl(sh)
                            .wrapping_add((g_4b as u32).wrapping_mul(param_3 as u32))
                            .wrapping_add(g_30 as u32);
                        fill(
                            addr as usize as *mut u8,
                            param_1,
                            param_2,
                            i_var2,
                            u_var1,
                            pat,
                            param_4,
                        );
                    }
                }
            }
        }
    }
}
