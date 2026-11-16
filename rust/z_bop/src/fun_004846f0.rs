// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a8c00: u32;
    static mut DAT_009ed228: u32;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut DAT_009e968a: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(param_1: u16) -> u32;
    fn FUN_00435a10(a: u8, b: u8, c: u8) -> u32;
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_00482640(a: u16, b: i32, c: i32) -> u64;
    fn FUN_004034e0(a: u32, b: u32);
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (addr as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (addr as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (addr as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004846f0(param_1: u16, param_2: u16) -> u16 {
    if param_1 == 0xffff {
        return 0xffff;
    }
    // SAFETY: the globals and callees are the game's own; all pointers are
    // the game's records, read where the original reads.
    unsafe {
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_10: u16 = 0xffff;
        let mut local_e: u16 = DAT_00a07cec;
        if (local_e as i32) < DAT_009a1fe0 {
            loop {
                'next: {
                    let u_var3: u32 = local_e as u32;
                    let i_var2: u32 = u_var3.wrapping_mul(0x23);
                    let base: u32 = DAT_009a1fe8 as u32;
                    let rec: u32 = base.wrapping_add(i_var2);
                    if !(rd8(rec.wrapping_add(3)) != 0
                        && local_e != param_1
                        && local_e != param_2
                        && rd8(rec.wrapping_add(2)) != 0x0d)
                    {
                        break 'next;
                    }
                    let mut b_var4: i32 = 0;
                    let t0 = rd32(rec.wrapping_add(0x1f));
                    let i_var5: u32 = rd32(t0.wrapping_add(0xa9));
                    if rd16(i_var5.wrapping_add(0x1e5)) == 0 {
                        break 'next;
                    }
                    if rd8(i_var5.wrapping_add(7)) != 0 {
                        break 'next;
                    }
                    let u6 = FUN_00484f80(local_e);
                    if (u6 as u8) != 0 {
                        break 'next;
                    }
                    let t0 = rd32(base.wrapping_add(0x1b).wrapping_add(i_var2));
                    if (t0 as i32) == -1 {
                        if rd16(i_var5.wrapping_add(0x42)) != param_1 {
                            break 'next;
                        }
                        let c = rd8(i_var5.wrapping_add(0x5e));
                        if c != 0x0c && c != 0x17 {
                            break 'next;
                        }
                        b_var4 = 1;
                    } else {
                        let u_var6: u32 = param_1 as u32;
                        if (u_var6 as i32) < DAT_009a1fe0 {
                            let i_var7: u32 =
                                u_var6.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32);
                            let t0 = rd32(i_var7.wrapping_add(0x1f));
                            let i_var1: u32 = rd32(t0.wrapping_add(0xa9));
                            if rd16(i_var1.wrapping_add(0x6a)) == local_e {
                                let t0 = rd32(i_var7.wrapping_add(0x1b));
                                if (t0 as i32) == -1 {
                                    b_var4 = 1;
                                } else {
                                    let u_var8 = FUN_00435a10(
                                        (DAT_009ed228 >> 24) as u8,
                                        DAT_009ed22c,
                                        DAT_009ed22d,
                                    );
                                    let d = ((u_var8 & 0xffff) as i32)
                                        .wrapping_sub(rd16(i_var1.wrapping_add(0x6c)) as i32);
                                    if d < 5 {
                                        b_var4 = 1;
                                    }
                                }
                            }
                        }
                        let i_var2b: u32 = rd32(
                            (DAT_009a1fe8 as u32)
                                .wrapping_add(0x1b)
                                .wrapping_add(i_var2),
                        );
                        let addr = (&raw mut DAT_009e968a as usize as u32)
                            .wrapping_add(i_var2b.wrapping_mul(0x5bd));
                        let s1 = rd16(addr);
                        if s1 == param_1 {
                            FUN_00403540(u_var3, u_var6);
                            if DAT_009a1ff8 < 0x10000 {
                                let u_var9 = FUN_00482640(param_1, 0, i_var2b as i32);
                                if (u_var9 as u16) != 0 {
                                    b_var4 = 1;
                                }
                            }
                            if rd16(i_var5.wrapping_add(0x23b)) != 0 {
                                b_var4 = 1;
                            }
                        }
                    }
                    if b_var4 != 0 {
                        FUN_004034e0(param_1 as u32, u_var3);
                        if DAT_009a8c00 < local_4 {
                            local_4 = DAT_009a8c00;
                            local_10 = local_e;
                        }
                    }
                }
                local_e = local_e.wrapping_add(1);
                if !((local_e as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        local_10
    }
}
