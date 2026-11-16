// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00550b88: *mut u8;
    static mut DAT_00550bd8: *mut u8;
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

unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: caller guarantees p points at four readable bytes.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: i32) {
    // SAFETY: caller guarantees p points at four writable bytes.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

unsafe fn put<T>(p: *mut u8, off: usize, v: T) {
    // SAFETY: caller guarantees the destination at p + off is writable.
    unsafe { p.wrapping_add(off).cast::<T>().write_unaligned(v) }
}

unsafe fn cpd(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        // SAFETY: caller guarantees both ranges cover n dwords.
        unsafe {
            let v = src.wrapping_add(i * 4).cast::<u32>().read_unaligned();
            dst.wrapping_add(i * 4).cast::<u32>().write_unaligned(v);
        }
        i += 1;
    }
}

unsafe fn cpb(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        // SAFETY: caller guarantees both ranges cover n bytes.
        unsafe {
            *dst.wrapping_add(i) = *src.wrapping_add(i);
        }
        i += 1;
    }
}

unsafe fn fwd(p: *mut u8, d: i32) {
    // SAFETY: caller guarantees p points at a game dword.
    unsafe {
        let mut v = rd(p);
        if v != 0 {
            v = v.wrapping_sub(d);
            wr(p, v);
            v = v.wrapping_add(1);
            wr(p, v);
        }
    }
}

unsafe fn back(p: *mut u8, d: i32) {
    // SAFETY: caller guarantees p points at a game dword.
    unsafe {
        let mut v = rd(p);
        if v != 0 {
            v = v.wrapping_add(-1);
            wr(p, v);
            v = v.wrapping_add(d);
            wr(p, v);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004167f0() {
    // SAFETY: all globals and the buffers they point to belong to the game;
    // reads and writes happen at the offsets the original uses.
    unsafe {
        let mut i_var10: i32 = 0;
        let mut p8: *mut u8 = DAT_00550b88;
        if 0 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            let mut i_var3: i32 = 0;
            let mut p7: *mut u8 = DAT_00550b88;
            loop {
                let cond = if i_var10 < DAT_009a8e24 {
                    true
                } else {
                    p8 = p7;
                    DAT_009cd274 <= i_var10
                };
                if cond {
                    let p12 = (DAT_009a1fe8 as u32).wrapping_add(i_var3 as u32) as usize as *mut u8;
                    p8 = p7.wrapping_add(1);
                    *p7 = *p12.wrapping_add(3);
                    if *p12.wrapping_add(3) != 0 {
                        fwd(p12.wrapping_add(0x1f), DAT_009d6820);
                        cpd(p8, p12, 8);
                        put::<u16>(p8, 32, p12.wrapping_add(32).cast::<u16>().read_unaligned());
                        *p8.wrapping_add(34) = *p12.wrapping_add(34);
                        p8 = p7.wrapping_add(0x24);
                        back(p12.wrapping_add(0x1f), DAT_009d6820);
                        let p13 = rd(p12.wrapping_add(0x1f)) as u32 as usize as *mut u8;
                        if !p13.is_null() {
                            fwd(p13.wrapping_add(0xa9), DAT_00a07bd0);
                            fwd(p13.wrapping_add(0xa5), DAT_009a8e18);
                            fwd(p13.wrapping_add(0xad), DAT_009a8da0);
                            cpd(p8, p13, 0x2c);
                            *p8.wrapping_add(0xb0) = *p13.wrapping_add(0xb0);
                            back(p13.wrapping_add(0xa9), DAT_00a07bd0);
                            back(p13.wrapping_add(0xa5), DAT_009a8e18);
                            back(p13.wrapping_add(0xad), DAT_009a8da0);
                            let mut p9 = p7.wrapping_add(0xd5);
                            let p14 = rd(p13.wrapping_add(0xa9)) as u32 as usize as *mut u8;
                            if !p14.is_null() {
                                let mut i4: usize = 0;
                                loop {
                                    fwd(p14.wrapping_add(0x41e + i4), DAT_009a1fe8);
                                    i4 += 4;
                                    if i4 >= 0x40 {
                                        break;
                                    }
                                }
                                fwd(p14.wrapping_add(0x45e), DAT_009a1fe8);
                                p9 = p7.wrapping_add(0x537);
                                let d = p7.wrapping_add(0xd5);
                                cpd(d, p14, 0x118);
                                put::<u16>(
                                    d,
                                    0x460,
                                    p14.wrapping_add(0x460).cast::<u16>().read_unaligned(),
                                );
                                let mut i4: usize = 0;
                                loop {
                                    back(p14.wrapping_add(0x41e + i4), DAT_009a1fe8);
                                    i4 += 4;
                                    if i4 >= 0x40 {
                                        break;
                                    }
                                }
                                back(p14.wrapping_add(0x45e), DAT_009a1fe8);
                            }
                            let s7 = rd(p13.wrapping_add(0xa5)) as u32 as usize as *mut u8;
                            p8 = p9;
                            if !s7.is_null() {
                                cpd(p9, s7, 1);
                                p8 = p9.wrapping_add(10);
                                cpd(p9.wrapping_add(4), s7.wrapping_add(4), 1);
                                put::<u16>(
                                    p9,
                                    8,
                                    s7.wrapping_add(8).cast::<u16>().read_unaligned(),
                                );
                            }
                            let s7 = rd(p13.wrapping_add(0xad)) as u32 as usize as *mut u8;
                            if !s7.is_null() {
                                cpd(p8, s7, 0x13);
                                p8 = p8.wrapping_add(0x4c);
                            }
                        }
                    }
                }
                i_var3 = i_var3.wrapping_add(0x23);
                i_var10 = i_var10.wrapping_add(1);
                p7 = p8;
                if !(i_var10 < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }

        put::<u32>(p8, 0, DAT_009ed228);
        put::<u32>(p8, 4, _DAT_009ed22c);
        put::<u32>(p8, 8, DAT_009d8b70);
        put::<u32>(p8, 12, DAT_009d8b74);
        let src = (&raw mut DAT_009a2000).cast::<u8>();
        let p9 = p8.wrapping_add(16);
        cpd(p9, src, 0x28);
        put::<u16>(
            p9,
            0xa0,
            src.wrapping_add(0xa0).cast::<u16>().read_unaligned(),
        );
        let n = (DAT_009a2000 as i32 as u32).wrapping_mul(0x126);
        cpb(
            p8.wrapping_add(0xb2),
            (&raw mut DAT_009a20c0).cast::<u8>(),
            n as usize,
        );
        p8 = p8
            .wrapping_add(0xb2)
            .wrapping_add((DAT_009a2000 as i32 as u32).wrapping_mul(0x126) as usize);
        let n = (DAT_009a2000 as i32 as u32).wrapping_mul(0x566);
        cpb(p8, (&raw mut DAT_009d8c30).cast::<u8>(), n as usize);
        p8 = p8.wrapping_add((DAT_009a2000 as i32 as u32).wrapping_mul(0x566) as usize);
        cpd(p8, (&raw mut DAT_009d6940).cast::<u8>(), 0x34c);
        let src = (&raw mut DAT_009ec5e0).cast::<u8>();
        let p9 = p8.wrapping_add(0xd30);
        cpd(p9, src, 5);
        put::<u16>(
            p9,
            0x14,
            src.wrapping_add(0x14).cast::<u16>().read_unaligned(),
        );
        put::<i16>(p8, 0xd46, DAT_00a08294);
        put::<u32>(p8, 0xd48, DAT_00a080f8);
        *p8.wrapping_add(0xd4c) = DAT_009d8c20;
        put::<u32>(p8, 0xd4d, DAT_009eca28);
        put::<u32>(p8, 0xd51, DAT_009a7ba4);
        put::<u32>(p8, 0xd55, DAT_009fd430);
        put::<u32>(p8, 0xd59, DAT_00a0814c);
        put::<u32>(p8, 0xd5d, DAT_009a7b50);
        put::<u32>(p8, 0xd61, DAT_009d767c);
        put::<u32>(p8, 0xd65, _DAT_00a07cec);
        put::<u32>(p8, 0xd69, DAT_009a1fe0);
        put::<u32>(p8, 0xd6d, DAT_009d1144);
        put::<u32>(p8, 0xd71, DAT_009fe7dc);
        put::<u32>(p8, 0xd75, DAT_009e9640);
        put::<u32>(p8, 0xd79, DAT_00a00854);
        put::<u32>(p8, 0xd7d, DAT_009a7b58);
        put::<u32>(p8, 0xd81, DAT_009d1140);
        put::<u32>(p8, 0xd85, DAT_009d1310);
        put::<u32>(p8, 0xd89, DAT_009ec47c);
        put::<i32>(p8, 0xd8d, DAT_009a8e24);
        put::<i32>(p8, 0xd91, DAT_009cd274);
        put::<i32>(p8, 0xd95, DAT_00a07ce8);
        put::<i32>(p8, 0xd99, DAT_009d1308);
        cpd(
            p8.wrapping_add(0xd9d),
            (&raw mut DAT_009d1320).cast::<u8>(),
            0x1540,
        );
        put::<u32>(p8, 0x629d, DAT_009a8068);
        put::<u32>(p8, 0x62a1, DAT_009a73a4);
        cpd(
            p8.wrapping_add(0x62a5),
            (&raw mut DAT_009a7a40).cast::<u8>(),
            0x40,
        );
        put::<i16>(p8, 0x63a5, DAT_009d113c);
        put::<i16>(p8, 0x63a7, DAT_009a8d40);
        put::<u32>(p8, 0x63a9, DAT_00523424);
        cpd(
            p8.wrapping_add(0x63ad),
            (&raw mut DAT_009e9670).cast::<u8>(),
            0xb7a,
        );
        DAT_00550bd8 = ((p8 as u32).wrapping_add(0x9195u32.wrapping_sub(DAT_00550b88 as u32)))
            as usize as *mut u8;
    }
}
