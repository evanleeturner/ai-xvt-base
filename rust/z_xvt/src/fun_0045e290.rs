// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f10dc: i32;
    static mut DAT_005f10d8: i32;
    static mut DAT_005f0fac: [u32; 0];
    static mut DAT_005f0fa8: [u32; 0];
    static mut DAT_00732f3c: u32;
}

#[inline(always)]
unsafe fn rf(addr: u32) -> f32 {
    // SAFETY: addr points into the game's vertex buffer.
    unsafe { (addr as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn wf(addr: u32, v: f32) {
    // SAFETY: addr points into the game's vertex buffer.
    unsafe { (addr as *mut f32).write_unaligned(v) }
}

#[inline(always)]
fn absf(f: f32) -> f32 {
    if f < 0.0 { -f } else { f }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045e290(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: globals and buffers are the game's own memory; addresses are
    // computed exactly as the original does.
    unsafe {
        let i_var17 = DAT_005f10dc;
        let base = param_3 as u32;
        let mut f2 = rf(base.wrapping_add((param_1 as u32).wrapping_mul(0x18)));
        let i18 = base.wrapping_add((param_1 as u32).wrapping_mul(0x18));
        let mut f3 = rf(base.wrapping_add((param_2 as u32).wrapping_mul(0x18)));
        let i1 = base.wrapping_add((param_2 as u32).wrapping_mul(0x18));
        let fac = (&raw mut DAT_005f0fac).cast::<u32>();
        let fa8 = (&raw mut DAT_005f0fa8).cast::<u32>();
        let k = (&raw const DAT_00732f3c).cast::<f32>();

        if (i18 as *const u32).read_unaligned() < 0x80000001u32 {
            if 0x80000000u32 < (i1 as *const u32).read_unaligned() {
                let f4 = rf(i18.wrapping_add(0xc));
                let f5 = rf(i1.wrapping_add(0xc));
                let f6 = rf(i18.wrapping_add(0x10));
                let mut f7 = rf(i1.wrapping_add(0x10));
                let mut f8 = rf(i18.wrapping_add(0x14));
                let f9 = rf(i1.wrapping_add(0x14));
                let f10 = rf(i18.wrapping_add(8));
                let f11 = rf(i1.wrapping_add(8));
                let f12 = rf(i1.wrapping_add(4)) - rf(i18.wrapping_add(4));
                let f13 = f5 - f4;
                let f14 = f7 - f6;
                let f15 = f9 - f8;
                let f16 = f11 - f10;
                let p: u32;
                if -f3 <= f2 {
                    f2 = -f3 / (f3 - f2);
                    p = base.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wf(p.wrapping_add(4), f2 * f12 + rf(i1.wrapping_add(4)));
                    wf(p.wrapping_add(12), f2 * f13 + f5);
                    f3 = f2 * f16 + f11;
                    if absf(f16) == 0.0 {
                        wf(p.wrapping_add(8), f3);
                        f7 = f2 * f14 + f7;
                        f8 = f2 * f15 + f9;
                    } else {
                        wf(p.wrapping_add(8), f3);
                        f2 = k.read();
                        f2 = (f2
                            / rf(base
                                .wrapping_add(8)
                                .wrapping_add((i_var17 as u32).wrapping_mul(0x18)))
                            - f2 / f11)
                            / (f2 / f10 - f2 / f11);
                        f7 = f7 - f14 * f2;
                        f8 = f9 - f2 * f15;
                    }
                } else {
                    f2 = f2 / (f3 - f2);
                    p = base.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                    DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                    wf(p.wrapping_add(4), rf(i18.wrapping_add(4)) - f2 * f12);
                    wf(p.wrapping_add(12), f4 - f2 * f13);
                    f3 = f10 - f2 * f16;
                    if absf(f16) == 0.0 {
                        wf(p.wrapping_add(8), f3);
                        f7 = f6 - f2 * f14;
                        f8 = f8 - f2 * f15;
                    } else {
                        wf(p.wrapping_add(8), f3);
                        f2 = k.read();
                        f2 = (f2
                            / rf(base
                                .wrapping_add(8)
                                .wrapping_add((i_var17 as u32).wrapping_mul(0x18)))
                            - f2 / f10)
                            / (f2 / f11 - f2 / f10);
                        f7 = f14 * f2 + f6;
                        f8 = f2 * f15 + f8;
                    }
                }
                wf(p.wrapping_add(16), f7);
                wf(p.wrapping_add(20), f8);
                (p as *mut u32).write_unaligned(0);
                DAT_005f10d8 = DAT_005f10d8.wrapping_add(1);
                fac.offset(DAT_005f10d8 as isize).write(i_var17 as u32);
                return;
            }
            DAT_005f10d8 = DAT_005f10d8.wrapping_add(1);
            fac.offset(DAT_005f10d8 as isize).write(param_2 as u32);
        } else if (i1 as *const u32).read_unaligned() < 0x80000001u32 {
            let f4 = rf(i1.wrapping_add(0xc));
            let f5 = rf(i18.wrapping_add(0xc));
            let f6 = rf(i1.wrapping_add(0x10));
            let f7 = rf(i18.wrapping_add(0x10));
            let f8 = rf(i1.wrapping_add(0x14));
            let mut f9 = rf(i18.wrapping_add(0x14));
            let f10 = rf(i1.wrapping_add(8));
            let f11 = rf(i18.wrapping_add(8));
            let f12 = rf(i1.wrapping_add(4)) - rf(i18.wrapping_add(4));
            let f13 = f4 - f5;
            let f14 = f6 - f7;
            let f15 = f8 - f9;
            let f16 = f10 - f11;
            let p: u32;
            if f3 <= -f2 {
                f3 = f3 / (f3 - f2);
                p = base.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wf(p.wrapping_add(4), rf(i1.wrapping_add(4)) - f3 * f12);
                wf(p.wrapping_add(12), f4 - f3 * f13);
                f2 = f10 - f3 * f16;
                if absf(f16) == 0.0 {
                    wf(p.wrapping_add(8), f2);
                } else {
                    wf(p.wrapping_add(8), f2);
                    f2 = k.read();
                    f3 = (f2
                        / rf(base
                            .wrapping_add(8)
                            .wrapping_add((i_var17 as u32).wrapping_mul(0x18)))
                        - f2 / f10)
                        / (f2 / f11 - f2 / f10);
                }
                wf(p.wrapping_add(16), f6 - f14 * f3);
                f9 = f8 - f3 * f15;
            } else {
                f2 = -f2 / (f3 - f2);
                p = base.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18));
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wf(p.wrapping_add(4), f2 * f12 + rf(i18.wrapping_add(4)));
                wf(p.wrapping_add(12), f2 * f13 + f5);
                f3 = f2 * f16 + f11;
                if absf(f16) == 0.0 {
                    wf(p.wrapping_add(8), f3);
                    f9 = f2 * f15 + f9;
                    wf(p.wrapping_add(16), f2 * f14 + f7);
                } else {
                    wf(p.wrapping_add(8), f3);
                    f2 = k.read();
                    f2 = (f2
                        / rf(base
                            .wrapping_add(8)
                            .wrapping_add((i_var17 as u32).wrapping_mul(0x18)))
                        - f2 / f11)
                        / (f2 / f10 - f2 / f11);
                    f9 = f2 * f15 + f9;
                    wf(p.wrapping_add(16), f14 * f2 + f7);
                }
            }
            wf(p.wrapping_add(20), f9);
            (p as *mut u32).write_unaligned(0);
            let i18n = (DAT_005f10d8 as u32).wrapping_add(2);
            DAT_005f10d8 = i18n as i32;
            fa8.add(i18n as usize).write(i_var17 as u32);
            fac.add(i18n as usize).write(param_2 as u32);
        }
    }
}
