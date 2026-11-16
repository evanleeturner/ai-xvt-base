// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb7374: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009a1fe0: u32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a07cec: u32;
    static mut DAT_00521038: [u8; 1];
    static mut DAT_00521039: u8;
    static mut DAT_00521049: u8;
    static mut DAT_0052104d: u8;
    static mut DAT_00521050: [u8; 1];
    static mut DAT_00521051: u8;
    static mut DAT_00521067: u8;
    static mut DAT_00a0aba0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425d10() -> u16;
    fn FUN_00430610(a: u32, b: i8, c: i8, d: i8, e: i16) -> i32;
    fn FUN_004305d0(a: u16) -> u16;
    fn FUN_00430600() -> u8;
    fn FUN_00430540(a: i32, b: u32) -> u32;
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042fb70(
    param_1: i32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: u16,
) -> u32 {
    let mut param_2 = param_2;
    let mut param_6 = param_6;
    // SAFETY: all globals, callees and addresses are the game's own memory,
    // read and written where the original does.
    unsafe {
        if DAT_00bb7374 == 0 {
            return 0;
        }
        if param_1 == -1 {
            return 0;
        }
        if param_1 != DAT_009ecc34 {
            return 0;
        }
        let tbl = (&raw mut DAT_009e9670).cast::<u8>();
        let mut i_var5: i32 = tbl
            .byte_add((param_1 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i32>()
            .read_unaligned();
        if i_var5 == -1 {
            return 0;
        }
        if param_6 != 0xffff {
            if DAT_00bb7374 == 1 {
                param_6 >>= 1;
            }
            let u_var2 = FUN_00425d10();
            if param_6 <= u_var2 {
                return 0;
            }
        }
        let base = DAT_009a1fe8 as u32;
        let mut au_stack_18: [u32; 6] = [0; 6];
        let mut u_stack_24: u32 = 0;
        if param_2 == 0xffffffff {
            if DAT_00a07cec < DAT_009a1fe0 {
                let mut idx: usize = 0;
                let mut pcv = DAT_00a07cec
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(base);
                let mut u_var11 = DAT_00a07cec;
                loop {
                    let pt = r32(pcv.wrapping_add(0x1c));
                    let pi = r32(pcv.wrapping_add(0x18)) as i32;
                    if r8(pcv) != 0
                        && r8(pt) == 0
                        && pi != param_1
                        && r8((i_var5 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(0x16)
                            .wrapping_add(base))
                            == r8(pcv.wrapping_add(0x13))
                    {
                        au_stack_18.as_mut_ptr().add(idx).write(u_var11);
                        idx += 1;
                        u_stack_24 = u_stack_24.wrapping_add(1);
                    }
                    pcv = pcv.wrapping_add(0x23);
                    u_var11 = u_var11.wrapping_add(1);
                    if !(u_var11 < DAT_009a1fe0) {
                        break;
                    }
                }
            }
            if u_stack_24 == 0 {
                if param_3 == 0xc {
                    FUN_00430610(0x25, 0, 0, 0, -1);
                    FUN_00430610(0x25, 0, 0, 0, -1);
                }
                return 0;
            }
            let u_var2 = FUN_004305d0(u_stack_24 as u16);
            let u_var11 = u_var2 as u32;
            param_2 = au_stack_18.as_ptr().add(u_var11 as usize).read();
            if param_5 == param_2 {
                let mut u_var4 = u_var11.wrapping_add(1);
                if u_stack_24 <= u_var4 {
                    u_var4 = 0;
                }
                if u_var4 == u_var11 {
                    return 0;
                }
            }
        } else {
            if (DAT_009a1fe0 as i32) <= (param_2 as i32) {
                return 0;
            }
            if param_3 != 1
                && r8((i_var5 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x16)
                    .wrapping_add(base))
                    != r8(param_2
                        .wrapping_mul(0x23)
                        .wrapping_add(0x16)
                        .wrapping_add(base))
            {
                return 0;
            }
        }
        let pp = r32(base
            .wrapping_add(param_2.wrapping_mul(0x23))
            .wrapping_add(0x1f));
        let pu7 = r32(pp.wrapping_add(0xa9));
        let mut u_stack_1c = r32(pu7);
        if 6 < u_stack_1c {
            u_stack_1c = 0;
        }
        let a0: u32 = (r32(pu7.wrapping_add(0xb0)) as u8 as u32).wrapping_mul(0x61);
        au_stack_18[0] = a0;
        let mut u_var11: u32 = 0;
        i_var5 = a0.wrapping_add(0x72) as i32;
        let s_var10: i16;
        if param_5 == 0xffffffff || (DAT_009a1fe0 as i32) <= (param_5 as i32) {
            s_var10 = -1;
        } else {
            let pcv = param_5.wrapping_mul(0x23).wrapping_add(base);
            s_var10 = (pcv as usize as *const i16).read_unaligned();
            let pp2 = r32(pcv.wrapping_add(0x1f));
            let q = r32(pp2.wrapping_add(0xa9));
            u_var11 = r8(q.wrapping_add(0xb0)) as u32;
        }
        let t38 = (&raw mut DAT_00521038).cast::<u8>();
        let t50 = (&raw mut DAT_00521050).cast::<u8>();
        let abuf = (&raw mut DAT_00a0aba0).cast::<u8>();
        let u_var4: u32 = t38.offset(param_3 as isize).read() as u32;
        let c12 = param_3 as i8;
        let i5 = i_var5 as u32;
        if param_4 == -1 {
            match param_3 {
                3 | 5 => {
                    if FUN_00430600() == 0 {
                        return 0;
                    }
                    let u_var3 = FUN_00430540(param_3, u_var11);
                    if u_var3 == 0xffffffff {
                        return 0;
                    }
                    if u_stack_1c != 0 {
                        let i8_ = u_var11.wrapping_mul(0x61).wrapping_add(u_var3);
                        let p = abuf.byte_add(u_var4.wrapping_add(i8_) as usize);
                        p.write(p.read().wrapping_add(1));
                        FUN_00430610(
                            (t38.add(2).read() as u32)
                                .wrapping_add(u_stack_1c)
                                .wrapping_add(i5)
                                .wrapping_sub(1),
                            1,
                            2,
                            0,
                            s_var10,
                        );
                    }
                    FUN_00430610(
                        u_var3.wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        c12,
                        1,
                        s_var10,
                    );
                    return 1;
                }
                6 => {
                    if FUN_00430600() == 0 {
                        return 0;
                    }
                    let u_var3 = FUN_00430540(param_3, u_var11);
                    if u_var3 == 0xffffffff {
                        return 0;
                    }
                    let i8_ = u_var11.wrapping_mul(0x61).wrapping_add(u_var3);
                    let p = abuf.byte_add(u_var4.wrapping_add(i8_) as usize);
                    p.write(p.read().wrapping_add(1));
                    FUN_00430610(
                        u_var3.wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        c12,
                        0,
                        s_var10,
                    );
                    return 1;
                }
                9 | 10 | 0x15 => {
                    let u_var3 = FUN_00430540(param_3, u_var11);
                    if u_var3 == 0xffffffff {
                        return 0;
                    }
                    let i8_ = u_var11.wrapping_mul(0x61).wrapping_add(u_var3);
                    let p = abuf.byte_add(u_var4.wrapping_add(i8_) as usize);
                    p.write(p.read().wrapping_add(1));
                    FUN_00430610(
                        u_var3.wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        c12,
                        0,
                        s_var10,
                    );
                    return 1;
                }
                0xc => {
                    let u_var2 = FUN_004305d0(4);
                    FUN_00430610(
                        (u_var2 as u32).wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        c12,
                        0,
                        s_var10,
                    );
                    return 1;
                }
                0x10 => {
                    let u_var3 = FUN_00430540(param_3, u_var11);
                    if u_var3 == 0xffffffff {
                        return 0;
                    }
                    let p = abuf.byte_add(
                        u_var4
                            .wrapping_add(u_var11.wrapping_mul(0x61))
                            .wrapping_add(u_var3) as usize,
                    );
                    p.write(p.read().wrapping_add(1));
                    FUN_00430610(
                        u_var3.wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        c12,
                        0,
                        s_var10,
                    );
                    FUN_00430610(
                        (DAT_00521049 as u32).wrapping_add(u_var11).wrapping_add(i5),
                        1,
                        0x11,
                        1,
                        s_var10,
                    );
                    return 1;
                }
                _ => {}
            }
        } else {
            if param_3 == 1 {
                if u_stack_1c != 0 {
                    FUN_00430610(
                        (t38.read() as u32)
                            .wrapping_add(u_stack_1c)
                            .wrapping_add(i5)
                            .wrapping_sub(1),
                        1,
                        0,
                        0,
                        s_var10,
                    );
                }
                let u_var2 = FUN_00425d10();
                if u_var2 < 0x5555 {
                    FUN_00430610((DAT_00521039 as u32).wrapping_add(i5), 1, 1, 1, s_var10);
                }
                if param_4 < DAT_00521051 as i32 {
                    FUN_00430610(
                        (param_4 as u32).wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        1,
                        2,
                        s_var10,
                    );
                    return 1;
                }
                return 0;
            }
            if param_3 != 0x17 {
                if param_4 < t50.offset(param_3 as isize).read() as i32 {
                    FUN_00430610(
                        (param_4 as u32).wrapping_add(i5).wrapping_add(u_var4),
                        1,
                        c12,
                        0,
                        s_var10,
                    );
                    return 1;
                }
                return 0;
            }
            let u_var2 = FUN_00425d10();
            if u_var2 < 0x8000 {
                let u_var2 = FUN_004305d0(DAT_00521067 as u16);
                FUN_00430610(
                    (DAT_0052104d as u32)
                        .wrapping_add(i5)
                        .wrapping_add(u_var2 as u32),
                    1,
                    0x15,
                    0,
                    s_var10,
                );
            }
            if (DAT_00521067 as i32) <= param_4 {
                return 0;
            }
            FUN_00430610(
                i5.wrapping_add(param_4 as u32).wrapping_add(u_var4),
                1,
                0x17,
                1,
                s_var10,
            );
            let p = abuf.byte_add(
                u_var4
                    .wrapping_add(param_4 as u32)
                    .wrapping_add(au_stack_18[0]) as usize,
            );
            p.write(p.read().wrapping_add(1));
        }
        1
    }
}
