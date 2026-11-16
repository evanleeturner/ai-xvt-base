// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e968a: [u8; 0];
    static mut DAT_009e9698: [u8; 0];
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1ff8: i32;
    static mut DAT_009ed22b: u8;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut _DAT_009ed22e: i16;
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_0051c5b9: [u8; 0];
    static mut DAT_00a08374: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a10(a: u8, b: u8, c: u8) -> u32;
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_00482640(param_3: u16, param_4: i16, param_5: i32) -> u64;
    fn FUN_00440760(a: i32, b: u32);
    fn FUN_0042f030(a: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004abe50();
    fn FUN_004aba50();
}

#[inline(always)]
unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn rw(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn arr(p: *mut u8, off: u32) -> u32 {
    p.wrapping_add(off as usize) as u32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043e790() {
    // SAFETY: all addresses are the game's own memory, read where the original reads.
    unsafe {
        let mut u_stack_10: u16 = 0;
        let mut local_12: u16 = 0;
        let mut u_stack_e: u16 = 0;
        let a9670 = &raw mut DAT_009e9670 as *mut u8;
        let a968a = &raw mut DAT_009e968a as *mut u8;
        let a9698 = &raw mut DAT_009e9698 as *mut u8;
        let a8c84 = &raw mut DAT_009d8c84 as *mut u8;
        let a51c5b9 = &raw mut DAT_0051c5b9 as *mut u8;

        let mut u_var11: u32 = rd(arr(a9670, (DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
        let mut local_4: u32 = _DAT_00a07cec as u32;
        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut local_8: i32 = local_4.wrapping_mul(0x23) as i32;
            loop {
                let mut i_var8: u32 = (local_8 as u32).wrapping_add(DAT_009a1fe8 as u32);
                if rb(i_var8.wrapping_add(3)) != 0 {
                    let i_tmp_a = rd(i_var8.wrapping_add(0x1f));
                    if rb(i_tmp_a) == 0 {
                        let i_var4 = rd(i_tmp_a.wrapping_add(0xa9));
                        let s_tmp = rw(i_var4.wrapping_add(0x1e5)) as i16;
                        if s_tmp != 0 && rb(i_var4.wrapping_add(7)) == 0 {
                            let i_tmp_b = rd(i_var8.wrapping_add(0x1b)) as i32;
                            if i_tmp_b == -1 {
                                if rw(i_var4.wrapping_add(0x42)) as u32 == u_var11
                                    && (rb(i_var4.wrapping_add(0x5e)) == 0x0c
                                        || rb(i_var4.wrapping_add(0x5e)) == 0x17)
                                {
                                    let mut i: i32 = 0;
                                    if rb(i_var4.wrapping_add(0x21b)) != 0 {
                                        loop {
                                            let c_var1 = rb((i as u32)
                                                .wrapping_add(0x21e)
                                                .wrapping_add(i_var4))
                                                as i8;
                                            if (c_var1 == -0x75 || c_var1 == -0x77)
                                                && rb((i as u32)
                                                    .wrapping_add(0x220)
                                                    .wrapping_add(i_var4))
                                                    != 0
                                            {
                                                u_stack_10 = 1;
                                            }
                                            i = i.wrapping_add(1);
                                            if !(i < rb(i_var4.wrapping_add(0x21b)) as i32) {
                                                break;
                                            }
                                        }
                                    }
                                    if rb(i_var4.wrapping_add(0x241)) != 0 {
                                        let s = rw(i_var4.wrapping_add(0x23f)) as i16;
                                        if s != 0 {
                                            let b = rb(i_var4.wrapping_add(0x23d));
                                            if b != 0 && b < 3 {
                                                u_stack_e = b as u16;
                                            }
                                        }
                                    }
                                }
                            } else {
                                let i_tmp_a2 = rd(u_var11
                                    .wrapping_mul(0x23)
                                    .wrapping_add(0x1f)
                                    .wrapping_add(DAT_009a1fe8 as u32));
                                let i_var8b = rd(i_tmp_a2.wrapping_add(0xa9));
                                if rw(i_var8b.wrapping_add(0x6a)) as u32 == local_4 {
                                    let u_var6 =
                                        FUN_00435a10(DAT_009ed22b, DAT_009ed22c, DAT_009ed22d);
                                    let d = (u_var6 & 0xffff)
                                        .wrapping_sub(rw(i_var8b.wrapping_add(0x6c)) as u32)
                                        as i32;
                                    if d < 5 {
                                        u_stack_10 = 1;
                                    }
                                }
                                let i_var8c = rd((DAT_009a1fe8 as u32)
                                    .wrapping_add(0x1b)
                                    .wrapping_add(local_8 as u32));
                                let i_var5 = i_var8c.wrapping_mul(0x5bd);
                                let w_tmp = rw(arr(a968a, i_var5));
                                if w_tmp as u32 == u_var11 && rb(arr(a9698, i_var5)) == 0 {
                                    FUN_00403540(local_4, u_var11);
                                    if DAT_009a1ff8 < 0x10000 {
                                        let u_var12 =
                                            FUN_00482640(u_var11 as u16, 0, i_var8c as i32);
                                        if (u_var12 as i16) != 0 {
                                            u_stack_10 = 1;
                                        }
                                    }
                                }
                                let w_tmp2 = rw(arr(a968a, i_var5));
                                if w_tmp2 as u32 == u_var11 && rb(i_var4.wrapping_add(0x241)) != 0 {
                                    let s = rw(i_var4.wrapping_add(0x23f)) as i16;
                                    if s != 0 {
                                        let b = rb(i_var4.wrapping_add(0x23d));
                                        if b != 0 && b < 3 {
                                            u_stack_e = b as u16;
                                        }
                                    }
                                }
                            }
                            let idx = (rb((DAT_009a1fe8 as u32)
                                .wrapping_add(0x16)
                                .wrapping_add(local_8 as u32))
                                as u32)
                                .wrapping_mul(0x566);
                            if rb(arr(a8c84, idx)) != 5 {
                                let mut i: i32 = 0;
                                if rb(i_var4.wrapping_add(0x21d)) != 0 {
                                    let mut pc: u32 = i_var4.wrapping_add(0x370);
                                    loop {
                                        let off = (i as u32).wrapping_mul(8).wrapping_add(
                                            (rb(i_var4.wrapping_add(4)) as u32).wrapping_mul(0x101),
                                        );
                                        if rb(pc) == 2
                                            && rw(pc.wrapping_add(0x42)) as u32 == u_var11
                                            && rb((rb(arr(a51c5b9, off)) as u32)
                                                .wrapping_add(0x33c)
                                                .wrapping_add(i_var4))
                                                != 0
                                        {
                                            local_12 = 1;
                                        }
                                        pc = pc.wrapping_add(4);
                                        i = i.wrapping_add(1);
                                        if !(i < rb(i_var4.wrapping_add(0x21d)) as i32) {
                                            break;
                                        }
                                    }
                                }
                                if rb(i_var4.wrapping_add(0x241)) != 0 {
                                    let s = rw(i_var4.wrapping_add(0x23f)) as i16;
                                    if s != 0 && rw(i_var4.wrapping_add(0x244)) as u32 == u_var11 {
                                        let b = rb(i_var4.wrapping_add(0x23d));
                                        if b != 0 && b < 3 {
                                            u_stack_e = b as u16;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                i_var8 = 0;
                let _ = i_var8;
                local_8 = (local_8 as u32).wrapping_add(0x23) as i32;
                local_4 = local_4.wrapping_add(1);
                if !((local_4 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        FUN_00440760((DAT_00a08374 as i32).wrapping_add(0x5a), u_stack_10 as u32);
        FUN_00440760((DAT_00a08374 as i32).wrapping_add(0x5b), local_12 as u32);
        FUN_00440760((DAT_00a08374 as i32).wrapping_add(0x5c), u_stack_e as u32);

        let mut s_var7: i16 = 0;
        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut cnt: i32 = (DAT_009a1fe0 as u32).wrapping_sub(_DAT_00a07cec as u32) as i32;
            let mut p: u32 = (_DAT_00a07cec as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32);
            loop {
                if rb(p.wrapping_sub(28)) != 0 {
                    let i_tmp_a = rd(p);
                    if rb(i_tmp_a) == 0 {
                        let i_var4 = rd(i_tmp_a.wrapping_add(0xa9));
                        let s_tmp = rw(i_var4.wrapping_add(0x1e5)) as i16;
                        if s_tmp != 0 && rb(i_var4.wrapping_add(7)) == 0 {
                            let i_tmp_b = rd(p.wrapping_sub(4)) as i32;
                            if i_tmp_b == -1 {
                                if rw(i_var4.wrapping_add(0x42)) as u32 == u_var11
                                    && rb(i_var4.wrapping_add(0x5e)) == 0x17
                                {
                                    let s3 = rw(i_var4.wrapping_add(0x23b)) as i16;
                                    if s_var7 < s3 {
                                        s_var7 = s3;
                                    }
                                }
                            } else {
                                let i_var5 = (i_tmp_b as u32).wrapping_mul(0x5bd);
                                let w_tmp = rw(arr(a968a, i_var5));
                                if w_tmp as u32 == u_var11 && rb(arr(a9698, i_var5)) != 0 {
                                    let s3 = rw(i_var4.wrapping_add(0x23b)) as i16;
                                    if s_var7 < s3 {
                                        s_var7 = s3;
                                    }
                                }
                            }
                        }
                    }
                }
                p = p.wrapping_add(0x23);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        if s_var7 < 0x3b1 {
            if s_var7 < 1 {
                u_var11 = 0;
            } else {
                u_var11 = (((_DAT_009ed22e as i32) / 0x3b) as u32) & 1;
            }
        } else {
            u_var11 = 2;
        }
        FUN_00440760((DAT_00a08374 as i32).wrapping_add(0x5d), u_var11);
        FUN_004abe50();
        FUN_0042f030(u_var11);
        FUN_004aba50();
    }
}
