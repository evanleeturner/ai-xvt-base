// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_00732a3c: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_00732904: i32;
    static mut DAT_006db798: i32;
    static mut DAT_0071b704: i32;
    static mut DAT_007092c1: i8;
    static mut DAT_00525bf7: [i8; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ed40(param_1: *mut u8, param_2: u16, param_3: u32);
    fn FUN_0042b150(param_1: i32) -> i32;
}

unsafe fn r32(a: u32) -> i32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const i32).read_unaligned() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040cd90(param_1: *mut u8, param_2: u16) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the offsets the original uses.
    unsafe {
        let mut u_var11: u32 = param_2 as u32;
        let mut i_var6: u32 = (param_2 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988 as u32);
        let mut v1b: i32 = r32(i_var6.wrapping_add(0x1b));
        if v1b == -1 {
            let mut v1f: i32 = r32(i_var6.wrapping_add(0x1f));
            let u_var9_init: u16 = r16((v1f as u32).wrapping_add(0x83));
            let mut u_var9: u16 = u_var9_init;
            match r8(i_var6.wrapping_add(2)) {
                0..=2 => {
                    if DAT_007092c1 == 0 {
                        let mut u_var12: u32 = (DAT_00732a3c as u32) & 0xffff;
                        let mut i_var7: i32 = -1;
                        if (u_var12 as i32) < DAT_006d4980 {
                            loop {
                                i_var7 = u_var12.wrapping_mul(0x23) as i32;
                                let i_var1: u32 = (DAT_006d4988 as u32).wrapping_add(i_var7 as u32);
                                u_var9 = u_var12 as u16;
                                if r8((DAT_006d4988 as u32)
                                    .wrapping_add(0x16)
                                    .wrapping_add(i_var7 as u32))
                                    != r8(i_var6.wrapping_add(0x16))
                                    && r8(i_var1.wrapping_add(3)) != 0
                                {
                                    v1b = r32(i_var1.wrapping_add(0x1b));
                                    v1f = r32(i_var1.wrapping_add(0x1f));
                                    if v1b != -1 && v1f != 0 {
                                        FUN_0040ed40(
                                            (v1f as u32).wrapping_add(0x12) as *mut u8,
                                            u_var9,
                                            u_var11,
                                        );
                                    }
                                    let c_var2 = r8((DAT_006d4988 as u32)
                                        .wrapping_add(2)
                                        .wrapping_add(i_var7 as u32));
                                    if c_var2 == 4 || c_var2 == 3 || c_var2 == 5 {
                                        let mut i1 = r32((DAT_006d4988 as u32)
                                            .wrapping_add(i_var7 as u32)
                                            .wrapping_add(0x1f));
                                        if i1 != 0 {
                                            i1 = r32((i1 as u32).wrapping_add(0xa7));
                                            if i1 != 0 {
                                                let i_var8 = FUN_0042b150(i1);
                                                if r8((i_var8 as u32).wrapping_add(0x34)) != 0x1e
                                                    && r16((i1 as u32).wrapping_add(0x68)) as i16
                                                        == -1
                                                {
                                                    let vx = r32((DAT_006d4988 as u32)
                                                        .wrapping_add(0x1f)
                                                        .wrapping_add(i_var7 as u32));
                                                    FUN_0040ed40(
                                                        (vx as u32).wrapping_add(0x12) as *mut u8,
                                                        u_var9,
                                                        u_var11,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                u_var12 = u_var9.wrapping_add(1) as u32;
                                if !((u_var12 as i32) < DAT_006d4980) {
                                    break;
                                }
                            }
                        }
                        u_var11 = ((i_var7 as u32) & 0xffff0000) | (DAT_00732a38 as u16 as u32);
                        if (((DAT_00732a38 as u32) & 0xffff) as i32)
                            < DAT_00703c88.wrapping_add(DAT_00732a38)
                        {
                            loop {
                                let b_var3 = r8((u_var11 & 0xffff)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(3)
                                    .wrapping_add(DAT_006d4988 as u32));
                                if 99 < b_var3 && b_var3 < 0x6a {
                                    FUN_0040ed40(param_1, param_2, u_var11);
                                }
                                u_var9 = (u_var11 as u16).wrapping_add(1);
                                u_var11 = (u_var11 & 0xffff0000) | (u_var9 as u32);
                                if !((u_var9 as i32) < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                                    break;
                                }
                            }
                            return;
                        }
                    }
                }
                3..=5 => {
                    i_var6 = r32((v1f as u32).wrapping_add(0xa7)) as u32;
                    let i_var7 = FUN_0042b150(i_var6 as i32);
                    if r8((i_var7 as u32).wrapping_add(0x34)) != 0x1e
                        && r16(i_var6.wrapping_add(0x68)) as i16 == -1
                    {
                        let mut u_var12: u32 = DAT_00732a3c as u16 as u32;
                        if (((DAT_00732a3c as u32) & 0xffff) as i32) < DAT_006d4980 {
                            let mut pbvar14: *mut u8 = param_1;
                            let mut u_var15: u32 = u_var11;
                            let mut u_var16: u32 = u_var12;
                            loop {
                                if (u_var12 as u16) != param_2 {
                                    i_var6 = (u_var12 & 0xffff)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(DAT_006d4988 as u32);
                                    if r8(i_var6.wrapping_add(3)) != 0 {
                                        let mut do_call = false;
                                        v1b = r32(i_var6.wrapping_add(0x1b));
                                        if v1b == -1 {
                                            if r8(i_var6.wrapping_add(2)) != 13 {
                                                pbvar14 = param_1;
                                                u_var15 = u_var11;
                                                u_var16 = u_var12;
                                                do_call = true;
                                            }
                                        } else {
                                            v1f = r32(i_var6.wrapping_add(0x1f));
                                            if v1f != 0 {
                                                pbvar14 =
                                                    (v1f as u32).wrapping_add(0x12) as *mut u8;
                                                u_var15 = u_var12;
                                                u_var16 = u_var11;
                                                do_call = true;
                                            }
                                        }
                                        if do_call {
                                            FUN_0040ed40(pbvar14, u_var15 as u16, u_var16);
                                        }
                                    }
                                }
                                u_var9 = (u_var12 as u16).wrapping_add(1);
                                u_var12 = (u_var12 & 0xffff0000) | (u_var9 as u32);
                                if DAT_006d4980 <= (u_var9 as i32) {
                                    return;
                                }
                            }
                        }
                    }
                }
                6 | 7 => {
                    u_var11 = 0xffff0000 | (DAT_00732a3c as u16 as u32);
                    let u_var4: u16 = r16((DAT_006db798 as u32).wrapping_add(4).wrapping_add(
                        (param_2 as u32)
                            .wrapping_sub(DAT_0071b704 as u32)
                            .wrapping_mul(8),
                    ));
                    if (((DAT_00732a3c as u32) & 0xffff) as i32) < DAT_00732904 {
                        loop {
                            i_var6 = (u_var11 & 0xffff)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_006d4988 as u32);
                            let u_var10: u16 = u_var11 as u16;
                            let mut cond = false;
                            if r8(i_var6.wrapping_add(3)) != 0 {
                                if ((u_var11 & 0xffff) as i32) < DAT_006d4980 {
                                    cond = true;
                                } else {
                                    let idx = r8(i_var6.wrapping_add(3)) as usize;
                                    let t = (&raw const DAT_00525bf7).cast::<i8>().add(idx).read();
                                    if t != 0 && u_var10 != param_2 {
                                        let vx = r32(i_var6.wrapping_add(0x1f));
                                        let u16v = r16((vx as u32).wrapping_add(0x83));
                                        if u16v != u_var9 {
                                            cond = true;
                                        }
                                    }
                                }
                            }
                            if cond && u_var9 != u_var10 && r8(i_var6.wrapping_add(2)) != 13 {
                                let mut b_var5 = false;
                                let u_var12 = u_var4 as u32;
                                if (u_var12 as i32) < DAT_006d4980
                                    && DAT_00732a3c <= (u_var12 as i32)
                                {
                                    v1b = r32(u_var12
                                        .wrapping_mul(0x23)
                                        .wrapping_add(0x1b)
                                        .wrapping_add(DAT_006d4988 as u32));
                                    if v1b != -1 {
                                        b_var5 = true;
                                    }
                                }
                                if !b_var5 {
                                    v1b = r32((u_var9 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(0x1b)
                                        .wrapping_add(DAT_006d4988 as u32));
                                }
                                if b_var5 || v1b != -1 || u_var4 == u_var10 {
                                    FUN_0040ed40(param_1, param_2, u_var11);
                                }
                            }
                            u_var11 = (u_var11 & 0xffff0000) | (u_var10.wrapping_add(1) as u32);
                            if !((u_var10.wrapping_add(1) as i32) < DAT_00732904) {
                                break;
                            }
                        }
                    }
                    if DAT_007092c1 == 0 {
                        u_var11 = DAT_00732a38 as u16 as u32;
                        if (((DAT_00732a38 as u32) & 0xffff) as i32)
                            < DAT_00703c88.wrapping_add(DAT_00732a38)
                        {
                            loop {
                                if r8((u_var11 & 0xffff)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(3)
                                    .wrapping_add(DAT_006d4988 as u32))
                                    != 0
                                {
                                    FUN_0040ed40(param_1, param_2, u_var11);
                                }
                                u_var9 = (u_var11 as u16).wrapping_add(1);
                                u_var11 = (u_var11 & 0xffff0000) | (u_var9 as u32);
                                if !((u_var9 as i32) < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            let v1f = r32(i_var6.wrapping_add(0x1f));
            let vx = r32((v1f as u32).wrapping_add(0xa7));
            let s16 = r16((vx as u32).wrapping_add(0x1df)) as i16;
            if s16 != 0 {
                u_var11 = ((v1b as u32) & 0xffff0000) | (DAT_00732a3c as u16 as u32);
                if (((DAT_00732a3c as u32) & 0xffff) as i32) < DAT_006d4980 {
                    loop {
                        let i_var7 = (u_var11 & 0xffff)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32);
                        if r8(i_var7.wrapping_add(3)) != 0
                            && (u_var11 as u16) != param_2
                            && r8(i_var7.wrapping_add(2)) != 13
                            && r8(i_var6.wrapping_add(0x16)) != r8(i_var7.wrapping_add(0x16))
                        {
                            FUN_0040ed40(param_1, param_2, u_var11);
                        }
                        let u_var9 = (u_var11 as u16).wrapping_add(1);
                        u_var11 = (u_var11 & 0xffff0000) | (u_var9 as u32);
                        if !((u_var9 as i32) < DAT_006d4980) {
                            break;
                        }
                    }
                }
            }
            if DAT_007092c1 == 0 {
                u_var11 = DAT_00732a38 as u16 as u32;
                if (((DAT_00732a38 as u32) & 0xffff) as i32)
                    < DAT_00703c88.wrapping_add(DAT_00732a38)
                {
                    loop {
                        if r8((u_var11 & 0xffff)
                            .wrapping_mul(0x23)
                            .wrapping_add(3)
                            .wrapping_add(DAT_006d4988 as u32))
                            != 0
                        {
                            FUN_0040ed40(param_1, param_2, u_var11);
                        }
                        let u_var9 = (u_var11 as u16).wrapping_add(1);
                        u_var11 = (u_var11 & 0xffff0000) | (u_var9 as u32);
                        if !((u_var9 as i32) < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                            break;
                        }
                    }
                    return;
                }
            }
        }
    }
}
