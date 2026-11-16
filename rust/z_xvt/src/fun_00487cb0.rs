// SPDX-License-Identifier: MIT

fn range_bad(p: f32, q: f32, v0: f32, v1: f32, v2: f32, v3: f32, has4: bool) -> bool {
    if p < v0 || q < v0 {
        v0 < p || v0 < q || v1 < p || v1 < q || v2 < p || v2 < q || (has4 && (v3 < p || v3 < q))
    } else {
        p < v1 || q < v1 || p < v2 || q < v2 || (has4 && (p < v3 || q < v3))
    }
}

fn same(a: f32, b: f32) -> bool {
    if a.to_bits() < 0x80000001u32 {
        b.to_bits() < 0x80000001u32
    } else {
        0x80000000u32 < b.to_bits()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00487cb0(param_1: i32, param_2: *mut f32, param_3: *mut f32) -> u32 {
    // SAFETY: all pointers are the game's own structures, read where the original reads.
    unsafe {
        let rd_i = |a: u32| -> i32 { (a as *const i32).read_unaligned() };
        let base: u32 = rd_i((param_1 as u32).wrapping_add(0x7c)) as u32;
        let fl = |a: u32| -> f32 {
            // SAFETY: base points at the game's vertex array.
            unsafe { (base.wrapping_add(a) as *const f32).read_unaligned() }
        };
        let fi = |i: i32| -> f32 { fl((i as u32).wrapping_mul(4)) };
        let x12 = |v: i32| -> f32 { fl((v as u32).wrapping_mul(0xc)) };

        let f2 = param_2.read_unaligned();
        let f3 = param_2.add(1).read_unaligned();
        let f4 = param_2.add(2).read_unaligned();
        let mut local_4: i32 = 0;
        let f5 = param_3.read_unaligned();
        let f6 = param_3.add(1).read_unaligned();
        let f7 = param_3.add(2).read_unaligned();
        let mut local_14: *mut i32 = rd_i((param_1 as u32).wrapping_add(0xa0)) as u32 as *mut i32;
        let mut local_8: *mut f32 = rd_i((param_1 as u32).wrapping_add(0x94)) as u32 as *mut f32;
        let count_addr = (param_1 as u32).wrapping_add(0x8c);
        if 0 < rd_i(count_addr) {
            loop {
                'body: {
                    let mut i12 = local_14.add(3).read_unaligned();
                    let i8 = local_14.read_unaligned();
                    let mut i13 = i8.wrapping_mul(3);
                    let mut i14 = local_14.add(1).read_unaligned();
                    let mut i11 = i14.wrapping_mul(3);
                    let mut i16 = local_14.add(2).read_unaligned();
                    let mut i19 = i16.wrapping_mul(3);
                    let mut i15 = i12.wrapping_mul(3);
                    let mut i17: i32;
                    let mut i18: i32;

                    let v3 = if i15 != -3 { x12(i12) } else { 0.0 };
                    if !range_bad(f2, f5, x12(i8), x12(i14), x12(i16), v3, i15 != -3) {
                        break 'body;
                    }

                    i12 = i13.wrapping_add(1);
                    i16 = i11.wrapping_add(1);
                    i18 = i19.wrapping_add(1);
                    i14 = i15.wrapping_add(1);
                    let v3 = if i14 != -2 { fi(i14) } else { 0.0 };
                    if !range_bad(
                        f3,
                        f6,
                        fl(4u32.wrapping_add((i8 as u32).wrapping_mul(0xc))),
                        fi(i16),
                        fi(i18),
                        v3,
                        i14 != -2,
                    ) {
                        break 'body;
                    }

                    i18 = i13.wrapping_add(2);
                    i17 = i11.wrapping_add(2);
                    i14 = i19.wrapping_add(2);
                    i16 = i15.wrapping_add(2);
                    let v3 = if i16 != -1 { fi(i16) } else { 0.0 };
                    if !range_bad(
                        f4,
                        f7,
                        fl(4u32.wrapping_add((i12 as u32).wrapping_mul(4))),
                        fi(i17),
                        fi(i14),
                        v3,
                        i16 != -1,
                    ) {
                        break 'body;
                    }

                    let l0 = local_8.read_unaligned();
                    let l1 = local_8.add(1).read_unaligned();
                    let l2 = local_8.add(2).read_unaligned();

                    let mut f9 = l1
                        * (f3 - fl(0xfffffffcu32.wrapping_add((i18 as u32).wrapping_mul(4))))
                        + l2 * (f4 - fi(i18))
                        + (f2 - x12(i8)) * l0;
                    let mut f10 = (f5 - x12(i8)) * l0
                        + l2 * (f7 - fi(i18))
                        + l1 * (f6 - fl(0xfffffffcu32.wrapping_add((i18 as u32).wrapping_mul(4))));

                    let go = if f9.to_bits() < 0x80000001u32 {
                        (0x421fffffu32 < f9.to_bits()) && (0x80000000u32 < f10.to_bits())
                    } else {
                        (0xc21fffffu32 < f9.to_bits()) && (0 < (f10.to_bits() as i32))
                    };
                    if !go {
                        break 'body;
                    }

                    f10 = -f9 / f10;
                    let local_3c: i32;
                    let local_1c: f32;
                    let local_18: f32;
                    let local_10: i32;
                    let mut do_2d9 = false;
                    if (l2 <= l0) || (l2 <= l1) {
                        local_3c = i17;
                        if (l1 <= l0) || (l1 <= l2) {
                            local_1c = (f6 - f3) * f10 + f3;
                            local_18 = (f7 - f4) * f10 + f4;
                            local_10 = i13.wrapping_add(2);
                            i12 = -1;
                            i19 = i19.wrapping_add(1);
                            i15 = -1;
                            i13 = i13.wrapping_add(1);
                            do_2d9 = true;
                        } else {
                            local_1c = (f5 - f2) * f10 + f2;
                            local_18 = (f7 - f4) * f10 + f4;
                            local_10 = i13.wrapping_add(2);
                            i12 = -2;
                        }
                    } else {
                        local_1c = (f5 - f2) * f10 + f2;
                        local_18 = (f6 - f3) * f10 + f3;
                        i12 = -2;
                        i15 = -2;
                        i14 = i19.wrapping_add(1);
                        local_3c = i11.wrapping_add(1);
                        local_10 = i13.wrapping_add(1);
                        do_2d9 = true;
                    }
                    if do_2d9 {
                        i15 = i16.wrapping_add(i15);
                        i16 = i15.wrapping_add(1);
                    }
                    i17 = i17.wrapping_add(i12);
                    i11 = i11; // keep binding used
                    f9 = (fi(local_3c) - fi(local_10)) * (local_1c - fi(i13))
                        - (local_18 - fi(local_10)) * (fi(i17) - fi(i13));
                    f10 = (local_1c - fi(i17)) * (fi(i14) - fi(local_3c))
                        - (local_18 - fi(local_3c)) * (fi(i19) - fi(i17));
                    if !same(f9, f10) {
                        break 'body;
                    }
                    if i15 < 0 {
                        f10 = (local_1c - fi(i19)) * (fi(local_10) - fi(i14))
                            - (local_18 - fi(i14)) * (fi(i13) - fi(i19));
                        if same(f9, f10) {
                            return 1;
                        }
                    } else {
                        f10 = (local_1c - fi(i19)) * (fi(i16) - fi(i14))
                            - (local_18 - fi(i14)) * (fi(i15) - fi(i19));
                        if same(f9, f10) {
                            f10 = (local_1c - fi(i15)) * (fi(local_10) - fi(i16))
                                - (local_18 - fi(i16)) * (fi(i13) - fi(i15));
                            if same(f9, f10) {
                                return 1;
                            }
                        }
                    }
                }
                local_4 = local_4.wrapping_add(1);
                local_14 = local_14.add(0x10);
                local_8 = local_8.add(3);
                if !(local_4 < rd_i(count_addr)) {
                    break;
                }
            }
        }
        0
    }
}
