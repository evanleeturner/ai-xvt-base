// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_00655178: i16;
    static mut _DAT_0071f2de: i16;
    static mut DAT_006d499c: i32;
    static mut DAT_00655abc: [i16; 1];
    static mut DAT_00655abe: [i16; 1];
    static mut DAT_00655ab0: [i16; 1];
    static mut DAT_00655ab2: [i16; 1];
    static mut DAT_00655184: u16;
    static mut DAT_00655a8c: i16;
    static mut DAT_00655a88: i16;
    static mut DAT_00655150: i32;
    static mut DAT_00655814: i32;
    static mut DAT_00655a8e: u16;
    static mut DAT_00655812: u16;
    static mut DAT_00656c82: i16;
    static mut DAT_00656c84: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413ca0(c: i32, d: i32) -> i64;
    fn FUN_00483410(a: i16, b: i16, p: i32);
    fn FUN_00483540(a: i16, b: i32, p: i32);
    fn FUN_0042a1a0(a: u32, b: u32);
    fn FUN_004785e0(a: i32, b: i32, c: i32);
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn rd16(addr: i32) -> i16 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (addr as u32 as usize as *const i16).read_unaligned() }
}

unsafe fn rd8(addr: i32) -> u8 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (addr as u32 as usize as *const u8).read() }
}

unsafe fn wr16(addr: i32, v: i16) {
    // SAFETY: the address is a game-memory location the original writes.
    unsafe { (addr as u32 as usize as *mut i16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9bd0(param_1: u16) {
    // SAFETY: all globals and callees are the game's own; every address
    // dereferenced is one the original dereferences.
    unsafe {
        let mut iv3 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let iv5 = DAT_006d4988.wrapping_add(iv3);
        let tbl = (&raw mut DAT_0071b730) as *mut u8;
        let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
        let rec = tbl.byte_add(idx as usize) as usize as i32;
        let mut iv7 = (rd32(rec) as u32).wrapping_mul(0x23) as i32;
        let iv4 = DAT_006d4988.wrapping_add(iv7);

        let iv9 = rd32(iv5.wrapping_add(4)).wrapping_sub(rd32(iv4.wrapping_add(4)));
        let iv8 = rd32(iv5.wrapping_add(8)).wrapping_sub(rd32(iv4.wrapping_add(8)));
        let iv5b = rd32(iv5.wrapping_add(0xc)).wrapping_sub(rd32(iv4.wrapping_add(0xc)));

        let pc = rd32(iv4.wrapping_add(0x1f));
        if rd8(pc.wrapping_add(0x90)) != 0 {
            let a = rd16(iv4.wrapping_add(0x10));
            let b = rd16(iv4.wrapping_add(0x12));
            FUN_00483410(b, a, iv4);
            let c = rd16(DAT_006d4988.wrapping_add(iv7).wrapping_add(0x14));
            FUN_00483540(c, 0, DAT_006d4988.wrapping_add(iv7));
        }

        let off = |o: i32| -> i32 {
            let p = rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(iv7));
            rd16(p.wrapping_add(o)) as i32
        };

        let mut u11 = FUN_00413ca0(iv9, off(0x91));
        let mut u12 = FUN_00413ca0(iv8, off(0x93));
        let mut u13 = FUN_00413ca0(iv5b, off(0x95));

        let mut iv6 = (u11 as i32)
            .wrapping_add(u12 as i32)
            .wrapping_add(u13 as i32);

        u11 = FUN_00413ca0(iv9, off(0x97));
        u12 = FUN_00413ca0(iv8, off(0x99));
        u13 = FUN_00413ca0(iv5b, off(0x9b));
        let u14 = FUN_00413ca0(iv9, off(0x9d));
        let u15 = FUN_00413ca0(iv8, off(0x9f));
        let u16 = FUN_00413ca0(iv5b, off(0xa1));

        let neg = iv6 < 0;
        if neg {
            iv6 = iv6.wrapping_neg();
        }

        iv3 = DAT_006d4988.wrapping_add(iv3);
        let pc = rd32(iv3.wrapping_add(0x1f));
        if pc == 0 {
            DAT_00655178 = 0x2f;
        } else if rd8(iv3.wrapping_add(2)) == 9 {
            DAT_00655178 = 0x2f;
        } else if rd8(pc) == 1 {
            let s = _DAT_0071f2de as i32;
            let t = (s as u32).wrapping_add(((s >> 0x1f) as u32) & 3) as i32;
            if (t >> 2) & 1 == 0 {
                DAT_00655178 = 0x37;
            } else {
                DAT_00655178 = 0x3b;
            }
        } else {
            match rd8(pc.wrapping_add(0x86)) {
                0 => DAT_00655178 = 0x3f,
                1 | 4 => DAT_00655178 = 0x37,
                2 => DAT_00655178 = 0x33,
                3 => DAT_00655178 = 0x3b,
                5 => DAT_00655178 = 0xd3,
                _ => {}
            }
        }

        let r = rd32(rec);
        FUN_0042a1a0(r as u32, param_1 as u32);

        if DAT_006d499c < 0x1dd37 {
            if 0xee9b < DAT_006d499c {
                if DAT_00655178 == 0x2f {
                    DAT_00655178 = 0x2e;
                } else if DAT_00655178 == 0xd3 {
                    DAT_00655178 = 0xd4;
                } else {
                    DAT_00655178 = DAT_00655178.wrapping_sub(1);
                }
            }
        } else if DAT_00655178 == 0x2f {
            DAT_00655178 = 0x2d;
        } else if DAT_00655178 == 0xd3 {
            DAT_00655178 = 0xd5;
        } else {
            DAT_00655178 = DAT_00655178.wrapping_sub(2);
        }

        FUN_004785e0(
            (u11 as i32)
                .wrapping_add(u12 as i32)
                .wrapping_add(u13 as i32),
            (u14 as i32)
                .wrapping_add(u15 as i32)
                .wrapping_add(u16 as i32)
                .wrapping_neg(),
            iv6,
        );

        iv7 = 0;
        let _ = iv7;
        let k = (DAT_00655184 as u32).wrapping_mul(6) as usize;
        if neg {
            DAT_00655a8c = DAT_00655a8c.wrapping_add(
                ((&raw mut DAT_00655abc) as *mut i16)
                    .add(k)
                    .read_unaligned(),
            );
            DAT_00655a88 = DAT_00655a88.wrapping_add(
                ((&raw mut DAT_00655abe) as *mut i16)
                    .add(k)
                    .read_unaligned(),
            );
            if DAT_00655a88 < 0 {
                DAT_00655a88 = 0;
            }
            let b = DAT_00655150.wrapping_add((DAT_00655a8e as u32).wrapping_mul(6) as i32);
            wr16(b, DAT_00655a8c);
            wr16(b.wrapping_add(2), DAT_00655a88);
            wr16(b.wrapping_add(4), DAT_00655178);
            let u2 = DAT_00655a8e;
            DAT_00655a8e = DAT_00655a8e.wrapping_add(1);
            if DAT_00655a8e == 0x30 {
                DAT_00655a8e = u2;
            }
        } else {
            DAT_00655a8c = DAT_00655a8c.wrapping_add(
                ((&raw mut DAT_00655ab0) as *mut i16)
                    .add(k)
                    .read_unaligned(),
            );
            DAT_00655a88 = DAT_00655a88.wrapping_add(
                ((&raw mut DAT_00655ab2) as *mut i16)
                    .add(k)
                    .read_unaligned(),
            );
            if DAT_00655a88 < 0 {
                DAT_00655a88 = 0;
            }
            let b = DAT_00655814.wrapping_add((DAT_00655812 as u32).wrapping_mul(6) as i32);
            wr16(b, DAT_00655a8c);
            wr16(b.wrapping_add(2), DAT_00655a88);
            wr16(b.wrapping_add(4), DAT_00655178);
            let u2 = DAT_00655812;
            DAT_00655812 = DAT_00655812.wrapping_add(1);
            if DAT_00655812 == 0x30 {
                DAT_00655812 = u2;
            }
        }

        let v = rd16(rec.wrapping_add(0x1a)) as u16;
        if v == param_1 {
            DAT_00656c82 = DAT_00655a8c;
            DAT_00656c84 = DAT_00655a88;
        }
    }
}
