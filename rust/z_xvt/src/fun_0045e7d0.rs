// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f10dc: i32;
    static mut DAT_005f10b8: i32;
    static mut DAT_005f102c: [i32; 1];
    static mut DAT_007091ae: u16;
    static mut DAT_00732f3c: f32;
}

unsafe fn rf(a: u32) -> f32 {
    // SAFETY: the address is the game's own vertex memory.
    unsafe { (a as *const f32).read_unaligned() }
}

unsafe fn wf(a: u32, v: f32) {
    // SAFETY: the address is the game's own vertex memory.
    unsafe { (a as *mut f32).write_unaligned(v) }
}

unsafe fn set_arr(i: i32, v: i32) {
    // SAFETY: the array is the game's own, indexed as the original does.
    unsafe {
        (&raw mut DAT_005f102c)
            .cast::<i32>()
            .offset(i as isize)
            .write(v)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045e7d0(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: all globals and the vertex memory addressed through the
    // parameters belong to the game; accessed in the original's order.
    unsafe {
        let b3 = param_3 as u32;
        let i_var18 = DAT_005f10dc;
        let i_var19 = b3.wrapping_add((param_1 as u32).wrapping_mul(0x18));
        let mut f2 = rf(i_var19);
        let i_var1 = b3.wrapping_add((param_2 as u32).wrapping_mul(0x18));
        let mut f3 = rf(i_var1);
        let f12 = DAT_007091ae as f32;
        let fabs = |x: f32| if x < 0.0 { -x } else { x };

        if f2 <= f12 {
            if f3 > f12 {
                let f4 = rf(i_var19.wrapping_add(0xc));
                let f5 = rf(i_var1.wrapping_add(0xc));
                let f6 = rf(i_var19.wrapping_add(0x10));
                let f7 = rf(i_var1.wrapping_add(0x10));
                let mut f8 = rf(i_var19.wrapping_add(0x14));
                let f9 = rf(i_var1.wrapping_add(0x14));
                let f10 = rf(i_var19.wrapping_add(8));
                let f11 = rf(i_var1.wrapping_add(8));
                let f13 = rf(i_var1.wrapping_add(4)) - rf(i_var19.wrapping_add(4));
                let f14 = f5 - f4;
                let f15 = f7 - f6;
                let f16 = f9 - f8;
                let f17 = f11 - f10;
                let p: u32;
                if f3 - f12 <= f12 - f2 {
                    f2 = (f3 - f12) / (f3 - f2);
                    p = b3.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wf(p + 4, rf(i_var1.wrapping_add(4)) - f2 * f13);
                    wf(p + 12, f5 - f14 * f2);
                    f3 = f11 - f17 * f2;
                    if fabs(f17) == 0.0 {
                        wf(p + 8, f3);
                    } else {
                        wf(p + 8, f3);
                        f2 = DAT_00732f3c;
                        f2 = (f2
                            / rf(b3
                                .wrapping_add(8)
                                .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                            - f2 / f11)
                            / (f2 / f10 - f2 / f11);
                    }
                    wf(p + 16, f7 - f15 * f2);
                    f8 = f9 - f2 * f16;
                } else {
                    f2 = (f12 - f2) / (f3 - f2);
                    p = b3.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wf(p + 4, f2 * f13 + rf(i_var19.wrapping_add(4)));
                    wf(p + 12, f14 * f2 + f4);
                    f3 = f17 * f2 + f10;
                    if fabs(f17) == 0.0 {
                        wf(p + 8, f3);
                        f8 = f2 * f16 + f8;
                        wf(p + 16, f15 * f2 + f6);
                    } else {
                        wf(p + 8, f3);
                        f2 = DAT_00732f3c;
                        f2 = (f2
                            / rf(b3
                                .wrapping_add(8)
                                .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                            - f2 / f10)
                            / (f2 / f11 - f2 / f10);
                        f8 = f2 * f16 + f8;
                        wf(p + 16, f15 * f2 + f6);
                    }
                }
                wf(p + 20, f8);
                wf(p, DAT_007091ae as f32);
                DAT_005f10b8 = DAT_005f10b8.wrapping_add(1);
                set_arr(DAT_005f10b8, i_var18);
                return;
            }
            DAT_005f10b8 = DAT_005f10b8.wrapping_add(1);
            set_arr(DAT_005f10b8, param_2);
        } else if f3 <= f12 {
            let f4 = rf(i_var1.wrapping_add(0xc));
            let f5 = rf(i_var19.wrapping_add(0xc));
            let mut f6 = rf(i_var1.wrapping_add(0x10));
            let f7 = rf(i_var19.wrapping_add(0x10));
            let f8 = rf(i_var1.wrapping_add(0x14));
            let mut f9 = rf(i_var19.wrapping_add(0x14));
            let f10 = rf(i_var1.wrapping_add(8));
            let f11 = rf(i_var19.wrapping_add(8));
            let f13 = rf(i_var1.wrapping_add(4)) - rf(i_var19.wrapping_add(4));
            let f14 = f4 - f5;
            let f15 = f6 - f7;
            let f16 = f8 - f9;
            let f17 = f10 - f11;
            let p: u32;
            if f12 - f3 <= f2 - f12 {
                f2 = (f12 - f3) / (f3 - f2);
                p = b3.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wf(p + 4, f2 * f13 + rf(i_var1.wrapping_add(4)));
                wf(p + 12, f14 * f2 + f4);
                f3 = f17 * f2 + f10;
                if fabs(f17) == 0.0 {
                    wf(p + 8, f3);
                    f6 = f15 * f2 + f6;
                    f9 = f2 * f16 + f8;
                } else {
                    wf(p + 8, f3);
                    f2 = DAT_00732f3c;
                    f2 = (f2
                        / rf(b3
                            .wrapping_add(8)
                            .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                        - f2 / f10)
                        / (f2 / f11 - f2 / f10);
                    f6 = f6 - f15 * f2;
                    f9 = f8 - f2 * f16;
                }
            } else {
                f2 = (f2 - f12) / (f3 - f2);
                p = b3.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wf(p + 4, rf(i_var19.wrapping_add(4)) - f2 * f13);
                wf(p + 12, f5 - f14 * f2);
                f3 = f11 - f17 * f2;
                if fabs(f17) == 0.0 {
                    wf(p + 8, f3);
                    f6 = f7 - f15 * f2;
                    f9 = f9 - f2 * f16;
                } else {
                    wf(p + 8, f3);
                    f2 = DAT_00732f3c;
                    f2 = (f2
                        / rf(b3
                            .wrapping_add(8)
                            .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                        - f2 / f11)
                        / (f2 / f10 - f2 / f11);
                    f6 = f15 * f2 + f7;
                    f9 = f2 * f16 + f9;
                }
            }
            wf(p + 16, f6);
            wf(p + 20, f9);
            wf(p, DAT_007091ae as f32);
            let i19 = DAT_005f10b8.wrapping_add(2);
            DAT_005f10b8 = i19;
            set_arr(i19.wrapping_sub(1), i_var18);
            set_arr(i19, param_2);
            return;
        }
    }
}
