// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: *mut core::ffi::c_void;
    static mut DAT_009d7680: u32;
    static mut DAT_009cc450: u16;
    static mut DAT_009a8c30: unsafe extern "C" fn(u32, u32) -> u32;
    static mut DAT_0052747c: u16;
    static mut DAT_009a8074: u32;
    static mut DAT_00a08374: i32;
    static mut DAT_00556540: [u8; 1];
    static mut DAT_00556360: [u8; 1];
    static mut DAT_00556720: [u8; 1];
    static mut DAT_005233ec: i32;
    static mut DAT_00527e90: i32;
    static mut DAT_009e9689: [u8; 1];
    static mut DAT_009e968a: [u8; 1];
    static mut DAT_009e9699: [u8; 1];
    static mut DAT_009e9a5b: [u8; 1];
    static mut DAT_009e9a5f: [u8; 1];
    static mut DAT_009e9a63: [u8; 1];
    static mut DAT_009e9a67: [u8; 1];
    static mut DAT_009e9a84: [u8; 1];
    static mut DAT_009e9683: [u8; 1];
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_00523408: u8;
    static mut DAT_005233c8: u8;
    static mut DAT_009a8062: u16;
    static mut DAT_009fd39c: i32;
    static mut DAT_009fd3a0: i32;
    static mut DAT_009fd398: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_009a8e2c: i32;
    static mut DAT_009a8e30: i32;
    static mut DAT_009a8e34: i32;
    static mut DAT_009ec476: u8;
    static mut DAT_00a08104: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a8e24: u32;
    static mut DAT_009a73f4: i8;
    static mut DAT_009d6940: [u8; 2];
    static mut DAT_009cd274: i16;
    static mut DAT_009a8d9c: u32;
    static mut DAT_00521f1c: [u8; 1];
    static mut DAT_005235f4: i32;
    static mut DAT_00a00860: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00426f40(a: u16, b: u16, c: i16, d: u32);
    fn FUN_0040b530();
    fn FUN_00442bf0(a: u32, b: i32, c: i32);
    fn FUN_00485f00(a: i32, b: i32);
    fn FUN_004aebe0(a: u32, b: u32) -> i32;
    fn FUN_004aed40(a: u32, b: u32) -> i32;
    fn FUN_004aec90(a: u32, b: u32) -> i32;
    fn FUN_004037b0(a: u32, b: i32, c: i32, d: i32);
    fn FUN_00427420(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00427490(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00427500(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00427a60(a: i16, b: i16, c: i16, d: i32, e: i32);
    fn FUN_0044fdf0(a: u32);
    fn FUN_0041fc20(a: u32);
    fn FUN_00472400(a: u32);
    fn FUN_0041ff70(a: u32);
    fn FUN_00446550(a: u16);
    fn FUN_0044ff10(a: u32, b: u32) -> u8;
    fn FUN_00401000(a: u32);
    fn FUN_004865e0();
    fn FUN_004012c0(a: i32);
    fn FUN_00427570(a: i32, b: i32) -> i32;
    fn FUN_00427590(a: i32, b: i32) -> i32;
    fn FUN_00442bc0(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_00486200();
    fn FUN_00427070();
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn rh(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn rw(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn plo() -> u32 {
    // SAFETY: the game's own global.
    unsafe { (DAT_009ecc34 as usize as u32).wrapping_mul(0x5bd) }
}

unsafe fn ent(i: u32) -> u32 {
    // SAFETY: the game's own global.
    unsafe { i.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004422c0(
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: i16,
) {
    // SAFETY: all globals, callees and addresses are the game's own memory,
    // accessed exactly where the original does.
    unsafe {
        let mut i_stack_1c: i32 = 0;
        let mut i_stack_18: i32 = 0;
        let mut i_stack_14: i32 = 0;

        let u_var9 = DAT_009d7680;
        let u_var8 = DAT_009cc450;
        let a5b = (&raw mut DAT_009e9a5b) as usize as u32;
        let a5f = (&raw mut DAT_009e9a5f) as usize as u32;
        let a63 = (&raw mut DAT_009e9a63) as usize as u32;
        let a89 = (&raw mut DAT_009e9689) as usize as u32;
        let a8a = (&raw mut DAT_009e968a) as usize as u32;
        let a99 = (&raw mut DAT_009e9699) as usize as u32;
        let a67 = (&raw mut DAT_009e9a67) as usize as u32;
        let a84 = (&raw mut DAT_009e9a84) as usize as u32;
        let a83 = (&raw mut DAT_009e9683) as usize as u32;
        let a70 = (&raw mut DAT_009e9670) as usize as u32;

        let pu_var4 = rw(a5b.wrapping_add(plo()));
        let u_var5 = rw(a5f.wrapping_add(plo()));
        let u_var6 = rw(a63.wrapping_add(plo()));
        DAT_009d7680 = 0;
        let u_var13 = DAT_009a8c30(param_1 & 0xffff, param_2 & 0xffff);
        FUN_00426f40(param_3, param_4, param_5, u_var13);
        if param_5 != 0 {
            let mut p17 = (DAT_0052747c as u32).wrapping_add(DAT_009a8074);
            let mut p16: u32 = if DAT_00a08374 == 0 {
                (&raw mut DAT_00556540) as usize as u32
            } else if DAT_00a08374 == 0x90 {
                (&raw mut DAT_00556360) as usize as u32
            } else if DAT_00a08374 == 0x120 {
                (&raw mut DAT_00556720) as usize as u32
            } else {
                pu_var4
            };
            let mut s_var12: i16 = if DAT_005233ec == 0x13 { 200 } else { 0x1e0 };
            loop {
                let v = rb(p16);
                p16 = p16.wrapping_add(1);
                (p17 as usize as *mut u8).write(v);
                p17 = p17.wrapping_add(1);
                s_var12 = s_var12.wrapping_sub(1);
                if s_var12 == 0 {
                    break;
                }
            }
        }
        if DAT_00527e90 != 0 {
            FUN_0040b530();
        }
        let mut t16 = rh(a8a.wrapping_add(plo()));
        let mut packed =
            (((DAT_009ecc34 as usize as u32).wrapping_mul(0x16f) >> 16) << 16) | (t16 as u32);
        FUN_00442bf0(packed, 1, DAT_009ecc34 as usize as i32);
        if rb(a89.wrapping_add(plo())) == 0 {
            DAT_009cc450 |= 0x400;
        } else {
            DAT_009cc450 |= 0x200;
        }
        DAT_00523408 = 0x30;
        FUN_00485f00(DAT_009ecc34 as usize as i32, 1);
        DAT_009a8062 = 0;
        let mut i_var14 = plo();
        let mut t32 = rw(a5b.wrapping_add(plo()));
        DAT_009fd39c = (DAT_00a07cdc as u32).wrapping_sub(t32) as i32;
        t32 = rw(a5f.wrapping_add(plo()));
        DAT_009fd3a0 = (DAT_00a07cd8 as u32).wrapping_sub(t32) as i32;
        t32 = rw(a63.wrapping_add(i_var14));
        DAT_009fd398 = (DAT_00a07cd4 as u32).wrapping_sub(t32) as i32;
        t16 = rh(a8a.wrapping_add(i_var14));
        if rb(a89.wrapping_add(i_var14)) != 0 && (t16 as i32) < DAT_009a1fe0 {
            let mut t16b = rh(a99.wrapping_add(i_var14));
            DAT_009d8a58 = FUN_004aebe0(rb(ent(t16 as u32).wrapping_add(3)) as u32, t16b as u32);
            t16 = rh(a8a.wrapping_add(plo()));
            t16b = rh(a99.wrapping_add(plo()));
            DAT_009d8a54 = FUN_004aed40(rb(ent(t16 as u32).wrapping_add(3)) as u32, t16b as u32);
            t16 = rh(a8a.wrapping_add(plo()));
            t16b = rh(a99.wrapping_add(plo()));
            let r = FUN_004aec90(rb(ent(t16 as u32).wrapping_add(3)) as u32, t16b as u32);
            DAT_009d8b60 = r.wrapping_neg();
            t16 = rh(a8a.wrapping_add(plo()));
            FUN_004037b0(ent(t16 as u32), DAT_009d8a58, DAT_009d8a54, DAT_009d8b60);
            t32 = rw(a5b.wrapping_add(plo()));
            i_stack_14 = (DAT_00a07cdc as u32)
                .wrapping_sub(t32)
                .wrapping_add(DAT_009d8a58 as u32) as i32;
            t32 = rw(a5f.wrapping_add(plo()));
            i_stack_18 = (DAT_00a07cd8 as u32)
                .wrapping_sub(t32)
                .wrapping_add(DAT_009d8a54 as u32) as i32;
            t32 = rw(a63.wrapping_add(plo()));
            i_stack_1c = (DAT_009d8b60 as u32)
                .wrapping_sub(t32)
                .wrapping_add(DAT_00a07cd4 as u32) as i32;
        }
        DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        let u_var18 = rh(a8a.wrapping_add(plo()));
        let e = ent(u_var18 as u32);
        let p_obj = rw(e.wrapping_add(0x1f));
        if p_obj == 0 {
            let ty = rb(e.wrapping_add(2));
            if 7 < ty && ty < 0xc {
                let s14 = rh(e.wrapping_add(0x14)) as i16;
                let s12 = rh(e.wrapping_add(0x12)) as i16;
                let s10 = rh(e.wrapping_add(0x10)) as i16;
                FUN_00427a60(s14, s12, s10, 0, 0);
                DAT_009ec476 = 1;
                FUN_00446550(u_var18);
            }
        } else {
            match rb(e.wrapping_add(2)) {
                0..=5 => {
                    DAT_00a08104 = rw(p_obj.wrapping_add(0xa9)) as i32;
                    let s14 = rh(e.wrapping_add(0x14)) as i16;
                    let s12 = rh(e.wrapping_add(0x12)) as i16;
                    let s10 = rh(e.wrapping_add(0x10)) as i16;
                    FUN_00427a60(s14, s12, s10, 0, e as i32);
                    t16 = rh(a8a.wrapping_add(plo()));
                    FUN_0044fdf0(t16 as u32);
                    t16 = rh(a8a.wrapping_add(plo()));
                    packed = (((DAT_009ecc34 as usize as u32).wrapping_mul(0x16f) >> 16) << 16)
                        | (t16 as u32);
                    FUN_0041fc20(packed);
                    t16 = rh(a8a.wrapping_add(plo()));
                    FUN_00472400(ent(t16 as u32));
                }
                6 | 7 => {
                    let s14 = rh(e.wrapping_add(0x14)) as i16;
                    let s12 = rh(e.wrapping_add(0x12)) as i16;
                    let s10 = rh(e.wrapping_add(0x10)) as i16;
                    FUN_00427a60(s14, s12, s10, 0, e as i32);
                    t16 = rh(a8a.wrapping_add(plo()));
                    FUN_0041ff70(t16 as u32);
                }
                _ => {}
            }
        }
        let mut u_var13: u32 = 0;
        if 0 < DAT_00a07ce8 {
            loop {
                if u_var13 == DAT_009a8e24 && (DAT_009a73f4 == 0 || DAT_009d6940[1] != 0) {
                    u_var13 = (DAT_009cd274 as u16) as u32;
                    if ((DAT_009cd274 as u16) as i32) == DAT_00a07ce8 {
                        break;
                    }
                }
                let u_var18 = u_var13 as u16;
                i_var14 = plo();
                let t16 = rh(a67.wrapping_add(i_var14));
                let t16b = rh(a84.wrapping_add(i_var14));
                if t16 != u_var18 || t16b != 0 || DAT_00a00860 != 0 {
                    let i_var7 = u_var13.wrapping_mul(0x23);
                    let mut b_var2 = rb((DAT_009a1fe8 as u32).wrapping_add(3).wrapping_add(i_var7));
                    let i_var15 = (DAT_009a1fe8 as u32).wrapping_add(i_var7);
                    if b_var2 != 0 {
                        DAT_009a8d9c = rw(((&raw mut DAT_00521f1c) as usize as u32)
                            .wrapping_add((b_var2 as u32).wrapping_mul(0x1a)));
                        b_var2 = rb(i_var15.wrapping_add(2));
                        if 5 < b_var2 {
                            if b_var2 < 8 {
                                if rb(a83.wrapping_add(i_var14)) == 0 {
                                    let p_obj = rw(i_var15.wrapping_add(0x1f));
                                    let u_var3 = rh(p_obj.wrapping_add(0x85));
                                    let t16 = rh(a8a.wrapping_add(i_var14));
                                    let t32 = rw(a70.wrapping_add(i_var14));
                                    if (t16 == u_var3 || t32 == u_var3 as u32)
                                        && FUN_0044ff10(u_var13, DAT_009a8d9c) != 0
                                    {
                                        let i_ent = (DAT_009a1fe8 as u32).wrapping_add(i_var7);
                                        let s14 = rh(i_ent.wrapping_add(0x14)) as i16;
                                        let s12 = rh(i_ent.wrapping_add(0x12)) as i16;
                                        let s10 = rh((DAT_009a1fe8 as u32)
                                            .wrapping_add(0x10)
                                            .wrapping_add(i_var7))
                                            as i16;
                                        FUN_00427a60(s14, s12, s10, 0, i_ent as i32);
                                        FUN_0041ff70(u_var18 as u32);
                                    }
                                } else {
                                    let p_obj = rw(i_var15.wrapping_add(0x1f));
                                    let u_var3 = rh(p_obj.wrapping_add(0x85));
                                    let t16 = rh(a8a.wrapping_add(i_var14));
                                    if u_var3 == t16 && FUN_0044ff10(u_var13, DAT_009a8d9c) != 0 {
                                        let i_ent = (DAT_009a1fe8 as u32).wrapping_add(i_var7);
                                        let s14 = rh(i_ent.wrapping_add(0x14)) as i16;
                                        let s12 = rh(i_ent.wrapping_add(0x12)) as i16;
                                        let s10 = rh((DAT_009a1fe8 as u32)
                                            .wrapping_add(0x10)
                                            .wrapping_add(i_var7))
                                            as i16;
                                        FUN_00427a60(s14, s12, s10, 0, i_ent as i32);
                                        FUN_0041ff70(u_var18 as u32);
                                    }
                                }
                            } else if b_var2 == 0xd && FUN_0044ff10(u_var13, DAT_009a8d9c) != 0 {
                                let i_ent = (DAT_009a1fe8 as u32).wrapping_add(i_var7);
                                let s14 = rh(i_ent.wrapping_add(0x14)) as i16;
                                let s12 = rh(i_ent.wrapping_add(0x12)) as i16;
                                let s10 = rh((DAT_009a1fe8 as u32)
                                    .wrapping_add(0x10)
                                    .wrapping_add(i_var7))
                                    as i16;
                                FUN_00427a60(s14, s12, s10, 0, i_ent as i32);
                                FUN_00401000(u_var13);
                            }
                        }
                    }
                }
                u_var13 = u_var18.wrapping_add(1) as u32;
                if (u_var13 as i32) >= DAT_00a07ce8 {
                    break;
                }
            }
        }
        FUN_004865e0();
        if DAT_009a8062 != 0 {
            DAT_005235f4 = 0;
            FUN_004012c0(0);
            DAT_005235f4 = 0;
        }
        if rb(a89.wrapping_add(plo())) != 0 {
            let t16 = rh(a8a.wrapping_add(plo()));
            let u_var13 = t16 as u32;
            if (u_var13 as i32) < DAT_009a1fe0 && rb(ent(u_var13).wrapping_add(2)) != 0 {
                DAT_009a8e2c = FUN_00427420(i_stack_14, i_stack_18, i_stack_1c);
                DAT_009a8e30 = FUN_00427490(i_stack_14, i_stack_18, i_stack_1c);
                DAT_009a8e34 = FUN_00427500(i_stack_14, i_stack_18, i_stack_1c);
                let i14 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                let i15 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                FUN_00442bc0(i14.wrapping_sub(2), i15.wrapping_sub(2), 4, 4, 0xce);
            }
        }
        FUN_00486200();
        DAT_00523408 = DAT_005233c8;
        FUN_00427070();
        let off = plo();
        DAT_009cc450 = u_var8;
        ((a5b.wrapping_add(off)) as usize as *mut u32).write_unaligned(pu_var4);
        ((a5f.wrapping_add(off)) as usize as *mut u32).write_unaligned(u_var5);
        ((a63.wrapping_add(off)) as usize as *mut u32).write_unaligned(u_var6);
        DAT_009d7680 = u_var9;
    }
}
