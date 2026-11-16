// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0054f99c: i32;
    static mut DAT_0054f978: i32;
    static mut DAT_0053f8ec: [i32; 1];
    static mut DAT_009d1260: u16;
    static mut DAT_00a081ec: f32;
}

unsafe fn rd(p: *mut f32, i: usize) -> f32 {
    // SAFETY: caller guarantees p points at the vertex floats.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wr(p: *mut f32, i: usize, v: f32) {
    // SAFETY: caller guarantees p points at writable vertex floats.
    unsafe { p.add(i).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00409c10(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and the vertex buffer at param_3 are the game's own memory.
    unsafe {
        let iv19 = DAT_0054f99c;
        let at =
            |off: u32| -> *mut f32 { param_3.wrapping_add(off as i32) as u32 as usize as *mut f32 };
        let arr = &raw mut DAT_0053f8ec as *mut i32;
        let p1 = at((param_1 as u32).wrapping_mul(0x18));
        let p2 = at((param_2 as u32).wrapping_mul(0x18));
        let mut f3 = rd(p1, 1);
        let mut f4 = rd(p2, 1);
        let f13 = DAT_009d1260 as f32;
        if f3 <= f13 {
            if f13 < f4 {
                let f5 = rd(p1, 3);
                let f6 = rd(p2, 3);
                let f7 = rd(p1, 4);
                let f8 = rd(p2, 4);
                let mut f9 = rd(p1, 5);
                let f10 = rd(p2, 5);
                let f11 = rd(p1, 2);
                let f14 = rd(p2, 0) - rd(p1, 0);
                let f12 = rd(p2, 2);
                let f15 = f6 - f5;
                let f16 = f8 - f7;
                let f17 = f10 - f9;
                let f18 = f12 - f11;
                let p21: *mut f32;
                if f4 - f13 <= f13 - f3 {
                    f3 = (f4 - f13) / (f4 - f3);
                    p21 = at((DAT_0054f99c as u32).wrapping_mul(0x18));
                    DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                    wr(p21, 0, rd(p2, 0) - f3 * f14);
                    wr(p21, 3, f6 - f15 * f3);
                    f4 = f12 - f18 * f3;
                    if (if f18 < 0.0 { -f18 } else { f18 }) == 0.0 {
                        wr(p21, 2, f4);
                    } else {
                        wr(p21, 2, f4);
                        f3 = DAT_00a081ec;
                        let z = rd(at(8u32.wrapping_add((iv19 as u32).wrapping_mul(0x18))), 0);
                        f3 = (f3 / z - f3 / f12) / (f3 / f11 - f3 / f12);
                    }
                    wr(p21, 4, f8 - f3 * f16);
                    f9 = f10 - f3 * f17;
                } else {
                    f3 = (f13 - f3) / (f4 - f3);
                    p21 = at((DAT_0054f99c as u32).wrapping_mul(0x18));
                    DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                    wr(p21, 0, f3 * f14 + rd(p1, 0));
                    wr(p21, 3, f15 * f3 + f5);
                    f4 = f18 * f3 + f11;
                    if (if f18 < 0.0 { -f18 } else { f18 }) == 0.0 {
                        wr(p21, 2, f4);
                        f9 = f3 * f17 + f9;
                        wr(p21, 4, f3 * f16 + f7);
                    } else {
                        wr(p21, 2, f4);
                        f3 = DAT_00a081ec;
                        let z = rd(at(8u32.wrapping_add((iv19 as u32).wrapping_mul(0x18))), 0);
                        f3 = (f3 / z - f3 / f11) / (f3 / f12 - f3 / f11);
                        f9 = f3 * f17 + f9;
                        wr(p21, 4, f3 * f16 + f7);
                    }
                }
                wr(p21, 5, f9);
                wr(p21, 1, DAT_009d1260 as f32);
                DAT_0054f978 = DAT_0054f978.wrapping_add(1);
                arr.offset(DAT_0054f978 as isize).write(iv19);
                return;
            }
            DAT_0054f978 = DAT_0054f978.wrapping_add(1);
            arr.offset(DAT_0054f978 as isize).write(param_2);
        } else if f4 < f13 {
            let f5 = rd(p2, 3);
            let f6 = rd(p1, 3);
            let f7 = rd(p2, 4);
            let mut f8 = rd(p1, 4);
            let mut f9 = rd(p2, 5);
            let f10 = rd(p1, 5);
            let f11 = rd(p2, 2);
            let f14 = rd(p2, 0) - rd(p1, 0);
            let f12 = rd(p1, 2);
            let f15 = f5 - f6;
            let f16 = f7 - f8;
            let f17 = f9 - f10;
            let f18 = f11 - f12;
            let p21: *mut f32;
            if f13 - f4 <= f3 - f13 {
                f3 = (f13 - f4) / (f4 - f3);
                p21 = at((DAT_0054f99c as u32).wrapping_mul(0x18));
                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                wr(p21, 0, f3 * f14 + rd(p2, 0));
                wr(p21, 3, f15 * f3 + f5);
                f4 = f18 * f3 + f11;
                if (if f18 < 0.0 { -f18 } else { f18 }) == 0.0 {
                    wr(p21, 2, f4);
                    f8 = f3 * f16 + f7;
                    f9 = f3 * f17 + f9;
                } else {
                    wr(p21, 2, f4);
                    f3 = DAT_00a081ec;
                    let z = rd(at(8u32.wrapping_add((iv19 as u32).wrapping_mul(0x18))), 0);
                    f3 = (f3 / z - f3 / f11) / (f3 / f12 - f3 / f11);
                    f8 = f7 - f3 * f16;
                    f9 = f9 - f3 * f17;
                }
            } else {
                f3 = (f3 - f13) / (f4 - f3);
                p21 = at((DAT_0054f99c as u32).wrapping_mul(0x18));
                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                wr(p21, 0, rd(p1, 0) - f3 * f14);
                wr(p21, 3, f6 - f15 * f3);
                f4 = f12 - f18 * f3;
                if (if f18 < 0.0 { -f18 } else { f18 }) == 0.0 {
                    wr(p21, 2, f4);
                    f8 = f8 - f3 * f16;
                    f9 = f10 - f3 * f17;
                } else {
                    wr(p21, 2, f4);
                    f3 = DAT_00a081ec;
                    let z = rd(at(8u32.wrapping_add((iv19 as u32).wrapping_mul(0x18))), 0);
                    f3 = (f3 / z - f3 / f12) / (f3 / f11 - f3 / f12);
                    f8 = f3 * f16 + f8;
                    f9 = f3 * f17 + f10;
                }
            }
            wr(p21, 4, f8);
            wr(p21, 5, f9);
            wr(p21, 1, DAT_009d1260 as f32);
            let iv20 = DAT_0054f978.wrapping_add(2);
            DAT_0054f978 = iv20;
            arr.offset(iv20.wrapping_sub(1) as isize).write(iv19);
            arr.offset(iv20 as isize).write(param_2);
            return;
        }
    }
}
