// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009d6990: [u32; 0];
    static mut DAT_009d69f4: [u16; 0];
    static mut DAT_009d9192: [u8; 0];
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009e96dd: [u8; 0];
    static mut DAT_009e9901: [u8; 0];
    static mut DAT_009e9903: [u8; 0];
    static mut DAT_009e9905: [u8; 0];
    static mut DAT_009e9907: [u8; 0];
    static mut DAT_009e9678: [u8; 0];
    static mut DAT_009ec460: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435ad0(param_1: u32) -> i32;
    fn FUN_00434500(param_1: u32, param_2: u32);
    fn FUN_00425c60(param_1: u32, param_2: u32) -> u32;
}

unsafe fn rd32(p: *const u8) -> u32 {
    // SAFETY: caller guarantees p points at readable game memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn rd16(p: *const u8) -> u16 {
    // SAFETY: caller guarantees p points at readable game memory.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn inc16(p: *mut u8) {
    // SAFETY: caller guarantees p points at writable game memory.
    unsafe {
        let v = p.cast::<u16>().read_unaligned().wrapping_add(1);
        p.cast::<u16>().write_unaligned(v);
    }
}

unsafe fn sum_dwords(mut p: *const u8, n: i32) -> u32 {
    let mut s: u32 = 0;
    let mut c = n;
    // SAFETY: p walks n dwords of the game's record.
    unsafe {
        loop {
            s = s.wrapping_add(rd32(p));
            p = p.add(4);
            c -= 1;
            if c == 0 {
                break;
            }
        }
    }
    s
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435250(param_1: u32, param_2: i32) -> u32 {
    let mut local_4: u32 = 0xffffffff;
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the offsets the original uses.
    unsafe {
        let i_var5 = FUN_00435ad0(param_1);
        let mut u_var6: u32 = (i_var5 / 2) as u32;
        let rec = param_1.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32) as *mut u8;
        let b16 = rec.add(0x16).read() as u32;
        let idx = (&raw mut DAT_009d8c88)
            .cast::<u8>()
            .add(b16.wrapping_mul(0x566) as usize)
            .read() as usize;
        let p1 = (&raw mut DAT_009d6990).cast::<u32>().add(idx);
        p1.write(p1.read().wrapping_sub(u_var6));
        let p2 = (&raw mut DAT_009d69f4).cast::<u16>().add(idx);
        p2.write(p2.read().wrapping_add(1));
        let mut i_var5 = rd32(rec.add(0x1b)) as i32;
        if i_var5 == -1 {
            i_var5 = rd32(
                (&raw mut DAT_009d9192)
                    .cast::<u8>()
                    .add(b16.wrapping_mul(0x566) as usize),
            ) as i32;
            if i_var5 == -1 {
                return u_var6;
            }
            if (&raw mut DAT_009e9680)
                .cast::<i8>()
                .add((i_var5 as u32).wrapping_mul(0x5bd) as usize)
                .read()
                == 0
            {
                return u_var6;
            }
        }
        let tmp_a = rd32(rec.add(0x1f));
        let tmp_b = rd32((tmp_a as usize + 0xa9) as *const u8);
        let i_var13 = tmp_b as usize as *mut u8;
        let i_var4 = (i_var5 as u32).wrapping_mul(0x5bd) as i32;
        let p96dd = (&raw mut DAT_009e96dd)
            .cast::<u8>()
            .byte_offset(i_var4 as isize);
        p96dd
            .cast::<u32>()
            .write_unaligned(rd32(p96dd).wrapping_sub(u_var6));
        if param_2 != 0 {
            let u_var2 = rd32(i_var13.add(0xc3));
            let u_var3 = rd32(i_var13.add(0xcb));
            if u_var2 < u_var3 {
                u_var6 = u_var3.wrapping_sub(u_var2);
            } else {
                u_var6 = 0;
            }
            let u_var2 = rd32(i_var13.add(0x211));
            if 0 < (u_var2 as i32) {
                u_var6 = u_var6.wrapping_add(u_var2 & 0xffff);
            }
            let u_var2 = rd32(i_var13.add(0x215));
            if 0 < (u_var2 as i32) {
                u_var6 = u_var6.wrapping_add(u_var2 & 0xffff);
            }
            let mut u_var10 = sum_dwords(i_var13.add(0x1a7), 8);
            u_var10 = u_var10.wrapping_add(sum_dwords(i_var13.add(0x1c7), 6));
            if (u_var6 >> 1) < u_var10 {
                FUN_00434500(param_1, param_1);
            }
        }
        let u_var2 = rd32(i_var13.add(0xd7));
        let u_var3 = rd32(i_var13.add(0xd3));
        u_var6 = FUN_00425c60(u_var2, u_var3);
        if 0x7fff < (u_var6 as u16) {
            inc16(
                (&raw mut DAT_009e9901)
                    .cast::<u8>()
                    .byte_offset(i_var4 as isize),
            );
            let mut p15 = i_var13.add(0x1a7) as *const u8;
            let u_var6 = sum_dwords(p15, 8);
            let mut p7 = i_var13.add(0x1c7) as *const u8;
            let u_var10 = sum_dwords(p7, 6);
            let u_var14 = rd32(i_var13.add(0xdb));
            let u_var2 = rd32(i_var13.add(0xdf));
            let u_var3 = rd32(i_var13.add(0xe3));
            let mut u_var9 = u_var2 | u_var14 | u_var3 | u_var10 | u_var6;
            if u_var9 != 0 {
                if u_var14 < u_var3 || u_var14 < u_var2 || u_var14 < u_var6 || u_var14 < u_var10 {
                    if u_var2 < u_var3 || u_var2 < u_var14 || u_var2 < u_var6 || u_var2 < u_var10 {
                        if u_var3 < u_var2
                            || u_var3 < u_var14
                            || u_var3 < u_var6
                            || u_var3 < u_var10
                        {
                            if u_var6 < u_var2
                                || u_var6 < u_var14
                                || u_var6 < u_var3
                                || u_var6 < u_var10
                            {
                                let mut m: u32 = 0;
                                let mut c = 6;
                                loop {
                                    let t = rd32(p7);
                                    if m < t {
                                        m = t;
                                    }
                                    p7 = p7.add(4);
                                    c -= 1;
                                    if c == 0 {
                                        break;
                                    }
                                }
                            } else {
                                let mut m: u32 = 0;
                                let mut u_var10: u32 = 0xffffffff;
                                let mut cnt: u32 = 0;
                                let mut p11 = (&raw mut DAT_009e9678).cast::<u8>() as *const u8;
                                let end = (&raw mut DAT_009ec460).cast::<u8>() as usize;
                                loop {
                                    let t = rd32(p15);
                                    if m < t {
                                        u_var10 = rd16(p11) as u32;
                                        u_var9 = cnt;
                                        m = t;
                                    }
                                    p11 = p11.add(0x5bd);
                                    p15 = p15.add(4);
                                    cnt = cnt.wrapping_add(1);
                                    if (p11 as usize) >= end {
                                        break;
                                    }
                                }
                                if u_var10 != 0xffffffff {
                                    local_4 = u_var9;
                                }
                            }
                        } else {
                            inc16(
                                (&raw mut DAT_009e9907)
                                    .cast::<u8>()
                                    .byte_offset(i_var4 as isize),
                            );
                        }
                    } else {
                        inc16(
                            (&raw mut DAT_009e9905)
                                .cast::<u8>()
                                .byte_offset(i_var4 as isize),
                        );
                    }
                } else {
                    inc16(
                        (&raw mut DAT_009e9903)
                            .cast::<u8>()
                            .byte_offset(i_var4 as isize),
                    );
                }
            }
        }
    }
    local_4
}
