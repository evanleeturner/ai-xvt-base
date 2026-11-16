// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a08104: *mut u8;
    static mut DAT_009ecc34: *mut u8;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_009a8c00: u16;
    static mut DAT_00a08292: u16;
    static mut DAT_00a08c74: u16;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: u32;
    static mut DAT_009d8cd4: [u8; 0];
    static mut DAT_009d8cd5: [u8; 0];
    static mut DAT_009d8cd2: [u8; 0];
    static mut DAT_009d8c80: [u8; 0];
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_009d8c85: [u8; 0];
    static mut DAT_009d8c87: [u8; 0];
    static mut DAT_0051c637: [u8; 0];
    static mut DAT_0051c639: [u8; 0];
    static mut DAT_0051c63b: [u8; 0];
    static mut DAT_0051c63d: [u8; 0];
    static mut DAT_0051c63f: [u8; 0];
    static mut DAT_0051c632: [u8; 0];
    static mut DAT_0051c5aa: [u8; 0];
    static mut DAT_0051c5ac: [u8; 0];
    static mut DAT_0051c5b0: [u8; 0];
    static mut DAT_00524208: [u8; 0];
    static mut DAT_00524120: [u8; 0];
    static mut DAT_00524148: [u8; 0];
    static mut DAT_009a7a40: [u8; 0];
    static mut DAT_009a6ff0: [u8; 0];
    static mut DAT_009a7338: [u8; 0];
    static mut DAT_009a1ed0: [u8; 0];
    static mut DAT_009ec4d0: [u8; 0];
    static mut DAT_009a20d6: [u8; 0];
    static mut DAT_009a20da: [u8; 0];
    static mut DAT_009a20de: [u8; 0];
    static mut DAT_009a20e2: [u8; 0];
    static mut DAT_009a20fe: [u8; 0];
    static mut DAT_009a210a: [u8; 0];
    static mut DAT_009a210c: [u8; 0];
    static mut DAT_009a210e: [u8; 0];
    static mut DAT_009a211c: [u8; 0];
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e968a: [u8; 0];
    static mut DAT_009e9a77: [u8; 0];
    static mut DAT_009d1320: [i8; 0];
    static mut DAT_0051a254: [i8; 0];
    static mut DAT_0051a230: [i8; 0];
    static mut DAT_0051a208: [i8; 0];
    static mut DAT_0051a21c: [i8; 0];
    static mut DAT_0051a1f4: [i8; 0];
    static mut DAT_0051a278: [i8; 0];
    static mut DAT_0051a244: [i8; 0];
    static mut DAT_0051a264: [i8; 0];
    static mut DAT_00521210: [i8; 0];
    static mut DAT_0051a1bc: [i8; 0];
    static mut DAT_00521628: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004035d0(a: i32, b: i32, c: i16, d: i16);
    fn FUN_00458e20(a: u32, b: i32);
    fn FUN_004a44d0(a: i32, b: i32);
    fn FUN_004a4d70(a: u32, b: u32) -> u32;
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_0042e4d0(a: i32, b: i32, c: i32) -> u32;
    fn FUN_004525a0(a: u32, b: i32, c: *mut u8);
    fn FUN_00451bf0(a: i32, b: *mut u8);
    fn FUN_00430200(a: i32, b: i32, c: u32, d: u32) -> u32;
    fn FUN_00425b70(a: u32, b: u16) -> u32;
    fn FUN_004aba50();
    fn FUN_00440da0(a: u32, b: i32);
    fn FUN_004abe50();
    fn FUN_00451c20(a: u32, b: *mut u32, c: i32);
    fn FUN_004a3750(a: u32, b: i32, c: i32);
    fn FUN_0046b610(a: *mut i8) -> i32;
    fn FUN_00402cb0(a: u16);
    fn FUN_00402970(a: u32);
    fn FUN_00434500(a: u32, b: u32);
    fn FUN_00425cb0() -> u32;
}

macro_rules! gp {
    ($g:ident) => {
        (&raw mut $g) as *mut u8
    };
}

macro_rules! ga {
    ($g:ident) => {
        (&raw mut $g) as *mut u8 as u32
    };
}

fn ad(x: u32, y: u32) -> u32 {
    x.wrapping_add(y)
}

fn c22(hi: u16, lo: u16) -> u32 {
    ((hi as u32).wrapping_shl(16)) | (lo as u32)
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn s16(a: u32) -> i16 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { r16(a) as i16 }
}

unsafe fn w8(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as *mut u8).write(v) }
}

unsafe fn w16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

unsafe fn w32(a: u32, v: u32) {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as *mut u32).write_unaligned(v) }
}

unsafe fn sb() -> u32 {
    // SAFETY: the packed pointer global is the game's own memory.
    unsafe { ((&raw const DAT_009a73b6) as *const u32).read_unaligned() }
}

unsafe fn ents() -> u32 {
    // SAFETY: reading the game's global pointer.
    unsafe { DAT_009a1fe8 as u32 }
}

unsafe fn hdr() -> u32 {
    // SAFETY: reading the game's global pointer.
    unsafe { DAT_00a08104 as u32 }
}

unsafe fn eecc() -> u32 {
    // SAFETY: reading the game's global pointer.
    unsafe { DAT_009ecc34 as u32 }
}

unsafe fn selfent() -> u32 {
    // SAFETY: reading the game's globals.
    unsafe { ad(ents(), (DAT_009a73b0 as u32).wrapping_mul(0x23)) }
}

unsafe fn ea(i3: u32, off: u32) -> u32 {
    // SAFETY: reading the game's global pointer.
    unsafe { ad(ad(ents(), i3), off) }
}

unsafe fn hb4() -> u32 {
    // SAFETY: reads the game's header record.
    unsafe { (r8(ad(hdr(), 4)) as u32).wrapping_mul(0x101) }
}

unsafe fn cmpn(a: u32, b: u32, n: u32) -> bool {
    let mut a = a;
    let mut b = b;
    let mut n = n;
    let mut z = false;
    // SAFETY: byte compare over the game's strings, like repe cmpsb.
    unsafe {
        loop {
            if n == 0 {
                break;
            }
            n = n.wrapping_sub(1);
            z = r8(a) == r8(b);
            a = a.wrapping_add(1);
            b = b.wrapping_add(1);
            if !z {
                break;
            }
        }
    }
    z
}

unsafe fn swap_run(p19: u32, u6: u16, u12: u32) {
    let mut buf = [0u8; 0x37];
    // SAFETY: the game's own block and globals, saved and restored as the original does.
    unsafe {
        let saved = DAT_00a08104;
        core::ptr::copy_nonoverlapping(gp!(DAT_009a73b0) as *const u8, buf.as_mut_ptr(), 0x37);
        DAT_00a08104 = p19 as *mut u8;
        FUN_00402cb0(u6);
        FUN_00402970(u12);
        DAT_00a08104 = saved;
        core::ptr::copy_nonoverlapping(buf.as_ptr(), gp!(DAT_009a73b0), 0x37);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a2020() -> u32 {
    // SAFETY: all memory touched is the game's own, accessed where the original does.
    unsafe {
        let mut local_38: u32 = 0;
        let mut local_3c: u32 = 0;
        let mut u_var6: u16 = r16(ad(sb(), 0x18));
        let mut pbvar8: u32 = ad(sb(), 0x18);
        let mut u_var13: u16 = r8(ad(
            ga!(DAT_009d8cd4),
            (DAT_009a73c0 as u32)
                .wrapping_mul(0x566)
                .wrapping_add((DAT_009a73c2 as u32).wrapping_mul(0x52)),
        )) as u16;
        let mut u_var12: u32 = u_var6 as u32;
        let i_var3: u32 = u_var12.wrapping_mul(0x23);
        let mut pbvar9: u32 = ad(ents(), i_var3);
        let i_var15: u32 = r32(ad(pbvar9, 0x1f));
        let mut pbvar10: u32 = ad(pbvar9, 0x1f);
        let u_var2: u16;
        let u_stack_50: u16;
        let b_var17: u8;
        let p19: u32;
        if i_var15 == 0 {
            b_var17 = r8(ad(pbvar9, 0x16));
            u_stack_50 = 0xff;
            u_var2 = r16(pbvar9);
            p19 = local_38;
        } else {
            p19 = r32(ad(i_var15, 0xa9));
            local_3c = ad(p19, 0x2a);
            u_stack_50 = r8(ad(p19, 4)) as u16;
            b_var17 = r8(ad(pbvar9, 0x16));
            u_var2 = r16(pbvar9);
        }
        let local_52: u16 = b_var17 as u16;
        let mut s_stack_4c: i16 = ad(pbvar9, 0x16) as i16;
        let u_stack_3e: u16 = (pbvar8 >> 16) as u16;

        'sw: {
            match r8(ad(sb(), 0x35)) {
                0 => {
                    if i_var15 == 0 {
                        FUN_00458e20(u_var12, 0);
                        w32(ad(sb(), 0x1f), DAT_00a07cdc as u32);
                        w32(ad(sb(), 0x23), DAT_00a07cd8 as u32);
                        w32(ad(sb(), 0x27), (DAT_00a07cd4 as u32).wrapping_add(0x800));
                    } else {
                        let off = (u_stack_50 as u32).wrapping_mul(0x101);
                        let mut s_var14: i16;
                        if r8(ad(pbvar9, 2)) == 0 || r8(ad(pbvar9, 2)) == 1 {
                            s_var14 = s16(ad(ga!(DAT_0051c639), off))
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63d), hb4())));
                        } else if r8(ad(selfent(), 2)) == 0 || r8(ad(selfent(), 2)) == 1 {
                            s_var14 = s16(ad(ga!(DAT_0051c639), off))
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63f), hb4())))
                                .wrapping_add(s16(ad(ga!(DAT_0051c63b), off)));
                        } else {
                            s_var14 = s16(ad(ga!(DAT_0051c63b), off))
                                .wrapping_mul(2)
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63f), hb4())));
                        }
                        s_var14 = s_var14.wrapping_add(
                            s16(ad(ga!(DAT_0051c63b), off))
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63f), hb4())))
                                .wrapping_mul(2),
                        );
                        if s_var14 < 0 {
                            s_var14 = 0x7000;
                        }
                        FUN_004035d0(pbvar9 as i32, 0, s_var14, s16(ad(ga!(DAT_0051c637), off)));
                        w32(
                            ad(sb(), 0x1f),
                            r32(ea(i_var3, 4)).wrapping_add(DAT_009d8a58 as u32),
                        );
                        w32(
                            ad(sb(), 0x23),
                            r32(ea(i_var3, 8)).wrapping_add(DAT_009d8a54 as u32),
                        );
                        w32(
                            ad(sb(), 0x27),
                            r32(ea(i_var3, 0xc)).wrapping_add(DAT_009d8b60),
                        );
                    }
                    FUN_004a44d0(0, 1);
                    if 0x4000 < DAT_009a8c00 {
                        FUN_004a4d70(DAT_009a73b0 as u32, 0xffff);
                    }
                    if 0x2000 < DAT_009a8c00 {
                        let u = FUN_004a4d70(DAT_009a73b0 as u32, 0x8000);
                        return u & 0xffff0000;
                    }
                    if 0x800 < DAT_009a8c00 {
                        let u = FUN_004a4d70(DAT_009a73b0 as u32, 0x4000);
                        return u & 0xffff0000;
                    }
                    if r32(ea(i_var3, 0x1b)) == 0xffffffff
                        || r32(ea(i_var3, 0x1f)) == 0
                        || (r16(ad(r32(ea(i_var3, 0x1f)), 0x79)) as i16) == 0
                    {
                        let u = FUN_004a4d70(DAT_009a73b0 as u32, 0);
                        w8(ad(sb(), 0x35), 1);
                        return u & 0xffff0000;
                    }
                    pbvar9 = eecc();
                    if r32(ea(i_var3, 0x1b)) == eecc() {
                        u_var12 = 0;
                        pbvar9 = ga!(DAT_009a6ff0);
                        loop {
                            if r16(pbvar9) == 0x104 {
                                break;
                            }
                            pbvar9 = pbvar9.wrapping_add(0x54);
                            u_var12 = u_var12.wrapping_add(1);
                            if !(pbvar9 < ga!(DAT_009a7338)) {
                                break;
                            }
                        }
                        if 9 < u_var12 {
                            DAT_00a08292 = r8(ad(
                                ga!(DAT_009d8c87),
                                (DAT_009a73c0 as u32).wrapping_mul(0x566),
                            )) as u16;
                            FUN_00450650(0x104, eecc() as i32);
                            let u = FUN_0042e4d0(0x24, -1, eecc() as i32);
                            return u & 0xffff0000;
                        }
                    }
                    break 'sw;
                }
                1 => {
                    let local_48: i32;
                    let i15: i32;
                    let i18: i32;
                    let s_var14: i16;
                    let local_40: u16;
                    if i_var15 == 0 {
                        FUN_00458e20(u_var12, 0);
                        w32(
                            ad(hdr(), 0xb1),
                            (DAT_00a07cdc as u32).wrapping_sub(r32(ad(selfent(), 4))),
                        );
                        local_48 = r32(ad(hdr(), 0xb1)) as i32;
                        w32(
                            ad(hdr(), 0xb5),
                            (DAT_00a07cd8 as u32).wrapping_sub(r32(ad(selfent(), 8))),
                        );
                        i15 = r32(ad(hdr(), 0xb5)) as i32;
                        s_stack_4c = 0;
                        s_var14 = 0;
                        w32(
                            ad(hdr(), 0xb9),
                            (DAT_00a07cd4 as u32)
                                .wrapping_sub(r32(ad(selfent(), 0xc)))
                                .wrapping_add(0x80),
                        );
                        local_40 = 0x4000;
                        i18 = r32(ad(hdr(), 0xb9)) as i32;
                    } else {
                        let off = (u_stack_50 as u32).wrapping_mul(0x101);
                        let sv: i16;
                        if r8(ad(pbvar9, 2)) == 0 || r8(ad(pbvar9, 2)) == 1 {
                            sv = s16(ad(ga!(DAT_0051c639), off))
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63d), hb4())));
                        } else if r8(ad(selfent(), 2)) == 0 || r8(ad(selfent(), 2)) == 1 {
                            sv = s16(ad(ga!(DAT_0051c639), off))
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63f), hb4())));
                        } else {
                            sv = s16(ad(ga!(DAT_0051c63b), off))
                                .wrapping_sub(s16(ad(ga!(DAT_0051c63f), hb4())));
                        }
                        FUN_004035d0(pbvar9 as i32, 0, sv, s16(ad(ga!(DAT_0051c637), off)));
                        w32(
                            ad(hdr(), 0xb1),
                            r32(ea(i_var3, 4))
                                .wrapping_sub(r32(ad(selfent(), 4)))
                                .wrapping_add(DAT_009d8a58 as u32),
                        );
                        local_48 = r32(ad(hdr(), 0xb1)) as i32;
                        w32(
                            ad(hdr(), 0xb5),
                            r32(ea(i_var3, 8))
                                .wrapping_sub(r32(ad(selfent(), 8)))
                                .wrapping_add(DAT_009d8a54 as u32),
                        );
                        i15 = r32(ad(hdr(), 0xb5)) as i32;
                        w32(
                            ad(hdr(), 0xb9),
                            r32(ea(i_var3, 0xc))
                                .wrapping_sub(r32(ad(selfent(), 0xc)))
                                .wrapping_add(DAT_009d8b60),
                        );
                        i18 = r32(ad(hdr(), 0xb9)) as i32;
                        s_stack_4c = s16(ea(i_var3, 0x14));
                        s_var14 = s16(ea(i_var3, 0x10));
                        local_40 = r16(ea(i_var3, 0x12));
                    }
                    if s16(ad(selfent(), 0x14)) != s_stack_4c {
                        w8(ad(hdr(), 0xa4), 1);
                        w8(ad(hdr(), 0xa5), 0);
                        w8(ad(hdr(), 0xa6), 0x80);
                        w16(ad(sb(), 0x30), s_stack_4c as u16);
                    }
                    local_38 = DAT_009a73b0 as u32;
                    if s16(ad(ents(), local_38.wrapping_mul(0x23).wrapping_add(0x10))) != s_var14 {
                        w8(ad(hdr(), 0xab), 2);
                        w8(ad(hdr(), 0xac), 0);
                        w8(ad(hdr(), 0xad), 0x80);
                        w16(ad(sb(), 0x32), s_var14 as u16);
                    }
                    if s16(ad(selfent(), 0x12)) != s16(ea(i_var3, 0x12)) {
                        w16(ad(sb(), 0x2e), local_40);
                        w8(ad(hdr(), 0x9e), 0);
                        w8(ad(hdr(), 0x9f), 0x80);
                        w8(ad(hdr(), 0x9d), 0);
                        if r16(ad(hdr(), 0xd)) < r16(ad(sb(), 0x2e)) {
                            w8(ad(hdr(), 0x9c), 2);
                        } else {
                            w8(ad(hdr(), 0x9c), 1);
                        }
                    }
                    let l48 = if local_48 < 0 {
                        local_48.wrapping_neg()
                    } else {
                        local_48
                    };
                    let l15 = if i15 < 0 { i15.wrapping_neg() } else { i15 };
                    let l18 = if i18 < 0 { i18.wrapping_neg() } else { i18 };
                    pbvar9 = (l15 as u32)
                        .wrapping_add(l18 as u32)
                        .wrapping_add(l48 as u32);
                    if 0xf < (pbvar9 as i32) {
                        break 'sw;
                    }
                    w32(ad(hdr(), 0xb1), 0);
                    w32(ad(hdr(), 0xb5), 0);
                    w32(ad(hdr(), 0xb9), 0);
                    w8(ad(sb(), 0x35), 2);
                    w32(ad(sb(), 0x36), (u_var13 as u32).wrapping_mul(0x49c));
                    w8(ad(sb(), 0x3a), 0xec);
                    w8(ad(sb(), 0x3b), 0);
                    if r32(ad(selfent(), 0x1f)) != 0 && r8(ad(hdr(), 0x7c)) == 0 {
                        w8(ad(hdr(), 0x7c), 1);
                        let u11 = DAT_009a73c0 as u32;
                        let i = u11.wrapping_mul(0x126);
                        let q = ad(ga!(DAT_009a20fe), i);
                        w16(q, r16(q).wrapping_add(1));
                        if r8(ad(ga!(DAT_009d8c80), u11.wrapping_mul(0x566))) == r8(ad(hdr(), 0xb0))
                        {
                            w8(ad(ga!(DAT_009a211c), i), 1);
                        }
                    }
                    pbvar10 = ents();
                    if r32(ea(i_var3, 0x1f)) != 0 {
                        pbvar10 = (ents() & 0xffffff00) | (r8(ad(p19, 0x7a)) as u32);
                        if r8(ad(p19, 0x7a)) == 0 {
                            w8(ad(p19, 0x7a), 1);
                            pbvar10 = (local_52 as u32).wrapping_mul(0x126);
                            let q = ad(pbvar10, 0x9a20fc);
                            w16(q, r16(q).wrapping_add(1));
                            if r8(ad(ga!(DAT_009d8c80), (local_52 as u32).wrapping_mul(0x566)))
                                == r8(ad(p19, 0xb0))
                            {
                                w8(ad(pbvar10, 0x9a211b), 1);
                            }
                        }
                    }
                    FUN_004525a0(
                        c22((pbvar10 >> 16) as u16, DAT_009a73b0),
                        1,
                        gp!(DAT_009a1ed0),
                    );
                    FUN_00451bf0(0, gp!(DAT_009a1ed0));
                    FUN_004525a0(c22(s_stack_4c as u16, u_var6), 1, gp!(DAT_009ec4d0));
                    FUN_00451bf0(1, gp!(DAT_009ec4d0));
                    DAT_00a08292 = r8(ad(r32(ad(selfent(), 0x1f)), 0x88)) as u16;
                    FUN_00450650(0xea, eecc() as i32);
                    'a87: {
                        if u_var13 < 2 {
                            break 'a87;
                        }
                        let base = ad(
                            ga!(DAT_009d1320),
                            (r8(ad(sb(), 0xb)) as u32).wrapping_mul(0x55),
                        );
                        let code: i32;
                        if cmpn(base, ga!(DAT_0051a254), 0xf) {
                            code = 0x25;
                        } else if cmpn(base, ga!(DAT_0051a230), 0x13) {
                            code = 0x25;
                        } else if cmpn(base, ga!(DAT_0051a208), 0x12) {
                            code = 0x25;
                        } else if cmpn(base, ga!(DAT_0051a21c), 0x11) {
                            code = 0x25;
                        } else if cmpn(base, ga!(DAT_0051a1f4), 0x11) {
                            code = 0x25;
                        } else if cmpn(base, ga!(DAT_0051a278), 0x12)
                            || cmpn(base, ga!(DAT_0051a244), 0xf)
                            || cmpn(base, ga!(DAT_0051a264), 0x12)
                        {
                            code = 0x27;
                        } else {
                            break 'a87;
                        }
                        FUN_00430200(1, code, DAT_009a73b0 as u32, 0xffff);
                    }
                    let u = FUN_00430200(1, 0x1e, u_var12, 0xffff);
                    return u & 0xffff0000;
                }
                2 => {
                    let mut b24: bool = r32(ad(sb(), 0x36)) == 0;
                    if !b24 {
                        let base = ad(
                            ga!(DAT_009d1320),
                            (r8(ad(sb(), 0xb)) as u32).wrapping_mul(0x55),
                        );
                        b24 = cmpn(base, ga!(DAT_0051a254), 0xf);
                        let mut cond = false;
                        if b24 && r32(ad(pbvar9, 0x1b)) != 0xffffffff {
                            pbvar9 = sb();
                            if r16(ad(sb(), 0x3a)) == 0 {
                                cond = true;
                            }
                        }
                        if cond {
                            u_var12 = 0;
                            b24 = false;
                            let mut b5 = false;
                            if r8(ad(p19, 0x232)) != 0 {
                                loop {
                                    b24 = b5;
                                    if r8(ad(ad(p19, u_var12), 0x233)) != 0 {
                                        let base50 = (u_stack_50 as u32).wrapping_mul(0x101);
                                        let i15 = base50.wrapping_add(u_var12);
                                        let b17 =
                                            r8(ad(ga!(DAT_0051c5ac), u_var12.wrapping_add(base50)));
                                        u_var6 = r8(ad(ga!(DAT_0051c5aa), i15)) as u16;
                                        while (u_var6 as u32) <= (b17 as u32) {
                                            let mut u11 = r8(ad(
                                                ga!(DAT_009d8c85),
                                                (r8(ea(i_var3, 0x16)) as u32).wrapping_mul(0x566),
                                            ))
                                                as u32;
                                            if (u_var12 as i16) == 1 {
                                                u11 = 5;
                                            }
                                            let hi = ((((r8(ea(i_var3, 0x16)) as u32)
                                                .wrapping_mul(0x2b3)
                                                >> 8)
                                                & 0xffffff)
                                                .wrapping_shl(8))
                                                | (r8(ad(ga!(DAT_0051c5b0), i15)) as u32);
                                            u11 = FUN_00425b70(
                                                hi & 0xffff00ff,
                                                r16(ad(ga!(DAT_00524208), u11.wrapping_mul(2))),
                                            );
                                            if (u11 as u16) == 0 {
                                                u11 = 1;
                                            }
                                            let k = r8(ad(
                                                ga!(DAT_009d8c84),
                                                (r8(ea(i_var3, 0x16)) as u32).wrapping_mul(0x566),
                                            ));
                                            if k == 1 {
                                                u11 = (u11 as u16).wrapping_mul(2) as u32;
                                            } else if r8(ad(
                                                ga!(DAT_009d8c84),
                                                (r8(ea(i_var3, 0x16)) as u32).wrapping_mul(0x566),
                                            )) == 2
                                            {
                                                u11 = ((u11 as u16) >> 1) as u32;
                                            }
                                            if (u11 as u16) == 0 {
                                                u11 = 1;
                                            }
                                            if 99 < (u11 as u16) {
                                                u11 = 99;
                                            }
                                            let u4 = u_var6 as u32;
                                            let b1 =
                                                r8(ad(p19, u4.wrapping_mul(4).wrapping_add(0x372)));
                                            local_38 = b1 as u32;
                                            if local_38 < (u11 & 0xffff) {
                                                b24 = true;
                                                w8(
                                                    ad(p19, u4.wrapping_mul(4).wrapping_add(0x372)),
                                                    b1.wrapping_add(1),
                                                );
                                            }
                                            w8(
                                                ad(p19, u4.wrapping_mul(4).wrapping_add(0x371)),
                                                0x7f,
                                            );
                                            u_var6 = u_var6.wrapping_add(1);
                                        }
                                    }
                                    u_var12 = (u_var12 as u16).wrapping_add(1) as u32;
                                    b5 = b24;
                                    if !(u_var12 < (r8(ad(p19, 0x232)) as u32)) {
                                        break;
                                    }
                                }
                            }
                            if r8(ad(p19, 0x246)) != 0 {
                                w8(
                                    ad(p19, 0x247),
                                    r8(ad(
                                        ga!(DAT_0051c632),
                                        (u_stack_50 as u32).wrapping_mul(0x101),
                                    )),
                                );
                            }
                            u_var6 = 1;
                            u_var13 = 0;
                            loop {
                                if (u_var6 & r16(ad(p19, 0x1e3))) != 0
                                    && (r16(ad(p19, 0x1e5)) & u_var6) == 0
                                {
                                    b24 = true;
                                    w16(ad(p19, 0x1e5), r16(ad(p19, 0x1e5)) | u_var6);
                                    break;
                                }
                                u_var6 = u_var6.wrapping_mul(2);
                                u_var13 = u_var13.wrapping_add(1);
                                if !(u_var13 < 10) {
                                    break;
                                }
                            }
                            pbvar9 = sb();
                            w8(ad(sb(), 0x3a), 0xd8);
                            pbvar8 = sb();
                            w8(ad(sb(), 0x3b), 1);
                            if b24 {
                                w8(ad(sb(), 0x36), 0x88);
                                w8(ad(sb(), 0x37), 5);
                                w8(ad(sb(), 0x38), 0);
                                w8(ad(sb(), 0x39), 0);
                                return pbvar8 & 0xffff0000;
                            }
                        }
                        break 'sw;
                    }
                    local_38 = ad(
                        ga!(DAT_009d1320),
                        (r8(ad(sb(), 0xb)) as u32).wrapping_mul(0x55),
                    );
                    let mut call: Option<(i32, u32)> = None;
                    if cmpn(local_38, ga!(DAT_0051a254), 0xf) {
                        if r32(pbvar10) != 0 {
                            let mut u13: u16 = 0;
                            loop {
                                let u7 = u13.wrapping_add(1);
                                w8(
                                    ad(p19, (u13 as u32).wrapping_add(0x201)),
                                    r8(ad(hdr(), (u13 as u32).wrapping_add(0x201))),
                                );
                                u13 = u7;
                                if !(u7 < 0x10) {
                                    break;
                                }
                            }
                            w8(ad(p19, 0x200), 2);
                        }
                        local_38 = r8(ad(
                            ga!(DAT_009d8cd5),
                            (DAT_009a73c0 as u32)
                                .wrapping_mul(0x566)
                                .wrapping_add((r8(sb()) as u32).wrapping_mul(0x52)),
                        )) as u32;
                        if local_38
                            <= (r8(ad(sb(), (r8(sb()) as u32).wrapping_add(5))) as u32)
                                .wrapping_add(1)
                        {
                            w8(ad(hdr(), 0x201), 0);
                        }
                        w8(ad(hdr(), 0x200), 1);
                        pbvar10 = ents();
                        if r32(ea(i_var3, 0x1b)) != 0xffffffff {
                            pbvar10 = eecc();
                            if r32(ea(i_var3, 0x1b)) == eecc() {
                                pbvar10 = eecc().wrapping_mul(0x16f);
                                let i15 =
                                    r32(ad(ga!(DAT_009e9670), eecc().wrapping_mul(0x5bd))) as i32;
                                let flag: bool;
                                if i15 == -1 {
                                    flag = false;
                                } else {
                                    pbvar10 = (i15 as u32).wrapping_mul(7);
                                    let b17 = r8(ad(
                                        ents(),
                                        (i15 as u32).wrapping_mul(0x23).wrapping_add(3),
                                    ));
                                    if b17 != 1 && b17 != 2 && b17 != 3 && b17 != 0xe && b17 != 4 {
                                        flag = false;
                                    } else {
                                        flag = true;
                                    }
                                }
                                if flag {
                                    let mut u13: u16 = 1;
                                    let mut s14: i16 = 0xd;
                                    loop {
                                        if (r16(ad(p19, 0x1df)) & u13) != 0
                                            && (u13 & r16(ad(p19, 0x1e1))) == 0
                                        {
                                            w16(ad(p19, 0x1e1), r16(ad(p19, 0x1e1)) | u13);
                                        }
                                        u13 = u13.wrapping_mul(2);
                                        s14 = s14.wrapping_sub(1);
                                        if s14 == 0 {
                                            break;
                                        }
                                    }
                                    FUN_004aba50();
                                    FUN_00440da0(
                                        r8(ad(ga!(DAT_009e9a77), eecc().wrapping_mul(0x5bd)))
                                            as u32,
                                        eecc() as i32,
                                    );
                                    FUN_004abe50();
                                    pbvar10 = 0;
                                }
                            }
                            pbvar10 &= 0xffff0000;
                            loop {
                                u_var12 = pbvar10 & 0xffff;
                                w8(ad(p19, u_var12.wrapping_add(0x2a1)), pbvar10 as u8);
                                let u13 = (pbvar10 as u16).wrapping_add(1);
                                pbvar10 = c22((pbvar10 >> 16) as u16, u13);
                                w16(ad(p19, u_var12.wrapping_mul(2).wrapping_add(0x2ac)), 100);
                                w16(ad(p19, u_var12.wrapping_mul(2).wrapping_add(0x2c2)), 0);
                                if !(u13 < 10) {
                                    break;
                                }
                            }
                        }
                        FUN_00451c20(
                            c22((pbvar10 >> 16) as u16, DAT_009a73b0),
                            hdr() as *mut u32,
                            0x9c,
                        );
                        call = Some((0x26, DAT_009a73b0 as u32));
                    } else if cmpn(local_38, ga!(DAT_0051a244), 0xf) {
                        if r32(pbvar10) != 0 {
                            let mut u13: u16 = 0;
                            loop {
                                let u7 = u13.wrapping_add(1);
                                let b = r8(ad(p19, (u13 as u32).wrapping_add(0x201)));
                                w8(ad(hdr(), (u13 as u32).wrapping_add(0x201)), b);
                                u13 = u7;
                                if !(u7 < 0x10) {
                                    break;
                                }
                            }
                            w8(ad(p19, 0x201), 0);
                            w8(ad(p19, 0x200), 1);
                        }
                        w8(ad(hdr(), 0x200), 2);
                        FUN_00451c20(DAT_009a73b0 as u32, hdr() as *mut u32, 0x9c);
                        call = Some((0x28, DAT_009a73b0 as u32));
                    } else if cmpn(local_38, ga!(DAT_0051a230), 0x13) {
                        if r32(pbvar10) != 0 {
                            let mut u13: u16 = 0;
                            loop {
                                let u7 = u13.wrapping_add(1);
                                u_var12 = u13 as u32;
                                let b17 = r8(ad(p19, u_var12.wrapping_add(0x201)));
                                let b1 = r8(ad(hdr(), u_var12.wrapping_add(0x201)));
                                w8(ad(hdr(), u_var12.wrapping_add(0x201)), b17);
                                w8(ad(p19, u_var12.wrapping_add(0x201)), b1);
                                u13 = u7;
                                if !(u7 < 0x10) {
                                    break;
                                }
                            }
                            w8(ad(p19, 0x200), 2);
                        }
                        w8(ad(hdr(), 0x200), 2);
                        FUN_00451c20(DAT_009a73b0 as u32, hdr() as *mut u32, 0x9c);
                    } else if !cmpn(local_38, ga!(DAT_0051a278), 0x12) {
                        if cmpn(local_38, ga!(DAT_0051a264), 0x12) {
                            if r32(pbvar10) != 0 {
                                FUN_00434500(
                                    c22((r32(pbvar10) >> 16) as u16, DAT_009a73b0),
                                    c22(s_stack_4c as u16, u_var6),
                                );
                                let i15 = r32(ea(i_var3, 0x1f));
                                if (r16(ad(i15, 0x81)) as i16) != 0 {
                                    let u13 = FUN_00425cb0() as u16;
                                    w16(
                                        ad(i15, 0x81),
                                        (((u13 as u32 & 0xf).wrapping_add(0xf)).wrapping_mul(0xec))
                                            as u16,
                                    );
                                }
                                let r = FUN_0046b610(ga!(DAT_00521210) as *mut i8);
                                w8(ad(local_3c, 10), r as u8);
                                swap_run(p19, u_var6, u_var12);
                            }
                            call = Some((0x28, DAT_009a73b0 as u32));
                        } else if cmpn(local_38, ga!(DAT_0051a21c), 0x11) {
                            if r32(pbvar10) == 0 {
                                let q = ad(
                                    ga!(DAT_009a20d6),
                                    (r8(ad(pbvar9, 0x16)) as u32).wrapping_mul(0x126),
                                );
                                w16(q, r16(q).wrapping_add(1));
                                w8(ad(pbvar9, 3), 0);
                            } else {
                                FUN_004a3750(u_var12, p19 as i32, 0x80);
                                w16(ad(hdr(), 0x66), u_var6);
                                w16(ad(p19, 0x68), DAT_009a73b0);
                                w16(ad(p19, 0x1e5), 0);
                            }
                            call = Some((0x26, DAT_009a73b0 as u32));
                        } else if cmpn(local_38, ga!(DAT_0051a208), 0x12) {
                            if r32(pbvar10) != 0 {
                                w8(ad(p19, 0x200), 2);
                            }
                            FUN_00451c20(DAT_009a73b0 as u32, hdr() as *mut u32, 0x9c);
                            call = Some((0x28, DAT_009a73b0 as u32));
                        } else if cmpn(local_38, ga!(DAT_0051a1f4), 0x11) {
                            let mut u16v: u16 = 0;
                            if r32(pbvar10) != 0 {
                                u16v = (r32(pbvar10) >> 16) as u16;
                                w16(ad(p19, 0x1e5), r16(ad(p19, 0x1e3)));
                                w16(ad(p19, 0xcf), 0);
                                w8(ad(p19, 7), 0);
                                w8(ad(p19, 0x200), 3);
                            }
                            FUN_00451c20(c22(u16v, DAT_009a73b0), hdr() as *mut u32, 0x9c);
                            FUN_00430200(1, 0x28, DAT_009a73b0 as u32, 0xffff);
                            call = Some((0x1d, u_var12));
                        }
                    } else {
                    }
                    if let Some((code, u12)) = call {
                        FUN_00430200(1, code, u12, 0xffff);
                    }
                    if false {}
                    let _ = 0;
                    // capture branch handled below
                    if cmpn(local_38, ga!(DAT_0051a278), 0x12)
                        && !cmpn(local_38, ga!(DAT_0051a254), 0xf)
                        && !cmpn(local_38, ga!(DAT_0051a244), 0xf)
                        && !cmpn(local_38, ga!(DAT_0051a230), 0x13)
                    {
                        if r32(pbvar10) != 0 {
                            FUN_004a3750(u_var12, p19 as i32, 0x80);
                            if r8(ad(p19, 0x1eb)) == 0 {
                                let b17: u8;
                                if (r8(ad(p19, 5)) as i8) == -1 {
                                    b17 = r8(ad(
                                        ga!(DAT_00524120),
                                        r8(ad(
                                            ga!(DAT_009d8cd2),
                                            (local_52 as u32).wrapping_mul(0x566),
                                        )) as u32,
                                    ));
                                } else {
                                    b17 = r8(ad(
                                        ga!(DAT_00524148),
                                        r8(ad(
                                            ga!(DAT_009d8cd2),
                                            (local_52 as u32).wrapping_mul(0x566),
                                        )) as u32,
                                    ));
                                }
                                w8(ad(local_3c, 10), r8(ad(ga!(DAT_009a7a40), b17 as u32)));
                            } else {
                                let pc: u32 = if (r16(ad(p19, 0x92)) as i16) == 0 {
                                    ga!(DAT_0051a1bc)
                                } else {
                                    ga!(DAT_00521628)
                                };
                                let r = FUN_0046b610(pc as *mut i8);
                                w8(ad(local_3c, 10), r as u8);
                                FUN_00451c20(c22(s_stack_4c as u16, u_var6), p19 as *mut u32, 0x8b);
                            }
                            FUN_00430200(1, 0x28, DAT_009a73b0 as u32, 0xffff);
                            FUN_00430200(1, 0x1f, u_var12, 0xffff);
                            swap_run(p19, u_var6, u_var12);
                        }
                    }
                    let mut u13: u16 = 0;
                    if r32(ea(i_var3, 0x1f)) != 0 {
                        if r8(ad(r32(ad(selfent(), 0x1f)), 0x88))
                            == r8(ad(r32(ea(i_var3, 0x1f)), 0x88))
                        {
                            u_var12 = r8(ad(r32(ad(selfent(), 0x1f)), 0x89)) as u32;
                            if r8(ad(ad(u_var12, 0x1f6), p19)) == 0 {
                                let mut b17: u8 = 0;
                                loop {
                                    let b1 = r8(ad(ad(u13 as u32, 0x1f6), p19));
                                    if b17 < b1 {
                                        b17 = b1;
                                    }
                                    u13 = u13.wrapping_add(1);
                                    if !(u13 < 10) {
                                        break;
                                    }
                                }
                                w8(ad(ad(u_var12, 0x1f6), p19), b17.wrapping_add(1));
                                let e16 = r8(ea(i_var3, 0x16)) as u32;
                                let q = ad(ga!(DAT_009a20da), e16.wrapping_mul(0x126));
                                w16(q, r16(q).wrapping_add(1));
                                let e16b = r8(ea(i_var3, 0x16)) as u32;
                                if r8(ad(ga!(DAT_009d8c80), e16b.wrapping_mul(0x566)))
                                    == r8(ad(p19, 0xb0))
                                {
                                    w8(ad(ga!(DAT_009a210a), e16b.wrapping_mul(0x126)), 1);
                                }
                            }
                        } else {
                            FUN_0042e4d0(0x24, -1, eecc() as i32);
                        }
                    }
                    let slot = ad(sb(), (r8(sb()) as u32).wrapping_add(5));
                    w8(slot, r8(slot).wrapping_add(1));
                    w16(
                        ad(
                            ad(hdr(), (r8(ad(hdr(), 0x7d)) as u32).wrapping_mul(2)),
                            0x7e,
                        ),
                        u_var2,
                    );
                    w8(ad(hdr(), 0x7d), r8(ad(hdr(), 0x7d)).wrapping_add(1));
                    if 9 < r8(ad(hdr(), 0x7d)) {
                        w8(ad(hdr(), 0x7d), r8(ad(hdr(), 0x7d)).wrapping_sub(1));
                    }
                    if r8(ad(hdr(), 0x7d)) == 1 {
                        u_var12 = DAT_009a73c0 as u32;
                        let i = u_var12.wrapping_mul(0x126);
                        let q = ad(ga!(DAT_009a20e2), i);
                        w16(q, r16(q).wrapping_add(1));
                        if r8(ad(ga!(DAT_009d8c80), u_var12.wrapping_mul(0x566)))
                            == r8(ad(hdr(), 0xb0))
                        {
                            w8(ad(ga!(DAT_009a210e), i), 1);
                        }
                    }
                    if r32(ea(i_var3, 0x1f)) != 0 {
                        w8(ad(p19, 0x7b), r8(ad(p19, 0x7b)).wrapping_add(1));
                        let i = (local_52 as u32).wrapping_mul(0x126);
                        let q = ad(ga!(DAT_009a20de), i);
                        w16(q, r16(q).wrapping_add(1));
                        if r8(ad(ga!(DAT_009d8c80), (local_52 as u32).wrapping_mul(0x566)))
                            == r8(ad(p19, 0xb0))
                        {
                            w8(ad(ga!(DAT_009a210c), i), 1);
                        }
                    }
                    w8(ad(sb(), 0x35), 3);
                    w8(ad(sb(), 0x36), 0x38);
                    pbvar8 = sb();
                    w8(ad(sb(), 0x37), 9);
                    w8(ad(sb(), 0x38), 0);
                    w8(ad(sb(), 0x39), 0);
                    if r16(ad(ga!(DAT_009e968a), eecc().wrapping_mul(0x5bd))) == u_var6 {
                        DAT_00a08c74 = 0xfffd;
                    }
                    pbvar9 = ents();
                    if r32(ea(i_var3, 0x1b)) != 0xffffffff {
                        w8(ad(sb(), 0x2b), 0xff);
                        w8(ad(sb(), 0x2c), 0xff);
                        return pbvar8 & 0xffff0000;
                    }
                    break 'sw;
                }
                3 => {
                    if r32(ad(sb(), 0x36)) == 0 {
                        w8(pbvar8, 0xff);
                        w8(ad(pbvar8, 1), 0xff);
                        w8(ad(sb(), 0x1a), 0);
                        w8(ad(sb(), 0x1b), 0);
                        w8(ad(sb(), 0x1e), 0);
                        return c22(u_stack_3e, 1);
                    }
                    if i_var15 != 0 {
                        FUN_004035d0(selfent() as i32, 0, 0x4000, 0);
                        w32(
                            ad(hdr(), 0xb1),
                            r32(ea(i_var3, 4))
                                .wrapping_sub(r32(ad(selfent(), 4)))
                                .wrapping_add(DAT_009d8a58 as u32),
                        );
                        w32(
                            ad(hdr(), 0xb5),
                            r32(ea(i_var3, 8))
                                .wrapping_sub(r32(ad(selfent(), 8)))
                                .wrapping_add(DAT_009d8a54 as u32),
                        );
                        let u = DAT_009d8b60 & 0xffff0000;
                        w32(
                            ad(hdr(), 0xb9),
                            r32(ea(i_var3, 0xc))
                                .wrapping_sub(r32(ad(selfent(), 0xc)))
                                .wrapping_add(DAT_009d8b60),
                        );
                        return u;
                    }
                    w32(ad(hdr(), 0xb1), 0);
                    w32(ad(hdr(), 0xb5), 0);
                    pbvar9 = hdr();
                    w8(ad(hdr(), 0xb9), 0xf4);
                    w8(ad(hdr(), 0xba), 1);
                    w8(ad(hdr(), 0xbb), 0);
                    w8(ad(hdr(), 0xbc), 0);
                }
                _ => {}
            }
        }
        pbvar9 & 0xffff0000
    }
}
