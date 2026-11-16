// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_00a08104: u32;
    static mut DAT_009a8c00: u32;
    static mut DAT_009d8a58: u32;
    static mut DAT_009d8a54: u32;
    static mut DAT_009d8b60: u32;
    static mut DAT_009d8cd4: [u8; 1];
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_00521f1c: [u8; 1];
    static mut DAT_00527768: [i16; 1];
    static mut DAT_005277a0: [i16; 1];
    static mut DAT_005277d8: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004034e0(a: u32, b: u32);
    fn FUN_00404620(a: u32) -> u32;
    fn FUN_004a4cb0(a: u32);
    fn FUN_004a4d70(a: u32, b: u32) -> u32;
    fn FUN_004035d0(a: u32, b: i16, c: i16, d: i16);
    fn FUN_004a44d0(a: i16, b: i32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a1a50() -> u32 {
    // SAFETY: all globals, callees and addresses are the game's own memory;
    // reads and writes happen in the original's order.
    unsafe {
        let arr_b6 = (&raw mut DAT_009a73b6) as *const u8;
        let tab_1f1c = ((&raw mut DAT_00521f1c) as *mut u8) as usize as u32;
        let pb = || (arr_b6 as *const u32).read_unaligned();

        let u_var9: u32 = DAT_009a73b0 as u32;
        let mut u_var8: u32 = 0xff;
        let mut u_var6: u16 = DAT_00a07cec;
        let mut i_var7: u32;
        while (u_var6 as i32) < DAT_009a1fe0 {
            i_var7 = (u_var6 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8);
            if r8(i_var7.wrapping_add(3)) != 0 {
                let p = pb();
                if r8(i_var7.wrapping_add(0x16)) == r8(p.wrapping_add(0x2d)) {
                    let t1 = r32(i_var7.wrapping_add(0x1f));
                    let t2 = r32(t1.wrapping_add(0xa9));
                    if r8(t2.wrapping_add(5)) == 0xff {
                        u_var8 = u_var6 as u32;
                        break;
                    }
                }
            }
            u_var6 = u_var6.wrapping_add(1);
        }
        if (u_var8 as i16) as i32 == 0xff {
            FUN_004a44d0(0, 1);
            u_var8 = FUN_004a4d70(DAT_009a73b0 as u32, 0x8000);
            return u_var8 & 0xffff0000;
        }
        FUN_004034e0(u_var8, u_var9);
        let i_var4 = u_var8.wrapping_mul(0x23);
        let mut i_var5 = DAT_009a1fe8.wrapping_add(i_var4);
        let t1 = r32(i_var5.wrapping_add(0x1f));
        i_var7 = r32(t1.wrapping_add(0xa9));
        u_var8 = 0x20000;
        let t2 = r32(tab_1f1c.wrapping_add((r8(i_var5.wrapping_add(3)) as u32).wrapping_mul(0x1a)));
        if (t2 as i32) < 3000 {
            u_var8 = 0x8000;
        }
        let s1 = r16(i_var7.wrapping_add(0x1e5)) as i16;
        if u_var8 < DAT_009a8c00 || s1 == 0 {
            let p = pb();
            let t1 = r32(i_var5.wrapping_add(4));
            w32(p.wrapping_add(0x1f), t1);
            let p = pb();
            let t1 = r32(DAT_009a1fe8.wrapping_add(8).wrapping_add(i_var4));
            w32(p.wrapping_add(0x23), t1);
            let p = pb();
            let t1 = r32(DAT_009a1fe8.wrapping_add(0xc).wrapping_add(i_var4));
            w32(p.wrapping_add(0x27), t1);
            let s1 = r16(i_var7.wrapping_add(0x1e5)) as i16;
            let s_var10: i16 = if s1 == 0 { 0x4000 } else { 0 };
            FUN_004a44d0(s_var10, 1);
            let u_var11: u16 = if (DAT_009a8c00 as i32) < 0x10001 {
                0x4000
            } else {
                0xffff
            };
            FUN_004a4d70(u_var9, u_var11 as u32);
            w32(DAT_00a08104.wrapping_add(0xb1), 0);
            w32(DAT_00a08104.wrapping_add(0xb5), 0);
            u_var8 = DAT_00a08104;
            w32(DAT_00a08104.wrapping_add(0xb9), 0);
            return u_var8 & 0xffff0000;
        }
        let mut doit = false;
        if r8(i_var7.wrapping_add(0xab)) == 2 {
            let s1 = r16(i_var7.wrapping_add(0x5c));
            let p = pb();
            w16(p.wrapping_add(0x32), s1);
            doit = true;
        } else {
            let s1 = r16(DAT_009a1fe8
                .wrapping_add(0x10)
                .wrapping_add(u_var9.wrapping_mul(0x23))) as i16;
            let s2 = r16(i_var5.wrapping_add(0x10)) as i16;
            if s1 != s2 {
                let s1 = r16(i_var5.wrapping_add(0x10));
                let p = pb();
                w16(p.wrapping_add(0x32), s1);
                doit = true;
            }
        }
        if doit {
            u_var8 = FUN_00404620(DAT_00a08104);
            FUN_004a4cb0((((u_var8 >> 3) as u16 & 0x1fff) as u32).wrapping_add(0x4000));
        }
        let t1 = r32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4));
        u_var6 = r16(t1.wrapping_add(0x79));
        i_var5 = u_var9.wrapping_mul(0x23);
        let t1 = r32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var5));
        let u_var2 = r16(t1.wrapping_add(0x79));
        'l: {
            let u_var11: u16;
            if u_var2 < u_var6 {
                let u_var3 = r16(DAT_00a08104.wrapping_add(0xbd));
                FUN_004a4d70(
                    u_var9,
                    (u_var3 as u32).wrapping_add(((u_var6 - u_var2) as u32).wrapping_mul(0x32)),
                );
                let us1 = r16(DAT_00a08104.wrapping_add(0xbd));
                if u_var3 <= us1 {
                    break 'l;
                }
                u_var11 = 0xffff;
            } else {
                if u_var2 <= u_var6 {
                    break 'l;
                }
                let u_var3 = r16(DAT_00a08104.wrapping_add(0xbd));
                FUN_004a4d70(
                    u_var9,
                    (u_var3 as u32).wrapping_add(
                        ((u_var2 - u_var6) as u32).wrapping_mul(0x32u32.wrapping_neg()),
                    ),
                );
                let us1 = r16(DAT_00a08104.wrapping_add(0xbd));
                if us1 <= u_var3 {
                    break 'l;
                }
                u_var11 = 0;
            }
            FUN_004a4d70(u_var9, u_var11 as u32);
        }
        let s_var10 = r16(i_var7.wrapping_add(0xd)) as i16;
        let s1 = r16(DAT_00a08104.wrapping_add(0xd)) as i16;
        u_var6 = (s1 as u32).wrapping_sub(s_var10 as u32) as u16;
        if 0x7fff < u_var6 {
            u_var6 = u_var6.wrapping_neg();
        }
        if u_var6 < 0x400 {
            w16(DAT_00a08104.wrapping_add(0xd), s_var10 as u16);
            w8(DAT_00a08104.wrapping_add(0x9c), 0);
        } else {
            let p = pb();
            w16(p.wrapping_add(0x2e), s_var10 as u16);
            let us1 = r16(DAT_00a08104.wrapping_add(0xd));
            let p = pb();
            let u_var3 = r16(p.wrapping_add(0x2e));
            if us1 < u_var3 {
                w8(DAT_00a08104.wrapping_add(0x9c), 2);
            } else {
                w8(DAT_00a08104.wrapping_add(0x9c), 1);
            }
            w16(DAT_00a08104.wrapping_add(0x9e), 0xffff);
            w8(DAT_00a08104.wrapping_add(0x9d), 0);
        }
        let s_var10 = r16(DAT_009a1fe8.wrapping_add(0x14).wrapping_add(i_var4)) as i16;
        let psv1 = DAT_009a1fe8.wrapping_add(0x14).wrapping_add(i_var5);
        let s1 = r16(psv1) as i16;
        u_var6 = (s1 as u32).wrapping_sub(s_var10 as u32) as u16;
        if 0x7fff < u_var6 {
            u_var6 = u_var6.wrapping_neg();
        }
        if u_var6 < 0x400 {
            w16(psv1, s_var10 as u16);
            let t1 = r32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var5));
            w8(t1.wrapping_add(0x92), 1);
            w8(DAT_00a08104.wrapping_add(0xa4), 0);
        } else {
            let p = pb();
            w16(p.wrapping_add(0x30), s_var10 as u16);
            w16(DAT_00a08104.wrapping_add(0xa5), 0xffff);
            w8(DAT_00a08104.wrapping_add(0xa4), 1);
        }
        let bidx = (&raw mut DAT_009d8cd4)
            .cast::<u8>()
            .add(
                (DAT_009a73c0 as u32)
                    .wrapping_mul(0x566)
                    .wrapping_add((DAT_009a73c2 as u32).wrapping_mul(0x52))
                    as usize,
            )
            .read() as usize;
        FUN_004035d0(
            DAT_009a1fe8.wrapping_add(i_var4),
            (&raw mut DAT_00527768).cast::<i16>().add(bidx).read(),
            (&raw mut DAT_005277a0).cast::<i16>().add(bidx).read(),
            (&raw mut DAT_005277d8).cast::<i16>().add(bidx).read(),
        );
        let t2 = r32(tab_1f1c.wrapping_add(
            (r8(DAT_009a1fe8.wrapping_add(i_var4).wrapping_add(3)) as u32).wrapping_mul(0x1a),
        ));
        if 2999 < t2 as i32 {
            DAT_009d8a58 = DAT_009d8a58.wrapping_shl(4);
            DAT_009d8a54 = DAT_009d8a54.wrapping_shl(4);
            DAT_009d8b60 = DAT_009d8b60.wrapping_shl(4);
        }
        i_var7 = (r8(i_var7.wrapping_add(5)) as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        let t1 = r32(i_var7.wrapping_add(0x1b));
        if t1 != 0xffffffff {
            let t2 = r32(i_var7.wrapping_add(0x1f));
            let us1 = r16(t2.wrapping_add(0x79));
            if us1 < 0xb {
                w32(DAT_00a08104.wrapping_add(0xb1), 0);
                w32(DAT_00a08104.wrapping_add(0xb5), 0);
                u_var8 = DAT_00a08104;
                w32(DAT_00a08104.wrapping_add(0xb9), 0);
                return u_var8 & 0xffff0000;
            }
        }
        let t1 = r32(DAT_009a1fe8.wrapping_add(i_var4).wrapping_add(4));
        let t2 = r32((DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(4)
            .wrapping_add(DAT_009a1fe8));
        let t3 = t1.wrapping_sub(t2).wrapping_add(DAT_009d8a58);
        w32(DAT_00a08104.wrapping_add(0xb1), t3);
        let t1 = r32(DAT_009a1fe8.wrapping_add(8).wrapping_add(i_var4));
        let t2 = r32((DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(8)
            .wrapping_add(DAT_009a1fe8));
        let t3 = t1.wrapping_sub(t2).wrapping_add(DAT_009d8a54);
        w32(DAT_00a08104.wrapping_add(0xb5), t3);
        u_var8 = DAT_009d8b60 & 0xffff0000;
        let t1 = r32(DAT_009a1fe8.wrapping_add(0xc).wrapping_add(i_var4));
        let t2 = r32((DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0xc)
            .wrapping_add(DAT_009a1fe8));
        let t4 = t1.wrapping_sub(t2).wrapping_add(DAT_009d8b60);
        w32(DAT_00a08104.wrapping_add(0xb9), t4);
        u_var8
    }
}
