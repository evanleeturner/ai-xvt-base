// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1ff8: i32;
    static mut DAT_009a73d8: i8;
    static mut DAT_009a73b0: u16;
    static mut DAT_0099f930: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433390(a: u32, b: u16, c: u16) -> u8;
    fn FUN_00403540(a: u32, b: u32);
}

#[inline(always)]
unsafe fn rd8(addr: i32) -> i8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as u32 as usize as *const i8).read() }
}

#[inline(always)]
unsafe fn rd16(addr: i32) -> i16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as u32 as usize as *const i16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00461460(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u32,
) {
    // SAFETY: the globals and callees are the game's own; the pointers read
    // are the game's records, accessed at the offsets the original uses.
    unsafe {
        let mut u_var7: u32 = DAT_00a07cec as u32;
        if (u_var7 as i32) < DAT_009a1fe0 {
            loop {
                let i_var4: i32 = DAT_009a1fe8;
                let u_var8: u32 = u_var7 & 0xffff;
                let idx = u_var8.wrapping_mul(2).wrapping_add(param_6 & 0xffff);
                (&raw mut DAT_0099f930)
                    .cast::<u8>()
                    .add(idx as usize)
                    .write(0);
                let rec = i_var4.wrapping_add(u_var8.wrapping_mul(0x23) as i32);
                if rd8(rec.wrapping_add(3)) != 0 {
                    let b1 = FUN_00433390(u_var7, param_1, param_2);
                    let mut u_var6: u16 = b1 as u16;
                    let b2 = FUN_00433390(u_var7, param_4, param_5);
                    let u_var3: u16 = b2 as u16;
                    if param_3 == 1 {
                        u_var6 |= u_var3;
                    } else {
                        u_var6 &= u_var3;
                    }
                    if u_var6 != 0 {
                        let mut pass = false;
                        if DAT_009a73d8 == 0 {
                            pass = true;
                        } else {
                            let a = rd32(
                                DAT_009a1fe8
                                    .wrapping_add(0x1f)
                                    .wrapping_add(u_var8.wrapping_mul(0x23) as i32),
                            );
                            let b = rd32(a.wrapping_add(0xa9));
                            let s = rd16(b.wrapping_add(0x1e5));
                            if s != 0 {
                                pass = true;
                            }
                        }
                        if pass {
                            let r: i32 = 'r: {
                                if (u_var7 as i16) == -1 {
                                    break 'r 0;
                                }
                                let rec2 =
                                    DAT_009a1fe8.wrapping_add(u_var8.wrapping_mul(0x23) as i32);
                                let tmp_a = rd32(rec2.wrapping_add(0x1f));
                                if tmp_a == 0 {
                                    let b = (rd8(rec2.wrapping_add(3)) == 0) as i32;
                                    break 'r 1 - b;
                                }
                                if rd8(rec2.wrapping_add(3)) == 0 {
                                    break 'r 0;
                                }
                                let iv = rd32(tmp_a.wrapping_add(0xa9));
                                if iv == 0 {
                                    break 'r 1;
                                }
                                if rd8(iv.wrapping_add(0x5e)) == 0x15
                                    && rd8(iv.wrapping_add(0x5f)) != 0
                                {
                                    break 'r 0;
                                }
                                let c1 = rd8(iv.wrapping_add(7));
                                if c1 == 1 || c1 == 3 || c1 == 4 {
                                    break 'r 0;
                                }
                                if rd8(iv.wrapping_add(0x23d)) == 3
                                    && rd8(iv.wrapping_add(0x241)) != 0
                                {
                                    FUN_00403540(u_var8, DAT_009a73b0 as u32);
                                    let c2 = rd8((DAT_009a73b0 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(2)
                                        .wrapping_add(DAT_009a1fe8 as u32)
                                        as i32);
                                    if c2 == 0 || c2 == 1 || c2 == 2 {
                                        if !(DAT_009a1ff8 < 0x8001) {
                                            break 'r 0;
                                        }
                                    } else if !(DAT_009a1ff8 < 0x10001) {
                                        break 'r 0;
                                    }
                                }
                                // LAB_00461620
                                let i_var5 =
                                    DAT_009a1fe8.wrapping_add(u_var8.wrapping_mul(0x23) as i32);
                                let t = rd32(i_var5.wrapping_add(0x1b));
                                if t == -1 {
                                    let ta = rd32(i_var5.wrapping_add(0x1f));
                                    let tb = rd32(ta.wrapping_add(0xa9));
                                    let ua = rd32(tb.wrapping_add(0xcb)) as u32;
                                    let ub = rd32(iv.wrapping_add(0xc3)) as u32;
                                    let b = (ua < ub) as i32;
                                    break 'r 1 - b;
                                }
                                1
                            };
                            if r != 0 {
                                (&raw mut DAT_0099f930)
                                    .cast::<u8>()
                                    .add(idx as usize)
                                    .write(1);
                            }
                        }
                    }
                }
                let u_var6: u16 = (u_var7 as u16).wrapping_add(1);
                u_var7 = (u_var7 & 0xffff0000) | (u_var6 as u32);
                if !((u_var6 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
    }
}
