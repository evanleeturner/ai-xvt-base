// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00473ad0(param_1: i32, param_2: *mut f32, param_3: *mut f32) -> u32 {
    // SAFETY: all pointers come from the caller's mesh structure and are read
    // at the same offsets and in the same order as the original.
    unsafe {
        let rd = |off: i32| -> i32 { (param_1.wrapping_add(off) as *const i32).read_unaligned() };
        let i_var1: i32 = rd(0x7c);
        let f2 = param_2.read_unaligned();
        let f3 = param_2.add(1).read_unaligned();
        let f4 = param_2.add(2).read_unaligned();
        let mut local_4: i32 = 0;
        let f5 = param_3.read_unaligned();
        let f6 = param_3.add(1).read_unaligned();
        let f7 = param_3.add(2).read_unaligned();
        let mut local_14: *const i32 = rd(0xa0) as *const i32;
        let mut local_8: *const f32 = rd(0x94) as *const f32;

        let f = |i: i32| -> f32 {
            ((i_var1 as u32).wrapping_add((i as u32).wrapping_mul(4)) as *const f32)
                .read_unaligned()
        };
        let chk = |a: f32, b: f32, p: i32, q: i32, r: i32, s: i32, sent: i32| -> bool {
            if a < f(p) || b < f(p) {
                f(p) < a
                    || f(p) < b
                    || f(q) < a
                    || f(q) < b
                    || f(r) < a
                    || f(r) < b
                    || (s != sent && (f(s) < a || f(s) < b))
            } else {
                a < f(q)
                    || b < f(q)
                    || a < f(r)
                    || b < f(r)
                    || (s != sent && (a < f(s) || b < f(s)))
            }
        };
        let same = |a: f32, b: f32| -> bool {
            let ua = a.to_bits();
            let ub = b.to_bits();
            if ua < 0x80000001 {
                ub < 0x80000001
            } else {
                0x80000000 < ub
            }
        };

        if 0 < rd(0x8c) {
            loop {
                'next: {
                    let i12 = local_14.add(3).read_unaligned();
                    let i8 = local_14.read_unaligned();
                    let mut i13 = i8.wrapping_mul(3);
                    let i14 = local_14.add(1).read_unaligned();
                    let i11 = i14.wrapping_mul(3);
                    let i16 = local_14.add(2).read_unaligned();
                    let mut i19 = i16.wrapping_mul(3);
                    let mut i15 = i12.wrapping_mul(3);

                    if !chk(f2, f5, i13, i11, i19, i15, -3) {
                        break 'next;
                    }
                    let i12 = i13.wrapping_add(1);
                    let i16 = i11.wrapping_add(1);
                    let i18 = i19.wrapping_add(1);
                    let i14 = i15.wrapping_add(1);
                    if !chk(f3, f6, i12, i16, i18, i14, -2) {
                        break 'next;
                    }
                    let mut i18 = i13.wrapping_add(2);
                    let mut i17 = i11.wrapping_add(2);
                    let mut i14 = i19.wrapping_add(2);
                    let mut i16 = i15.wrapping_add(2);
                    if !chk(f4, f7, i18, i17, i14, i16, -1) {
                        break 'next;
                    }

                    let l80 = local_8.read_unaligned();
                    let l81 = local_8.add(1).read_unaligned();
                    let l82 = local_8.add(2).read_unaligned();
                    let mut f9 = l81 * (f3 - f(i18.wrapping_add(-1)))
                        + l82 * (f4 - f(i18))
                        + (f2 - f(i13)) * l80;
                    let mut f10 = (f5 - f(i13)) * l80
                        + l82 * (f7 - f(i18))
                        + l81 * (f6 - f(i18.wrapping_add(-1)));
                    let u9 = f9.to_bits();
                    let u10 = f10.to_bits();
                    let go = if u9 < 0x80000001 {
                        (0x421fffff < (u9 as i32)) && (0x80000000 < u10)
                    } else {
                        (0xc21fffffu32 < u9) && (0 < (u10 as i32))
                    };
                    if !go {
                        break 'next;
                    }

                    f10 = -f9 / f10;
                    let local_3c: i32;
                    let local_1c: f32;
                    let local_18: f32;
                    let local_10: i32;
                    let mut i12: i32;
                    let goto_f9: bool;
                    if (l82 <= l80) || (l82 <= l81) {
                        local_3c = i17;
                        if (l81 <= l80) || (l81 <= l82) {
                            local_1c = (f6 - f3) * f10 + f3;
                            local_18 = (f7 - f4) * f10 + f4;
                            local_10 = i13.wrapping_add(2);
                            i12 = -1;
                            i19 = i19.wrapping_add(1);
                            i15 = -1;
                            i13 = i13.wrapping_add(1);
                            goto_f9 = true;
                        } else {
                            local_1c = (f5 - f2) * f10 + f2;
                            local_18 = (f7 - f4) * f10 + f4;
                            local_10 = i13.wrapping_add(2);
                            i12 = -2;
                            goto_f9 = false;
                        }
                    } else {
                        local_1c = (f5 - f2) * f10 + f2;
                        local_18 = (f6 - f3) * f10 + f3;
                        i12 = -2;
                        i15 = -2;
                        i14 = i19.wrapping_add(1);
                        local_3c = i11.wrapping_add(1);
                        local_10 = i13.wrapping_add(1);
                        goto_f9 = true;
                    }
                    if goto_f9 {
                        i15 = i16.wrapping_add(i15);
                        i16 = i15.wrapping_add(1);
                    }
                    i17 = i17.wrapping_add(i12);
                    let _ = &mut i18;
                    let _ = &mut i12;
                    f9 = (f(local_3c) - f(local_10)) * (local_1c - f(i13))
                        - (local_18 - f(local_10)) * (f(i17) - f(i13));
                    f10 = (f(i14) - f(local_3c)) * (local_1c - f(i17))
                        - (local_18 - f(local_3c)) * (f(i19) - f(i17));
                    if !same(f9, f10) {
                        break 'next;
                    }

                    f10 = local_1c - f(i19);
                    if i15 < 0 {
                        f10 =
                            (f(local_10) - f(i14)) * f10 - (local_18 - f(i14)) * (f(i13) - f(i19));
                        if same(f9, f10) {
                            return 1;
                        }
                    } else {
                        f10 = (f(i16) - f(i14)) * f10 - (local_18 - f(i14)) * (f(i15) - f(i19));
                        if same(f9, f10) {
                            f10 = (f(local_10) - f(i16)) * (local_1c - f(i15))
                                - (local_18 - f(i16)) * (f(i13) - f(i15));
                            if same(f9, f10) {
                                return 1;
                            }
                        }
                    }
                }
                local_4 = local_4.wrapping_add(1);
                local_14 = local_14.add(0x10);
                local_8 = local_8.add(3);
                if !(local_4 < rd(0x8c)) {
                    break;
                }
            }
        }
        0
    }
}
