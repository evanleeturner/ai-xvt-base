// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00550b88: *mut u32;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a8e24: i32;
    static mut DAT_009cd274: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d6820: i32;
    static mut DAT_00a07bd0: i32;
    static mut DAT_009a8e18: i32;
    static mut DAT_009a8da0: i32;

    static mut DAT_009ed228: u32;
    static mut _DAT_009ed22c: u32;
    static mut DAT_009d8b70: u32;
    static mut DAT_009d8b74: u32;
    static mut DAT_009a2000: i16;
    static mut DAT_009a20c0: u32;
    static mut DAT_009d8c30: u32;
    static mut DAT_009d6940: u32;
    static mut DAT_009ec5e0: u32;
    static mut DAT_00a08294: i16;
    static mut DAT_00a080f8: u32;
    static mut DAT_009d8c20: u8;
    static mut DAT_009eca28: u32;
    static mut DAT_009a7ba4: u32;
    static mut DAT_009fd430: u32;
    static mut DAT_00a0814c: u32;
    static mut DAT_009a7b50: u32;
    static mut DAT_009d767c: u32;
    static mut _DAT_00a07cec: u32;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009d1144: u32;
    static mut DAT_009fe7dc: u32;
    static mut DAT_009e9640: u32;
    static mut DAT_00a00854: u32;
    static mut DAT_009a7b58: u32;
    static mut DAT_009d1140: u32;
    static mut DAT_009d1310: u32;
    static mut DAT_009ec47c: u32;
    static mut DAT_009d1320: u32;
    static mut DAT_009a8068: u32;
    static mut DAT_009a73a4: u32;
    static mut DAT_009a7a40: u32;
    static mut DAT_009d113c: i16;
    static mut DAT_009a8d40: i16;
    static mut DAT_00523424: u32;
    static mut DAT_009e9670: u32;
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u8).read() }
}

unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as *mut u32).write_unaligned(v) }
}

unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as *mut u8).write(v) }
}

unsafe fn cpy(d: u32, s: u32, n: u32) {
    let mut i: u32 = 0;
    // SAFETY: source and destination are the game's own buffers, copied front to back.
    unsafe {
        while i.wrapping_add(4) <= n {
            w32(d.wrapping_add(i), r32(s.wrapping_add(i)));
            i = i.wrapping_add(4);
        }
        while i < n {
            w8(d.wrapping_add(i), r8(s.wrapping_add(i)));
            i = i.wrapping_add(1);
        }
    }
}

unsafe fn zero(d: u32, n: u32) {
    let mut i: u32 = 0;
    // SAFETY: the destination is the game's own buffer, cleared front to back.
    unsafe {
        while i.wrapping_add(4) <= n {
            w32(d.wrapping_add(i), 0);
            i = i.wrapping_add(4);
        }
        while i < n {
            w8(d.wrapping_add(i), 0);
            i = i.wrapping_add(1);
        }
    }
}

unsafe fn ent(o: u32, off: u32) -> u32 {
    // SAFETY: reads the game's own entity table base.
    unsafe { (DAT_009a1fe8 as u32).wrapping_add(o).wrapping_add(off) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00416e50() {
    // SAFETY: every address is the game's own memory, the save stream and the
    // globals being read and written where the original does.
    unsafe {
        let mut i: i32 = 0;
        let mut p7: u32 = DAT_00550b88 as u32;
        if 0 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            let mut off: u32 = 0;
            let mut p6: u32 = p7;
            loop {
                if i < DAT_009a8e24 || {
                    p7 = p6;
                    DAT_009cd274 <= i
                } {
                    p7 = p6.wrapping_add(1);
                    w8(ent(3, off), r8(p6));
                    if r8(ent(3, off)) == 0 {
                        zero(ent(0, off), 31);
                        w32(ent(0x1b, off), 0xFFFF_FFFF);
                        let s = r32(ent(0x1f, off));
                        if s != 0 {
                            zero(s, 139);
                            let s = r32(ent(0x1f, off));
                            w8(s.wrapping_add(0x88), 0xff);
                            let s = r32(ent(0x1f, off));
                            let a = r32(s.wrapping_add(0xa9));
                            if a != 0 {
                                zero(a, 0x104 * 4 + 2);
                            }
                            let s = r32(ent(0x1f, off));
                            let b = r32(s.wrapping_add(0xa5));
                            if b != 0 {
                                zero(b, 10);
                            }
                            let s = r32(ent(0x1f, off));
                            let c = r32(s.wrapping_add(0xad));
                            if c != 0 {
                                zero(c, 0x13 * 4);
                            }
                        }
                    } else {
                        cpy(ent(0, off), p7, 35);
                        p7 = p6.wrapping_add(36);
                        let v = r32(ent(0x1f, off));
                        if v != 0 {
                            w32(ent(0x1f, off), v.wrapping_sub(1));
                            let t = r32(ent(0x1f, off));
                            w32(ent(0x1f, off), t.wrapping_add(DAT_009d6820 as u32));
                        }
                        let s = r32(ent(0x1f, off));
                        if s != 0 {
                            cpy(s, p7, 177);
                            let mut p10: u32 = p6.wrapping_add(0xd5);

                            let s = r32(ent(0x1f, off));
                            let v = r32(s.wrapping_add(0xa9));
                            if v != 0 {
                                w32(s.wrapping_add(0xa9), v.wrapping_sub(1));
                                let s = r32(ent(0x1f, off));
                                let t = r32(s.wrapping_add(0xa9));
                                w32(s.wrapping_add(0xa9), t.wrapping_add(DAT_00a07bd0 as u32));
                            }
                            let s = r32(ent(0x1f, off));
                            let v = r32(s.wrapping_add(0xa5));
                            if v != 0 {
                                w32(s.wrapping_add(0xa5), v.wrapping_sub(1));
                                let s = r32(ent(0x1f, off));
                                let t = r32(s.wrapping_add(0xa5));
                                w32(s.wrapping_add(0xa5), t.wrapping_add(DAT_009a8e18 as u32));
                            }
                            let s = r32(ent(0x1f, off));
                            let v = r32(s.wrapping_add(0xad));
                            if v != 0 {
                                w32(s.wrapping_add(0xad), v.wrapping_sub(1));
                                let s = r32(ent(0x1f, off));
                                let t = r32(s.wrapping_add(0xad));
                                w32(s.wrapping_add(0xad), t.wrapping_add(DAT_009a8da0 as u32));
                            }

                            let a = r32(r32(ent(0x1f, off)).wrapping_add(0xa9));
                            if a != 0 {
                                cpy(a, p10, 0x118 * 4 + 2);
                                p10 = p6.wrapping_add(0x537);
                                let mut k: u32 = 0;
                                loop {
                                    let a2 = r32(r32(ent(0x1f, off)).wrapping_add(0xa9));
                                    let pp = a2.wrapping_add(0x41e).wrapping_add(k);
                                    let v = r32(pp);
                                    if v != 0 {
                                        w32(
                                            pp,
                                            v.wrapping_sub(1).wrapping_add(DAT_009a1fe8 as u32),
                                        );
                                    }
                                    k = k.wrapping_add(4);
                                    if !(k < 0x40) {
                                        break;
                                    }
                                }
                                let a2 = r32(r32(ent(0x1f, off)).wrapping_add(0xa9));
                                let pp = a2.wrapping_add(0x45e);
                                let v = r32(pp);
                                if v != 0 {
                                    w32(pp, v.wrapping_sub(1).wrapping_add(DAT_009a1fe8 as u32));
                                }
                            }
                            let b = r32(r32(ent(0x1f, off)).wrapping_add(0xa5));
                            p7 = p10;
                            if b != 0 {
                                p7 = p10.wrapping_add(10);
                                cpy(b, p10, 10);
                            }
                            let c = r32(r32(ent(0x1f, off)).wrapping_add(0xad));
                            if c != 0 {
                                cpy(c, p7, 0x13 * 4);
                                p7 = p7.wrapping_add(0x13 * 4);
                            }
                        }
                    }
                }
                off = off.wrapping_add(0x23);
                i = i.wrapping_add(1);
                p6 = p7;
                if !(i < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }

        let c = p7;
        DAT_009ed228 = r32(c);
        _DAT_009ed22c = r32(c.wrapping_add(4));
        DAT_009d8b70 = r32(c.wrapping_add(8));
        DAT_009d8b74 = r32(c.wrapping_add(12));
        cpy(
            (&raw mut DAT_009a2000) as u32,
            c.wrapping_add(16),
            0x28 * 4 + 2,
        );
        let n1 = (DAT_009a2000 as i32 as u32).wrapping_mul(0x126);
        cpy((&raw mut DAT_009a20c0) as u32, c.wrapping_add(0xb2), n1);
        let c2 = c
            .wrapping_add(0xb2)
            .wrapping_add((DAT_009a2000 as i32 as u32).wrapping_mul(0x126));
        let n2 = (DAT_009a2000 as i32 as u32).wrapping_mul(0x566);
        cpy((&raw mut DAT_009d8c30) as u32, c2, n2);
        let c3 = c2.wrapping_add((DAT_009a2000 as i32 as u32).wrapping_mul(0x566));
        cpy((&raw mut DAT_009d6940) as u32, c3, 0x34c * 4);
        cpy(
            (&raw mut DAT_009ec5e0) as u32,
            c3.wrapping_add(0x34c * 4),
            5 * 4 + 2,
        );
        DAT_00a08294 = r16(c3.wrapping_add(0xd46)) as i16;
        DAT_00a080f8 = r32(c3.wrapping_add(0x352 * 4));
        DAT_009d8c20 = r8(c3.wrapping_add(0x353 * 4));
        DAT_009eca28 = r32(c3.wrapping_add(0xd4d));
        DAT_009a7ba4 = r32(c3.wrapping_add(0xd51));
        DAT_009fd430 = r32(c3.wrapping_add(0xd55));
        DAT_00a0814c = r32(c3.wrapping_add(0xd59));
        DAT_009a7b50 = r32(c3.wrapping_add(0xd5d));
        DAT_009d767c = r32(c3.wrapping_add(0xd61));
        _DAT_00a07cec = r32(c3.wrapping_add(0xd65));
        DAT_009a1fe0 = r32(c3.wrapping_add(0xd69));
        DAT_009d1144 = r32(c3.wrapping_add(0xd6d));
        DAT_009fe7dc = r32(c3.wrapping_add(0xd71));
        DAT_009e9640 = r32(c3.wrapping_add(0xd75));
        DAT_00a00854 = r32(c3.wrapping_add(0xd79));
        DAT_009a7b58 = r32(c3.wrapping_add(0xd7d));
        DAT_009d1140 = r32(c3.wrapping_add(0xd81));
        DAT_009d1310 = r32(c3.wrapping_add(0xd85));
        DAT_009ec47c = r32(c3.wrapping_add(0xd89));
        DAT_009a8e24 = r32(c3.wrapping_add(0xd8d)) as i32;
        DAT_009cd274 = r32(c3.wrapping_add(0xd91)) as i32;
        DAT_00a07ce8 = r32(c3.wrapping_add(0xd95)) as i32;
        DAT_009d1308 = r32(c3.wrapping_add(0xd99)) as i32;
        cpy(
            (&raw mut DAT_009d1320) as u32,
            c3.wrapping_add(0xd9d),
            0x1540 * 4,
        );
        DAT_009a8068 = r32(c3.wrapping_add(0x629d));
        DAT_009a73a4 = r32(c3.wrapping_add(0x62a1));
        cpy(
            (&raw mut DAT_009a7a40) as u32,
            c3.wrapping_add(0x62a5),
            0x40 * 4,
        );
        DAT_009d113c = r16(c3.wrapping_add(0x63a5)) as i16;
        DAT_009a8d40 = r16(c3.wrapping_add(0x63a7)) as i16;
        DAT_00523424 = r32(c3.wrapping_add(0x63a9));
        cpy(
            (&raw mut DAT_009e9670) as u32,
            c3.wrapping_add(0x63ad),
            0xb7a * 4,
        );
    }
}
