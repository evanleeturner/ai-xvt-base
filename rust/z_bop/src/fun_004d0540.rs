// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acf56d: [u8; 1];
    static mut DAT_00acf571: [u8; 1];
    static mut DAT_00b53575: [u8; 1];
    static mut DAT_00b53579: [u8; 1];
    static mut DAT_00b63d81: [u8; 1];
    static mut DAT_00b63d91: [u8; 1];
    static mut DAT_00b63f95: [u8; 1];
    static mut DAT_00b63f9d: [u8; 1];
    static mut DAT_00b69a89: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004ce130();
    fn FUN_004d1ea0();
    fn FUN_004d23b0(param_1: u32) -> u32;
    fn FUN_004d2170(param_1: u32) -> i32;
    fn FUN_004d2080(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: u32) -> u32;
    fn FUN_004cf830(param_1: u32, param_2: *mut u32, param_3: i32);
}

unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn rec(i: u32) -> *mut u8 {
    // SAFETY: only the address of the game's table is taken.
    unsafe {
        (&raw mut DAT_00acf571)
            .cast::<u8>()
            .wrapping_add(i.wrapping_mul(0x210) as usize)
    }
}

unsafe fn plr(i: u32) -> *mut u8 {
    // SAFETY: only the address of the game's table is taken.
    unsafe {
        (&raw mut DAT_00b63f95)
            .cast::<u8>()
            .wrapping_add(i.wrapping_mul(0x238) as usize)
    }
}

unsafe fn g_acf56d() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00acf56d).cast::<u8>() }
}
unsafe fn g_b53575() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00b53575).cast::<u8>() }
}
unsafe fn g_b53579() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00b53579).cast::<u8>() }
}
unsafe fn g_b63d81() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00b63d81).cast::<u8>() }
}
unsafe fn g_b63d91() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00b63d91).cast::<u8>() }
}
unsafe fn g_b63f9d() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00b63f9d).cast::<u8>() }
}
unsafe fn g_b69a89() -> *mut u8 {
    // SAFETY: address only.
    unsafe { (&raw mut DAT_00b69a89).cast::<u8>() }
}

unsafe fn copy_rec(src: *const u8) {
    // SAFETY: the record and the copy buffer are distinct 0x210-byte regions.
    unsafe { core::ptr::copy_nonoverlapping(src, g_b63d81(), 0x210) }
}

unsafe fn sp(buf: *mut i8, f: &'static [u8], v: u32) {
    // SAFETY: f is NUL-terminated and buf is the caller's 256-byte buffer.
    unsafe {
        sprintf(buf, f.as_ptr().cast::<i8>(), v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d0540(param_1: *mut u32, param_2: *mut u32) -> *mut u8 {
    let mut local_350 = [0u8; 40];
    let mut local_328 = [0x5au8; 40];
    let mut local_3c8 = [0u8; 120];
    let mut local_300 = [0i8; 256];
    let mut local_200 = [0u32; 3];
    let mut local_3f0: u32 = 0;
    let mut local_3d8: i32 = 0;
    let mut local_3d4: u32 = 0;
    let c350 = local_350.as_mut_ptr();
    let c328 = local_328.as_mut_ptr();
    let c3c8 = local_3c8.as_mut_ptr();
    let c300 = local_300.as_mut_ptr();
    // SAFETY: all globals and callees are the game's own; pointers are
    // the caller's out-parameters and the game's tables, used as the original does.
    unsafe {
        FUN_004ce130();
        FUN_004d1ea0();
        if rd(g_b53579()) == 0 {
            return core::ptr::null_mut();
        }
        let mut u17 = rd(g_b53575());
        let mut i13 = rd(g_b53579()) as i32;
        if 0 < (rd(g_b69a89()) as i32) {
            let mut u5 = rd(g_b69a89());
            let mut pb = c3c8;
            let mut pc = g_b63f9d() as *const u8;
            loop {
                *pb = *pc;
                u5 = u5.wrapping_sub(1);
                *pb.add(1) = *pc.wrapping_sub(8);
                *pb.add(2) = *pc.wrapping_add(8);
                pb = pb.add(3);
                pc = pc.wrapping_add(0x238);
                if u5 == 0 {
                    break;
                }
            }
        }
        loop {
            if i13 < 1 {
                u17 = rd(g_b53575());
                i13 = rd(g_b53579()) as i32;
                if 0x3fe < (rd(g_b53579()) as i32) {
                    while 0 < i13 {
                        let r = rec(u17);
                        if rd(r) == 0 {
                            u17 = u17.wrapping_add(1);
                        } else {
                            let u5 = *r.add(0xe) as u32;
                            let c2 = *r.add(0xd) as i8;
                            let u11 = FUN_004d23b0(rd(r));
                            if u11 < rd(g_b69a89()) && u11 < 0x28 {
                                let i12 = if c2 != 0 {
                                    if c2 != 2 {
                                        rd(plr(u11)) as i32
                                    } else {
                                        rd(plr(u11).add(0x10)) as i32
                                    }
                                } else {
                                    rd(plr(u11).add(8)) as i32
                                };
                                local_3f0 = (i12 as u32).wrapping_add(1);
                                if 0x7f < (local_3f0 as i32) {
                                    local_3f0 = 0;
                                }
                            }
                            let i12 = u5.wrapping_sub(local_3f0) as i32;
                            if u11 < rd(g_b69a89()) && (i12 < -0x1b || (-1 < i12 && i12 < 100)) {
                                if *r.add(0xc) != 0 {
                                    if c2 != 0 {
                                        if c2 != 2 {
                                            sp(c300, b"(ROOS %u) \0", u5);
                                            wr(plr(u11), u5);
                                        } else {
                                            sp(c300, b"(ROOG %u) \0", u5);
                                            wr(plr(u11).add(0x10), u5);
                                        }
                                    } else {
                                        sp(c300, b"(ROOB %u) \0", u5);
                                        wr(plr(u11).add(8), u5);
                                    }
                                    wr(
                                        plr(u11).add(0x224),
                                        rd(plr(u11).add(0x224)).wrapping_add(1),
                                    );
                                    copy_rec(rec(u17));
                                    FUN_004d2170(u17);
                                    *param_1 = rd(g_b63d81());
                                    *param_2 = rd(g_b63d81().add(4));
                                    return g_b63d91();
                                }
                                u17 = u17.wrapping_add(1);
                            } else {
                                let i6 = FUN_004d2170(u17);
                                if i6 != 0 {
                                    u17 = u17.wrapping_add(1);
                                }
                            }
                        }
                        if 0x3ff < (u17 as i32) {
                            u17 = 0;
                        }
                        i13 = i13.wrapping_sub(1);
                    }
                }
                return core::ptr::null_mut();
            }
            let mut adv = false;
            'body: {
                let r = rec(u17);
                if rd(r) == 0 {
                    if rd(g_b53575()) == u17 {
                        copy_rec(r);
                        *param_1 = rd(r);
                        *param_2 = rd(r.add(4));
                        wr(g_b53579(), rd(g_b53579()).wrapping_sub(1));
                        wr(g_b53575(), rd(g_b53575()).wrapping_add(1));
                        if 0x3ff < (rd(g_b53575()) as i32) {
                            wr(g_b53575(), 0);
                        }
                        return g_b63d91();
                    }
                    adv = true;
                    break 'body;
                }
                let mut b1 = *r.add(0xe);
                let u7 = b1 as u32;
                let u11 = (*r.add(0xd) == 0) as u32;
                let u3 = (*r.add(0xd) == 2) as u32;
                let mut u5 = rd(r.add(0x10));
                let u8v = FUN_004d23b0(rd(r));
                if local_3d4 == u8v && u8v < 0x28 {
                    let i = u8v.wrapping_mul(3) as usize;
                    *c3c8.add(i) = 0x7f;
                    *c3c8.add(i + 1) = 0x7f;
                    *c3c8.add(i + 2) = 0x7f;
                }
                if rd(g_b69a89()) <= u8v {
                    // fall into the shared block
                } else if 0x32 < u5 {
                    if *c328.add(u8v as usize) < *c350.add(u8v as usize) {
                        adv = true;
                        break 'body;
                    }
                    let d = GetTickCount();
                    wr(plr(u8v).add(0x230), d);
                    if *r.add(0xf) == 0 {
                        *c350.add(u8v as usize) = (*c350.add(u8v as usize)).wrapping_add(1);
                    }
                    let i12 = if u11 == 0 {
                        if u3 == 0 {
                            rd(plr(u8v)) as i32
                        } else {
                            rd(plr(u8v).add(0x10)) as i32
                        }
                    } else {
                        rd(plr(u8v).add(8)) as i32
                    };
                    local_3f0 = (i12 as u32).wrapping_add(1);
                    if 0x7f < (local_3f0 as i32) {
                        local_3f0 = 0;
                    }
                } else {
                    if *r.add(0xf) == 0 {
                        if u11 == 0 {
                            if u3 == 0 {
                                wr(plr(u8v), u7);
                                *c3c8.add(u8v.wrapping_mul(3).wrapping_add(1) as usize) = b1;
                            } else {
                                wr(plr(u8v).add(0x10), u7);
                                *c3c8.add(u8v.wrapping_mul(3).wrapping_add(2) as usize) = b1;
                            }
                        } else {
                            *c3c8.add(u8v.wrapping_mul(3) as usize) = b1;
                            wr(plr(u8v).add(8), u7);
                        }
                    }
                    let i6 = FUN_004d2170(u17);
                    adv = i6 != 0;
                    break 'body;
                }
                // LAB_004d07c7
                let i12 = u7.wrapping_sub(local_3f0) as i32;
                if u8v < rd(g_b69a89()) && (i12 < -0x1c || (-1 < i12 && i12 < 100)) {
                    if local_3f0 == u7 {
                        let d = GetTickCount();
                        wr(plr(u8v).add(0x220), d);
                        if u11 == 0 {
                            if u3 == 0 {
                                sp(c300, b"(RS %u) \0", u7);
                                wr(plr(u8v), u7);
                            } else {
                                sp(c300, b"(RG %u) \0", u7);
                                wr(plr(u8v).add(0x10), u7);
                            }
                        } else {
                            sp(c300, b"(RB %u) \0", u7);
                            wr(plr(u8v).add(8), u7);
                        }
                        wr(plr(u8v).add(0x224), rd(plr(u8v).add(0x224)).wrapping_add(1));
                        copy_rec(r);
                        FUN_004d2170(u17);
                        *param_1 = rd(g_b63d81());
                        *param_2 = rd(g_b63d81().add(4));
                        return g_b63d91();
                    }
                    if *r.add(0xf) == 0 {
                        if u11 == 0 {
                            local_3d4 = 2u32.wrapping_sub((u3 == 0) as u32);
                        } else {
                            local_3d4 = 0;
                        }
                        let idx = u8v.wrapping_mul(3).wrapping_add(local_3d4) as usize;
                        u5 = (*c3c8.add(idx) as u32).wrapping_add(1);
                        *c3c8.add(idx) = b1;
                        if 0x7f < u5 {
                            u5 = 0;
                        }
                        local_3d8 = u7.wrapping_sub(u5) as i32;
                        if local_3d8 < 0 {
                            local_3d8 = (local_3d8 as u32).wrapping_add(0x80) as i32;
                        }
                        let pb = c328.add(u8v as usize);
                        let mut b4 = false;
                        let mut u16v = u5;
                        if (*pb as i32) < local_3d8 {
                            *pb = 0;
                        } else {
                            *pb = (*pb).wrapping_sub(local_3d8 as u8);
                        }
                        while u7 != u16v {
                            let u10 = FUN_004d2080(u17, u16v, u11, u3, u8v);
                            if (u10 as i32) < 0x400 && -1 < (u10 as i32) {
                                if local_3f0 == u16v {
                                    let d = GetTickCount();
                                    wr(plr(u8v).add(0x220), d);
                                    copy_rec(rec(u10));
                                    if u11 == 0 {
                                        if u3 == 0 {
                                            sp(c300, b"(ROOS %u) \0", local_3f0);
                                            wr(plr(u8v), local_3f0);
                                        } else {
                                            sp(c300, b"(ROOG %u) \0", local_3f0);
                                            wr(plr(u8v).add(0x10), local_3f0);
                                        }
                                    } else {
                                        sp(c300, b"(ROOB %u) \0", local_3f0);
                                        wr(plr(u8v).add(8), local_3f0);
                                    }
                                    wr(
                                        plr(u8v).add(0x224),
                                        rd(plr(u8v).add(0x224)).wrapping_add(1),
                                    );
                                    *param_1 = rd(g_b63d81());
                                    *param_2 = rd(g_b63d81().add(4));
                                    if local_3d8 < 2 {
                                        *r.add(0xc) = 0;
                                        wr(r.add(8), 0);
                                    }
                                    FUN_004d2170(u10);
                                    return g_b63d91();
                                }
                                local_3d8 = (local_3d8 as u32).wrapping_sub(1) as i32;
                            } else {
                                'e: {
                                    if *r.add(0xc) == 0 {
                                        sp(c300, b"(RP %d) \0", u16v);
                                        if 0x14 < rd(plr(u8v).add(0x224)) {
                                            wr(
                                                plr(u8v).add(0x22c),
                                                rd(plr(u8v).add(0x22c)).wrapping_add(1),
                                            );
                                        }
                                    } else {
                                        let d = GetTickCount();
                                        let lim: u32;
                                        if rd(g_acf56d()) == 1 {
                                            b1 = 0;
                                            lim = 20000;
                                        } else {
                                            b1 = 0x14;
                                            lim = 1000;
                                        }
                                        if d.wrapping_sub(rd(r.add(8))) <= lim {
                                            break 'e;
                                        }
                                        if b1 < *r.add(0xc) {
                                            let d2 = GetTickCount();
                                            wr(plr(u8v).add(0x220), d2);
                                            loop {
                                                u16v = FUN_004d2080(u17, u5, u11, u3, u8v);
                                                if !((u16v as i32) < 0 || 0x400 < (u16v as i32)) {
                                                    break;
                                                }
                                                if u7 == u5 {
                                                    if u11 == 0 {
                                                        if u3 == 0 {
                                                            sp(c300, b"(ROOS %u) \0", u7);
                                                            wr(plr(u8v), u7);
                                                        } else {
                                                            sp(c300, b"(ROOG %u) \0", u7);
                                                            wr(plr(u8v).add(0x10), u7);
                                                        }
                                                    } else {
                                                        sp(c300, b"(ROOB %u) \0", u7);
                                                        wr(plr(u8v).add(8), u7);
                                                    }
                                                    wr(
                                                        plr(u8v).add(0x224),
                                                        rd(plr(u8v).add(0x224)).wrapping_add(1),
                                                    );
                                                    copy_rec(r);
                                                    FUN_004d2170(u17);
                                                    *param_1 = rd(g_b63d81());
                                                    *param_2 = rd(g_b63d81().add(4));
                                                    return g_b63d91();
                                                }
                                                u5 = u5.wrapping_add(1);
                                                if 0x7f < (u5 as i32) {
                                                    u5 = 0;
                                                }
                                            }
                                            copy_rec(rec(u16v));
                                            FUN_004d2170(u16v);
                                            if u11 == 0 {
                                                if u3 == 0 {
                                                    sp(c300, b"(ROOS %u) \0", local_3f0);
                                                    wr(plr(u8v), u5);
                                                } else {
                                                    sp(c300, b"(ROOG %u) \0", local_3f0);
                                                    wr(plr(u8v).add(0x10), u5);
                                                }
                                            } else {
                                                sp(c300, b"(ROOB %u) \0", local_3f0);
                                                wr(plr(u8v).add(8), u5);
                                            }
                                            wr(
                                                plr(u8v).add(0x224),
                                                rd(plr(u8v).add(0x224)).wrapping_add(1),
                                            );
                                            *param_1 = rd(g_b63d81());
                                            *param_2 = rd(g_b63d81().add(4));
                                            return g_b63d91();
                                        }
                                        sp(c300, b"(RP %d) \0", u16v);
                                    }
                                    local_200[2] = 0;
                                    if u11 == 0 {
                                        local_200[2] = 2u32.wrapping_sub((u3 == 0) as u32);
                                    }
                                    local_200[0] = 0x38;
                                    local_200[1] = u16v;
                                    FUN_004cf830(rd(r), local_200.as_mut_ptr(), 0xc);
                                    b4 = true;
                                }
                            }
                            // LAB_004d09c8
                            u16v = u16v.wrapping_add(1);
                            if 0x7f < (u16v as i32) {
                                u16v = 0;
                            }
                        }
                        if local_3d8 < 1 {
                            *r.add(0xc) = 0;
                            wr(r.add(8), 0);
                        } else if b4 {
                            *r.add(0xc) = (*r.add(0xc)).wrapping_add(1);
                            let d = GetTickCount();
                            wr(r.add(8), d);
                        }
                    } else if rd(g_b53575()) == u17 {
                        let i6 = FUN_004d2170(u17);
                        adv = i6 != 0;
                        break 'body;
                    }
                    adv = true;
                } else {
                    let i6 = FUN_004d2170(u17);
                    if i6 == 0 {
                        adv = false;
                        break 'body;
                    }
                    adv = true;
                }
            }
            if adv {
                u17 = u17.wrapping_add(1);
                if 0x3ff < (u17 as i32) {
                    u17 = 0;
                }
            }
            i13 = i13.wrapping_sub(1);
        }
    }
}
