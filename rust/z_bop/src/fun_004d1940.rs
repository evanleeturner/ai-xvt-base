// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebc5: u8;
    static mut DAT_00acebd1: u8;
    static mut DAT_00acebd5: u8;
    static mut DAT_00acebd9: u8;
    static mut DAT_00acebdd: u8;
    static mut DAT_00acebe1: u8;
    static mut DAT_00acebe5: u8;
    static mut DAT_00acebe9: u8;
    static mut DAT_00acebed: u8;
    static mut DAT_00acebf1: u8;
    static mut DAT_00acebf5: u8;
    static mut DAT_00acf125: u8;
    static mut DAT_00b53579: u8;
    static mut DAT_00b53575: u8;
    static mut DAT_00b53571: u8;
    static mut DAT_00acf571: u8;
    static mut DAT_00b69a89: u8;
    static mut DAT_00b63f91: u8;
    static mut DAT_00acf14d: u8;
    static mut DAT_00acf151: u8;
    static mut DAT_00acf351: u8;
    static mut DAT_00acf355: u8;
    static mut DAT_00acf359: u8;
    static mut DAT_00acf35d: u8;
    static mut DAT_00acf55d: u8;
    static mut DAT_00acf561: u8;
    static mut DAT_00b5357d: u8;
    static mut DAT_00b63d7d: u8;
}

#[inline(always)]
unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: caller passes a valid game address.
    unsafe { p.cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut u32, v: u32) {
    // SAFETY: caller passes a valid destination.
    unsafe { p.write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1940(
    param_1: *mut u32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
    param_6: *mut u32,
    param_7: i32,
    param_8: *mut i32,
    param_9: *mut i32,
    param_10: *mut u32,
    param_11: *mut u32,
    param_12: *mut i32,
    param_13: *mut u32,
    param_14: *mut u32,
    param_15: *mut u32,
    param_16: *mut u32,
    param_17: *mut u32,
    param_18: *mut u32,
    param_19: *mut u32,
    param_20: *mut u32,
    param_21: *mut u32,
    param_22: *mut u32,
) -> u32 {
    // SAFETY: the globals are the game's own memory; the pointer parameters
    // point at caller buffers of the sizes the original copies.
    unsafe {
        let mut param_6 = param_6;
        let mut param_11 = param_11;
        let mut param_14 = param_14;
        let mut param_18 = param_18;
        let mut param_21 = param_21;

        wr(param_1, rd(&raw mut DAT_00acebc5));
        wr(param_2, rd(&raw mut DAT_00acebd1));
        wr(param_2.add(1), rd(&raw mut DAT_00acebd5));
        wr(param_2.add(2), rd(&raw mut DAT_00acebd9));
        wr(param_2.add(3), rd(&raw mut DAT_00acebdd));
        wr(param_3, rd(&raw mut DAT_00acebe1));
        wr(param_3.add(1), rd(&raw mut DAT_00acebe5));
        wr(param_3.add(2), rd(&raw mut DAT_00acebe9));
        wr(param_3.add(3), rd(&raw mut DAT_00acebed));
        wr(param_4, rd(&raw mut DAT_00acebf5));
        wr(param_5, rd(&raw mut DAT_00acebf1));
        let mut src = &raw mut DAT_00acf125 as *const u8;
        for _ in 0..10 {
            wr(param_6, rd(src));
            src = src.add(4);
            param_6 = param_6.add(1);
        }
        param_9.write_unaligned(rd(&raw mut DAT_00b53579) as i32);
        param_8.write_unaligned(rd(&raw mut DAT_00b53575) as i32);
        wr(param_10, rd(&raw mut DAT_00b53571));
        let mut i3: i32 = 0;
        let mut i1: i32 = rd(&raw mut DAT_00b53575) as i32;
        let mut i4: i32 = rd(&raw mut DAT_00b53579) as i32;
        if 0 < i4 {
            loop {
                let mut i5 = (i1 as u32).wrapping_add(1) as i32;
                let off = (i1 as u32).wrapping_mul(0x210);
                let mut s = (&raw mut DAT_00acf571 as *const u8).wrapping_add(off as usize);
                let mut d = off.wrapping_add(param_7 as u32) as usize as *mut u32;
                for _ in 0..0x84 {
                    wr(d, rd(s));
                    s = s.wrapping_add(4);
                    d = d.wrapping_add(1);
                }
                if 0x3ff < i5 {
                    i5 = 0;
                }
                i3 = i3.wrapping_add(1);
                i1 = i5;
                i4 = rd(&raw mut DAT_00b53579) as i32;
                if !(i3 < i4) {
                    break;
                }
            }
        }
        i1 = 0;
        param_12.write_unaligned(rd(&raw mut DAT_00b69a89) as i32);
        i4 = rd(&raw mut DAT_00b69a89) as i32;
        if 0 < i4 {
            let mut p4 = &raw mut DAT_00b63f91 as *const u8;
            loop {
                let mut s = p4;
                let mut d = param_11;
                for _ in 0..0x8e {
                    wr(d, rd(s));
                    s = s.add(4);
                    d = d.add(1);
                }
                param_11 = param_11.add(0x8e);
                i1 = i1.wrapping_add(1);
                p4 = p4.add(0x238);
                i4 = rd(&raw mut DAT_00b69a89) as i32;
                if !(i1 < i4) {
                    break;
                }
            }
        }
        wr(param_13, rd(&raw mut DAT_00acf14d));
        let mut src = &raw mut DAT_00acf151 as *const u8;
        for _ in 0..0x80 {
            wr(param_14, rd(src));
            src = src.add(4);
            param_14 = param_14.add(1);
        }
        wr(param_15, rd(&raw mut DAT_00acf351));
        wr(param_16, rd(&raw mut DAT_00acf355));
        wr(param_17, rd(&raw mut DAT_00acf359));
        let mut src = &raw mut DAT_00acf35d as *const u8;
        for _ in 0..0x80 {
            wr(param_18, rd(src));
            src = src.add(4);
            param_18 = param_18.add(1);
        }
        wr(param_19, rd(&raw mut DAT_00acf55d));
        wr(param_20, rd(&raw mut DAT_00acf561));
        let mut z = param_21;
        for _ in 0..0x4200 {
            wr(z, 0);
            z = z.add(1);
        }
        let mut p4 = &raw mut DAT_00b5357d as *const u8;
        let end = &raw mut DAT_00b63d7d as usize;
        loop {
            let p7 = p4.wrapping_add(0x210);
            let mut d = param_21;
            for _ in 0..0x84 {
                wr(d, rd(p4));
                p4 = p4.wrapping_add(4);
                d = d.add(1);
            }
            param_21 = param_21.add(0x84);
            p4 = p7;
            if !((p7 as usize) < end) {
                break;
            }
        }
        wr(param_22, rd(&raw mut DAT_00b63d7d));
        1
    }
}
