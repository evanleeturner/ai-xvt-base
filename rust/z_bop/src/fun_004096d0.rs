// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0054f99c: i32;
    static mut DAT_0054f998: i32;
    static mut DAT_0052f868: [i32; 1];
    static mut DAT_0052f86c: [i32; 1];
    static mut DAT_00a081ec: f32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004096d0(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: all globals and the vertex buffer at param_3 are the game's own
    // memory; accesses mirror the original's addresses and order.
    unsafe {
        let i_var18 = DAT_0054f99c;
        let base = param_3 as u32;
        let p1 = base.wrapping_add((param_1 as u32).wrapping_mul(0x18)) as *mut f32;
        let p2 = base.wrapping_add((param_2 as u32).wrapping_mul(0x18)) as *mut f32;
        let mut f3: f32 = p1.add(1).read_unaligned();
        let mut f4: f32 = p2.add(1).read_unaligned();
        let u3 = f3.to_bits();
        let u4 = f4.to_bits();
        let arr868 = (&raw mut DAT_0052f868).cast::<i32>();
        let arr86c = (&raw mut DAT_0052f86c).cast::<i32>();

        if u3 < 0x80000001 {
            if 0x80000000 < u4 {
                let f5 = p1.add(3).read_unaligned();
                let f6 = p2.add(3).read_unaligned();
                let f7 = p1.add(4).read_unaligned();
                let mut f8 = p2.add(4).read_unaligned();
                let mut f9 = p1.add(5).read_unaligned();
                let f10 = p2.add(5).read_unaligned();
                let f11 = p1.add(2).read_unaligned();
                let f13 = p2.read_unaligned() - p1.read_unaligned();
                let f12 = p2.add(2).read_unaligned();
                let f14 = f6 - f5;
                let f15 = f8 - f7;
                let f16 = f10 - f9;
                let f17 = f12 - f11;
                let p20;
                if -f4 <= f3 {
                    f3 = -f4 / (f4 - f3);
                    p20 = base.wrapping_add((DAT_0054f99c as u32).wrapping_mul(0x18)) as *mut f32;
                    DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                    p20.write_unaligned(f3 * f13 + p2.read_unaligned());
                    p20.add(3).write_unaligned(f14 * f3 + f6);
                    f4 = f17 * f3 + f12;
                    if f17 == 0.0 {
                        p20.add(2).write_unaligned(f4);
                        f8 = f15 * f3 + f8;
                        f9 = f16 * f3 + f10;
                    } else {
                        p20.add(2).write_unaligned(f4);
                        f3 = DAT_00a081ec;
                        let m = (base
                            .wrapping_add(8)
                            .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                            as *const f32;
                        f3 = (f3 / m.read_unaligned() - f3 / f12) / (f3 / f11 - f3 / f12);
                        f8 = f8 - f15 * f3;
                        f9 = f10 - f3 * f16;
                    }
                } else {
                    f3 = f3 / (f4 - f3);
                    p20 = base.wrapping_add((DAT_0054f99c as u32).wrapping_mul(0x18)) as *mut f32;
                    DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                    p20.write_unaligned(p1.read_unaligned() - f3 * f13);
                    p20.add(3).write_unaligned(f5 - f3 * f14);
                    f4 = f11 - f3 * f17;
                    if f17 == 0.0 {
                        p20.add(2).write_unaligned(f4);
                        f8 = f7 - f3 * f15;
                        f9 = f9 - f3 * f16;
                    } else {
                        p20.add(2).write_unaligned(f4);
                        f3 = DAT_00a081ec;
                        let m = (base
                            .wrapping_add(8)
                            .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                            as *const f32;
                        f3 = (f3 / m.read_unaligned() - f3 / f11) / (f3 / f12 - f3 / f11);
                        f8 = f15 * f3 + f7;
                        f9 = f3 * f16 + f9;
                    }
                }
                p20.add(4).write_unaligned(f8);
                p20.add(5).write_unaligned(f9);
                p20.add(1).write_unaligned(0.0);
                DAT_0054f998 = DAT_0054f998.wrapping_add(1);
                *arr86c.offset(DAT_0054f998 as isize) = i_var18;
                return;
            }
            DAT_0054f998 = DAT_0054f998.wrapping_add(1);
            *arr86c.offset(DAT_0054f998 as isize) = param_2;
        } else if u4 < 0x80000001 {
            let f5 = p2.add(3).read_unaligned();
            let f6 = p1.add(3).read_unaligned();
            let f7 = p2.add(4).read_unaligned();
            let f8 = p1.add(4).read_unaligned();
            let f9 = p2.add(5).read_unaligned();
            let mut f10 = p1.add(5).read_unaligned();
            let f11 = p2.add(2).read_unaligned();
            let f13 = p2.read_unaligned() - p1.read_unaligned();
            let f12 = p1.add(2).read_unaligned();
            let f14 = f5 - f6;
            let f15 = f7 - f8;
            let f16 = f9 - f10;
            let f17 = f11 - f12;
            let p20;
            if f4 <= -f3 {
                f4 = f4 / (f4 - f3);
                p20 = base.wrapping_add((DAT_0054f99c as u32).wrapping_mul(0x18)) as *mut f32;
                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                p20.write_unaligned(p2.read_unaligned() - f4 * f13);
                p20.add(3).write_unaligned(f5 - f14 * f4);
                f3 = f11 - f17 * f4;
                if f17 == 0.0 {
                    p20.add(2).write_unaligned(f3);
                } else {
                    p20.add(2).write_unaligned(f3);
                    f3 = DAT_00a081ec;
                    let m = (base
                        .wrapping_add(8)
                        .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                        as *const f32;
                    f4 = (f3 / m.read_unaligned() - f3 / f11) / (f3 / f12 - f3 / f11);
                }
                p20.add(4).write_unaligned(f7 - f15 * f4);
                f10 = f9 - f4 * f16;
            } else {
                f3 = -f3 / (f4 - f3);
                p20 = base.wrapping_add((DAT_0054f99c as u32).wrapping_mul(0x18)) as *mut f32;
                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                p20.write_unaligned(f3 * f13 + p1.read_unaligned());
                p20.add(3).write_unaligned(f3 * f14 + f6);
                f4 = f3 * f17 + f12;
                if f17 == 0.0 {
                    p20.add(2).write_unaligned(f4);
                    f10 = f3 * f16 + f10;
                    p20.add(4).write_unaligned(f3 * f15 + f8);
                } else {
                    p20.add(2).write_unaligned(f4);
                    f3 = DAT_00a081ec;
                    let m = (base
                        .wrapping_add(8)
                        .wrapping_add((i_var18 as u32).wrapping_mul(0x18)))
                        as *const f32;
                    f3 = (f3 / m.read_unaligned() - f3 / f12) / (f3 / f11 - f3 / f12);
                    f10 = f3 * f16 + f10;
                    p20.add(4).write_unaligned(f15 * f3 + f8);
                }
            }
            p20.add(5).write_unaligned(f10);
            p20.add(1).write_unaligned(0.0);
            let i_var19 = DAT_0054f998.wrapping_add(2);
            DAT_0054f998 = i_var19;
            *arr868.offset(i_var19 as isize) = i_var18;
            *arr86c.offset(i_var19 as isize) = param_2;
            return;
        }
    }
}
