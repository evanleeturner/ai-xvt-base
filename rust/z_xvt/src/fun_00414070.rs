// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae18: i32;
    static mut DAT_00525500: i32;
    static mut DAT_005254fc: i32;
    static mut DAT_00525730: i32;
    static mut _DAT_0052005c: f32;
    static mut _DAT_00520060: f32;
    static mut _DAT_00520064: f32;
    static mut _DAT_0052006c: f32;
    static mut _DAT_00520070: f32;
    static mut _DAT_00520078: f64;
    static mut _DAT_00520080: f32;
    static mut _DAT_00520084: f32;
    static mut _DAT_00520088: f32;
    static mut _DAT_00732ea0: f32;
    static mut DAT_00732f30: f32;
    static mut _DAT_00732f34: f32;
    static mut DAT_0072f460: [f32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00487680(param_1: i32, param_2: *mut f32, param_3: *mut f32) -> i32;
}

fn ab(x: f32) -> f32 {
    if x.to_bits() > 0x8000_0000 { -x } else { x }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00414070(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut f32,
    param_4: *mut f32,
    param_5: *mut f32,
) {
    // SAFETY: all pointers and globals belong to the game; accessed as the original does.
    unsafe {
        let p_out = param_2.wrapping_add(0xc) as *mut f32;
        let mut v: [f32; 3] = [0.0; 3];

        if DAT_0052ae18 != 0 {
            let c = ((param_1.read_unaligned()).wrapping_add(2) as *const i8).read();
            if c == 7 || c == 6 {
                p_out.write_unaligned(f32::from_bits(0x3f800000));
                return;
            }
        }
        if DAT_00525500 == 0 {
            p_out.write_unaligned(f32::from_bits(0x3ecccccd));
        } else {
            let r = (_DAT_00732ea0 * _DAT_0052005c * param_3.add(2).read_unaligned()
                + DAT_00732f30 * _DAT_0052005c * param_3.read_unaligned()
                + _DAT_00732f34 * _DAT_0052005c * param_3.add(1).read_unaligned())
                * _DAT_00520060;
            p_out.write_unaligned(r);
            if r.to_bits() < 0x80000001 {
                v[2] = _DAT_00732ea0 + param_4.add(2).read_unaligned();
                v[0] = DAT_00732f30 + param_4.read_unaligned();
                v[1] = _DAT_00732f34 + param_4.add(1).read_unaligned();
                let r6 = FUN_00487680(param_1.read_unaligned(), param_4, v.as_mut_ptr());
                if r6 != 0 {
                    p_out.write_unaligned(f32::from_bits(0));
                }
            } else {
                p_out.write_unaligned(f32::from_bits(0));
            }
        }
        let mut i: i32 = 0;
        if DAT_00525730 < 1 {
            return;
        }
        let mut p7: *mut f32 = (&raw mut DAT_0072f460) as *mut f32;
        loop {
            'next: {
                v[0] = p7.read_unaligned();
                v[1] = p7.add(1).read_unaligned();
                v[2] = p7.add(2).read_unaligned();
                let mut f1 = v[0] - param_4.read_unaligned();
                let mut f2 = v[1] - param_4.add(1).read_unaligned();
                let mut f3 = v[2] - param_4.add(2).read_unaligned();
                let mut f14 = param_3.add(2).read_unaligned() * f3
                    + param_3.add(1).read_unaligned() * f2
                    + param_3.read_unaligned() * f1;
                if (DAT_0052ae18 != 0 || f14.to_bits() < 0x80000001)
                    && FUN_00487680(param_1.read_unaligned(), param_4, v.as_mut_ptr()) == 0
                {
                    let mut f24 = ab(f1);
                    let mut f28 = ab(f2);
                    let f2c = ab(f3);
                    let f4v: f32;
                    if f24 < f28 || f24 < f2c {
                        if f28 < f24 || f28 < f2c {
                            f4v = (f28 + f24) * _DAT_0052006c + f2c;
                        } else {
                            f4v = (f2c + f24) * _DAT_0052006c + f28;
                        }
                    } else {
                        f4v = (f2c + f28) * _DAT_0052006c + f24;
                    }
                    if DAT_0052ae18 != 0 {
                        if f14 / f4v < _DAT_00520070 {
                            break 'next;
                        }
                        f14 = f4v * (_DAT_00520078 as f32);
                    }
                    let mut f30: f32;
                    if DAT_005254fc == 0 {
                        f30 = 0.0;
                    } else {
                        let mut a = (param_5.read_unaligned() - param_4.read_unaligned()) + f1;
                        let mut b = (param_5.add(1).read_unaligned()
                            - param_4.add(1).read_unaligned())
                            + f2;
                        let mut c = (param_5.add(2).read_unaligned()
                            - param_4.add(2).read_unaligned())
                            + f3;
                        f1 = param_3.read_unaligned() * a;
                        f3 = param_3.add(1).read_unaligned() * b;
                        f2 = param_3.add(2).read_unaligned() * c;
                        a = ab(a);
                        b = ab(b);
                        c = ab(c);
                        let f5: f32;
                        if a < b || a < c {
                            if b < a || b < c {
                                f5 = (b + a) * _DAT_00520084;
                                a = c;
                            } else {
                                f5 = (c + a) * _DAT_00520084;
                                a = b;
                            }
                        } else {
                            f5 = (c + b) * _DAT_00520084;
                        }
                        f30 = ((f2 + f1 + f3) * _DAT_00520080) / (a * _DAT_00520088 + f5);
                        if f30 < _DAT_00520080 {
                            f30 = 0.0;
                        } else {
                            f30 = f30 * f30 * f30;
                            f30 = f30 * f30;
                            f30 = f30 * f30;
                            f30 = f30 * f30;
                            f30 = f30 * f30;
                        }
                    }
                    f30 = f14 / (f4v * f4v) + f30;
                    if _DAT_00520064 < f30 {
                        let nv = p7.add(3).read_unaligned() * f30 + p_out.read_unaligned();
                        p_out.write_unaligned(nv);
                        if 0x3f7fffff < (nv.to_bits() as i32) {
                            p_out.write_unaligned(f32::from_bits(0x3f800000));
                            return;
                        }
                    }
                }
            }
            p7 = p7.add(4);
            i = i.wrapping_add(1);
            if !(i < DAT_00525730) {
                break;
            }
        }
    }
}
