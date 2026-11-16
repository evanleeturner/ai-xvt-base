// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4980: i32;
    static mut DAT_0070ad44: [u8; 1];
    static mut DAT_0070b215: [u8; 1];
    static mut DAT_0070ad49: [u8; 1];
    static mut DAT_0070ad40: [u8; 1];
    static mut DAT_0070ac42: [u16; 1];
    static mut DAT_0070ac44: [u16; 1];
    static mut DAT_00703abc: u16;
    static mut _DAT_00732a3c: i32;
    static mut DAT_00732a3c: u16;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_007092d4: i8;
    static mut DAT_007092c1: i8;
    static mut DAT_007092c0: i8;
    static mut DAT_00730880: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071b73e: [u8; 1];
    static mut DAT_0071b73c: [u8; 1];
    static mut DAT_0071b741: [u8; 1];
    static mut _DAT_0071e6a6: u16;
    static mut DAT_00732910: u16;
    static mut _DAT_00732950: u16;
    static mut _DAT_00732952: u16;
    static mut DAT_0085e795: i8;
    static mut DAT_0052686c: [u8; 1];
    static mut DAT_0052686a: [u8; 1];
    static mut DAT_0052b09b: [u8; 1];
    static mut DAT_0052b08b: [u8; 1];
    static mut DAT_00523b70: [u16; 1];
    static mut DAT_00523b88: [u8; 1];
    static mut DAT_00523b98: [u8; 1];
    static mut DAT_00523bb0: [u8; 1];
    static mut DAT_006d4a86: [u8; 1];
    static mut DAT_006d4ab0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_0044a6c0(a: u32, b: u32) -> u32;
    fn FUN_0041a1d0(a: u32, b: i16, c: u16, d: u32) -> u16;
    fn FUN_0040b730(a: i32, b: u32, c: i32, d: i32, e: u32, f: u16) -> i32;
    fn FUN_0040c070(a: i32, b: i32, c: u32, d: i32) -> i32;
    fn FUN_00478470() -> u32;
    fn FUN_0040a1a0(a: u32, b: u32, c: i32) -> u32;
    fn FUN_0044a600(a: u32, b: u32) -> u32;
    fn FUN_00478320(a: u32, b: u32) -> u32;
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_004a34c0(a: u32, b: *mut core::ffi::c_void, c: i32);
    fn FUN_00444040(a: u32, b: u32) -> u8;
    fn FUN_0042b1e0(a: i32, b: u32) -> *mut u16;
    fn FUN_004784d0() -> u32;
    fn FUN_004a6a80();
    fn FUN_004afb90();
    fn FUN_004a6f90();
    fn FUN_004a01a0(a: u32, b: u32);
    fn FUN_004a10d0(a: u32, b: i32) -> u32;
    fn FUN_00423d00(a: i32);
    fn FUN_00443b90(a: i32, b: u32, c: u32);
    fn FUN_0049f9c0(a: u16, b: u16, c: i32);
    fn FUN_0044ad00(a: u32) -> i32;
    fn FUN_004490f0(a: u32) -> u32;
    fn FUN_00449790(a: u32, b: u32) -> i32;
    fn FUN_00449b30(a: u32, b: u32) -> i32;
    fn FUN_00421ff0(a: u32, b: i8) -> u32;
}

unsafe fn rd8(b: u32, o: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { *(b.wrapping_add(o) as usize as *const u8) }
}
unsafe fn wr8(b: u32, o: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { *(b.wrapping_add(o) as usize as *mut u8) = v }
}
unsafe fn ld16(b: u32, o: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (b.wrapping_add(o) as usize as *const u16).read_unaligned() }
}
unsafe fn ld32(b: u32, o: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (b.wrapping_add(o) as usize as *const u32).read_unaligned() }
}
unsafe fn st16(b: u32, o: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (b.wrapping_add(o) as usize as *mut u16).write_unaligned(v) }
}
unsafe fn st32(b: u32, o: u32, v: u32) {
    // SAFETY: address inside the game's own memory.
    unsafe { (b.wrapping_add(o) as usize as *mut u32).write_unaligned(v) }
}

macro_rules! addr {
    ($x:ident) => {
        ((&raw mut $x) as usize as u32)
    };
}
macro_rules! sel {
    () => {
        (DAT_0071ecf4.wrapping_mul(0x5bd) as u32)
    };
}
macro_rules! rb {
    () => {
        (DAT_006d4988 as usize as u32)
    };
}
macro_rules! t70 {
    ($i:expr) => {
        ld16(addr!(DAT_00523b70), ($i as u32).wrapping_mul(2))
    };
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_unsafe
)]
pub unsafe extern "C" fn FUN_004110e0(
    param_1: u32,
    param_2: u32,
    mut param_3: u32,
    mut param_4: u32,
    param_5: u32,
    param_6: u16,
    param_7: u32,
    param_8: u16,
    param_9: u32,
) -> u32 {
    // SAFETY: all globals, callees and record pointers belong to the game;
    // accesses mirror the original's reads and writes in order.
    unsafe {
        let mut u_var29: u32 = param_4;
        let u_var10: u16 = DAT_00703abc;
        let mut b_var9: u8 = 0;
        let mut b_var12: u8 = 0;
        let mut local_1f: u8 = 1;
        if DAT_00525550 != 0 {
            return 1;
        }
        let u_var18: u32 = param_1 & 0xffff;
        let i_var30: u32 = u_var18.wrapping_mul(0x23);
        macro_rules! r30 {
            () => {
                rb!().wrapping_add(i_var30)
            };
        }
        let mut u_var28: u32 = rd8(r30!(), 0x16) as u32;
        let mut i_var19: i32 = u_var28.wrapping_mul(0x566) as i32;
        if rd8(addr!(DAT_0070ad44), i_var19 as u32) == 0x14
            || rd8(addr!(DAT_0070b215), i_var19 as u32) == 0x14
        {
            return ((((i_var19 as u32) >> 16) & 0xffff) << 16) | 1;
        }
        u_var28 = param_3 & 0xffff;
        if _DAT_00732a3c <= u_var28 as i32 && (u_var28 as i32) < DAT_006d4980 {
            let u_var26 = rd8(rb!(), u_var28.wrapping_mul(0x23).wrapping_add(0x16)) as u32;
            i_var19 = u_var26.wrapping_mul(0x566) as i32;
            if rd8(addr!(DAT_0070ad44), i_var19 as u32) == 0x14
                || rd8(addr!(DAT_0070b215), i_var19 as u32) == 0x14
            {
                return ((((i_var19 as u32) >> 16) & 0xffff) << 16) | 1;
            }
        }
        let p5: u32 = ld32(ld32(r30!(), 0x1f), 0xa7);
        i_var19 = FUN_0042b150(p5 as i32);
        DAT_00703abc = ld16(i_var19 as u32, 0x16);
        let mut u_stack_1a: u16;
        if param_3 as u16 == 0xfffe {
            b_var12 = 1;
            param_3 = (param_3 & 0xffff0000) | 0xffff;
            u_stack_1a = 0x35;
            param_4 = 0;
            u_var29 &= 0xffff;
        } else if param_3 as u16 == 0xffff {
            u_stack_1a = 0x35;
            u_var29 = 0x20000;
        } else {
            let i_var24 = rb!().wrapping_add(u_var28.wrapping_mul(0x23));
            if ld32(i_var24, 0x1f) == 0 {
                u_stack_1a = rd8(i_var24, 3) as u16;
                let w = ld16(addr!(DAT_0052686c), (u_stack_1a as u32).wrapping_mul(0x16));
                if 0x7fff < w {
                    u_var29 = 0x20000;
                } else {
                    u_var29 = (w as u32) << 2;
                }
            } else {
                u_var29 = ld16(ld32(i_var24, 0x1f), 0x7d) as u32;
                u_stack_1a = rd8(i_var24, 3) as u16;
            }
        }
        let mut c_var3: i8 = rd8(r30!(), 2) as i8;
        if c_var3 == 4 || c_var3 == 5 {
            u_var29 >>= 4;
        }
        if c_var3 == 3 {
            u_var29 >>= 2;
        }
        if 0x7ffe < u_var29 {
            u_var29 = 0x7ffe;
        }
        let mut u_var13: u16 = u_var29 as u16;
        let mut i_var24: i32 = (u_var13 as i16) as i32;
        st32(p5, 0xcd, ld32(p5, 0xcd).wrapping_add(i_var24 as u32));
        let mut local_1c: u16 = 0xffff;
        let mut i_var23: u32 = 0;
        'after: {
            'l147d: {
                if param_3 as u16 != 0xffff {
                    let i_var7 = (param_3 & 0xffff).wrapping_mul(0x23);
                    i_var23 = rb!().wrapping_add(i_var7);
                    let pc6 = ld32(rb!().wrapping_add(i_var7), 0x1f);
                    if pc6 == 0 {
                        break 'l147d;
                    }
                    c_var3 = rd8(pc6, 0) as i8;
                    let mut i_var25: i32;
                    if c_var3 == 0 {
                        i_var25 = ld32(i_var23, 0x1b) as i32;
                        local_1c = param_3 as u16;
                    } else {
                        local_1c = ld16(ld32(i_var23, 0x1f), 0x83);
                        i_var25 = ld32(
                            (local_1c as u32).wrapping_mul(0x23).wrapping_add(0x1b),
                            rb!(),
                        ) as i32;
                        if c_var3 == 1 {
                            let q = ld32(ld32(i_var23, 0x1f), 0xa3);
                            if q != 0 {
                                i_var25 = rd8(q, 0) as i8 as i32;
                            }
                        }
                    }
                    if i_var25 == -1 {
                        if c_var3 == 0 {
                            st32(p5, 0xd5, ld32(p5, 0xd5).wrapping_add(i_var24 as u32));
                        } else {
                            u_var29 = local_1c as u32;
                            i_var23 = rb!().wrapping_add(u_var29.wrapping_mul(0x23));
                            if rd8(i_var23, 2) == 0 {
                                let idx = (rd8(i_var23, 0x16) as u32).wrapping_mul(0x566);
                                let pi = p5
                                    .wrapping_add(
                                        (rd8(addr!(DAT_0070ad49), idx) as u32).wrapping_mul(4),
                                    )
                                    .wrapping_add(0x1c1);
                                st32(pi, 0, ld32(pi, 0).wrapping_add(i_var24 as u32));
                            } else if (u_var29 as i32) < DAT_00732a38
                                || DAT_00703c88.wrapping_add(DAT_00732a38) <= u_var29 as i32
                            {
                                st32(p5, 0xd9, ld32(p5, 0xd9).wrapping_add(i_var24 as u32));
                            } else {
                                st32(p5, 0xdd, ld32(p5, 0xdd).wrapping_add(i_var24 as u32));
                            }
                        }
                    } else {
                        let o = (i_var25 as u32).wrapping_mul(4).wrapping_add(0x1a1);
                        st32(p5, o, ld32(p5, o).wrapping_add(i_var24 as u32));
                        if rd8(ld32(rb!().wrapping_add(i_var7), 0x1f), 0) == 0 {
                            st32(p5, 0xd5, ld32(p5, 0xd5).wrapping_add(i_var24 as u32));
                        }
                    }
                    let pi = p5
                        .wrapping_add(
                            (rd8(
                                rb!(),
                                (local_1c as u32).wrapping_mul(0x23).wrapping_add(0x16),
                            ) as u32)
                                .wrapping_mul(4),
                        )
                        .wrapping_add(0xe1);
                    st32(pi, 0, ld32(pi, 0).wrapping_add(i_var24 as u32));
                    break 'after;
                }
            }
            // LAB_0041147d
            let c = param_3 as u16 == 0xffff || {
                i_var23 = rb!().wrapping_add((param_3 & 0xffff).wrapping_mul(0x23));
                ld32(i_var23, 0x1f) != 0
            };
            if c {
                if b_var12 != 0 {
                    st32(p5, 0xd9, ld32(p5, 0xd9).wrapping_add(i_var24 as u32));
                }
            } else if rd8(i_var23, 2) == 10 {
                st32(p5, 0xd5, ld32(p5, 0xd5).wrapping_add(i_var24 as u32));
            }
        }
        if ld32(r30!(), 0x1b) as i32 != -1 {
            st32(p5, 0xd1, ld32(p5, 0xd1).wrapping_add(i_var24 as u32));
        }
        if param_2 as i16 != -1 {
            let sum = (ld16(p5, 0x20b) as i16 as i32).wrapping_add(ld16(p5, 0x20d) as i16 as i32);
            if sum == 0 || DAT_007092d4 == 0 {
                u_var29 = FUN_0044a6c0(rd8(r30!(), 3) as u32, (param_2 & 0xffff).wrapping_sub(1));
                if u_var29 != 0 {
                    u_var13 = FUN_0041a1d0(
                        param_1,
                        param_2 as i16,
                        u_var13,
                        ((u_stack_1a as u32) << 16) | local_1c as u32,
                    );
                }
            }
        }
        c_var3 = rd8(r30!(), 2) as i8;
        if c_var3 == 0x0e {
            u_var13 = 0;
        }
        let ps2: u32 = p5
            .wrapping_add((param_4 & 0xffff).wrapping_mul(2))
            .wrapping_add(0x20b);
        let mut s_var16: i16 = ld16(ps2, 0) as i16;

        'l4126c7: {
            'post: {
                'efc: {
                    if (u_var13 as i16) < s_var16 {
                        st16(ps2, 0, s_var16.wrapping_sub(u_var13 as i16) as u16);
                        i_var24 = ld32(r30!(), 0x1b) as i32;
                        if i_var24 != -1 {
                            wr16_arr_0070ac42(i_var24);
                            DAT_00730880 = param_4 as u8;
                            u_var29 = (param_4 & 0xffff) ^ 1;
                            s_var16 = ld16(p5, u_var29.wrapping_mul(2).wrapping_add(0x20b)) as i16;
                            if (ld16(ps2, 0) as i16) < s_var16 {
                                let s_var14 =
                                    ((s_var16 as i32 - ld16(ps2, 0) as i16 as i32) / 2) as i16;
                                st16(
                                    p5,
                                    u_var29.wrapping_mul(2).wrapping_add(0x20b),
                                    s_var16.wrapping_sub(s_var14) as u16,
                                );
                                st16(ps2, 0, (ld16(ps2, 0) as i16).wrapping_add(s_var14) as u16);
                                wr8(p5, 0x210, 1);
                            }
                        }
                        if ld32(addr!(DAT_0071b730), sel!()) != u_var18 {
                            FUN_0040b730(DAT_0071ecf4, u_var18, 3, -1, u_var18, 0x2000);
                        }
                    } else {
                        if DAT_0085e795 == 2 && c_var3 != 0 && s_var16 != 0 && ld16(p5, 0xc3) == 0 {
                            FUN_0040c070(1, 0x10, u_var18, 0xffff);
                        }
                        u_var13 = (u_var13 as i32).wrapping_sub(ld16(ps2, 0) as i16 as i32) as u16;
                        st16(ps2, 0, 0);
                        if ld32(r30!(), 0x1b) as i32 != -1 {
                            if param_4 as i16 == 0 && b_var12 == 0 && {
                                let u_var15 = FUN_00478470() as u16;
                                u_var15 < 0x1000
                            } {
                                FUN_0040a1a0(0x56, 0xffffffff, ld32(r30!(), 0x1b) as i32);
                            }
                            u_var29 = (param_4 & 0xffff) ^ 1;
                            let o = u_var29.wrapping_mul(2).wrapping_add(0x20b);
                            s_var16 = ld16(p5, o) as i16;
                            if s_var16 != 0 {
                                st16(p5, o, s_var16.wrapping_sub(s_var16 / 2) as u16);
                                st16(
                                    ps2,
                                    0,
                                    (ld16(ps2, 0) as i16).wrapping_add(s_var16 / 2) as u16,
                                );
                                wr8(p5, 0x210, 1);
                            }
                        }
                        if u_var13 == 0 {
                            break 'efc;
                        }
                        if u_stack_1a == 0x8d || u_stack_1a == 0x8e || u_stack_1a == 0x93 {
                            if ld16(p5, 0xc9) < 1000 {
                                if u_stack_1a == 0x8d {
                                    st16(p5, 0xc9, ld16(p5, 0xc9).wrapping_add(1));
                                }
                                if u_stack_1a == 0x8e {
                                    st16(p5, 0xc9, ld16(p5, 0xc9).wrapping_add(2));
                                }
                                if u_stack_1a == 0x93 {
                                    st16(p5, 0xc9, ld16(p5, 0xc9).wrapping_add(4));
                                }
                            }
                            s_var16 =
                                (ld16(addr!(DAT_0052b08b), (rd8(p5, 4) as u32).wrapping_mul(0xef))
                                    as i16)
                                    .wrapping_sub(ld16(p5, 0xc9) as i16);
                            if ld16(p5, 0x1df) as i16 != 0 && s_var16 < 0xb {
                                if 0 < u_var13 as i16 {
                                    u_var29 =
                                        ((u_var13 as u32).wrapping_add(199) as u16 as u32) / 200;
                                    loop {
                                        u_var28 = 0;
                                        loop {
                                            if (ld16(p5, 0x1df) & t70!(u_var28)) != 0 {
                                                b_var9 = 1;
                                                st16(
                                                    p5,
                                                    0x1df,
                                                    (t70!(u_var28) ^ 0x3ff) & ld16(p5, 0x1df),
                                                );
                                                break;
                                            }
                                            u_var13 = (u_var28 as u16).wrapping_add(1);
                                            u_var28 = u_var13 as u32;
                                            if !(u_var13 < 10) {
                                                break;
                                            }
                                        }
                                        if (u_var28 as u16) < 10 && ld32(r30!(), 0x1b) as i32 != -1
                                        {
                                            i_var24 = u_var28.wrapping_mul(2) as i32;
                                            st16(p5, (i_var24 as u32).wrapping_add(0x29e), 0);
                                            st16(
                                                p5,
                                                (i_var24 as u32).wrapping_add(0x2b4),
                                                ld16(addr!(DAT_00523b98), i_var24 as u32),
                                            );
                                        }
                                        u_var13 = (u_var29 as u16).wrapping_sub(1);
                                        u_var29 = u_var13 as u32;
                                        if u_var13 == 0 {
                                            break;
                                        }
                                    }
                                }
                                if s_var16 < 1 && ld32(r30!(), 0x1b) as i32 != -1 {
                                    u_var29 = 0;
                                    loop {
                                        i_var24 = u_var29.wrapping_mul(2) as i32;
                                        if (ld16(p5, 0x1df) & t70!(u_var29)) != 0 {
                                            st16(
                                                p5,
                                                0x1df,
                                                (t70!(u_var29) ^ 0x3ff) & ld16(p5, 0x1df),
                                            );
                                            st16(p5, (i_var24 as u32).wrapping_add(0x29e), 0);
                                            st16(
                                                p5,
                                                (i_var24 as u32).wrapping_add(0x2b4),
                                                ld16(addr!(DAT_00523b98), i_var24 as u32),
                                            );
                                        }
                                        u_var13 = (u_var29 as u16).wrapping_add(1);
                                        u_var29 = u_var13 as u32;
                                        if !(u_var13 < 10) {
                                            break;
                                        }
                                    }
                                    st16(p5, 0x1df, 0);
                                }
                                let mut b_var8: u8 = 0;
                                if ld16(p5, 0x1df) as i16 == 0 {
                                    st16(
                                        p5,
                                        0xc9,
                                        ld16(
                                            addr!(DAT_0052b08b),
                                            (rd8(p5, 4) as u32).wrapping_mul(0xef),
                                        ),
                                    );
                                    st16(p5, 0x20b, 0);
                                    st16(p5, 0x20d, 0);
                                    FUN_004a34c0(
                                        param_1,
                                        p5 as usize as *mut core::ffi::c_void,
                                        0x88,
                                    );
                                    i_var24 = FUN_0040c070(1, 0x1c, u_var18, 0xffff);
                                    if i_var24 != 0
                                        && {
                                            let b_var11 = FUN_00444040(
                                                param_1,
                                                ld16(addr!(DAT_0071b73c), sel!()) as u32,
                                            );
                                            b_var11 == 0
                                        }
                                        && {
                                            i_var24 = ld32(addr!(DAT_0071b730), sel!()) as i32;
                                            i_var24 != -1
                                        }
                                    {
                                        c_var3 = rd8(
                                            rb!(),
                                            (i_var24 as u32).wrapping_mul(0x23).wrapping_add(3),
                                        ) as i8;
                                        if c_var3 == 1
                                            || c_var3 == 2
                                            || c_var3 == 3
                                            || c_var3 == 0x0e
                                            || c_var3 == 4
                                        {
                                            b_var8 = 1;
                                        }
                                        if b_var8 != 0 {
                                            u_var29 = 0x54;
                                        } else {
                                            u_var29 = 0x24;
                                        }
                                        FUN_0040a1a0(u_var29, 0xffffffff, DAT_0071ecf4);
                                    }
                                    if ld32(r30!(), 0x1b) as i32 == -1 {
                                        u_var29 = 0;
                                        if rd8(p5, 0x213) != 0 {
                                            loop {
                                                if rd8(
                                                    p5,
                                                    u_var29.wrapping_add(0xd1).wrapping_mul(4),
                                                ) == 2
                                                {
                                                    let pu = FUN_0042b1e0(p5 as i32, u_var29);
                                                    pu.write_unaligned(0xffff);
                                                }
                                                u_var29 = u_var29.wrapping_add(1);
                                                if !(u_var29 < rd8(p5, 0x213) as u32) {
                                                    break;
                                                }
                                            }
                                        }
                                        u_var29 = 0;
                                        if rd8(p5, 0x211) != 0 {
                                            loop {
                                                wr8(p5, u_var29.wrapping_add(0x216), 0);
                                                u_var29 = u_var29.wrapping_add(1);
                                                if !(u_var29 < rd8(p5, 0x211) as u32) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    u_var29 = rd8(r30!(), 0x16) as u32;
                                    i_var24 = u_var29.wrapping_mul(0x126) as i32;
                                    let a86 = addr!(DAT_006d4a86);
                                    st16(
                                        a86,
                                        i_var24 as u32,
                                        ld16(a86, i_var24 as u32).wrapping_add(1),
                                    );
                                    if rd8(addr!(DAT_0070ad40), u_var29.wrapping_mul(0x566))
                                        == rd8(p5, 0xb0)
                                    {
                                        wr8(addr!(DAT_006d4ab0), i_var24 as u32, 1);
                                    }
                                }
                            }
                            if ld32(r30!(), 0x1b) as i32 == DAT_0071ecf4 {
                                FUN_0040a1a0(0x21, u_var18, DAT_0071ecf4);
                            }
                        } else {
                            if param_2 as i16 != -1 && {
                                u_var29 = FUN_0044a600(
                                    rd8(r30!(), 3) as u32,
                                    (param_2 & 0xffff).wrapping_sub(1),
                                );
                                u_var29 != 0
                            } {
                                u_var13 = FUN_0041a1d0(
                                    param_1,
                                    param_2 as i16,
                                    u_var13,
                                    ((u_stack_1a as u32) << 16) | local_1c as u32,
                                );
                            }
                            let u_var15: u16 = ld16(p5, 0xc3);
                            st16(p5, 0xc3, u_var13.wrapping_add(u_var15));
                            'l866: {
                                if DAT_0085e795 == 2 && rd8(r30!(), 2) != 0 {
                                    u_var29 = FUN_00478320(
                                        ((rb!() >> 16) << 16) | ld16(p5, 199) as u32,
                                        0xf333,
                                    );
                                    if u_var15 < u_var29 as u16
                                        && (u_var29 as u16) <= ld16(p5, 0xc3)
                                    {
                                        i_var24 = 0x16;
                                    } else {
                                        u_var29 = FUN_00478320(
                                            ((u_var29 >> 16) << 16) | ld16(p5, 199) as u32,
                                            0xc000,
                                        );
                                        if u_var15 < u_var29 as u16
                                            && (u_var29 as u16) <= ld16(p5, 0xc3)
                                        {
                                            i_var24 = 0x15;
                                        } else {
                                            u_var29 = FUN_00478320(
                                                ((u_var29 >> 16) << 16) | ld16(p5, 199) as u32,
                                                0x4000,
                                            );
                                            if (u_var29 as u16) <= u_var15
                                                || ld16(p5, 0xc3) < u_var29 as u16
                                            {
                                                break 'l866;
                                            }
                                            i_var24 = 0x14;
                                        }
                                    }
                                    FUN_0040c070(1, i_var24, u_var18, 0xffff);
                                }
                            }
                            i_var24 = ld32(r30!(), 0x1b) as i32;
                            if i_var24 != -1 && b_var12 == 0 {
                                if i_var24 != -1 {
                                    wradd_arr_0070ac44(i_var24);
                                }
                                u_var13 = FUN_00478470() as u16;
                                if u_var13 < 0x4000 {
                                    let a = FUN_00478470() as u16;
                                    let b = FUN_00478470() as u16;
                                    let c = FUN_00478470() as u16;
                                    u_var13 = a;
                                    u_var29 =
                                        ((a & 7).wrapping_add(b & 1).wrapping_add(c & 1)) as u32;
                                    if (ld16(p5, 0x1df) & t70!(u_var29)) != 0 {
                                        st16(p5, 0x1df, (t70!(u_var29) ^ 0x3ff) & ld16(p5, 0x1df));
                                        _DAT_00732950 = rd8(addr!(DAT_00523b88), u_var29) as u16;
                                        _DAT_00732952 = 0x57;
                                        if ld32(r30!(), 0x1b) as i32 == DAT_0071ecf4
                                            && rd8(addr!(DAT_0071b741), sel!()) == 0
                                        {
                                            FUN_004a1d70(0x56, DAT_0071ecf4);
                                        }
                                        st16(p5, u_var29.wrapping_mul(2).wrapping_add(0x29e), 0);
                                        st16(
                                            p5,
                                            u_var29.wrapping_mul(2).wrapping_add(0x2b4),
                                            ld16(addr!(DAT_00523b98), u_var29.wrapping_mul(2)),
                                        );
                                    }
                                }
                            }
                        }
                        if ld16(p5, 0xc3) < ld16(p5, 0xc5) || b_var12 != 0 {
                            if ld32(r30!(), 0x1b) as i32 == DAT_0071ecf4 && b_var12 == 0 {
                                u_var29 = FUN_004784d0();
                                if (u_var29 & 0x8000) == 0 {
                                    u_var29 = 0x20;
                                } else {
                                    u_var29 = 0x1f;
                                }
                                FUN_0040a1a0(u_var29, u_var18, DAT_0071ecf4);
                                local_1f = 0;
                            }
                            if ld32(addr!(DAT_0071b730), sel!()) != u_var18 {
                                FUN_0040b730(DAT_0071ecf4, u_var18, 3, -1, u_var18, 0x4000);
                            }
                        } else {
                            u_var29 = FUN_00478470();
                            u_var13 = ld16(addr!(DAT_00523bb0), (u_var29 & 0xf).wrapping_mul(2));
                            if (DAT_007092c1 == 0 || u_var13 != 1)
                                && (ld16(p5, 0x1d9) & u_var13) != 0
                            {
                                i_var24 = ld32(r30!(), 0x1b) as i32;
                                if i_var24 != -1 {
                                    FUN_0040a1a0(0x1e, u_var18, i_var24);
                                    b_var12 = 0;
                                    local_1f = 0;
                                    c_var3 = rd8(r30!(), 3) as i8;
                                    if c_var3 == 1
                                        || c_var3 == 2
                                        || c_var3 == 3
                                        || c_var3 == 0x0e
                                        || c_var3 == 4
                                    {
                                        b_var12 = 1;
                                    }
                                    if b_var12 != 0 {
                                        if u_var13 < 9 {
                                            if u_var13 == 8 || u_var13 == 2 || u_var13 == 4 {
                                                u_var13 = 0xe;
                                            }
                                        } else if u_var13 < 0x101 {
                                            if u_var13 == 0x100 || u_var13 == 0x80 {
                                                u_var13 = 0x180;
                                            }
                                        } else if u_var13 < 0x401 {
                                            if u_var13 == 0x400 || u_var13 == 0x200 {
                                                u_var13 = 0xe00;
                                            }
                                        } else if u_var13 == 0x800 || u_var13 == 0x1000 {
                                            u_var13 = 0xe00;
                                        }
                                    }
                                }
                                b_var9 = 1;
                                st16(p5, 0x1db, ld16(p5, 0x1db) & !u_var13);
                            }
                            if rd8(p5, 7) == 0 && ld32(addr!(DAT_0071b730), sel!()) != u_var18 {
                                FUN_0040b730(DAT_0071ecf4, u_var18, 5, -1, u_var18, 0x6000);
                            }
                        }
                        if b_var9 == 0
                            || ld32(r30!(), 0x1b) as i32 != DAT_0071ecf4
                            || DAT_00732910 != 0
                        {
                            break 'efc;
                        }
                        if DAT_00525550 == 0 {
                            FUN_004a6a80();
                            FUN_004afb90();
                            FUN_004a6f90();
                            break 'efc;
                        }
                        break 'post;
                    }
                    // LAB_00411efc
                    if DAT_00525550 == 0 {
                        u_var29 = 0;
                        if (rd8(p5, 7) != 0 && rd8(p5, 7) != 5) || {
                            u_var29 = 0;
                            ld16(p5, 0xc3) < ld16(p5, 199)
                        } {
                            break 'l4126c7;
                        }
                        if param_3 as u16 != 0xffff && {
                            let q = ld32(
                                rb!().wrapping_add((param_3 & 0xffff).wrapping_mul(0x23)),
                                0x1f,
                            );
                            if q != 0 {
                                if rd8(q, 0) != 0 {
                                    param_3 = (((q >> 16) & 0xffff) << 16) | ld16(q, 0x83) as u32;
                                }
                                FUN_004a01a0(param_3, param_1);
                            }
                            q != 0
                        } {}
                        u_var29 = FUN_004a10d0(u_var18, 0);
                        i_var24 = ld32(r30!(), 0x1b) as i32;
                        if i_var24 != -1 {
                            FUN_00423d00(i_var24);
                        }
                        if DAT_007092c1 == 0 {
                            i_var24 = ld32(r30!(), 0x1b) as i32;
                            if i_var24 != -1 {
                                FUN_00443b90(i_var24, local_1c as u32, u_var29);
                            }
                        } else if ld32(r30!(), 0x1b) as i32 == DAT_0071ecf4 {
                            DAT_007092c0 = 1;
                        }
                        if ld32(r30!(), 0x1b) as i32 != -1 {
                            _DAT_0071e6a6 = 0;
                        }
                        let u_var15: u16 = param_1 as u16;
                        FUN_0049f9c0(u_var15, rd8(r30!(), 0x16) as u16, 2);
                        u_var13 = DAT_00732a3c;
                        if ((_DAT_00732a3c & 0xffff) as i32) < DAT_006d4980 {
                            loop {
                                let a = rb!().wrapping_add((u_var13 as u32).wrapping_mul(0x23));
                                if rd8(a, 3) != 0 {
                                    let b = ld32(ld32(a, 0x1f), 0xa7);
                                    if ld16(b, 0x66) == u_var15 {
                                        st16(b, 0x66, 0xffff);
                                    }
                                }
                                u_var13 = u_var13.wrapping_add(1);
                                if !((u_var13 as u32 as i32) < DAT_006d4980) {
                                    break;
                                }
                            }
                        }
                        u_var13 = DAT_00732a3c;
                        if ((_DAT_00732a3c & 0xffff) as i32) < DAT_006d4980 {
                            loop {
                                let a = rb!().wrapping_add((u_var13 as u32).wrapping_mul(0x23));
                                if rd8(a, 3) != 0 {
                                    let b = ld32(ld32(a, 0x1f), 0xa7);
                                    if ld16(b, 0x68) == u_var15 {
                                        st16(b, 0x68, 0xffff);
                                    }
                                }
                                u_var13 = u_var13.wrapping_add(1);
                                if !((u_var13 as u32 as i32) < DAT_006d4980) {
                                    break;
                                }
                            }
                        }
                        FUN_004a34c0(param_1, p5 as usize as *mut core::ffi::c_void, 0x87);
                        u_var13 = ld16(addr!(DAT_0071b73e), sel!());
                        if rd8(r30!(), 0x16) as u16 == u_var13 {
                            i_var24 = FUN_0040b730(DAT_0071ecf4, u_var18, 6, -1, u_var18, 0xc000);
                            if i_var24 == 0 {
                                FUN_0040b730(DAT_0071ecf4, 0xffffffff, 0x10, -1, u_var18, 0xc000);
                                u_var13 = 0xc000;
                                u_var29 = 0xffffffff;
                            }
                        } else {
                            u_var29 = local_1c as u32;
                            if u_var29 != 0xffffffff
                                && (u_var29 as i32) < DAT_006d4980
                                && ld32(addr!(DAT_0071b730), sel!()) != u_var29
                                && rd8(rb!(), u_var29.wrapping_mul(0x23).wrapping_add(0x16)) as u16
                                    == u_var13
                            {
                                c_var3 = rd8(r30!(), 2) as i8;
                                if c_var3 == 0 || c_var3 == 1 {
                                    u_var13 = 0xffff;
                                    i_var24 = 9;
                                } else {
                                    u_var13 = 0xa000;
                                    i_var24 = 10;
                                }
                                FUN_0040b730(DAT_0071ecf4, u_var29, i_var24, -1, u_var18, u_var13);
                            }
                        }
                        i_var24 = FUN_0040c070(1, 0x1b, u_var18, 0xffff);
                        if i_var24 != 0
                            && {
                                b_var12 =
                                    FUN_00444040(param_1, ld16(addr!(DAT_0071b73c), sel!()) as u32);
                                b_var12 == 0
                            }
                            && {
                                i_var24 = ld32(addr!(DAT_0071b730), sel!()) as i32;
                                i_var24 != -1
                            }
                        {
                            c_var3 = rd8(rb!(), (i_var24 as u32).wrapping_mul(0x23).wrapping_add(3))
                                as i8;
                            if c_var3 == 1
                                || c_var3 == 2
                                || c_var3 == 3
                                || c_var3 == 0x0e
                                || c_var3 == 4
                            {
                                b_var12 = 1;
                            } else {
                                b_var12 = 0;
                            }
                            if b_var12 != 0 {
                                u_var29 = 0x54;
                            } else {
                                u_var29 = 0x24;
                            }
                            FUN_0040a1a0(u_var29, 0xffffffff, DAT_0071ecf4);
                        }
                        u_var28 = rd8(r30!(), 3) as u32;
                        i_var24 = FUN_0044ad00(u_var28);
                        if i_var24 == 0 {
                            let b_var4 = rd8(p5, 4);
                            u_var13 = FUN_00478470() as u16;
                            u_var13 = (u_var13 & 0x3fff).wrapping_add(0x2000);
                            while ld16(addr!(DAT_0052b09b), (b_var4 as u32).wrapping_mul(0xef))
                                < u_var13
                            {
                                u_var13 >>= 1;
                            }
                            let u15b = FUN_00478470() as u16;
                            if u15b < 0x8000 {
                                u_var13 = u_var13.wrapping_neg();
                            }
                            st16(ld32(r30!(), 0x1f), 0x77, u_var13);
                            u_var29 = FUN_00478470();
                            let t = u_var29 & 0xffff0007;
                            s_var16 = ((t as u16 as i16 as i32).wrapping_add(8)).wrapping_mul(0xec)
                                as i16;
                            u_var29 = (((t >> 16) & 0xffff) << 16) | s_var16 as u16 as u32;
                            st16(ld32(r30!(), 0x1f), 0x7f, s_var16 as u16);
                            wr8(p5, 7, 3);
                        } else if (ld16(
                            addr!(DAT_0052686c),
                            (u_stack_1a as u32).wrapping_mul(0x16),
                        ) < 0x448
                            || rd8(addr!(DAT_0052686a), (u_stack_1a as u32).wrapping_mul(0x16))
                                != 0)
                            && (u_stack_1a != 0x3a && (ld16(ld32(r30!(), 0x1f), 0x79) as i16 != 0))
                        {
                            u_var13 = FUN_00478470() as u16;
                            if u_var13 < 0x4000 && ld32(r30!(), 0x1b) as i32 != -1 {
                                st16(ld32(r30!(), 0x1f), 0x7f, 1);
                                u_var29 = FUN_0040a1a0(0x15, u_var18, DAT_0071ecf4);
                                local_1f = 0;
                                wr8(p5, 7, 4);
                            } else {
                                u_var29 = FUN_004490f0(u_var28);
                                let mut u_var17: u16 = u_var29 as u16;
                                local_1c = 0;
                                u_var13 = 0;
                                let mut u_var15: u16 = 0;
                                if 1 < u_var17 {
                                    let mut u_var27: u16 = 0;
                                    u_var15 = FUN_00478470() as u16;
                                    u_var15 &= 1;
                                    if u_var17 != 0 {
                                        loop {
                                            let u_var26 = u_var27 as u32;
                                            i_var24 = FUN_00449790(u_var28, u_var26);
                                            if rd8(p5, u_var26.wrapping_add(0x2ca)) == 0
                                                && (i_var24 == 2 || i_var24 == 0x14)
                                            {
                                                if u_var15 == 0 {
                                                    i_var24 = FUN_00449b30(u_var28, u_var26);
                                                    if 0 < i_var24 {
                                                        break;
                                                    }
                                                } else {
                                                    i_var24 = FUN_00449b30(u_var28, u_var26);
                                                    if i_var24 < 0 {
                                                        break;
                                                    }
                                                }
                                            }
                                            u_var27 = u_var27.wrapping_add(1);
                                            if !(u_var27 < u_var17) {
                                                break;
                                            }
                                        }
                                        if u_var27 < u_var17 && {
                                            u_var28 = FUN_00421ff0(param_1, u_var27 as i8);
                                            u_var28 as i16 != -1
                                        } {
                                            u_var13 = FUN_00478470() as u16;
                                            u_var13 = (u_var13 & 0x3fff).wrapping_add(0x4000);
                                            u_var17 = FUN_00478470() as u16;
                                            local_1c = (u_var17 & 0x7ff).wrapping_add(0x800);
                                            u_var17 = local_1c;
                                            if u_var15 != 0 {
                                                u_var17 = local_1c.wrapping_neg();
                                                u_var13 = u_var13.wrapping_neg();
                                            }
                                            let a23 = (u_var28 & 0xffff).wrapping_mul(0x23);
                                            st16(
                                                ld32(rb!().wrapping_add(a23), 0x1f),
                                                0x77,
                                                u_var13,
                                            );
                                            let pb = rb!().wrapping_add(a23).wrapping_add(0x10);
                                            st16(pb, 0, ld16(pb, 0).wrapping_add(u_var17));
                                            wr8(ld32(rb!().wrapping_add(a23), 0x1f), 0x90, 1);
                                            let q = ld32(rb!().wrapping_add(a23), 0x1f);
                                            wr8(q, 0x89, rd8(q, 0x90));
                                            wr8(rb!().wrapping_add(a23), 0x1a, 2);
                                            wr8(p5, (u_var27 as u32).wrapping_add(0x2ca), 4);
                                            u_var28 = FUN_004784d0();
                                            if (u_var28 & 0x8000) == 0 {
                                                u_var28 = 0x1b;
                                            } else {
                                                u_var28 = 0x1a;
                                            }
                                            FUN_0040a1a0(u_var28, u_var18, DAT_0071ecf4);
                                            local_1f = 0;
                                        }
                                    }
                                }
                                let b_var4 = rd8(p5, 4);
                                u_var17 = FUN_00478470() as u16;
                                u_var17 = (u_var17 & 0x3fff).wrapping_add(0x2000);
                                while ld16(addr!(DAT_0052b09b), (b_var4 as u32).wrapping_mul(0xef))
                                    < u_var17
                                {
                                    u_var17 >>= 1;
                                }
                                if u_var13 < 0x8000 {
                                    u_var17 = u_var17.wrapping_neg();
                                }
                                st16(ld32(r30!(), 0x1f), 0x77, u_var17);
                                if local_1c != 0 {
                                    local_1c >>= 1;
                                    if u_var15 == 0 {
                                        local_1c = local_1c.wrapping_neg();
                                    }
                                    let pb = r30!().wrapping_add(0x10);
                                    st16(pb, 0, ld16(pb, 0).wrapping_add(local_1c));
                                    wr8(ld32(r30!(), 0x1f), 0x90, 1);
                                    let q = ld32(r30!(), 0x1f);
                                    wr8(q, 0x89, rd8(q, 0x90));
                                }
                                wr8(p5, 7, 3);
                                let a30 = r30!();
                                if ld32(a30, 0x1b) as i32 == -1 {
                                    let a = FUN_00478470() as u16;
                                    let b = FUN_00478470() as u16;
                                    u_var13 = a;
                                    u_var15 = b;
                                    st16(
                                        ld32(a30, 0x1f),
                                        0x7f,
                                        (((a & 3) as u32 + (b & 7) as u32 + 1).wrapping_mul(0xec))
                                            as u16,
                                    );
                                } else {
                                    st16(ld32(a30, 0x1f), 0x7f, 0x2c4);
                                }
                                u_var29 &= 0xffff;
                                wr8(p5, u_var29.wrapping_add(0x2ca), 2);
                            }
                        } else {
                            st16(ld32(r30!(), 0x1f), 0x7f, 1);
                            u_var29 = FUN_0040a1a0(0x15, u_var18, DAT_0071ecf4);
                            local_1f = 0;
                            wr8(p5, 7, 4);
                        }
                        break 'l4126c7;
                    }
                }
                // post code
                c_var3 = rd8(p5, 7) as i8;
                u_var29 = ((((DAT_00525550 as u32) >> 8) & 0xffffff) << 8) | (c_var3 as u8 as u32);
                if c_var3 == 0 || c_var3 == 5 {
                    u_var13 = ld16(p5, 199);
                    let u_var22: u16 = ((DAT_00525550 as u32) >> 16) as u16;
                    u_var29 = ((u_var22 as u32) << 16) | u_var13 as u32;
                    if u_var13 <= ld16(p5, 0xc3) {
                        u_var29 = ((u_var22 as u32) << 16) | u_var13.wrapping_sub(1) as u32;
                        st16(p5, 0xc3, u_var13.wrapping_sub(1));
                    }
                }
            }
        }
        // LAB_004126c7
        let u_var21: u32 = ((u_var29 >> 16) << 16) | DAT_00703abc as u32;
        st16(i_var19 as u32, 0x16, DAT_00703abc);
        DAT_00703abc = u_var10;
        ((((u_var21 >> 8) & 0xffffff) << 8) | local_1f as u32) & 0xffff00ff
    }
}

#[allow(non_snake_case)]
unsafe fn wr16_arr_0070ac42(i: i32) {
    // SAFETY: element of the game's own u16 table.
    unsafe { st16(addr!(DAT_0070ac42), (i as u32).wrapping_mul(16), 0x3b) }
}

#[allow(non_snake_case)]
unsafe fn wradd_arr_0070ac44(i: i32) {
    // SAFETY: element of the game's own u16 table.
    unsafe {
        let o = (i as u32).wrapping_mul(16);
        let a = addr!(DAT_0070ac44);
        st16(a, o, ld16(a, o).wrapping_add(0x3b));
    }
}
