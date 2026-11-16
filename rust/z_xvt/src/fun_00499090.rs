// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645a80: *mut u32;
    static mut DAT_00645ad0: *mut u8;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_006db7a4: i32;
    static mut DAT_00703ac0: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_007091a0: i32;
    static mut DAT_00732940: i32;
    static mut DAT_006db798: i32;
    static mut DAT_006db71c: i32;
    static mut DAT_0071f2d8: u32;
    static mut _DAT_0071f2dc: u32;
    static mut DAT_0070ac30: u32;
    static mut _DAT_0070ac34: u32;
    static mut DAT_006d49a0: [i16; 81];
    static mut DAT_006d4a60: [u8; 0];
    static mut DAT_0070acf0: [u8; 0];
    static mut DAT_007092c0: [u8; 0];
    static mut DAT_0071e6a0: [u8; 0];
    static mut DAT_00732fe4: i16;
    static mut DAT_00732e48: u32;
    static mut DAT_0070ace0: u8;
    static mut DAT_0071eae8: u32;
    static mut DAT_006da530: u32;
    static mut DAT_0072f4e0: u32;
    static mut DAT_00732e9c: u32;
    static mut DAT_006da4f0: u32;
    static mut DAT_0070a000: u32;
    static mut _DAT_00732a3c: u32;
    static mut DAT_006d4980: u32;
    static mut DAT_00703acc: u32;
    static mut DAT_0073088c: u32;
    static mut DAT_0071b704: u32;
    static mut DAT_00732904: u32;
    static mut DAT_006da4f4: u32;
    static mut DAT_00703ac8: u32;
    static mut DAT_00703c90: u32;
    static mut DAT_0071e53c: u32;
    static mut DAT_00703ca0: [u8; 0];
    static mut DAT_006da9e8: u32;
    static mut DAT_006d9d44: u32;
    static mut DAT_006da3e0: [u8; 0];
    static mut DAT_00703abc: i16;
    static mut DAT_006db6c0: i16;
    static mut DAT_00525564: u32;
    static mut DAT_0071b730: [u8; 0];
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game pointer, read as the original does.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn wr(a: i32, v: i32) {
    // SAFETY: the address is a game pointer, written as the original does.
    unsafe { (a as u32 as usize as *mut i32).write_unaligned(v) }
}

unsafe fn slot(i3: i32) -> i32 {
    // SAFETY: reads the game's own global and the field it points into.
    unsafe { rd(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i3)) }
}

unsafe fn fwd(i3: i32, off: i32, d: i32) {
    // SAFETY: game pointers dereferenced exactly as in the original.
    unsafe {
        let x = slot(i3);
        let v = rd(x.wrapping_add(off));
        if v != 0 {
            wr(x.wrapping_add(off), v.wrapping_sub(d));
            let q = slot(i3).wrapping_add(off);
            wr(q, rd(q).wrapping_add(1));
        }
    }
}

unsafe fn back(i3: i32, off: i32, d: i32) {
    // SAFETY: game pointers dereferenced exactly as in the original.
    unsafe {
        let x = slot(i3);
        let v = rd(x.wrapping_add(off));
        if v != 0 {
            wr(x.wrapping_add(off), v.wrapping_sub(1));
            let q = slot(i3).wrapping_add(off);
            wr(q, rd(q).wrapping_add(d));
        }
    }
}

unsafe fn cpd(dst: *mut u32, src: *const u32, n: u32) {
    let mut k: usize = 0;
    while (k as u32) < n {
        // SAFETY: copies dword by dword between the game's buffers.
        unsafe {
            dst.add(k).write_unaligned(src.add(k).read_unaligned());
        }
        k += 1;
    }
}

unsafe fn cpb(dst: *mut u8, src: *const u8, n: u32) {
    let mut k: usize = 0;
    while (k as u32) < n {
        // SAFETY: copies byte by byte between the game's buffers.
        unsafe {
            dst.add(k).write(src.add(k).read());
        }
        k += 1;
    }
}

unsafe fn put32(base: *mut u32, off: usize, v: u32) {
    // SAFETY: writes into the game's output buffer.
    unsafe { base.byte_add(off).cast::<u32>().write_unaligned(v) }
}

unsafe fn put16(base: *mut u32, off: usize, v: u16) {
    // SAFETY: writes into the game's output buffer.
    unsafe { base.byte_add(off).cast::<u16>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00499090() {
    // SAFETY: all globals and buffers are the game's own memory; reads,
    // writes and copies follow the original's order and sizes.
    unsafe {
        let mut i10: i32 = 0;
        let mut p8: *mut u32 = DAT_00645a80;
        let total = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32;
        if 0 < total {
            let mut i3: i32 = 0;
            let mut p7: *mut u32 = DAT_00645a80;
            loop {
                let mut enter = i10 < DAT_006db7a4;
                if !enter {
                    p8 = p7;
                    enter = DAT_00703ac0 <= i10;
                }
                if enter {
                    p8 = p7.byte_add(1);
                    let b = DAT_006d4988;
                    p7.cast::<u8>().write(
                        (b.wrapping_add(3).wrapping_add(i3) as u32 as usize as *const u8).read(),
                    );
                    let i4 = b.wrapping_add(i3);
                    if (i4.wrapping_add(3) as u32 as usize as *const i8).read() != 0 {
                        let a = DAT_006d4988.wrapping_add(0x1f).wrapping_add(i3);
                        let v = rd(i4.wrapping_add(0x1f));
                        if v != 0 {
                            wr(i4.wrapping_add(0x1f), v.wrapping_sub(DAT_007091a0));
                            wr(a, rd(a).wrapping_add(1));
                        }
                        let mut p9: *mut u32 =
                            DAT_006d4988.wrapping_add(i3) as u32 as usize as *mut u32;
                        cpd(p8, p9, 8);
                        p9 = p9.add(8);
                        p8 = p8.add(8);
                        p8.cast::<u16>()
                            .write_unaligned(p9.cast::<u16>().read_unaligned());
                        p8.byte_add(2)
                            .cast::<u8>()
                            .write(p9.byte_add(2).cast::<u8>().read());
                        p8 = p7.add(9);
                        let a = DAT_006d4988.wrapping_add(0x1f).wrapping_add(i3);
                        let v = rd(a);
                        if v != 0 {
                            wr(a, v.wrapping_sub(1));
                            wr(a, rd(a).wrapping_add(DAT_007091a0));
                        }
                        let a = DAT_006d4988.wrapping_add(0x1f).wrapping_add(i3);
                        if rd(a) != 0 {
                            fwd(i3, 0xa7, DAT_00732940);
                            fwd(i3, 0xa3, DAT_006db798);
                            fwd(i3, 0xab, DAT_006db71c);
                            p9 = slot(i3) as u32 as usize as *mut u32;
                            cpd(p8, p9, 0x2b);
                            p9 = p9.add(0x2b);
                            p8 = p8.add(0x2b);
                            p8.cast::<u16>()
                                .write_unaligned(p9.cast::<u16>().read_unaligned());
                            p8.byte_add(2)
                                .cast::<u8>()
                                .write(p9.byte_add(2).cast::<u8>().read());
                            back(i3, 0xa7, DAT_00732940);
                            back(i3, 0xa3, DAT_006db798);
                            back(i3, 0xab, DAT_006db71c);
                            p9 = p7.byte_add(0xd3);
                            let i11 = slot(i3);
                            let i12 = rd(i11.wrapping_add(0xa7));
                            if i12 != 0 {
                                let mut i4: i32 = 0;
                                loop {
                                    let i11 = slot(i3);
                                    let i12 = rd(i11.wrapping_add(0xa7));
                                    let q = i12.wrapping_add(0x3f2).wrapping_add(i4);
                                    if rd(q) != 0 {
                                        wr(q, rd(q).wrapping_sub(DAT_006d4988));
                                        wr(q, rd(q).wrapping_add(1));
                                    }
                                    i4 = i4.wrapping_add(4);
                                    if !(i4 < 0x40) {
                                        break;
                                    }
                                }
                                let i11 = slot(i3);
                                let i4 = rd(i11.wrapping_add(0xa7));
                                let v = rd(i4.wrapping_add(0x432));
                                if v != 0 {
                                    wr(i4.wrapping_add(0x432), v.wrapping_sub(DAT_006d4988));
                                    let i11 = slot(i3);
                                    let i12 = rd(i11.wrapping_add(0xa7));
                                    let q = i12.wrapping_add(0x432);
                                    wr(q, rd(q).wrapping_add(1));
                                }
                                p9 = p7.byte_add(0x509);
                                let i11 = slot(i3);
                                p8 = rd(i11.wrapping_add(0xa7)) as u32 as usize as *mut u32;
                                p7 = p7.byte_add(0xd3);
                                cpd(p7, p8, 0x10d);
                                p8 = p8.add(0x10d);
                                p7 = p7.add(0x10d);
                                p7.cast::<u16>()
                                    .write_unaligned(p8.cast::<u16>().read_unaligned());
                                let mut i4: i32 = 0;
                                loop {
                                    let i11 = slot(i3);
                                    let i12 = rd(i11.wrapping_add(0xa7));
                                    let q = i12.wrapping_add(0x3f2).wrapping_add(i4);
                                    if rd(q) != 0 {
                                        wr(q, rd(q).wrapping_add(0xffff_ffffu32 as i32));
                                        wr(q, rd(q).wrapping_add(DAT_006d4988));
                                    }
                                    i4 = i4.wrapping_add(4);
                                    if !(i4 < 0x40) {
                                        break;
                                    }
                                }
                                let i11 = slot(i3);
                                let i4 = rd(i11.wrapping_add(0xa7));
                                let v = rd(i4.wrapping_add(0x432));
                                if v != 0 {
                                    wr(i4.wrapping_add(0x432), v.wrapping_sub(1));
                                    let i11 = slot(i3);
                                    let i12 = rd(i11.wrapping_add(0xa7));
                                    let q = i12.wrapping_add(0x432);
                                    wr(q, rd(q).wrapping_add(DAT_006d4988));
                                }
                            }
                            let i11 = slot(i3);
                            let mut s7 = rd(i11.wrapping_add(0xa3)) as u32 as usize as *mut u32;
                            p8 = p9;
                            if !s7.is_null() {
                                p9.write_unaligned(s7.read_unaligned());
                                p8 = p9.add(2);
                                p9.add(1).write_unaligned(s7.add(1).read_unaligned());
                            }
                            let i11 = slot(i3);
                            s7 = rd(i11.wrapping_add(0xab)) as u32 as usize as *mut u32;
                            if !s7.is_null() {
                                cpd(p8, s7, 0x13);
                                p8 = p8.add(0x13);
                            }
                        }
                    }
                }
                i3 = i3.wrapping_add(0x23);
                i10 += 1;
                p7 = p8;
                let t = (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32;
                if !(i10 < t) {
                    break;
                }
            }
        }
        put32(p8, 0, DAT_0071f2d8);
        put32(p8, 4, _DAT_0071f2dc);
        put32(p8, 8, DAT_0070ac30);
        put32(p8, 12, _DAT_0070ac34);
        let mut p7: *const u32 = (&raw mut DAT_006d49a0) as *const u32;
        let mut p9: *mut u32 = p8.add(4);
        cpd(p9, p7, 0x28);
        p7 = p7.add(0x28);
        p9 = p9.add(0x28);
        p9.cast::<u16>()
            .write_unaligned(p7.cast::<u16>().read_unaligned());
        let n0 = DAT_006d49a0[0] as i32;
        let src = (&raw mut DAT_006d4a60) as *const u8;
        let dst = p8.byte_add(0xb2).cast::<u8>();
        let sz = (n0 as u32).wrapping_mul(0x126);
        cpd(dst.cast::<u32>(), src.cast::<u32>(), sz >> 2);
        let adv = ((sz >> 2) as usize) * 4;
        cpb(dst.add(adv), src.add(adv), sz & 3);
        p8 = p8.byte_add(0xb2usize.wrapping_add(sz as usize));
        let sz2 = (DAT_006d49a0[0] as i32 as u32).wrapping_mul(0x566);
        let src = (&raw mut DAT_0070acf0) as *const u8;
        let dst = p8.cast::<u8>();
        cpd(dst.cast::<u32>(), src.cast::<u32>(), sz2 >> 2);
        let adv = ((sz2 >> 2) as usize) * 4;
        cpb(dst.add(adv), src.add(adv), sz2 & 3);
        p8 = p8.byte_add(sz2 as usize);
        cpd(p8, (&raw mut DAT_007092c0) as *const u32, 0x34c);
        p9 = p8.add(0x34c);
        p7 = (&raw mut DAT_0071e6a0) as *const u32;
        cpd(p9, p7, 5);
        p7 = p7.add(5);
        p9 = p9.add(5);
        p9.cast::<u16>()
            .write_unaligned(p7.cast::<u16>().read_unaligned());
        put16(p8, 0xd46, DAT_00732fe4 as u16);
        put32(p8, 0x352 * 4, DAT_00732e48);
        p8.add(0x353).cast::<u8>().write(DAT_0070ace0);
        put32(p8, 0xd4d, DAT_0071eae8);
        put32(p8, 0xd51, DAT_006da530);
        put32(p8, 0xd55, DAT_0072f4e0);
        put32(p8, 0xd59, DAT_00732e9c);
        put32(p8, 0xd5d, DAT_006da4f0);
        put32(p8, 0xd61, DAT_0070a000);
        put32(p8, 0xd65, _DAT_00732a3c);
        put32(p8, 0xd69, DAT_006d4980);
        put32(p8, 0xd6d, DAT_00703acc);
        put32(p8, 0xd71, DAT_0073088c);
        put32(p8, 0xd75, DAT_0071b704);
        put32(p8, 0xd79, DAT_00732904);
        put32(p8, 0xd7d, DAT_006da4f4);
        put32(p8, 0xd81, DAT_00703ac8);
        put32(p8, 0xd85, DAT_00703c90);
        put32(p8, 0xd89, DAT_0071e53c);
        put32(p8, 0xd8d, DAT_006db7a4 as u32);
        put32(p8, 0xd91, DAT_00703ac0 as u32);
        put32(p8, 0xd95, DAT_00732a38 as u32);
        put32(p8, 0xd99, DAT_00703c88 as u32);
        cpd(
            p8.byte_add(0xd9d),
            (&raw mut DAT_00703ca0) as *const u32,
            0x1540,
        );
        put32(p8, 0x629d, DAT_006da9e8);
        put32(p8, 0x62a1, DAT_006d9d44);
        cpd(
            p8.byte_add(0x62a5),
            (&raw mut DAT_006da3e0) as *const u32,
            0x40,
        );
        put16(p8, 0x63a5, DAT_00703abc as u16);
        put16(p8, 0x63a7, DAT_006db6c0 as u16);
        put32(p8, 0x63a9, DAT_00525564);
        cpd(
            p8.byte_add(0x63ad),
            (&raw mut DAT_0071b730) as *const u32,
            0xb7a,
        );
        let off = 0x9195i32.wrapping_sub(DAT_00645a80 as usize as i32);
        DAT_00645ad0 = (p8 as usize as i32).wrapping_add(off) as u32 as usize as *mut u8;
    }
}
