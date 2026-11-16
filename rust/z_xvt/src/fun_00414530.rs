// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c2c: i32;
    static mut DAT_00523c30: i32;
    static mut DAT_00523c34: *mut i32;
    static mut DAT_005254fc: i32;
    static mut DAT_00525730: i32;
    static mut DAT_00536930: [f32; 1];
    static mut DAT_00536960: [f32; 1];
    static mut _DAT_00536950: f32;
    static mut _DAT_00536954: f32;
    static mut _DAT_00536958: f32;
    static mut DAT_005369c0: f32;
    static mut DAT_005369c4: f32;
    static mut DAT_005369c8: f32;
    static mut DAT_0072f460: [f32; 1];
    static mut _DAT_00520058: f32;
    static mut _DAT_0052005c: f32;
    static mut _DAT_00520064: f32;
    static mut _DAT_0052006c: f32;
    static mut _DAT_00520080: f32;
    static mut _DAT_00520084: f32;
    static mut _DAT_00520088: f32;
    static mut _DAT_0052008c: f32;
    static mut _DAT_00520090: f32;
    static mut _DAT_00520094: f32;
    static mut _DAT_006a8a08: f32;
    static mut DAT_006d4984: u16;
    static mut DAT_007091ae: u16;
    static mut DAT_00709ffc: i32;
    static mut _DAT_00732f34: f32;
    static mut _DAT_00732ea0: f32;
    static mut DAT_00732f30: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045b420(param_1: i32);
}

fn abs_bits(x: f32) -> f32 {
    if x.to_bits() > 0x8000_0000u32 { -x } else { x }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00414530(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: f32,
) -> f64 {
    // SAFETY: all globals and the pointers reachable from param_1 belong to the
    // game; they are read and written exactly where the original does.
    unsafe {
        let pi7 = param_1.add(1).read() as u32 as usize as *mut i32;
        if pi7.is_null() {
            return _DAT_00520064 as f64;
        }
        let pf7 = pi7 as *mut f32;
        let m = |i: usize| -> f32 { pf7.add(i).read() };
        if DAT_00523c2c != pi7.read() {
            FUN_0045b420(pi7.read());
            let mut iv13: i32 = 0;
            DAT_00523c30 = DAT_00525730;
            DAT_00523c2c = pi7.read();
            if 0 < DAT_00525730 {
                let mut p14 = &raw mut DAT_00536930 as *mut f32;
                let mut p12 = &raw mut DAT_0072f460 as *mut f32;
                let mut p15 = &raw mut DAT_00536960 as *mut f32;
                loop {
                    iv13 = iv13.wrapping_add(1);
                    p15.write(p12.read());
                    p15.add(1).write(p12.add(1).read());
                    p15.add(2).write(p12.add(2).read());
                    let f1 = p15.read();
                    let f2 = p15.add(1).read();
                    let f3 = p15.add(2).read();
                    p15.write(m(5) * f1 + m(8) * f2 + m(0xb) * f3);
                    p15.add(1).write(m(6) * f1 + m(9) * f2 + m(0xc) * f3);
                    p15.add(2).write(f3 * m(0xd) + m(10) * f2 + m(7) * f1);
                    p15.write(p15.read() + m(2));
                    p15.add(1).write(p15.add(1).read() + m(3));
                    p15.add(2).write(p15.add(2).read() + m(4));
                    if DAT_00525730 <= iv13 {
                        break;
                    }
                    p14.write(p12.add(3).read());
                    p14 = p14.add(1);
                    p12 = p12.add(4);
                    p15 = p15.add(3);
                }
                p14.write(p12.add(3).read());
            }
            let f1 = _DAT_00732f34 * _DAT_0052005c;
            let f2 = _DAT_00732ea0 * _DAT_0052005c;
            let f3 = DAT_00732f30 * _DAT_0052005c;
            _DAT_00536950 = m(5) * f3 + m(0xb) * f2 + m(8) * f1;
            _DAT_00536954 = m(0xc) * f2 + m(6) * f3 + m(9) * f1;
            _DAT_00536958 = m(10) * f1 + f3 * m(7) + m(0xd) * f2;
        }
        let fv1 = _DAT_00520058 / param_4;
        let fv2 = (param_2.wrapping_sub((DAT_007091ae >> 1) as i32) as f32) * fv1 * _DAT_006a8a08;
        let fv3 = (param_3
            .wrapping_sub((DAT_006d4984 >> 1) as i32)
            .wrapping_sub(DAT_00709ffc) as f32)
            * fv1
            * _DAT_006a8a08;
        if DAT_00523c34 != param_1 {
            DAT_00523c34 = param_1;
            let table = pi7.add(0x25).read() as u32 as usize as *mut f32;
            let i8v = param_1.read();
            let e = table.byte_add((i8v as u32).wrapping_mul(12) as usize);
            let f4 = e.read();
            let f5 = e.add(1).read();
            let f6 = e.add(2).read();
            DAT_005369c0 = m(5) * f4 + m(0xb) * f6 + m(8) * f5;
            DAT_005369c4 = m(6) * f4 + m(0xc) * f6 + m(9) * f5;
            DAT_005369c8 = f6 * m(0xd) + m(10) * f5 + m(7) * f4;
        }
        let mut local_4: f32 = 0.0;
        if DAT_005254fc != 0 {
            let f4 = -fv2;
            let f5 = -fv3;
            let f6 = -fv1;
            let mut l3c = abs_bits(f4);
            let l40 = abs_bits(f5);
            let l44 = abs_bits(f6);
            let f9: f32;
            if (l3c < l40) || (l3c < l44) {
                if (l40 < l3c) || (l40 < l44) {
                    f9 = (l40 + l3c) * _DAT_0052008c;
                    l3c = l44;
                } else {
                    f9 = (l44 + l3c) * _DAT_0052008c;
                    l3c = l40;
                }
            } else {
                f9 = (l44 + l40) * _DAT_0052008c;
            }
            let f9 = _DAT_00520058 / (l3c * _DAT_00520090 + f9);
            let mut l38 = (_DAT_00536958 + f9 * f6) * DAT_005369c8
                + (_DAT_00536954 + f5 * f9) * DAT_005369c4
                + (_DAT_00536950 + f4 * f9) * DAT_005369c0;
            if l38 <= _DAT_00520064 {
                l38 = 0.0;
            } else {
                l38 = l38 * _DAT_00520080;
                l38 = l38 * l38 * l38;
                l38 = l38 * l38;
                l38 = l38 * l38;
                l38 = l38 * l38;
                l38 = l38 * l38;
            }
            l38 = l38 * _DAT_00520094;
            if _DAT_00520064 < l38 {
                local_4 = l38;
                if (l38.to_bits() as i32) > 0x3f7fffff {
                    return _DAT_00520058 as f64;
                }
            }
        }
        let mut p14 = &raw mut DAT_00536960 as *mut f32;
        let mut iv13: i32 = 0;
        if 0 < DAT_00523c30 {
            let mut p15 = &raw mut DAT_00536930 as *mut f32;
            loop {
                let mut f4 = p14.read() - fv2;
                let mut f5 = p14.add(1).read() - fv3;
                let mut f6 = p14.add(2).read() - fv1;
                let f9 = f6 * DAT_005369c8 + f5 * DAT_005369c4 + f4 * DAT_005369c0;
                if _DAT_00520064 < f9 {
                    let mut l3c = abs_bits(f4);
                    let l40 = abs_bits(f5);
                    let l44 = abs_bits(f6);
                    let f10: f32;
                    if (l3c < l40) || (l3c < l44) {
                        if (l40 < l3c) || (l40 < l44) {
                            f10 = (l40 + l3c) * _DAT_0052006c + l44;
                        } else {
                            f10 = (l44 + l3c) * _DAT_0052006c + l40;
                        }
                    } else {
                        f10 = (l44 + l40) * _DAT_0052006c + l3c;
                    }
                    let mut l38: f32;
                    'a: {
                        if DAT_005254fc == 0 {
                            l38 = 0.0;
                            break 'a;
                        }
                        l3c = f4 - fv2;
                        let mut l40 = f5 - fv3;
                        let mut l44 = f6 - fv1;
                        f4 = l3c * DAT_005369c0;
                        f6 = l40 * DAT_005369c4;
                        f5 = l44 * DAT_005369c8;
                        l3c = abs_bits(l3c);
                        l40 = abs_bits(l40);
                        l44 = abs_bits(l44);
                        let f11: f32;
                        if (l3c < l40) || (l3c < l44) {
                            if (l40 < l3c) || (l40 < l44) {
                                f11 = (l40 + l3c) * _DAT_00520084;
                                l3c = l44;
                            } else {
                                f11 = (l44 + l3c) * _DAT_00520084;
                                l3c = l40;
                            }
                        } else {
                            f11 = (l44 + l40) * _DAT_00520084;
                        }
                        l38 = _DAT_00520058 / (l3c * _DAT_00520088 + f11);
                        f4 = l38 * (f5 + f6 + f4) * _DAT_00520080;
                        if f4 < _DAT_00520064 {
                            l38 = 0.0;
                            break 'a;
                        }
                        f4 = f4 * f4 * f4;
                        f4 = f4 * f4;
                        f4 = f4 * f4;
                        f4 = f4 * f4;
                        l38 = f4 * f4 * l38;
                    }
                    l38 = l38 + f9 / (f10 * f10);
                    if _DAT_00520064 < l38 {
                        local_4 = p15.read() * l38 + local_4;
                        if _DAT_00520058 <= local_4 {
                            local_4 = 1.0;
                            break;
                        }
                    }
                }
                p15 = p15.add(1);
                iv13 = iv13.wrapping_add(1);
                p14 = p14.add(3);
                if !(iv13 < DAT_00523c30) {
                    break;
                }
            }
        }
        local_4 as f64
    }
}
