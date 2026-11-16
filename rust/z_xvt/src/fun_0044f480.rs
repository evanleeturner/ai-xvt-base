// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d5a: u16;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d9d5c: u32;
    static mut DAT_006d9d78: u8;
    static mut DAT_006d9d56: [u8; 4];
    static mut DAT_00703ca0: [i8; 0];
    static mut DAT_006d499c: i32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_00732a3c: u16;
    static mut DAT_00732e54: *mut u8;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: i32;
    static mut s_disableldr1pln_00525028: [i8; 0];
    static mut s_capfreeldr1pln_00525060: [i8; 0];
    static mut DAT_00525ce0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(a: u32) -> u32;
    fn FUN_00429a20(a: u32) -> u8;
    fn FUN_0042a1a0(a: u32, b: u32) -> u32;
    fn FUN_00444040(a: u32, b: u32) -> u8;
    fn FUN_00429920(a: u32, b: u32, c: u32) -> i32;
    fn FUN_0042af80(a: u32) -> u32;
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

unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a as usize as *mut u8).write(v) }
}

unsafe fn base() -> u32 {
    // SAFETY: reads the game's global pointer.
    unsafe { DAT_006d4988 as usize as u32 }
}

unsafe fn p56() -> u32 {
    // SAFETY: reads the game's packed pointer global.
    unsafe { u32::from_ne_bytes(DAT_006d9d56) }
}

unsafe fn success(v: u16) -> u32 {
    // SAFETY: the game's globals and records are written where the original writes.
    unsafe {
        let p = p56();
        w16(p.wrapping_add(0x18), v);
        let us = r16(base().wrapping_add((v as u32).wrapping_mul(0x23)));
        let p = p56();
        w16(p.wrapping_add(0x1a), us);
        w8(p.wrapping_add(0x1e), 1);
        (base() & 0xffff0000) | 1
    }
}

unsafe fn same(a: *const i8, b: *const i8, n: usize) -> bool {
    let mut i = 0usize;
    while i < n {
        // SAFETY: both are the game's NUL-padded strings, at least n bytes long.
        if unsafe { a.add(i).read() != b.add(i).read() } {
            return false;
        }
        i += 1;
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0044f480() -> u32 {
    // SAFETY: all globals, callees and record pointers are the game's own,
    // accessed in the original's order.
    unsafe {
        let i_var12: u32 = (DAT_006d9d5a as u32).wrapping_mul(0x23);
        let mut i_var7: i32 = r32(base().wrapping_add(0x1b).wrapping_add(i_var12)) as i32;
        let mut u_var4: u32 = FUN_0042b150(DAT_006d9d5c);
        let mut p = p56();
        DAT_006d9d78 = (r8(p.wrapping_add(0xb)) == 0) as u8;
        let pc1: *mut i8 = (&raw mut DAT_00703ca0)
            .cast::<i8>()
            .byte_add((r8(p.wrapping_add(0xb)) as usize) * 0x55);
        let mut i_var6: i32 = 0xf;
        let mut pc13: *const i8 = pc1;
        let mut pc14: *const i8 = (&raw mut s_disableldr1pln_00525028).cast::<i8>();
        let mut pc11: u32;
        loop {
            pc11 = pc13 as usize as u32;
            if i_var6 == 0 {
                break;
            }
            i_var6 -= 1;
            pc11 = pc13.add(1) as usize as u32;
            DAT_006d9d78 = (pc13.read() == pc14.read()) as u8;
            pc13 = pc13.add(1);
            pc14 = pc14.add(1);
            if DAT_006d9d78 == 0 {
                break;
            }
        }

        let t = r8(u_var4.wrapping_add(0x34)) as i8;
        if t != 0xc && t != 0x17 && i_var7 == -1 {
            return u_var4 & 0xffff0000;
        }
        p = p56();
        let mut u_var8: u16 = r16(p.wrapping_add(0x2b));
        if u_var8 != 0xffff && u_var8 != 0xfb {
            pc11 = u_var8 as u32;
            let b = FUN_00429a20(pc11);
            if b != 0 {
                if i_var7 != -1 {
                    u_var4 = FUN_0042a1a0(DAT_006d9d50 as u32, pc11);
                    if 0x50000 < DAT_006d499c {
                        return u_var4 & 0xffff0000;
                    }
                }
                return success(u_var8);
            }
            p = p56();
            w16(p.wrapping_add(0x2b), 0xffff);
        }
        u_var8 = 0xffff;
        i_var7 = r32(base().wrapping_add(0x1b).wrapping_add(i_var12)) as i32;
        'end: {
            if i_var7 == -1 {
                u_var8 = r16(u_var4.wrapping_add(0x18));
                if u_var8 == 0xffff {
                    break 'end;
                }
            } else {
                u_var4 = ((pc11 >> 16) << 16) | (DAT_00732a3c as u32);
                if (DAT_00732a3c as i32) < DAT_006d4980 {
                    loop {
                        let pe = base().wrapping_add((u_var4 & 0xffff).wrapping_mul(0x23));
                        let u_var10 = u_var4 as u16;
                        if r8(pe.wrapping_add(3)) != 0 {
                            let mut pobj = r32(pe.wrapping_add(0x1f));
                            pobj = r32(pobj.wrapping_add(0xa7));
                            let us = r16(pobj.wrapping_add(0x1df));
                            if (DAT_006d9d78 == 0 || us != 0)
                                && r16((DAT_00732e54 as usize as u32).wrapping_add(0x342))
                                    != u_var10
                            {
                                let idx = r16(pobj.wrapping_add(0x6a));
                                let i6 = r32(base()
                                    .wrapping_add((idx as u32).wrapping_mul(0x23))
                                    .wrapping_add(0x1b))
                                    as i32;
                                if i6 == i_var7 {
                                    let q = r32(base().wrapping_add(0x1f).wrapping_add(i_var12));
                                    let b = FUN_00444040(u_var4, r8(q.wrapping_add(0x87)) as u32);
                                    if b != 0 {
                                        u_var8 = u_var10;
                                    }
                                }
                            }
                        }
                        let n = u_var10.wrapping_add(1);
                        u_var4 = (u_var4 & 0xffff0000) | (n as u32);
                        if !((n as i32) < DAT_006d4980) {
                            break;
                        }
                    }
                }
                if u_var8 == 0xffff {
                    loop {
                        u_var4 = 0;
                        let s = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32;
                        if !(0 < s) {
                            break;
                        }
                    }
                    break 'end;
                }
            }
            let mut pe = base().wrapping_add((u_var8 as u32).wrapping_mul(0x23));
            u_var4 = r32(pe.wrapping_add(0x1f));
            if u_var4 == 0 {
                let c_var2 = r8(pe.wrapping_add(0x16)) as i8;
                let mut u_var10: u16 = u_var8.wrapping_add(1);
                let sum = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32);
                if (sum as i32) <= (u_var10 as i32) {
                    u_var10 = DAT_00732a38 as u16;
                }
                u_var4 = (DAT_00732a38 as u32) & 0xffff;
                u_var8 = DAT_00732a38 as u16;
                if (u_var4 as i32) < (sum as i32) {
                    loop {
                        let us = r16(base()
                            .wrapping_add((u_var10 as u32).wrapping_mul(0x23))
                            .wrapping_add(0x17));
                        if (DAT_006d9d78 == 0 || us != 0)
                            && r16((DAT_00732e54 as usize as u32).wrapping_add(0x342)) != u_var10
                        {
                            pe = base().wrapping_add((u_var10 as u32).wrapping_mul(0x23));
                            if r8(pe.wrapping_add(3)) != 0
                                && (r8(pe.wrapping_add(0x16)) as i8) == c_var2
                            {
                                i_var7 = FUN_00429920(DAT_006d9d50 as u32, u_var10 as u32, 1);
                                if i_var7 != 0 {
                                    let u5 = FUN_0042af80(u_var10 as u32);
                                    if (u5 as i16) != 0 {
                                        return success(u_var10);
                                    }
                                }
                            }
                            u_var10 = u_var10.wrapping_add(1);
                            let s2 = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32;
                            if s2 <= (u_var10 as i32) {
                                u_var10 = DAT_00732a38 as u16;
                            }
                        }
                        u_var8 = u_var8.wrapping_add(1);
                        let s3 = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32);
                        if (s3 as i32) <= (u_var8 as i32) {
                            return s3 & 0xffff0000;
                        }
                    }
                }
            } else {
                let ti = r32(u_var4.wrapping_add(0xa7));
                if ti != 0 {
                    let c_var2 = r8(pe.wrapping_add(0x16)) as i8;
                    u_var8 = u_var8
                        .wrapping_add(r8((DAT_00732e54 as usize as u32).wrapping_add(0xb0)) as u16);
                    if DAT_006d4980 <= (u_var8 as i32) {
                        u_var8 = DAT_00732a3c;
                    }
                    u_var4 = DAT_00732a3c as u32;
                    let mut u_var10: u16 = DAT_00732a3c;
                    if (u_var4 as i32) < DAT_006d4980 {
                        loop {
                            pe = base().wrapping_add((u_var8 as u32).wrapping_mul(0x23));
                            let mut pobj = r32(pe.wrapping_add(0x1f));
                            pobj = r32(pobj.wrapping_add(0xa7));
                            let mut us: u16 = 0;
                            if pobj != 0 {
                                us = r16(pobj.wrapping_add(0x1df));
                            }
                            if pobj == 0
                                || (DAT_006d9d78 != 0 && us == 0)
                                || r16((DAT_00732e54 as usize as u32).wrapping_add(0x342))
                                    == u_var10
                            {
                                let u9 = u_var8.wrapping_add(1);
                                u_var8 = u_var8.wrapping_add(1);
                                if DAT_006d4980 <= (u9 as i32) {
                                    u_var8 = DAT_00732a3c;
                                }
                            } else {
                                if r8(pe.wrapping_add(3)) != 0
                                    && (r8(pe.wrapping_add(0x16)) as i8) == c_var2
                                {
                                    i_var7 = FUN_00429920(DAT_006d9d50 as u32, u_var8 as u32, 1);
                                    let mut b3 = i_var7 == 0;
                                    if !b3 {
                                        b3 = same(
                                            pc1,
                                            (&raw const s_capfreeldr1pln_00525060).cast::<i8>(),
                                            15,
                                        );
                                        if !b3 {
                                            b3 = same(
                                                pc1,
                                                (&raw const s_disableldr1pln_00525028).cast::<i8>(),
                                                15,
                                            );
                                            if !b3 {
                                                b3 = same(
                                                    pc1,
                                                    (&raw const DAT_00525ce0).cast::<i8>(),
                                                    13,
                                                );
                                                if !b3 {
                                                    return success(u_var8);
                                                }
                                            }
                                        }
                                        let u5 = FUN_0042af80(u_var8 as u32);
                                        if (u5 as i16) != 0 {
                                            return success(u_var8);
                                        }
                                    }
                                }
                                u_var8 = u_var8.wrapping_add(1);
                                if DAT_006d4980 <= (u_var8 as i32) {
                                    u_var8 = DAT_00732a3c;
                                }
                            }
                            u_var10 = u_var10.wrapping_add(1);
                            u_var4 = u_var10 as u32;
                            if !((u_var4 as i32) < DAT_006d4980) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        u_var4 & 0xffff0000
    }
}
