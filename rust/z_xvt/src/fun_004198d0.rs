// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a348: i32;
    static mut DAT_00524774: *mut i32;
    static mut _DAT_00536a20: f32;
    static mut _DAT_00536a24: f32;
    static mut _DAT_00536a28: f32;
    static mut _DAT_00536a30: f32;
    static mut _DAT_00536a34: f32;
    static mut _DAT_00536a38: f32;
    static mut DAT_00536a3c: u32;
    static mut DAT_00536a5c: u32;
    static mut DAT_00536a60: u32;
    static mut _DAT_00536a64: f32;
    static mut _DAT_005200b8: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048e480(param_1: i32, param_2: *mut i8) -> *mut u32;
    fn FUN_00419da0(a: *mut f32, b: *mut f32, c: *mut f32, d: *mut f32, e: *mut f32) -> i32;
    fn FUN_00419f10(a: *mut f32, b: i32, c: *mut i32, d: *mut f32) -> i32;
    fn FUN_00415300(a: *mut f32, b: *mut f32);
    fn FUN_00414fe0(a: *mut f32, b: *mut f32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004198d0(param_1: i32, param_2: *mut u32) -> u32 {
    let mut param_2 = param_2;
    // SAFETY: all pointers are the game's own scene-graph data and globals,
    // read and written exactly where the original does.
    unsafe {
        loop {
            if param_2.is_null() {
                return 0;
            }
            let mut local_64: i32 = 0;
            while param_2.add(1).read_unaligned() == 7 {
                if DAT_0052a348 == 0 {
                    param_2 = FUN_0048e480(param_1, param_2.add(5).read_unaligned() as *mut i8);
                } else if *(param_2.add(5).read_unaligned() as *const i8) == 0 {
                    param_2 = param_2.read_unaligned() as *mut u32;
                } else {
                    let p = FUN_0048e480(param_1, param_2.add(5).read_unaligned() as *mut i8);
                    param_2.write_unaligned(p as u32);
                    *(param_2.add(5).read_unaligned() as *mut u8) = 0;
                    param_2 = param_2.read_unaligned() as *mut u32;
                }
                if param_2.is_null() {
                    return 0;
                }
            }
            match param_2.add(1).read_unaligned() {
                1 | 0xf | 0x10 | 0x11 => {
                    let mut local_68: f32 = 0.0;
                    let mut local_5c: [f32; 3] = [0.0; 3];
                    let mut p5: u32 = param_2.add(5).read_unaligned().wrapping_add(4);
                    let mut local_60: i32 = 0;
                    let mut pf3: u32 = p5.wrapping_add(
                        param_2
                            .add(4)
                            .read_unaligned()
                            .wrapping_mul(0x10)
                            .wrapping_mul(4),
                    );
                    let i_var6: i32 = DAT_00524774.add(5).read_unaligned();
                    if 0 < param_2.add(4).read_unaligned() as i32 {
                        loop {
                            let idx = (p5 as *const i32).read_unaligned() as u32;
                            let i_var4 = FUN_00419da0(
                                pf3 as *mut f32,
                                (i_var6 as u32).wrapping_add(idx.wrapping_mul(0xc)) as *mut f32,
                                &raw mut _DAT_00536a20,
                                &raw mut _DAT_00536a30,
                                &raw mut local_68,
                            );
                            if i_var4 != 0 && local_68 < _DAT_00536a64 {
                                local_5c[0] =
                                    (_DAT_00536a30 - _DAT_00536a20) * local_68 + _DAT_00536a20;
                                local_5c[1] =
                                    (_DAT_00536a34 - _DAT_00536a24) * local_68 + _DAT_00536a24;
                                local_5c[2] =
                                    (_DAT_00536a38 - _DAT_00536a28) * local_68 + _DAT_00536a28;
                                let r = FUN_00419f10(
                                    pf3 as *mut f32,
                                    i_var6,
                                    p5 as *mut i32,
                                    local_5c.as_mut_ptr(),
                                );
                                if r != 0 {
                                    _DAT_00536a64 = local_68;
                                    DAT_00536a3c = DAT_00536a5c;
                                }
                            }
                            pf3 = pf3.wrapping_add(12);
                            p5 = p5.wrapping_add(0x40);
                            local_60 = local_60.wrapping_add(1);
                            if !(local_60 < param_2.add(4).read_unaligned() as i32) {
                                break;
                            }
                        }
                    }
                }
                3 => {
                    DAT_00524774 = param_2 as *mut i32;
                    let pf = param_2
                        .add(5)
                        .read_unaligned()
                        .wrapping_sub(0x18)
                        .wrapping_add(param_2.add(4).read_unaligned().wrapping_mul(0xc))
                        as *const f32;
                    let f = |i: usize| pf.add(i).read_unaligned();
                    if _DAT_00536a20 < f(0) && _DAT_00536a30 < f(0) {
                        return 1;
                    }
                    if _DAT_00536a24 < f(1) && _DAT_00536a34 < f(1) {
                        return 1;
                    }
                    if _DAT_00536a28 < f(2) && _DAT_00536a38 < f(2) {
                        return 1;
                    }
                    if f(3) < _DAT_00536a20 && f(3) < _DAT_00536a30 {
                        return 1;
                    }
                    if f(4) < _DAT_00536a24 && f(4) < _DAT_00536a34 {
                        return 1;
                    }
                    if f(5) < _DAT_00536a28 && f(5) < _DAT_00536a38 {
                        return 1;
                    }
                }
                0x15 => {
                    local_64 = 1;
                }
                0x17 => {
                    if (DAT_00536a60 & 0x7fffffff) != 0 {
                        let pf = param_2.add(5).read_unaligned() as *const f32;
                        let f = |i: usize| pf.add(i).read_unaligned();
                        let mut local_50: [f32; 4] = [0.0; 4];
                        let mut local_40: [f32; 16] = [0.0; 16];
                        _DAT_00536a20 = _DAT_00536a20 - f(0);
                        _DAT_00536a24 = _DAT_00536a24 - f(1);
                        _DAT_00536a28 = _DAT_00536a28 - f(2);
                        _DAT_00536a30 = _DAT_00536a30 - f(0);
                        _DAT_00536a34 = _DAT_00536a34 - f(1);
                        _DAT_00536a38 = _DAT_00536a38 - f(2);
                        local_50[0] = f(3) * _DAT_005200b8;
                        local_50[1] = f(4) * _DAT_005200b8;
                        local_50[2] = f(5) * _DAT_005200b8;
                        local_50[3] = f32::from_bits(DAT_00536a60);
                        FUN_00415300(local_40.as_mut_ptr(), local_50.as_mut_ptr());
                        FUN_00414fe0(&raw mut _DAT_00536a20, local_40.as_mut_ptr());
                        FUN_00414fe0(&raw mut _DAT_00536a30, local_40.as_mut_ptr());
                        _DAT_00536a20 = f(0) + _DAT_00536a20;
                        _DAT_00536a24 = f(1) + _DAT_00536a24;
                        _DAT_00536a28 = f(2) + _DAT_00536a28;
                        _DAT_00536a30 = f(0) + _DAT_00536a30;
                        _DAT_00536a34 = f(1) + _DAT_00536a34;
                        _DAT_00536a38 = f(2) + _DAT_00536a38;
                    }
                }
                _ => {}
            }
            if param_2.add(2).read_unaligned() == 0 {
                return 0;
            }
            if local_64 == 0 {
                let mut i_var6: i32 = 0;
                if 0 < param_2.add(2).read_unaligned() as i32 {
                    let mut i_var4: u32 = 0;
                    loop {
                        let child = ((param_2.add(3).read_unaligned()).wrapping_add(i_var4)
                            as *const u32)
                            .read_unaligned() as *mut u32;
                        let i_var2 = FUN_004198d0(param_1, child);
                        if i_var2 != 0 {
                            return 1;
                        }
                        i_var4 = i_var4.wrapping_add(4);
                        i_var6 = i_var6.wrapping_add(1);
                        if !(i_var6 < param_2.add(2).read_unaligned() as i32) {
                            break;
                        }
                    }
                }
                return 0;
            }
            if local_64 == -1 {
                return 0;
            }
            param_2 = ((param_2
                .add(3)
                .read_unaligned()
                .wrapping_sub(4)
                .wrapping_add((local_64 as u32).wrapping_mul(4)))
                as *const u32)
                .read_unaligned() as *mut u32;
        }
    }
}
