// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e9684: [u8; 0];
    static mut DAT_009e9685: [u8; 0];
    static mut DAT_009e96dd: [u8; 0];
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009ec5fe: u16;
    static mut DAT_009a2062: i8;
    static mut DAT_009d695b: i8;
    static mut DAT_009d8c91: [i8; 0];
    static mut DAT_0051c56e: [u8; 0];
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_00a07cec: i16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_009ecc34: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433d30(a: u16, b: u16, c: i32);
    fn FUN_004269e0(a: u32) -> u32;
    fn FUN_0042edc0(a: i32, b: i32);
    fn FUN_0042f030(a: i32);
    fn FUN_0042e460(a: i32);
    fn FUN_0045acd0(a: i32);
    fn FUN_00458780(a: i32);
    fn FUN_00455c80(a: u16);
    fn FUN_0045a1e0(a: i32, b: u32, c: u32, d: i32) -> i32;
    fn FUN_00485080(a: i32);
    fn FUN_004851d0(a: *mut u8);
    fn FUN_00452880(a: i32);
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_0042e4d0(a: u32, b: u32, c: i32);
    fn FUN_004ad9e0(a: i32, b: i32);
    fn FUN_004ada90(a: i32, b: i32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn r32(a: u32) -> i32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const i32).read_unaligned() }
}
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn w32(a: u32, v: i32) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}

unsafe fn near(a: u32, b: u32) -> bool {
    // SAFETY: a and b are entity records in the game's table; the table at
    // DAT_00521f1c is the game's own.
    unsafe {
        let mut i9 = r32(a.wrapping_add(4)).wrapping_sub(r32(b.wrapping_add(4)));
        let i13 = r32(a.wrapping_add(8)).wrapping_sub(r32(b.wrapping_add(8)));
        let mut i16v = r32(a.wrapping_add(0xc)).wrapping_sub(r32(b.wrapping_add(0xc)));
        if i9 < 0 {
            i9 = i9.wrapping_neg();
        }
        if i16v < 0 {
            i16v = i16v.wrapping_neg();
        }
        let base = (&raw mut DAT_00521f1c) as u32;
        let ta = r32(base.wrapping_add((r8(a.wrapping_add(3)) as u32).wrapping_mul(0x1a)));
        let tb = r32(base.wrapping_add((r8(b.wrapping_add(3)) as u32).wrapping_mul(0x1a)));
        i9.wrapping_sub(ta) < tb && i16v.wrapping_sub(ta) < tb && 0 < i13 && i13 < 0x40000
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402240(param_1: *mut u8, param_2: u32, param_3: i32) {
    let _ = (param_2, param_3);
    // SAFETY: all globals, callees and addressed memory are the game's own.
    unsafe {
        let a670 = (&raw mut DAT_009e9670) as u32;
        let a684 = (&raw mut DAT_009e9684) as u32;
        let a685 = (&raw mut DAT_009e9685) as u32;
        let a96dd = (&raw mut DAT_009e96dd) as u32;
        let p1 = param_1 as u32;
        let i4 = p1.wrapping_mul(0x5bd);
        let i11 = r32(a670.wrapping_add(i4));
        let i9 = (i11 as u32).wrapping_mul(0x23);
        let dat = DAT_009a1fe8 as u32;
        let i7 = dat.wrapping_add(i9);
        let tmp = r32(dat.wrapping_add(0x1f).wrapping_add(i9)) as u32;
        let i8 = r32(tmp.wrapping_add(0xa9)) as u32;
        if r8(i8.wrapping_add(7)) != 0 {
            w8(a684.wrapping_add(i4), 0);
        }
        let u15 = DAT_009ec5fe as u32;
        let mut u6 = (r32(a685.wrapping_add(i4)) as u32).wrapping_add(u15);
        w32(a685.wrapping_add(i4), u6 as i32);

        if r8(a684.wrapping_add(i4)) != 1 {
            if r8(a684.wrapping_add(i4)) != 2 {
                return;
            }
            if 0x49b < u6 {
                let b3 = r8(i7.wrapping_add(0x16));
                FUN_00433d30(i11 as u16, b3 as u16, 0x11);
                if DAT_009a2062 != 2
                    && DAT_009d695b == 1
                    && *(&raw mut DAT_009d8c91 as *mut i8)
                        .byte_add((b3 as u32).wrapping_mul(0x566) as usize)
                        != 0x63
                {
                    let arg = (((dat >> 8) << 8) | r8(dat.wrapping_add(3).wrapping_add(i9)) as u32)
                        & 0xffff00ff;
                    u6 = FUN_004269e0(arg);
                    let t = r32(a96dd.wrapping_add(i4)) as u32;
                    let idx = (u6 & 0xffff).wrapping_mul(0x101);
                    let v = r8(((&raw mut DAT_0051c56e) as u32).wrapping_add(idx)) as u32;
                    w32(
                        a96dd.wrapping_add(i4),
                        t.wrapping_add(v.wrapping_mul(0x28)) as i32,
                    );
                }
                if r32(a670.wrapping_add(i4)) != -1 {
                    FUN_0042edc0(0, p1 as i32);
                    FUN_0042f030(0);
                    FUN_0042e460(p1 as i32);
                }
                w8(dat.wrapping_add(3).wrapping_add(i9), 0);
                FUN_0045acd0(p1 as i32);
                FUN_00458780(i8 as i32);
                w8(a684.wrapping_add(i4), 0);
                FUN_00455c80(b3 as u16);
                let r = FUN_0045a1e0(p1 as i32, 0xffffffff, 0, 0);
                if r == 0 {
                    if param_1 == DAT_009ecc34 {
                        FUN_00452880(0x123);
                    }
                    return;
                }
                FUN_00485080(p1 as i32);
                FUN_004851d0(param_1);
                return;
            }
            let mut found = false;
            let i7e = r32(a670.wrapping_add(i4));
            let mut s1 = DAT_00a07cec;
            if u6 <= u15 {
                while (s1 as i32) < DAT_009a1fe0 {
                    let e = (s1 as i32 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(DAT_009a1fe8 as u32);
                    if r8(e.wrapping_add(3)) != 0 {
                        let o = (i7e as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_009a1fe8 as u32);
                        if near(e, o) {
                            found = true;
                            break;
                        }
                    }
                    s1 = s1.wrapping_add(1);
                }
                if !found {
                    s1 = DAT_00a07ce8 as i16;
                    while (s1 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
                        let e = (s1 as i32 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_009a1fe8 as u32);
                        if r8(e.wrapping_add(3)) != 0 {
                            let o = (i7e as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_009a1fe8 as u32);
                            if near(e, o) {
                                found = true;
                                break;
                            }
                        }
                        s1 = s1.wrapping_add(1);
                    }
                }
            }
            if !found {
                let u10 = (u6 / 0xec) as u16;
                let v: i32 = if u10 == 0 {
                    10
                } else if u10 == 1 {
                    0x19
                } else if u10 == 2 {
                    0x32
                } else if u10 == 3 {
                    100
                } else {
                    1000
                };
                FUN_004ad9e0(i7e, v);
                let addr = (i7e as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(8)
                    .wrapping_add(DAT_009a1fe8 as u32);
                let t = r32(addr) as u32;
                let add = (u10 as u32)
                    .wrapping_mul(DAT_009ec5fe as u32)
                    .wrapping_mul(0xe0);
                w32(addr, t.wrapping_add(add) as i32);
                return;
            }
            FUN_00450650(0x70, p1 as i32);
            let t = r32(a670.wrapping_add(i4));
            let flag;
            if t == -1 {
                flag = false;
            } else {
                let c = r8((t as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32)) as i8;
                flag = c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4;
            }
            u6 = if flag { 0x57 } else { 0x24 };
            FUN_0042e4d0(u6, 0xffffffff, p1 as i32);
            w8(a684.wrapping_add(i4), 0);
            let t = r32((i7e as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32)) as u32;
            w16(t.wrapping_add(0x79), 10);
            return;
        }

        let mut u10 = r16(i7.wrapping_add(0x14));
        let mut u14 = r16(i7.wrapping_add(0x10));
        let mut u12 = r16(i7.wrapping_add(0x12));
        'done: {
            if u10 == 0 && u12 == 0x4000 && u14 == 0 {
                u10 = 0;
                u14 = 0;
                if 0x49b < u6 {
                    w8(a684.wrapping_add(i4), 2);
                    FUN_00450650(0x6c, p1 as i32);
                    w32(a685.wrapping_add(i4), 0);
                }
                break 'done;
            }
            if u10 < 0x8000 {
                u10 = (u10 as u32).wrapping_add(u15.wrapping_mul(0xffffffe0)) as u16;
                if 0x7fff < u10 {
                    u10 = 0;
                }
            } else {
                u10 = (u10 as u32).wrapping_add(u15.wrapping_mul(0x20)) as u16;
                if u10 < 0x8000 {
                    u10 = 0;
                }
            }
            if u14 < 0x8000 {
                u14 = (u14 as u32).wrapping_add(u15.wrapping_mul(0xfffffff0)) as u16;
                if 0x7fff < u14 {
                    u14 = 0;
                }
            } else {
                u14 = (u14 as u32).wrapping_add(u15.wrapping_mul(0x10)) as u16;
                if u14 < 0x8000 {
                    u14 = 0;
                }
            }
            if u12 < 0xc000 && 0x4000 < u12 {
                u12 = (u12 as u32).wrapping_add(u15.wrapping_mul(0xfffffff0)) as u16;
                if 0x3fff < u12 {
                    break 'done;
                }
            } else if u12 == 0x4000 {
                break 'done;
            } else {
                u12 = (u12 as u32).wrapping_add(u15.wrapping_mul(0x10)) as u16;
                if u12 < 0x4001 || 0xbfff < u12 {
                    break 'done;
                }
            }
            u12 = 0x4000;
        }
        w16(dat.wrapping_add(0x14).wrapping_add(i9), u10);
        w16(dat.wrapping_add(0x10).wrapping_add(i9), u14);
        w16(dat.wrapping_add(0x12).wrapping_add(i9), u12);
        let t = r32(dat.wrapping_add(0x1f).wrapping_add(i9)) as u32;
        w8(t.wrapping_add(0x92), 1);
        let e = r32(dat.wrapping_add(0x1f).wrapping_add(i9)) as u32;
        w8(e.wrapping_add(0x8b), r8(e.wrapping_add(0x92)));
        let t = r32(dat.wrapping_add(0x1f).wrapping_add(i9)) as u32;
        let t2 = r32(t.wrapping_add(0xa9)) as u32;
        w16(t2.wrapping_add(0xd), u12);
        FUN_004ada90(i11, 200);
    }
}
