// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f10dc: i32;
    static mut DAT_005f10d8: i32;
    static mut DAT_005f0fa8: i32;
    static mut DAT_005f0fac: i32;
    static mut DAT_00732f3c: f32;
}

#[inline(always)]
unsafe fn rd(p: *mut f32, i: usize) -> f32 {
    // SAFETY: p points at the caller's vertex array.
    unsafe { p.byte_add(i * 4).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut f32, i: usize, v: f32) {
    // SAFETY: p points at the caller's vertex array.
    unsafe { p.byte_add(i * 4).write_unaligned(v) }
}

#[inline(always)]
fn fabs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffff)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045d7d0(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: globals and the vertex array are the game's own memory,
    // accessed where the original does.
    unsafe {
        let i_var18 = DAT_005f10dc;
        let p1 =
            (param_3 as u32).wrapping_add((param_1 as u32).wrapping_mul(0x18)) as usize as *mut f32;
        let p2 =
            (param_3 as u32).wrapping_add((param_2 as u32).wrapping_mul(0x18)) as usize as *mut f32;
        let mut f3 = rd(p1, 1);
        let mut f4 = rd(p2, 1);
        let b3 = f3.to_bits();
        let b4 = f4.to_bits();
        if b3 < 0x8000_0001 {
            if 0x8000_0000 < b4 {
                let f5 = rd(p1, 3);
                let f6 = rd(p2, 3);
                let mut f7 = rd(p1, 4);
                let f8 = rd(p2, 4);
                let f9 = rd(p1, 5);
                let mut f10 = rd(p2, 5);
                let f11 = rd(p1, 2);
                let f13 = rd(p2, 0) - rd(p1, 0);
                let f12 = rd(p2, 2);
                let f14 = f6 - f5;
                let f15 = f8 - f7;
                let f16 = f10 - f9;
                let f17 = f12 - f11;
                let p20;
                if -f4 <= f3 {
                    f3 = -f4 / (f4 - f3);
                    p20 = (param_3 as u32).wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18))
                        as usize as *mut f32;
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wr(p20, 0, f3 * f13 + rd(p2, 0));
                    wr(p20, 3, f14 * f3 + f6);
                    f4 = f17 * f3 + f12;
                    if fabs(f17) == 0.0 {
                        wr(p20, 2, f4);
                        f7 = f3 * f15 + f8;
                        f10 = f3 * f16 + f10;
                    } else {
                        wr(p20, 2, f4);
                        f3 = DAT_00732f3c;
                        let a = rd(
                            (param_3 as u32)
                                .wrapping_add(8)
                                .wrapping_add((i_var18 as u32).wrapping_mul(0x18))
                                as usize as *mut f32,
                            0,
                        );
                        f3 = (f3 / a - f3 / f12) / (f3 / f11 - f3 / f12);
                        f7 = f8 - f3 * f15;
                        f10 = f10 - f3 * f16;
                    }
                } else {
                    f3 = f3 / (f4 - f3);
                    p20 = (param_3 as u32).wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18))
                        as usize as *mut f32;
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wr(p20, 0, rd(p1, 0) - f3 * f13);
                    wr(p20, 3, f5 - f14 * f3);
                    f4 = f11 - f17 * f3;
                    if fabs(f17) == 0.0 {
                        wr(p20, 2, f4);
                        f7 = f7 - f3 * f15;
                        f10 = f9 - f3 * f16;
                    } else {
                        wr(p20, 2, f4);
                        f3 = DAT_00732f3c;
                        let a = rd(
                            (param_3 as u32)
                                .wrapping_add(8)
                                .wrapping_add((i_var18 as u32).wrapping_mul(0x18))
                                as usize as *mut f32,
                            0,
                        );
                        f3 = (f3 / a - f3 / f11) / (f3 / f12 - f3 / f11);
                        f7 = f3 * f15 + f7;
                        f10 = f3 * f16 + f9;
                    }
                }
                wr(p20, 4, f7);
                wr(p20, 5, f10);
                wr(p20, 1, 0.0);
                DAT_005f10d8 = DAT_005f10d8.wrapping_add(1);
                (&raw mut DAT_005f0fac)
                    .wrapping_offset(DAT_005f10d8 as isize)
                    .write(i_var18);
                return;
            }
            DAT_005f10d8 = DAT_005f10d8.wrapping_add(1);
            (&raw mut DAT_005f0fac)
                .wrapping_offset(DAT_005f10d8 as isize)
                .write(param_2);
        } else if b4 < 0x8000_0001 {
            let f5 = rd(p2, 3);
            let f6 = rd(p1, 3);
            let f7 = rd(p2, 4);
            let f8 = rd(p1, 4);
            let f9 = rd(p2, 5);
            let mut f10 = rd(p1, 5);
            let f11 = rd(p2, 2);
            let f13 = rd(p2, 0) - rd(p1, 0);
            let f12 = rd(p1, 2);
            let f14 = f5 - f6;
            let f15 = f7 - f8;
            let f16 = f9 - f10;
            let f17 = f11 - f12;
            let p20;
            if f4 <= -f3 {
                f4 = f4 / (f4 - f3);
                p20 = (param_3 as u32).wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18))
                    as usize as *mut f32;
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wr(p20, 0, rd(p2, 0) - f4 * f13);
                wr(p20, 3, f5 - f14 * f4);
                f3 = f11 - f17 * f4;
                if fabs(f17) == 0.0 {
                    wr(p20, 2, f3);
                } else {
                    wr(p20, 2, f3);
                    f3 = DAT_00732f3c;
                    let a = rd(
                        (param_3 as u32)
                            .wrapping_add(8)
                            .wrapping_add((i_var18 as u32).wrapping_mul(0x18))
                            as usize as *mut f32,
                        0,
                    );
                    f4 = (f3 / a - f3 / f11) / (f3 / f12 - f3 / f11);
                }
                wr(p20, 4, f7 - f4 * f15);
                f10 = f9 - f4 * f16;
            } else {
                f3 = -f3 / (f4 - f3);
                p20 = (param_3 as u32).wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18))
                    as usize as *mut f32;
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wr(p20, 0, f3 * f13 + rd(p1, 0));
                wr(p20, 3, f14 * f3 + f6);
                f4 = f17 * f3 + f12;
                if fabs(f17) == 0.0 {
                    wr(p20, 2, f4);
                    f10 = f3 * f16 + f10;
                    wr(p20, 4, f3 * f15 + f8);
                } else {
                    wr(p20, 2, f4);
                    f3 = DAT_00732f3c;
                    let a = rd(
                        (param_3 as u32)
                            .wrapping_add(8)
                            .wrapping_add((i_var18 as u32).wrapping_mul(0x18))
                            as usize as *mut f32,
                        0,
                    );
                    f3 = (f3 / a - f3 / f12) / (f3 / f11 - f3 / f12);
                    f10 = f3 * f16 + f10;
                    wr(p20, 4, f3 * f15 + f8);
                }
            }
            wr(p20, 5, f10);
            wr(p20, 1, 0.0);
            let i_var19 = DAT_005f10d8.wrapping_add(2);
            DAT_005f10d8 = i_var19;
            (&raw mut DAT_005f0fa8)
                .wrapping_offset(i_var19 as isize)
                .write(i_var18);
            (&raw mut DAT_005f0fac)
                .wrapping_offset(i_var19 as isize)
                .write(param_2);
            return;
        }
    }
}
