// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_00a08104: u32;
    static mut DAT_009d1320: [i8; 0];
    static mut DAT_00521f2e: [u8; 0];
    static mut DAT_0051a1d0: [i8; 0];
    static mut DAT_0051a1bc: [i8; 0];
    static mut DAT_0051a1a8: [i8; 0];
    static mut DAT_0051a254: [i8; 0];
    static mut DAT_0051a244: [i8; 0];
    static mut DAT_0051a230: [i8; 0];
    static mut DAT_0051a278: [i8; 0];
    static mut DAT_0051a264: [i8; 0];
    static mut DAT_0051a21c: [i8; 0];
    static mut DAT_0051a208: [i8; 0];
    static mut DAT_0051a1f4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433390(param_1: u32, param_2: u16, param_3: u16) -> u8;
    fn FUN_00432fa0(param_1: u16, param_2: u16, param_3: u16) -> u16;
    fn FUN_00403540(param_1: u32, param_2: u32);
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u8).read() }
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn eqn(p: *const i8, q: *const i8, n: u32) -> bool {
    let mut i = 0usize;
    while i < n as usize {
        // SAFETY: both pointers address the game's byte data, compared as repe cmpsb does.
        unsafe {
            if p.add(i).read() != q.add(i).read() {
                return false;
            }
        }
        i += 1;
    }
    true
}

unsafe fn name_ptr(idx: u8) -> *const i8 {
    // SAFETY: DAT_009d1320 is the game's name table; only its address is taken.
    unsafe {
        (&raw mut DAT_009d1320)
            .cast::<i8>()
            .add(idx as usize * 0x55) as *const i8
    }
}

unsafe fn board1(p: *const i8) -> bool {
    // SAFETY: the strings are the game's own data.
    unsafe {
        eqn(p, (&raw mut DAT_0051a254).cast::<i8>(), 0xf)
            || eqn(p, (&raw mut DAT_0051a244).cast::<i8>(), 0xf)
            || eqn(p, (&raw mut DAT_0051a230).cast::<i8>(), 0x13)
            || eqn(p, (&raw mut DAT_0051a278).cast::<i8>(), 0x12)
            || eqn(p, (&raw mut DAT_0051a264).cast::<i8>(), 0x12)
            || eqn(p, (&raw mut DAT_0051a21c).cast::<i8>(), 0x11)
            || eqn(p, (&raw mut DAT_0051a208).cast::<i8>(), 0x12)
            || eqn(p, (&raw mut DAT_0051a1f4).cast::<i8>(), 0x11)
    }
}

unsafe fn board2(p: *const i8) -> bool {
    // SAFETY: the strings are the game's own data.
    unsafe {
        eqn(p, (&raw mut DAT_0051a254).cast::<i8>(), 0xf)
            || eqn(p, (&raw mut DAT_0051a244).cast::<i8>(), 0xf)
            || eqn(p, (&raw mut DAT_0051a230).cast::<i8>(), 0x13)
            || eqn(p, (&raw mut DAT_0051a278).cast::<i8>(), 0x12)
            || eqn(p, (&raw mut DAT_0051a264).cast::<i8>(), 0x12)
            || eqn(p, (&raw mut DAT_0051a208).cast::<i8>(), 0x12)
            || eqn(p, (&raw mut DAT_0051a1f4).cast::<i8>(), 0x11)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004039e0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u32 {
    // SAFETY: the globals, callees and addresses are the game's own memory,
    // accessed in the same order and widths as the original.
    unsafe {
        let mut local_e: u16 = DAT_00a07cec;
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_12: u16 = 0xffff;
        let mut i_var6: u32;

        if (local_e as i32) < DAT_009a1fe0 {
            loop {
                i_var6 = (local_e as u32).wrapping_mul(0x23);
                if rd8(DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var6)) != 0 {
                    let mut u13: u16 = FUN_00433390(local_e as u32, param_1, param_2) as u16;
                    let u5: u16 = FUN_00433390(local_e as u32, param_4, param_5) as u16;
                    if param_3 == 1 {
                        u13 |= u5;
                    } else {
                        u13 &= u5;
                    }
                    if u13 != 0 {
                        let mut b3 = false;
                        let psv1: u32 = DAT_009a1fe8.wrapping_add(i_var6);
                        let i_ptr = rd32(psv1.wrapping_add(0x1f));
                        i_var6 = rd32(i_ptr.wrapping_add(0xa9));
                        let p11 = name_ptr(rd8(i_var6.wrapping_add(0x35)));
                        if eqn(p11, (&raw mut DAT_0051a1d0).cast::<i8>(), 8)
                            || eqn(p11, (&raw mut DAT_0051a1bc).cast::<i8>(), 0x11)
                            || eqn(p11, (&raw mut DAT_0051a1a8).cast::<i8>(), 0x11)
                            || rd8(psv1.wrapping_add(2)) == 5
                        {
                            b3 = true;
                        } else {
                            let it = rd32((&raw mut DAT_009a73b6) as u32);
                            let pc = name_ptr(rd8(it.wrapping_add(0xb)));
                            if eqn(pc, (&raw mut DAT_0051a278).cast::<i8>(), 0x12)
                                || eqn(pc, (&raw mut DAT_0051a264).cast::<i8>(), 0x12)
                            {
                                if rd16(i_var6.wrapping_add(0x1e5)) == 0 {
                                    b3 = true;
                                }
                            } else if rd16(i_var6.wrapping_add(0x1e5)) == 0
                                || rd8(i_var6.wrapping_add(0x5e)) == 0x13
                                || rd8(i_var6.wrapping_add(0x5e)) == 0x19
                            {
                                b3 = true;
                            }
                        }
                        if b3 {
                            let mut local_10: i16 = 0;
                            let mut v13: u16 = DAT_00a07cec;
                            while (v13 as i32) < DAT_009a1fe0 {
                                let mut iv6: u32 =
                                    (v13 as u32).wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                                let b4 = DAT_009a73b0 == v13;
                                if rd8(iv6.wrapping_add(3)) != 0 && !b4 {
                                    let ip = rd32(iv6.wrapping_add(0x1f));
                                    iv6 = rd32(ip.wrapping_add(0xa9));
                                    let pc11 = name_ptr(rd8(iv6.wrapping_add(0x35)));
                                    if board1(pc11)
                                        && (rd16(iv6.wrapping_add(0x42)) == local_e
                                            || rd16(iv6.wrapping_add(0x66)) == local_e)
                                    {
                                        local_10 = local_10.wrapping_add(1);
                                    }
                                }
                                v13 = v13.wrapping_add(1);
                            }
                            let mut u12: u32 = 0;
                            if rd8(DAT_00a08104.wrapping_add(0x7d)) != 0 {
                                loop {
                                    if rd16(DAT_00a08104.wrapping_add(0x7e).wrapping_add(u12 * 2))
                                        == rd16(psv1)
                                    {
                                        local_10 = local_10.wrapping_add(1);
                                    }
                                    u12 = (u12 as u16).wrapping_add(1) as u32;
                                    if !(u12 < rd8(DAT_00a08104.wrapping_add(0x7d)) as u32) {
                                        break;
                                    }
                                }
                            }
                            if local_10 == 0 {
                                FUN_00403540(DAT_009a73b0 as u32, local_e as u32);
                                if DAT_009a1ff8 < local_4 {
                                    local_4 = DAT_009a1ff8;
                                    local_12 = local_e;
                                }
                            }
                        }
                    }
                }
                local_e = local_e.wrapping_add(1);
                if !((local_e as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }

        local_e = DAT_00a07ce8 as u16;
        i_var6 = DAT_009d1308.wrapping_add(DAT_00a07ce8) as u32;
        if (DAT_00a07ce8 & 0xffff) < (i_var6 as i32) {
            loop {
                i_var6 = (local_e as u32).wrapping_mul(0x23);
                let b2 = rd8(DAT_009a1fe8.wrapping_add(i_var6).wrapping_add(3));
                let tbl = (&raw mut DAT_00521f2e).cast::<u8>();
                if b2 != 0 && (tbl.add(b2 as usize * 0x1a).read() & 2) != 0 {
                    let mut u13 = FUN_00432fa0(
                        rd8(DAT_009a1fe8.wrapping_add(i_var6).wrapping_add(0x16)) as u16,
                        param_1,
                        param_2,
                    );
                    let u5 = FUN_00432fa0(
                        rd8(DAT_009a1fe8.wrapping_add(0x16).wrapping_add(i_var6)) as u16,
                        param_4,
                        param_5,
                    );
                    if param_3 == 1 {
                        u13 |= u5;
                    } else {
                        u13 &= u5;
                    }
                    if u13 != 0 {
                        let mut s9: i16 = 0;
                        let mut v13: u16 = DAT_00a07cec;
                        while (v13 as i32) < DAT_009a1fe0 {
                            let mut iv7: u32 =
                                (v13 as u32).wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                            let b4 = DAT_009a73b0 == v13;
                            if rd8(iv7.wrapping_add(3)) != 0 && !b4 {
                                let ip = rd32(iv7.wrapping_add(0x1f));
                                iv7 = rd32(ip.wrapping_add(0xa9));
                                let pc11 = name_ptr(rd8(iv7.wrapping_add(0x35)));
                                if board2(pc11) && rd16(iv7.wrapping_add(0x42)) == local_e {
                                    s9 = s9.wrapping_add(1);
                                }
                            }
                            v13 = v13.wrapping_add(1);
                        }
                        let mut u12: u32 = 0;
                        if rd8(DAT_00a08104.wrapping_add(0x7d)) != 0 {
                            loop {
                                if rd16(DAT_00a08104.wrapping_add(0x7e).wrapping_add(u12 * 2))
                                    == rd16(DAT_009a1fe8.wrapping_add(i_var6))
                                {
                                    s9 = s9.wrapping_add(1);
                                }
                                u12 = (u12 as u16).wrapping_add(1) as u32;
                                if !(u12 < rd8(DAT_00a08104.wrapping_add(0x7d)) as u32) {
                                    break;
                                }
                            }
                        }
                        if s9 == 0 {
                            FUN_00403540(DAT_009a73b0 as u32, local_e as u32);
                            if DAT_009a1ff8 < local_4 {
                                local_4 = DAT_009a1ff8;
                                local_12 = local_e;
                            }
                        }
                    }
                }
                local_e = local_e.wrapping_add(1);
                i_var6 = DAT_009d1308.wrapping_add(DAT_00a07ce8) as u32;
                if !((local_e as i32) < (i_var6 as i32)) {
                    break;
                }
            }
        }
        ((i_var6 >> 16) << 16) | local_12 as u32
    }
}
