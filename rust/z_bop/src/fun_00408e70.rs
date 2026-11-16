// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099945c: i32;
    static mut DAT_0099946a: [u8; 4];
    static mut DAT_0099944a: [u8; 4];
    static mut DAT_0054f98c: i32;
    static mut DAT_0054f99c: i32;
    static mut DAT_0054f978: i32;
    static mut DAT_0054f998: i32;
    static mut DAT_0054f994: i32;
    static mut DAT_0054f9a0: i32;
    static mut DAT_0054f988: i32;
    static mut DAT_00528c5c: i32;
    static mut DAT_00999478: i32;
    static mut _DAT_00518070: f32;
    static mut DAT_0053f8f0: [i32; 1];
    static mut DAT_0053f8ec: [i32; 1];
    static mut DAT_0053f8f4: [i32; 1];
    static mut DAT_0052f870: [i32; 1];
    static mut DAT_0052f86c: [i32; 1];
    static mut DAT_009a1fe4: u32;
    static mut DAT_0053f970: i32;
    static mut DAT_00527e94: i32;
    static mut DAT_0051a54c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ac80(a: i32, b: i32, c: i32);
    fn FUN_004096d0(a: i32, b: i32, c: i32);
    fn FUN_00409c10(a: i32, b: i32, c: i32);
    fn FUN_0040a1a0(a: i32, b: i32, c: i32);
    fn FUN_0040a6e0(a: i32, b: i32, c: i32);
    fn FUN_0040b350(a: i32, b: i32) -> u32;
    fn __ftol() -> i64;
    fn FUN_00407e40(a: i32, b: i32, c: *mut u32, d: u32) -> *mut u8;
    fn FUN_00407f10(a: i32, b: i32, c: *mut u16, d: *mut u8) -> *mut u8;
}

fn ri(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}
fn rf(a: i32) -> f32 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *const f32).read_unaligned() }
}
fn rb(a: i32) -> u8 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *const u8).read() }
}
fn rs(a: i32) -> u16 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *const u16).read_unaligned() }
}
fn wi(a: i32, v: i32) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *mut i32).write_unaligned(v) }
}
fn wf(a: i32, v: f32) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *mut f32).write_unaligned(v) }
}
fn wb(a: i32, v: u8) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *mut u8).write(v) }
}
fn ws(a: i32, v: u16) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00408e70(param_1: *mut i32) {
    // SAFETY: all globals, callees and pointers are the game's own memory;
    // every access happens where the original performs it.
    unsafe {
        let p = param_1 as usize as i32;
        let a8f0 = (&raw mut DAT_0053f8f0).cast::<i32>() as usize as i32;
        let a8ec = (&raw mut DAT_0053f8ec).cast::<i32>() as usize as i32;
        let a8f4 = (&raw mut DAT_0053f8f4).cast::<i32>() as usize as i32;
        let a870 = (&raw mut DAT_0052f870).cast::<i32>() as usize as i32;
        let a86c = (&raw mut DAT_0052f86c).cast::<i32>() as usize as i32;

        let s3: i32 = (&raw mut DAT_0099946a).cast::<i32>().read_unaligned();
        let mut i_var7: i32 = 0;
        let mut local_8: i32 = 0;
        let i_var1: i32 = DAT_0099945c.wrapping_add(ri(p.wrapping_add(0xbc)).wrapping_mul(0x18));
        DAT_0054f98c = ri(p.wrapping_add(0xc8)).wrapping_add(ri(p.wrapping_add(0xbc)));
        let i_var9: i32 = (&raw mut DAT_0099944a).cast::<i32>().read_unaligned();
        let mut local_28: i32 = ri(p.wrapping_add(0xb8))
            .wrapping_mul(0x70)
            .wrapping_add(i_var9);
        let mut puvar4: i32 = s3;
        DAT_0054f99c = DAT_0054f98c;
        if 0 < DAT_0054f98c {
            loop {
                wi(puvar4, -1);
                i_var7 = i_var7.wrapping_add(1);
                puvar4 = puvar4.wrapping_add(4);
                if !(i_var7 < DAT_0054f98c) {
                    break;
                }
            }
        }
        let mut local_10: i32 = 0;
        let mut local_18: *mut u8 = core::ptr::null_mut();
        let mut local_4: *mut u8 = core::ptr::null_mut();
        if 0 < ri(p.wrapping_add(0xc4)) {
            loop {
                i_var7 = ri(local_28)
                    .wrapping_mul(0x40)
                    .wrapping_add(ri(p.wrapping_add(0xa0)));
                let mut local_c: i32 =
                    4i32.wrapping_sub((ri(i_var7.wrapping_add(0x1c)) == -1) as i32);
                DAT_0054f978 = local_c;
                if ri(DAT_00528c5c.wrapping_add(0x1c)) == 0 {
                    if 0 < local_c {
                        let mut pvar8 = i_var7.wrapping_add(0x20);
                        let mut pvar13 = a8f0;
                        loop {
                            i_var7 = ri(DAT_00999478
                                .wrapping_add(ri(pvar8.wrapping_sub(0x20)).wrapping_mul(4)));
                            wi(pvar13, i_var7);
                            let i_var11 = DAT_0054f99c;
                            let pf =
                                ri(p.wrapping_add(0x80)).wrapping_add(ri(pvar8).wrapping_mul(8));
                            let pu4 = i_var1.wrapping_add(i_var7.wrapping_mul(0x18));
                            if rf(i_var1
                                .wrapping_add(0x10)
                                .wrapping_add(i_var7.wrapping_mul(0x18)))
                                != rf(pf)
                                || rf(pu4.wrapping_add(0x14)) != rf(pf.wrapping_add(4))
                            {
                                let pu15 = i_var1.wrapping_add(DAT_0054f99c.wrapping_mul(0x18));
                                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                                wi(pu15, ri(pu4));
                                wi(pu15.wrapping_add(4), ri(pu4.wrapping_add(4)));
                                wi(pu15.wrapping_add(12), ri(pu4.wrapping_add(12)));
                                wi(pu15.wrapping_add(8), ri(pu4.wrapping_add(8)));
                                wi(
                                    pu15.wrapping_add(16),
                                    ri(ri(p.wrapping_add(0x80))
                                        .wrapping_add(ri(pvar8).wrapping_mul(8))),
                                );
                                wi(
                                    pu15.wrapping_add(20),
                                    ri(ri(p.wrapping_add(0x80))
                                        .wrapping_add(4)
                                        .wrapping_add(ri(pvar8).wrapping_mul(8))),
                                );
                                wi(pvar13, i_var11);
                            }
                            pvar8 = pvar8.wrapping_add(4);
                            pvar13 = pvar13.wrapping_add(4);
                            local_c = local_c.wrapping_sub(1);
                            if local_c == 0 {
                                break;
                            }
                        }
                    }
                } else {
                    let mut local_30: f32 = 1.0;
                    let i_var11 = ri(ri(p.wrapping_add(0xa8)).wrapping_add(0x10));
                    let i_var12 = ri(ri(p.wrapping_add(0xa8)).wrapping_add(0x14));
                    let mut i_var14 = i_var12;
                    if i_var12 < i_var11 {
                        loop {
                            local_30 *= _DAT_00518070;
                            i_var14 = i_var14.wrapping_mul(2);
                            if !(i_var14 < i_var11) {
                                break;
                            }
                        }
                    } else if i_var11 < i_var12 {
                        local_30 = 1.0;
                        i_var14 = i_var11;
                        loop {
                            local_30 *= _DAT_00518070;
                            i_var14 = i_var14.wrapping_mul(2);
                            if !(i_var14 < i_var12) {
                                break;
                            }
                        }
                    }
                    if 0 < local_c {
                        let mut pvar8 = i_var7.wrapping_add(0x20);
                        let mut local_1c = a8f0;
                        loop {
                            let mut local_24 =
                                rf(ri(p.wrapping_add(0x80))
                                    .wrapping_add(ri(pvar8).wrapping_mul(8)));
                            let mut local_20 = rf(ri(p.wrapping_add(0x80))
                                .wrapping_add(ri(pvar8).wrapping_mul(8))
                                .wrapping_add(4));
                            if i_var12 < i_var11 {
                                local_20 = local_30 * local_20;
                            } else if i_var11 < i_var12 {
                                local_24 = local_30 * local_24;
                            }
                            i_var7 = ri(DAT_00999478
                                .wrapping_add(ri(pvar8.wrapping_sub(0x20)).wrapping_mul(4)));
                            wi(local_1c, i_var7);
                            let i_var14b = DAT_0054f99c;
                            let pu4 = i_var1.wrapping_add(i_var7.wrapping_mul(0x18));
                            if rf(i_var1
                                .wrapping_add(0x10)
                                .wrapping_add(i_var7.wrapping_mul(0x18)))
                                != local_24
                                || rf(pu4.wrapping_add(0x14)) != local_20
                            {
                                let pu15 = i_var1.wrapping_add(DAT_0054f99c.wrapping_mul(0x18));
                                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                                wi(pu15, ri(pu4));
                                wi(pu15.wrapping_add(4), ri(pu4.wrapping_add(4)));
                                wi(pu15.wrapping_add(12), ri(pu4.wrapping_add(12)));
                                wi(pu15.wrapping_add(8), ri(pu4.wrapping_add(8)));
                                wf(pu15.wrapping_add(16), local_24);
                                wf(pu15.wrapping_add(20), local_20);
                                wi(local_1c, i_var14b);
                            }
                            pvar8 = pvar8.wrapping_add(4);
                            local_1c = local_1c.wrapping_add(4);
                            local_c = local_c.wrapping_sub(1);
                            if local_c == 0 {
                                break;
                            }
                        }
                    }
                }
                if ri(local_28.wrapping_add(12)) == -1 {
                    if 0 < DAT_0054f978 {
                        let mut s = a8f0;
                        let mut d = a870;
                        let mut n = DAT_0054f978;
                        while n != 0 {
                            wi(d, ri(s));
                            s = s.wrapping_add(4);
                            d = d.wrapping_add(4);
                            n = n.wrapping_sub(1);
                        }
                    }
                    i_var7 = DAT_0054f978;
                    let mut i_var11 = 0i32;
                    DAT_0054f998 = DAT_0054f978;
                    DAT_0054f978 = 0;
                    if 0 < i_var7 {
                        let mut pvar8 = a870;
                        i_var7 = ri(a86c.wrapping_add(i_var7.wrapping_mul(4)));
                        loop {
                            let i_var12 = ri(pvar8);
                            pvar8 = pvar8.wrapping_add(4);
                            i_var11 = i_var11.wrapping_add(1);
                            FUN_0040ac80(i_var7, i_var12, i_var1);
                            i_var7 = i_var12;
                            if !(i_var11 < DAT_0054f998) {
                                break;
                            }
                        }
                    }
                }
                i_var7 = 0;
                wi(local_28.wrapping_add(12), DAT_009a1fe4 as i32);
                DAT_0054f998 = 0;
                if 0 < DAT_0054f978 {
                    let mut pvar8 = a8f0;
                    let mut i_var11 = ri(a8ec.wrapping_add(DAT_0054f978.wrapping_mul(4)));
                    loop {
                        let i_var12 = ri(pvar8);
                        pvar8 = pvar8.wrapping_add(4);
                        i_var7 = i_var7.wrapping_add(1);
                        FUN_004096d0(i_var11, i_var12, i_var1);
                        i_var11 = i_var12;
                        if !(i_var7 < DAT_0054f978) {
                            break;
                        }
                    }
                }
                i_var7 = 0;
                DAT_0054f978 = 0;
                if 0 < DAT_0054f998 {
                    let mut pvar8 = a870;
                    let mut i_var11 = ri(a86c.wrapping_add(DAT_0054f998.wrapping_mul(4)));
                    loop {
                        let i_var12 = ri(pvar8);
                        pvar8 = pvar8.wrapping_add(4);
                        i_var7 = i_var7.wrapping_add(1);
                        FUN_00409c10(i_var11, i_var12, i_var1);
                        i_var11 = i_var12;
                        if !(i_var7 < DAT_0054f998) {
                            break;
                        }
                    }
                }
                i_var7 = 0;
                DAT_0054f998 = 0;
                if 0 < DAT_0054f978 {
                    let mut pvar8 = a8f0;
                    let mut i_var11 = ri(a8ec.wrapping_add(DAT_0054f978.wrapping_mul(4)));
                    loop {
                        let i_var12 = ri(pvar8);
                        pvar8 = pvar8.wrapping_add(4);
                        i_var7 = i_var7.wrapping_add(1);
                        FUN_0040a1a0(i_var11, i_var12, i_var1);
                        i_var11 = i_var12;
                        if !(i_var7 < DAT_0054f978) {
                            break;
                        }
                    }
                }
                i_var7 = 0;
                DAT_0054f978 = 0;
                if 0 < DAT_0054f998 {
                    let mut pvar8 = a870;
                    let mut i_var11 = ri(a86c.wrapping_add(DAT_0054f998.wrapping_mul(4)));
                    loop {
                        let i_var12 = ri(pvar8);
                        pvar8 = pvar8.wrapping_add(4);
                        i_var7 = i_var7.wrapping_add(1);
                        FUN_0040a6e0(i_var11, i_var12, i_var1);
                        i_var11 = i_var12;
                        if !(i_var7 < DAT_0054f998) {
                            break;
                        }
                    }
                }
                i_var7 = 0;
                if 0 < DAT_0054f978 {
                    let mut pvar8 = a8f0;
                    loop {
                        let i_var11 = ri(pvar8);
                        if i_var11 < DAT_0054f98c {
                            let slot = s3.wrapping_add(i_var11.wrapping_mul(4));
                            if ri(slot) == -1 {
                                let u5 = FUN_0040b350(i_var11, i_var1);
                                wi(slot, u5 as i32);
                            }
                            wi(pvar8, ri(s3.wrapping_add(ri(pvar8).wrapping_mul(4))));
                        } else {
                            let r = FUN_0040b350(i_var11, i_var1) as i32;
                            wi(pvar8, r);
                        }
                        pvar8 = pvar8.wrapping_add(4);
                        i_var7 = i_var7.wrapping_add(1);
                        if !(i_var7 < DAT_0054f978) {
                            break;
                        }
                    }
                }
                if 2 < DAT_0054f978 {
                    let mut i_var9: i32 = 0;
                    let i_var7b = ri(p.wrapping_add(0xac));
                    let mut i_var11 = ri(p.wrapping_add(0xa8));
                    let mut i_var12 = ri(i_var11.wrapping_add(0x10));
                    let mut i_var14 = ri(i_var11.wrapping_add(0x14));
                    if i_var14
                        .wrapping_mul(i_var12)
                        .wrapping_sub(ri(i_var11.wrapping_add(8)))
                        == 0
                    {
                        let l16 = __ftol();
                        i_var11 = l16 as i32;
                        while 0x100 < i_var11 && i_var12 != 8 && i_var14 != 8 {
                            i_var11 >>= 2;
                            let i_var6 = i_var14.wrapping_mul(i_var12);
                            i_var12 >>= 1;
                            i_var9 = i_var9.wrapping_add(i_var6);
                            i_var14 >>= 1;
                        }
                    }
                    let pbvar10 = i_var7b.wrapping_add(i_var9);
                    if local_8 != pbvar10 {
                        let i_var7c = ri(p.wrapping_add(0xb4));
                        local_4 = FUN_00407e40(
                            i_var12,
                            i_var14,
                            i_var7c.wrapping_add(0x1000) as u32 as usize as *mut u32,
                            pbvar10 as u32,
                        );
                        local_18 = core::ptr::null_mut();
                        local_8 = pbvar10;
                        if rs(i_var7c.wrapping_add(0x1200)) != 0 {
                            let b = rb(ri(p).wrapping_add(2));
                            if b == 6 || b == 7 {
                                ws(i_var7c.wrapping_add(0x1200), 0);
                            } else {
                                let q = ri(p.wrapping_add(0xa4));
                                if q != 0 && rb(q) != 0x5f {
                                    local_18 = FUN_00407f10(
                                        i_var12,
                                        i_var14,
                                        ri(p.wrapping_add(0xb4)) as u32 as usize as *mut u16,
                                        pbvar10 as u32 as usize as *mut u8,
                                    );
                                    if local_18.is_null()
                                        && ri(ri(p.wrapping_add(0xa8)).wrapping_add(0x10))
                                            == i_var12
                                        && ri(ri(p.wrapping_add(0xa8)).wrapping_add(0x14))
                                            == i_var14
                                    {
                                        wb(q, 0x5f);
                                    }
                                }
                            }
                        }
                    }
                }
                if !local_18.is_null() {
                    i_var7 = 0;
                    if 0 < DAT_0054f978 {
                        loop {
                            let mut pu4 = ri(a8f0.wrapping_add(i_var7.wrapping_mul(4)))
                                .wrapping_mul(0x20)
                                .wrapping_add(DAT_0053f970);
                            let mut pu15 = DAT_0054f994
                                .wrapping_add(i_var7)
                                .wrapping_mul(0x20)
                                .wrapping_add(DAT_0053f970);
                            let mut c = 8;
                            while c != 0 {
                                wi(pu15, ri(pu4));
                                pu4 = pu4.wrapping_add(4);
                                pu15 = pu15.wrapping_add(4);
                                c -= 1;
                            }
                            let i_var11 = DAT_0054f994.wrapping_add(i_var7);
                            i_var7 = i_var7.wrapping_add(1);
                            wi(
                                i_var11
                                    .wrapping_mul(0x20)
                                    .wrapping_add(0x10)
                                    .wrapping_add(DAT_0053f970),
                                -1,
                            );
                            if !(i_var7 < DAT_0054f978) {
                                break;
                            }
                        }
                    }
                    i_var7 = DAT_0054f994;
                    let mut i_var11 = 2i32;
                    DAT_0054f994 = DAT_0054f978.wrapping_add(DAT_0054f994);
                    if 2 < DAT_0054f978 {
                        loop {
                            let b = DAT_0054f9a0.wrapping_mul(0x14).wrapping_add(DAT_0054f988);
                            wi(b, i_var7);
                            wi(
                                b.wrapping_add(4),
                                i_var7.wrapping_add(i_var11).wrapping_sub(1),
                            );
                            wi(b.wrapping_add(8), i_var11.wrapping_add(i_var7));
                            wi(b.wrapping_add(0x10), local_18 as usize as i32);
                            wi(b.wrapping_add(0xc), 0x9813);
                            if DAT_00527e94 != 0 {
                                wi(
                                    b.wrapping_add(0xc),
                                    ri(b.wrapping_add(0xc)).wrapping_add(0x180),
                                );
                            }
                            i_var11 = i_var11.wrapping_add(1);
                            wi(
                                b.wrapping_add(0xc),
                                ri(b.wrapping_add(0xc)).wrapping_add(0x200),
                            );
                            DAT_0054f9a0 = DAT_0054f9a0.wrapping_add(1);
                            if !(i_var11 < DAT_0054f978) {
                                break;
                            }
                        }
                    }
                }
                i_var7 = 2;
                if 2 < DAT_0054f978 {
                    let mut pu4 = a8f4;
                    loop {
                        let b = DAT_0054f9a0.wrapping_mul(0x14).wrapping_add(DAT_0054f988);
                        wi(b, ri(a8f0));
                        wi(b.wrapping_add(4), ri(pu4));
                        wi(b.wrapping_add(8), ri(pu4.wrapping_add(4)));
                        wi(b.wrapping_add(0x10), local_4 as usize as i32);
                        wi(b.wrapping_add(0xc), 0x9813);
                        if DAT_00527e94 != 0 {
                            let b2 = DAT_0054f9a0.wrapping_mul(0x14).wrapping_add(DAT_0054f988);
                            wi(
                                b2.wrapping_add(0xc),
                                ri(b2.wrapping_add(0xc)).wrapping_add(0x180),
                            );
                        }
                        if DAT_0051a54c != 0 {
                            let b2 = DAT_0054f9a0.wrapping_mul(0x14).wrapping_add(DAT_0054f988);
                            wi(
                                b2.wrapping_add(0xc),
                                ri(b2.wrapping_add(0xc)).wrapping_add(0x200),
                            );
                            DAT_0051a54c = 0;
                        }
                        i_var7 = i_var7.wrapping_add(1);
                        DAT_0054f9a0 = DAT_0054f9a0.wrapping_add(1);
                        pu4 = pu4.wrapping_add(4);
                        if !(i_var7 < DAT_0054f978) {
                            break;
                        }
                    }
                }
                local_10 = local_10.wrapping_add(1);
                local_28 = local_28.wrapping_add(0x70);
                if !(local_10 < ri(p.wrapping_add(0xc4))) {
                    break;
                }
            }
        }
    }
}
