// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1ff8: i32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73d8: i8;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_00a07cec: u16;
    static mut DAT_00521f2e: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432fa0(a: u16, b: u16, c: u16) -> u16;
    fn FUN_00403540(a: u32, b: u32);
}

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    // SAFETY: the address is a game-memory address used as in the original.
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    // SAFETY: the address is a game-memory address used as in the original.
    unsafe { (addr as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    // SAFETY: the address is a game-memory address used as in the original.
    unsafe { (addr as *const u32).read_unaligned() }
}

unsafe fn check(idx: u16) -> i32 {
    // SAFETY: all reads are of the game's own memory, in the original's order.
    unsafe {
        let u_var9 = idx as u32;
        if idx == 0xffff {
            return 0;
        }
        let mut p = (DAT_009a1fe8 as u32).wrapping_add(u_var9.wrapping_mul(0x23));
        let ptr1 = rd32(p.wrapping_add(0x1f));
        if ptr1 == 0 {
            let b10 = (rd8(p.wrapping_add(3)) == 0) as u32;
            return 1u32.wrapping_sub(b10) as i32;
        }
        if rd8(p.wrapping_add(3)) == 0 {
            return 0;
        }
        let q = rd32(ptr1.wrapping_add(0xa9));
        if q == 0 {
            return 1;
        }
        if rd8(q.wrapping_add(0x5e)) == 0x15 && rd8(q.wrapping_add(0x5f)) != 0 {
            return 0;
        }
        let c = rd8(q.wrapping_add(7));
        if c == 1 || c == 3 || c == 4 {
            return 0;
        }
        if rd8(q.wrapping_add(0x23d)) == 3 && rd8(q.wrapping_add(0x241)) != 0 {
            FUN_00403540(u_var9, DAT_009a73b0 as u32);
            let c3 = rd8((DAT_009a73b0 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(2)
                .wrapping_add(DAT_009a1fe8 as u32));
            if c3 == 0 || c3 == 1 || c3 == 2 {
                if DAT_009a1ff8 >= 0x8001 {
                    return 0;
                }
            } else if DAT_009a1ff8 >= 0x10001 {
                return 0;
            }
        }
        p = (DAT_009a1fe8 as u32).wrapping_add(u_var9.wrapping_mul(0x23));
        let v1 = rd32(p.wrapping_add(0x1b));
        if v1 as i32 == -1 {
            let ptr1 = rd32(p.wrapping_add(0x1f));
            let t = rd32(ptr1.wrapping_add(0xa9));
            let a = rd32(t.wrapping_add(0xcb));
            let b = rd32(q.wrapping_add(0xc3));
            let b10 = (a < b) as u32;
            return 1u32.wrapping_sub(b10) as i32;
        }
        1
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045dfa0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> i32 {
    // SAFETY: globals and functions are the game's own; pointers are read
    // where the original reads them.
    unsafe {
        let mut local_6: u16 = DAT_00a07cec;
        let mut i_var1: i32 = 0;
        if (local_6 as i32) < DAT_009a1fe0 {
            loop {
                let u_var9 = local_6 as u32;
                let i_var7 = (DAT_009a1fe8 as u32).wrapping_add(u_var9.wrapping_mul(0x23));
                if rd8(i_var7.wrapping_add(3)) != 0 {
                    let u_var5 = rd8(i_var7.wrapping_add(0x16)) as u16;
                    if DAT_009a73c0 != u_var5 {
                        let mut u_var4 = FUN_00432fa0(u_var5, param_1, param_2);
                        let u_var5b = FUN_00432fa0(u_var5, param_4, param_5);
                        if param_3 == 1 {
                            u_var4 |= u_var5b;
                        } else {
                            u_var4 &= u_var5b;
                        }
                        if u_var4 != 0 {
                            let res = check(local_6);
                            if res != 0 {
                                let i7 = rd32(
                                    (DAT_009a1fe8 as u32)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(u_var9.wrapping_mul(0x23)),
                                );
                                let i8 = rd32(i7.wrapping_add(0xa9));
                                if DAT_009a73d8 == 0 {
                                    i_var1 = i_var1.wrapping_add(1);
                                } else if rd16(i8.wrapping_add(0x1e5)) != 0 {
                                    if DAT_009a73d8 == 0 || rd8(i8.wrapping_add(0x1eb)) == 0 {
                                        i_var1 = i_var1.wrapping_add(1);
                                    } else {
                                        let r = rd32(
                                            (DAT_009a73b0 as u32)
                                                .wrapping_mul(0x23)
                                                .wrapping_add(0x1f)
                                                .wrapping_add(DAT_009a1fe8 as u32),
                                        );
                                        if rd8(r.wrapping_add(0x89)) != rd8(i7.wrapping_add(0x89)) {
                                            i_var1 = i_var1.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                local_6 = local_6.wrapping_add(1);
                if !((local_6 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        let mut u_var5: u16 = DAT_00a07ce8 as u16;
        if (DAT_00a07ce8 & 0xffff) < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            loop {
                let u_var9 = u_var5 as u32;
                let i_var7 = (u_var9.wrapping_mul(0x23)).wrapping_add(DAT_009a1fe8 as u32);
                let b2 = rd8(i_var7.wrapping_add(3));
                if b2 != 0 {
                    let flag = (&raw mut DAT_00521f2e)
                        .cast::<u8>()
                        .add((b2 as usize).wrapping_mul(0x1a))
                        .read();
                    if (flag & 2) != 0 {
                        let b = rd8(i_var7.wrapping_add(0x16)) as u16;
                        let mut u_var4 = FUN_00432fa0(b, param_1, param_2);
                        let u_var6 = FUN_00432fa0(b, param_4, param_5);
                        if param_3 == 1 {
                            u_var4 |= u_var6;
                        } else {
                            u_var4 &= u_var6;
                        }
                        if u_var4 != 0 && check(u_var5) != 0 {
                            i_var1 = i_var1.wrapping_add(1);
                        }
                    }
                }
                u_var5 = u_var5.wrapping_add(1);
                if !((u_var5 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }
        if i_var1 == 0 {
            return 0xffff;
        }
        i_var1
    }
}
