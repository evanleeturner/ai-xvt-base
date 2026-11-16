// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db59c: *mut i32;
    static mut DAT_006da9e2: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f450: i32;
    static mut DAT_0072f448: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00525730: i32;
    static mut DAT_0071e536: u8;
    static mut DAT_007092d9: i8;
    static mut DAT_0052686c: [u8; 1];
    static mut DAT_0071bb1b: [u8; 1];
    static mut DAT_0071bb1f: [u8; 1];
    static mut DAT_0071bb23: [u8; 1];
    static mut DAT_0071bb27: [u8; 1];
    static mut DAT_0071b73c: [u8; 1];
    static mut DAT_0071b74a: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00465970(param_1: i32);
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004351e0(a: u32, b: i32, c: i32, d: i32);
    fn FUN_004832a0(a: i16, b: i16, c: i16, d: i32, e: i32);
    fn FUN_004010f0(a: u16);
    fn FUN_0045b420(a: i32);
    fn FUN_0049c2b0(a: u32);
    fn FUN_00486310(a: i32);
    fn FUN_00466020();
    fn FUN_00444040(a: u32, b: u32) -> u8;
    fn FUN_004440e0(a: u16) -> u32;
    fn FUN_00442720(a: u16, b: u8);
    fn FUN_0049c590(a: u16);
    fn FUN_0047f0f0(a: u32);
    fn FUN_00412e60(a: u16);
    fn FUN_0047f3d0(a: i32);
    fn FUN_004a6a80();
    fn FUN_00434990(a: u32);
    fn FUN_004a6f90();
    fn FUN_00465c40();
}

unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as usize as *const T).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004343f0(param_1: i32) {
    // SAFETY: all globals, callees and addresses belong to the game; reads
    // and writes happen in the original's order.
    unsafe {
        let pi_var3 = DAT_006db59c;
        DAT_006da9e2 = 0;
        loop {
            if DAT_006db59c.is_null() {
                DAT_006db59c = pi_var3;
                return;
            }
            let mut i_var6: i32 = 0;
            let u_var2: u32 = DAT_006db59c.add(1).read_unaligned() as u32;
            let t0: i32 = rd::<i32>(
                (DAT_006d4988 as u32)
                    .wrapping_add(0xc)
                    .wrapping_add(u_var2.wrapping_mul(0x23)),
            );
            if param_1 == 0 {
                if t0 < -0x10000 {
                    i_var6 = 1;
                }
            } else if -0x10001 < t0 {
                i_var6 = 1;
            }
            let i_var8: u32 = u_var2.wrapping_mul(0x23);
            if i_var6 != 0 {
                FUN_00465970(i_var6);
                let cur = (DAT_006d4988 as u32).wrapping_add(i_var8);
                let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
                let p1 = (&raw mut DAT_0071bb1b) as *mut u8 as usize as u32;
                let p2 = (&raw mut DAT_0071bb1f) as *mut u8 as usize as u32;
                let p3 = (&raw mut DAT_0071bb23) as *mut u8 as usize as u32;
                let p4 = (&raw mut DAT_0071bb27) as *mut u8 as usize as u32;
                let p5 = (&raw mut DAT_0071b73c) as *mut u8 as usize as u32;
                let p6 = (&raw mut DAT_0071b74a) as *mut u8 as usize as u32;
                let p7 = (&raw mut DAT_0052686c) as *mut u8 as usize as u32;
                let a0: u32 = rd(cur.wrapping_add(4));
                let b0: u32 = rd(p1.wrapping_add(idx));
                DAT_0072f44c = a0.wrapping_sub(b0) as i32;
                let a1: u32 = rd(cur.wrapping_add(8));
                let b1: u32 = rd(p2.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                DAT_0072f450 = a1.wrapping_sub(b1) as i32;
                let a2: u32 = rd(cur.wrapping_add(0xc));
                let b2: u32 = rd(p3.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                DAT_0072f448 = a2.wrapping_sub(b2) as i32;
                DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                let i6: u32 = (DAT_006d4988 as u32).wrapping_add(i_var8);
                let c_var1: i8 = rd::<i8>(i6.wrapping_add(2));
                let u_var9: u16 = u_var2 as u16;
                let mut path: u8 = 0;
                match c_var1 {
                    0..=5 | 0xe => {
                        let us0: u16 = rd(p7.wrapping_add(
                            (rd::<u8>(i6.wrapping_add(3)) as u32).wrapping_mul(0x16),
                        ));
                        if (us0 as i32) < (DAT_006db59c.read_unaligned() >> 4) {
                            FUN_004351e0(u_var2, DAT_006db7ac, DAT_006db7b0, DAT_006db7b4);
                        } else {
                            let t0: u32 = rd(i6.wrapping_add(0x1f));
                            let t1: i32 = rd(t0.wrapping_add(0xa7));
                            DAT_00732e54 = t1;
                            if c_var1 == 0xe {
                                DAT_0071e536 = 0;
                            }
                            let s0: i16 = rd(i6.wrapping_add(0x10));
                            let s1: i16 = rd(i6.wrapping_add(0x12));
                            let s2: i16 = rd(i6.wrapping_add(0x14));
                            FUN_004832a0(s2, s1, s0, 0, i6 as i32);
                            if c_var1 == 0xe {
                                FUN_004010f0(u_var9);
                            } else {
                                FUN_0045b420((DAT_006d4988 as u32).wrapping_add(i_var8) as i32);
                                FUN_0049c2b0(u_var2);
                                FUN_00486310((DAT_006d4988 as u32).wrapping_add(i_var8) as i32);
                                DAT_00525730 = 0;
                            }
                            DAT_0071e536 = 1;
                            FUN_00466020();
                        }
                        let us0: u16 =
                            rd(p4.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                        let t0: i32 = rd((DAT_006d4988 as u32)
                            .wrapping_add(i_var8)
                            .wrapping_add(0x1b));
                        if (us0 as u32 != u_var2) && (t0 != -1) {
                            let i6b: u32 = rd((DAT_006d4988 as u32)
                                .wrapping_add(i_var8)
                                .wrapping_add(0x1f));
                            let b7: u8 = match rd::<u8>(i6b.wrapping_add(0x86)) {
                                0 => 0x3f,
                                1 | 4 => 0x37,
                                2 => 0x33,
                                _ => 0x3b,
                            };
                            let cond1 = if DAT_007092d9 != 0 {
                                true
                            } else {
                                let us: u16 =
                                    rd(p5.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                                let u_var5 = us as u32;
                                let t2: u32 = rd(i6b.wrapping_add(0xa7));
                                if rd::<i8>(u_var5.wrapping_add(0x1f0).wrapping_add(t2)) != 0 {
                                    true
                                } else {
                                    FUN_00444040(u_var2, u_var5) == 0
                                }
                            };
                            if cond1 {
                                let u_var5 = FUN_004440e0(u_var9);
                                if (u_var5 as u8) == 0 {
                                    let us: u16 = rd(
                                        p6.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd))
                                    );
                                    if us as u32 != u_var2 {
                                        FUN_00442720(u_var9, b7);
                                    }
                                }
                            }
                        }
                    }
                    6 | 7 => {
                        let s0: i16 = rd(i6.wrapping_add(0x10));
                        let s1: i16 = rd(i6.wrapping_add(0x12));
                        let s2: i16 = rd(i6.wrapping_add(0x14));
                        FUN_004832a0(s2, s1, s0, 0, i6 as i32);
                        FUN_0049c590(u_var9);
                        FUN_00466020();
                    }
                    8 | 9 => {
                        let us0: u16 = rd(p7.wrapping_add(
                            (rd::<u8>(i6.wrapping_add(3)) as u32).wrapping_mul(0x16),
                        ));
                        if (DAT_006db59c.read_unaligned() >> 4) <= (us0 as i32) {
                            DAT_006da9e2 = 0;
                            if DAT_00732a38 <= (u_var2 as i32) {
                                path = 1;
                            } else {
                                let s0: i16 = rd(i6.wrapping_add(0x10));
                                let s1: i16 = rd(i6.wrapping_add(0x12));
                                let s2: i16 = rd(i6.wrapping_add(0x14));
                                FUN_004832a0(s2, s1, s0, 0, i6 as i32);
                                FUN_0047f0f0(u_var2);
                                path = 2;
                            }
                        } else {
                            FUN_004351e0(u_var2, DAT_006db7ac, DAT_006db7b0, DAT_006db7b4);
                        }
                    }
                    10 | 11 | 13 => {
                        DAT_006da9e2 = 0;
                        if (u_var2 as i32) < DAT_00732a38 {
                            let s0: i16 = rd(i6.wrapping_add(0x10));
                            let s1: i16 = rd(i6.wrapping_add(0x12));
                            let s2: i16 = rd(i6.wrapping_add(0x14));
                            FUN_004832a0(s2, s1, s0, 0, i6 as i32);
                            FUN_0047f0f0(u_var2);
                            path = 2;
                        } else {
                            path = 1;
                        }
                    }
                    _ => {}
                }
                if path == 1 {
                    DAT_006da9e2 = 0;
                    let s0: i16 = rd(i6.wrapping_add(0x10));
                    let s1: i16 = rd(i6.wrapping_add(0x12));
                    let s2: i16 = rd(i6.wrapping_add(0x14));
                    FUN_004832a0(s2, s1, s0, 0, 0);
                    FUN_00412e60(u_var9);
                    path = 2;
                }
                if path == 2 {
                    FUN_00466020();
                    FUN_0047f3d0(0);
                    DAT_006da9e2 = 0;
                }
                let us0: u16 = rd(p4.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                if us0 as u32 == u_var2 {
                    FUN_00442720(u_var9, 0x2f);
                } else {
                    let us0: u16 = rd(p6.wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd)));
                    if us0 as u32 == u_var2 {
                        FUN_00442720(u_var9, 0x3b);
                    }
                }
                FUN_004a6a80();
                FUN_00434990(u_var2);
                FUN_004a6f90();
                FUN_00465c40();
            }
            DAT_006db59c = DAT_006db59c.add(2).read_unaligned() as usize as *mut i32;
        }
    }
}
