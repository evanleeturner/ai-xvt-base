// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4a02: i8;
    static mut DAT_00703c88: i32;
    static mut DAT_007092db: i8;
    static mut DAT_0071e6be: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00732a3c: i16;
    static mut DAT_0070ad51: [u8; 0];
    static mut DAT_0052b07e: [u8; 0];
    static mut DAT_0052686c: [u8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b744: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f9c0(a: u16, b: u16, c: i32);
    fn FUN_004a7aa0(a: u32) -> u32;
    fn FUN_0040a980(a: i32, b: i32);
    fn FUN_0040abf0(a: i32);
    fn FUN_00423d00(a: i32);
    fn FUN_00421580(a: i32);
    fn FUN_0041e890(a: u16);
    fn FUN_00423200(a: i32, b: u32, c: u32, d: i32) -> i32;
    fn FUN_004441e0(a: i32);
    fn FUN_00444330(a: i32);
    fn FUN_004a41d0(a: i32);
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_0040a1a0(a: u32, b: u32, c: i32);
    fn FUN_00445ba0(a: i32, b: i32);
    fn FUN_00445c50(a: i32, b: i32);
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is game memory read where the original reads.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is game memory read where the original reads.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is game memory read where the original reads.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is game memory written where the original writes.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is game memory written where the original writes.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is game memory written where the original writes.
    unsafe { (a as usize as *mut u8).write(v) }
}

unsafe fn overlap(base: u32, tbl: u32, i11: u32, i7: u32) -> bool {
    // SAFETY: all addresses are the game's own tables and object records.
    unsafe {
        let i8 = i7.wrapping_mul(0x23).wrapping_add(base);
        let i9 = r32(i11.wrapping_add(4)).wrapping_sub(r32(i8.wrapping_add(4))) as i32;
        let i13 = r32(i11.wrapping_add(8)).wrapping_sub(r32(i8.wrapping_add(8))) as i32;
        let i15 = r32(i11.wrapping_add(0xc)).wrapping_sub(r32(i8.wrapping_add(0xc))) as i32;
        let i9 = if i9 < 0 { i9.wrapping_neg() } else { i9 };
        let i15 = if i15 < 0 { i15.wrapping_neg() } else { i15 };
        let ta = r16(tbl.wrapping_add((r8(i11.wrapping_add(3)) as u32).wrapping_mul(0x16))) as i32;
        let tb = r16(tbl.wrapping_add((r8(i8.wrapping_add(3)) as u32).wrapping_mul(0x16))) as i32;
        i9.wrapping_sub(ta) < tb && i15.wrapping_sub(ta) < tb && 0 < i13 && i13 < 0x40000
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00480210(param_1: i32, param_2: i32, param_3: i32) {
    let _ = (param_2, param_3);
    // SAFETY: all globals, tables and callees are the game's own memory and
    // code; addresses are accessed exactly as the original does.
    unsafe {
        let p730 = (&raw mut DAT_0071b730) as usize as u32;
        let p744 = (&raw mut DAT_0071b744) as usize as u32;
        let p0070ad51 = (&raw mut DAT_0070ad51) as usize as u32;
        let p52b07e = (&raw mut DAT_0052b07e) as usize as u32;
        let p52686c = (&raw mut DAT_0052686c) as usize as u32;

        let i4 = (param_1 as u32).wrapping_mul(0x5bd);
        let mut i11: u32 = r32(p730.wrapping_add(i4));
        let i9 = i11.wrapping_mul(0x23);
        let base = DAT_006d4988 as u32;
        let mut i7 = base.wrapping_add(i9);
        let i_tmp = r32(base.wrapping_add(0x1f).wrapping_add(i9));
        let i8 = r32(i_tmp.wrapping_add(0xa7));
        if r8(i8.wrapping_add(7)) != 0 {
            w8(p744.wrapping_add(i4), 0);
        }
        let u_tmp = r32(p730.wrapping_add(0x15).wrapping_add(i4));
        let mut u6 = (DAT_0071e6be as u32).wrapping_add(u_tmp);
        w32(p730.wrapping_add(0x15).wrapping_add(i4), u6);

        if r8(p744.wrapping_add(i4)) != 1 {
            if r8(p744.wrapping_add(i4)) != 2 {
                return;
            }
            if 0x49b < u6 {
                let b3 = r8(i7.wrapping_add(0x16));
                FUN_0049f9c0(i11 as u16, b3 as u16, 0x11);
                if DAT_006d4a02 != 2
                    && DAT_007092db == 1
                    && r8(p0070ad51.wrapping_add((b3 as u32).wrapping_mul(0x566))) != 0x63
                {
                    let arg = (((DAT_006d4988 as u32 >> 8) << 8)
                        | (r8(base.wrapping_add(3).wrapping_add(i9)) as u32))
                        & 0xffff00ff;
                    u6 = FUN_004a7aa0(arg);
                    let mut t = r32(p730.wrapping_add(0x6d).wrapping_add(i4));
                    t = t.wrapping_add(
                        (r8(p52b07e.wrapping_add((u6 & 0xffff).wrapping_mul(0xef))) as u32)
                            .wrapping_mul(0x28),
                    );
                    w32(p730.wrapping_add(0x6d).wrapping_add(i4), t);
                }
                let t = r32(p730.wrapping_add(i4));
                if t as i32 != -1 {
                    FUN_0040a980(0, param_1);
                    FUN_0040abf0(0);
                }
                w8(DAT_006d4988 as u32 + 3u32.wrapping_add(i9), 0);
                FUN_00423d00(param_1);
                FUN_00421580(i8 as i32);
                w8(p744.wrapping_add(i4), 0);
                FUN_0041e890(b3 as u16);
                let r = FUN_00423200(param_1, 0xffffffff, 0, 0);
                if r != 0 {
                    FUN_004441e0(param_1);
                    FUN_00444330(param_1);
                    return;
                }
                if DAT_0071ecf4 == param_1 {
                    FUN_004a41d0(0x119);
                }
                return;
            }
            let mut b5 = false;
            i7 = r32(p730.wrapping_add(i4));
            let mut s = DAT_00732a3c;
            while (s as i32) < DAT_006d4980 {
                let b = DAT_006d4988 as u32;
                i11 = ((s as i32) as u32).wrapping_mul(0x23).wrapping_add(b);
                if r8(i11.wrapping_add(3)) != 0 && overlap(b, p52686c, i11, i7) {
                    b5 = true;
                    break;
                }
                s = s.wrapping_add(1);
            }
            if !b5 {
                let mut s = DAT_00732a38 as i16;
                while (s as i32) < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32 {
                    let b = DAT_006d4988 as u32;
                    i11 = ((s as i32) as u32).wrapping_mul(0x23).wrapping_add(b);
                    if r8(i11.wrapping_add(3)) != 0 && overlap(b, p52686c, i11, i7) {
                        b5 = true;
                        break;
                    }
                    s = s.wrapping_add(1);
                }
                if !b5 {
                    let u12 = (u6 / 0xec) as u16;
                    let v: i32 = if u12 == 0 {
                        10
                    } else if u12 == 1 {
                        0x19
                    } else if u12 == 2 {
                        0x32
                    } else if u12 == 3 {
                        100
                    } else {
                        1000
                    };
                    FUN_00445ba0(i7 as i32, v);
                    let addr = i7
                        .wrapping_mul(0x23)
                        .wrapping_add(8)
                        .wrapping_add(DAT_006d4988 as u32);
                    let add = (u12 as u32)
                        .wrapping_mul(DAT_0071e6be as u32)
                        .wrapping_mul(0xe0);
                    w32(addr, r32(addr).wrapping_add(add));
                    return;
                }
            }
            FUN_004a1d70(0x6f, param_1);
            let t = r32(p730.wrapping_add(i4));
            let flag;
            if t as i32 == -1 {
                flag = false;
            } else {
                let c = r8(t
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_006d4988 as u32)) as i8;
                flag = !(c != 1 && c != 2 && c != 3 && c != 0xe && c != 4);
            }
            let a: u32 = if flag { 0x54 } else { 0x24 };
            FUN_0040a1a0(a, 0xffffffff, param_1);
            w8(p744.wrapping_add(i4), 0);
            let t = r32(i7
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988 as u32));
            w16(t.wrapping_add(0x79), 10);
            return;
        }

        let mut u12 = r16(i7.wrapping_add(0x14));
        let mut u14 = r16(i7.wrapping_add(0x10));
        let mut u10 = r16(i7.wrapping_add(0x12));
        let dt = DAT_0071e6be as u32;
        'lab: {
            if u12 == 0 && u10 == 0x4000 && u14 == 0 {
                u12 = 0;
                u14 = 0;
                if 0x49b < u6 {
                    w8(p744.wrapping_add(i4), 2);
                    FUN_004a1d70(0x6b, param_1);
                    w32(p730.wrapping_add(0x15).wrapping_add(i4), 0);
                    FUN_0040a1a0(0x51, 0xffffffff, param_1);
                }
                break 'lab;
            }
            if u12 < 0x8000 {
                u12 = (u12 as u32).wrapping_add(dt.wrapping_mul(0x20u32.wrapping_neg())) as u16;
                if 0x7fff < u12 {
                    u12 = 0;
                }
            } else {
                u12 = (u12 as u32).wrapping_add(dt.wrapping_mul(0x20)) as u16;
                if u12 < 0x8000 {
                    u12 = 0;
                }
            }
            if u14 < 0x8000 {
                u14 = (u14 as u32).wrapping_add(dt.wrapping_mul(0x10u32.wrapping_neg())) as u16;
                if 0x7fff < u14 {
                    u14 = 0;
                }
            } else {
                u14 = (u14 as u32).wrapping_add(dt.wrapping_mul(0x10)) as u16;
                if u14 < 0x8000 {
                    u14 = 0;
                }
            }
            if u10 < 0xc000 && 0x4000 < u10 {
                u10 = (u10 as u32).wrapping_add(dt.wrapping_mul(0x10u32.wrapping_neg())) as u16;
                if 0x3fff < u10 {
                    break 'lab;
                }
            } else {
                if u10 == 0x4000 {
                    break 'lab;
                }
                u10 = (u10 as u32).wrapping_add(dt.wrapping_mul(0x10)) as u16;
                if u10 < 0x4001 || 0xbfff < u10 {
                    break 'lab;
                }
            }
            u10 = 0x4000;
        }
        let b = DAT_006d4988 as u32;
        w16(b.wrapping_add(0x14).wrapping_add(i9), u12);
        w16(b.wrapping_add(0x10).wrapping_add(i9), u14);
        w16(b.wrapping_add(0x12).wrapping_add(i9), u10);
        let t = r32(b.wrapping_add(0x1f).wrapping_add(i9));
        w8(t.wrapping_add(0x90), 1);
        let p = r32(b.wrapping_add(0x1f).wrapping_add(i9));
        w8(p.wrapping_add(0x89), r8(p.wrapping_add(0x90)));
        let t = r32(b.wrapping_add(0x1f).wrapping_add(i9));
        let t2 = r32(t.wrapping_add(0xa7));
        w16(t2.wrapping_add(0xd), u10);
        FUN_00445c50(i11 as i32, 200);
    }
}
