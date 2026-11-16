// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703be4: u16;
    static mut DAT_00732f3c: i32;
    static mut DAT_005f10dc: i32;
    static mut DAT_005f10b8: i32;
    static mut DAT_005f102c: [i32; 0];
}

#[inline(always)]
unsafe fn rd(p: *mut f32, i: usize) -> f32 {
    // SAFETY: caller guarantees p points at the game's vertex data.
    unsafe { p.add(i).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut f32, i: usize, v: f32) {
    // SAFETY: caller guarantees p points at the game's vertex data.
    unsafe { p.add(i).write_unaligned(v) }
}

#[inline(always)]
unsafe fn push(idx: i32, v: i32) {
    // SAFETY: the array is the game's own memory, indexed as the original does.
    unsafe {
        (&raw mut DAT_005f102c)
            .cast::<i32>()
            .offset(idx as isize)
            .write_unaligned(v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045dd00(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: all globals and the vertex buffer at param_3 are the game's own
    // memory, read and written in the original's order.
    unsafe {
        let i_var19 = DAT_005f10dc;
        let base = param_3 as u32;
        let pf1 = base.wrapping_add((param_1 as u32).wrapping_mul(0x18)) as *mut f32;
        let pf2 = base.wrapping_add((param_2 as u32).wrapping_mul(0x18)) as *mut f32;
        let mut f3 = rd(pf1, 1);
        let mut f4 = rd(pf2, 1);
        let f13 = f32::from(DAT_00703be4);
        if f3 <= f13 {
            if f13 < f4 {
                let f5 = rd(pf1, 3);
                let f6 = rd(pf2, 3);
                let f7 = rd(pf1, 4);
                let f8 = rd(pf2, 4);
                let mut f9 = rd(pf1, 5);
                let f10 = rd(pf2, 5);
                let f11 = rd(pf1, 2);
                let f14 = rd(pf2, 0) - rd(pf1, 0);
                let f12 = rd(pf2, 2);
                let f15 = f6 - f5;
                let f16 = f8 - f7;
                let f17 = f10 - f9;
                let f18 = f12 - f11;
                let pf21: *mut f32;
                let slot = |n: i32| -> *mut f32 {
                    base.wrapping_add((n as u32).wrapping_mul(0x18)) as *mut f32
                };
                let prev = |n: i32| -> *mut f32 {
                    base.wrapping_add(8)
                        .wrapping_add((n as u32).wrapping_mul(0x18)) as *mut f32
                };
                if f4 - f13 <= f13 - f3 {
                    f3 = (f4 - f13) / (f4 - f3);
                    pf21 = slot(DAT_005f10dc);
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wr(pf21, 0, rd(pf2, 0) - f3 * f14);
                    wr(pf21, 3, f6 - f15 * f3);
                    f4 = f12 - f3 * f18;
                    if f18 == 0.0 {
                        wr(pf21, 2, f4);
                    } else {
                        wr(pf21, 2, f4);
                        f3 = DAT_00732f3c as f32;
                        f3 = (f3 / prev(i_var19).read_unaligned() - f3 / f12)
                            / (f3 / f11 - f3 / f12);
                    }
                    wr(pf21, 4, f8 - f3 * f16);
                    f9 = f10 - f3 * f17;
                } else {
                    f3 = (f13 - f3) / (f4 - f3);
                    pf21 = slot(DAT_005f10dc);
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wr(pf21, 0, f3 * f14 + rd(pf1, 0));
                    wr(pf21, 3, f15 * f3 + f5);
                    f4 = f3 * f18 + f11;
                    if f18 == 0.0 {
                        wr(pf21, 2, f4);
                        f9 = f3 * f17 + f9;
                        wr(pf21, 4, f3 * f16 + f7);
                    } else {
                        wr(pf21, 2, f4);
                        f3 = DAT_00732f3c as f32;
                        f3 = (f3 / prev(i_var19).read_unaligned() - f3 / f11)
                            / (f3 / f12 - f3 / f11);
                        f9 = f3 * f17 + f9;
                        wr(pf21, 4, f3 * f16 + f7);
                    }
                }
                wr(pf21, 5, f9);
                wr(pf21, 1, f32::from(DAT_00703be4));
                DAT_005f10b8 = DAT_005f10b8.wrapping_add(1);
                push(DAT_005f10b8, i_var19);
                return;
            }
            DAT_005f10b8 = DAT_005f10b8.wrapping_add(1);
            push(DAT_005f10b8, param_2);
        } else if f4 < f13 {
            let f5 = rd(pf2, 3);
            let f6 = rd(pf1, 3);
            let f7 = rd(pf2, 4);
            let mut f8 = rd(pf1, 4);
            let mut f9 = rd(pf2, 5);
            let f10 = rd(pf1, 5);
            let f11 = rd(pf2, 2);
            let f14 = rd(pf2, 0) - rd(pf1, 0);
            let f12 = rd(pf1, 2);
            let f15 = f5 - f6;
            let f16 = f7 - f8;
            let f17 = f9 - f10;
            let f18 = f11 - f12;
            let slot = |n: i32| -> *mut f32 {
                base.wrapping_add((n as u32).wrapping_mul(0x18)) as *mut f32
            };
            let prev = |n: i32| -> *mut f32 {
                base.wrapping_add(8)
                    .wrapping_add((n as u32).wrapping_mul(0x18)) as *mut f32
            };
            let pf21: *mut f32;
            if f13 - f4 <= f3 - f13 {
                f3 = (f13 - f4) / (f4 - f3);
                pf21 = slot(DAT_005f10dc);
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wr(pf21, 0, f3 * f14 + rd(pf2, 0));
                wr(pf21, 3, f15 * f3 + f5);
                f4 = f3 * f18 + f11;
                if f18 == 0.0 {
                    wr(pf21, 2, f4);
                    f8 = f3 * f16 + f7;
                    f9 = f3 * f17 + f9;
                } else {
                    wr(pf21, 2, f4);
                    f3 = DAT_00732f3c as f32;
                    f3 = (f3 / prev(i_var19).read_unaligned() - f3 / f11) / (f3 / f12 - f3 / f11);
                    f8 = f7 - f3 * f16;
                    f9 = f9 - f3 * f17;
                }
            } else {
                f3 = (f3 - f13) / (f4 - f3);
                pf21 = slot(DAT_005f10dc);
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wr(pf21, 0, rd(pf1, 0) - f3 * f14);
                wr(pf21, 3, f6 - f15 * f3);
                f4 = f12 - f3 * f18;
                if f18 == 0.0 {
                    wr(pf21, 2, f4);
                    f8 = f8 - f3 * f16;
                    f9 = f10 - f3 * f17;
                } else {
                    wr(pf21, 2, f4);
                    f3 = DAT_00732f3c as f32;
                    f3 = (f3 / prev(i_var19).read_unaligned() - f3 / f12) / (f3 / f11 - f3 / f12);
                    f8 = f3 * f16 + f8;
                    f9 = f3 * f17 + f10;
                }
            }
            wr(pf21, 4, f8);
            wr(pf21, 5, f9);
            wr(pf21, 1, f32::from(DAT_00703be4));
            let i_var20 = DAT_005f10b8.wrapping_add(2);
            DAT_005f10b8 = i_var20;
            push(i_var20.wrapping_sub(1), i_var19);
            push(i_var20, param_2);
            return;
        }
    }
}
