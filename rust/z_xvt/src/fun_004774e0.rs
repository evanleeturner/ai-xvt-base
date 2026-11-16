// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
    static mut _DAT_006da4fc: i32;
    static mut DAT_0070acc8: i32;
    static mut _DAT_006db7bc: i32;
    static mut DAT_005f1d44: i32;
    static mut DAT_006a888e: i32;
    static mut DAT_0052ae50: i32;
}

#[allow(non_snake_case)]
unsafe fn rf(addr: i32) -> f32 {
    // SAFETY: the address is a field of the game's own span record.
    unsafe { (addr as usize as *const f32).read_unaligned() }
}

#[allow(non_snake_case)]
unsafe fn copy_run(p1: i32, start: i32, mut n: i32, wide: bool) {
    // SAFETY: source and destination are the game's own buffers, copied as the original does.
    unsafe {
        if wide {
            let mut s = p1.wrapping_add(start.wrapping_mul(2)) as usize as *const u8;
            let mut d = DAT_0052ae50
                .wrapping_add(DAT_005f1d44)
                .wrapping_add(start.wrapping_mul(2)) as usize as *mut u8;
            loop {
                let b = s.add(1).read();
                d.write(s.read());
                d.add(1).write(b);
                s = s.add(2);
                d = d.add(2);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        } else {
            let mut s = p1.wrapping_add(start) as usize as *const u8;
            let mut d =
                DAT_0052ae50.wrapping_add(start).wrapping_add(DAT_005f1d44) as usize as *mut u8;
            loop {
                let b = s.read();
                s = s.add(1);
                d.write(b);
                d = d.add(1);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004774e0(
    mut param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    param_4: i32,
    param_5: f32,
) {
    // SAFETY: all globals, span records and pixel buffers belong to the game and
    // are accessed exactly as the original does.
    unsafe {
        let i_var8 = if DAT_00525518 == 2 {
            param_2.wrapping_mul(-2)
        } else {
            0i32.wrapping_sub(param_2)
        };
        param_1 = param_1.wrapping_add(i_var8);
        DAT_005f1d44 = _DAT_006da4fc
            .wrapping_add(param_4)
            .wrapping_mul(DAT_0070acc8)
            .wrapping_add(_DAT_006db7bc.wrapping_mul(DAT_00525518));
        let base = (&raw mut DAT_006a888e).read_unaligned();
        let mut p3: *const u32 = (base.wrapping_add(param_4.wrapping_mul(4)) as usize as *const u32)
            .read_unaligned() as usize as *const u32;

        'first: loop {
            if p3.is_null() {
                break;
            }
            let i8_ = p3.add(2).read() as i32;
            let mut i9 = param_2;
            'next: {
                if param_2 < i8_ {
                    if param_2 < p3.add(1).read() as i32 {
                        break 'first;
                    }
                    let i4 = p3.add(5).read() as i32;
                    i9 = i8_;
                    if rf(i4.wrapping_add(0x4c)) < param_5 {
                        i9 = param_2;
                        if param_5 < rf(i4.wrapping_add(0x48)) {
                            let slope = rf(i4.wrapping_add(0x28));
                            let f5 = (param_4 as f32) * rf(i4.wrapping_add(0x2c))
                                + rf(i4.wrapping_add(0x30))
                                + (param_2 as f32) * slope;
                            let u = slope.to_bits();
                            if param_5 <= f5 {
                                i9 = i8_;
                                if 0x80000000u32 < u {
                                    if i8_ < param_3 {
                                        if param_5
                                            <= (i8_.wrapping_sub(param_2) as f32) * slope + f5
                                        {
                                            break 'next;
                                        }
                                    } else if param_5
                                        <= (param_3.wrapping_sub(param_2) as f32) * slope + f5
                                    {
                                        return;
                                    }
                                    let d = ((param_5 - f5) / slope) as i64 as i32;
                                    i9 = param_2.wrapping_add(d);
                                }
                                if param_3 <= i9 {
                                    return;
                                }
                            } else if 0 < (u as i32) {
                                let f6 = if param_3 < i8_ {
                                    (param_3.wrapping_sub(param_2) as f32) * slope
                                } else {
                                    (i8_.wrapping_sub(param_2) as f32) * slope
                                };
                                if param_5 < f6 + f5 {
                                    let d = ((param_5 - f5) / slope) as i64 as i32;
                                    param_3 = param_2.wrapping_add(d);
                                    if param_3 <= param_2 {
                                        return;
                                    }
                                }
                            }
                        }
                    } else if param_3 <= i9 {
                        return;
                    }
                }
            }
            param_2 = i9;
            p3 = (p3.read() as usize) as *const u32;
        }

        loop {
            if p3.is_null() {
                let n = param_3.wrapping_sub(param_2);
                if 0 < n {
                    if DAT_00525518 == 2 {
                        DAT_00525518 = 2;
                        copy_run(param_1, param_2, n, true);
                        return;
                    }
                    copy_run(param_1, param_2, n, false);
                }
                return;
            }
            let x1 = p3.add(1).read() as i32;
            if param_3 <= x1 {
                let n = param_3.wrapping_sub(param_2);
                if 0 < n {
                    if DAT_00525518 == 2 {
                        DAT_00525518 = 2;
                        copy_run(param_1, param_2, n, true);
                        return;
                    }
                    copy_run(param_1, param_2, n, false);
                }
                return;
            }
            let i9 = p3.add(5).read() as i32;
            if param_5 <= rf(i9.wrapping_add(0x4c)) {
                let n = x1.wrapping_sub(param_2);
                if 0 < n {
                    copy_run(param_1, param_2, n, DAT_00525518 == 2);
                }
                param_2 = p3.add(2).read() as i32;
                if param_3 <= param_2 {
                    return;
                }
            } else if param_5 < rf(i9.wrapping_add(0x48)) {
                let slope = rf(i9.wrapping_add(0x28));
                let f5 = (param_4 as f32) * rf(i9.wrapping_add(0x2c))
                    + rf(i9.wrapping_add(0x30))
                    + (x1 as f32) * slope;
                if param_5 <= f5 {
                    let n = x1.wrapping_sub(param_2);
                    if 0 < n {
                        copy_run(param_1, param_2, n, DAT_00525518 == 2);
                    }
                    param_2 = p3.add(2).read() as i32;
                    let slope2 = rf((p3.add(5).read() as i32).wrapping_add(0x28));
                    let u = slope2.to_bits();
                    let mut skip_check = false;
                    if 0x80000000u32 < u {
                        if param_2 < param_3 {
                            let endz = (param_2.wrapping_sub(x1) as f32) * slope2 + f5;
                            if endz < param_5 {
                                let d = ((endz - param_5) / slope2) as i64 as i32;
                                param_2 = param_2.wrapping_sub(d);
                            }
                            skip_check = true;
                        } else {
                            let endz = (param_3.wrapping_sub(x1) as f32) * slope2 + f5;
                            if param_5 <= endz {
                                return;
                            }
                            let d = ((endz - param_5) / slope2) as i64 as i32;
                            param_2 = param_3.wrapping_sub(d);
                        }
                    }
                    if !skip_check && param_3 <= param_2 {
                        return;
                    }
                } else {
                    let u = slope.to_bits();
                    if 0 < (u as i32) {
                        let e = p3.add(2).read() as i32;
                        if e < param_3 {
                            let endz = (e.wrapping_sub(x1) as f32) * slope + f5;
                            if param_5 < endz {
                                let d = ((endz - param_5) / slope) as i64 as i32;
                                let n = e.wrapping_sub(d).wrapping_sub(param_2);
                                if 0 < n {
                                    copy_run(param_1, param_2, n, DAT_00525518 == 2);
                                }
                                param_2 = p3.add(2).read() as i32;
                            }
                        } else {
                            let endz = (param_3.wrapping_sub(x1) as f32) * slope + f5;
                            if param_5 < endz {
                                let d = ((param_5 - f5) / slope) as i64 as i32;
                                let n = (p3.add(1).read() as i32)
                                    .wrapping_add(d)
                                    .wrapping_sub(param_2);
                                if n < 1 {
                                    return;
                                }
                                copy_run(param_1, param_2, n, DAT_00525518 == 2);
                                return;
                            }
                        }
                    }
                }
            }
            p3 = (p3.read() as usize) as *const u32;
        }
    }
}
