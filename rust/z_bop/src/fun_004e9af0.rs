// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ad74: i8;
    static mut DAT_0052c4f0: i8;
    static mut DAT_0052c4fc: i8;
    static mut DAT_0052c4d8: i8;
    static mut DAT_0052c4e4: i8;
    static mut DAT_0052c4c0: i8;
    static mut DAT_0052c4cc: i8;
    static mut DAT_0052c4a8: i8;
    static mut DAT_0052c4b4: i8;
    static mut DAT_0052c490: i8;
    static mut DAT_0052c49c: i8;

    static mut DAT_00665d2c: i32;
    static mut DAT_00665d68: i32;
    static mut DAT_00665d6c: i32;
    static mut DAT_00665d28: i32;
    static mut DAT_00665d64: f32;
    static mut DAT_00665d74: f32;
    static mut DAT_00665d38: [u32; 11];
    static mut DAT_00aa60f4: i32;
    static mut DAT_00aa60fc: i32;
    static mut _DAT_005182ec: f32;
    static mut _DAT_005182f0: f64;
    static mut _DAT_00518300: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dc1e0() -> u8;
    fn FUN_004dc200() -> u8;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004daba0(param_1: *mut i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004da780(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
    fn FUN_0042b010();
    fn FUN_00426a00(param_1: *mut u32);
    fn FUN_00429e70(param_1: *mut i8);
    fn FUN_0042b0e0(param_1: i32, param_2: i32, param_3: i32);
}

#[allow(non_snake_case)]
unsafe fn any_key() -> bool {
    // SAFETY: the callees are the game's own.
    unsafe {
        FUN_004dc1e0() != 0 || FUN_004dc200() != 0 || FUN_004dc220() != 0 || FUN_004dc240() != 0
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e9af0() -> u32 {
    let mut local_38: i32 = 0;
    let mut local_34: i32 = 0;
    let mut local_30: [i32; 4] = [0; 4];
    let mut local_20: [i32; 8] = [1, 1, 1, 0, 0, 1, 1, 0];
    // SAFETY: callees and globals are the game's own; all pointers are to
    // locals or game memory read and written where the original does.
    unsafe {
        let l30 = local_30.as_mut_ptr();
        let l20 = local_20.as_mut_ptr();
        FUN_004de090(&raw mut local_34, &raw mut local_38);
        FUN_004d6460(l30, 0x16, 0x72, 0x2a, 0x8a);
        FUN_004de090(&raw mut local_34, &raw mut local_38);
        let mut i: usize = 0;
        loop {
            if local_20[i] != 0 {
                if FUN_004d6b90(l30, local_34, local_38) != 0 && any_key() {
                    local_20[i] = 2;
                }
            }
            i += 1;
            FUN_004d64a0(l30, 0, 0x1c);
            if i >= 3 {
                break;
            }
        }
        FUN_004d6460(l30, 0x16, 0x132, 0x2a, 0x14a);
        i = 5;
        loop {
            if local_20[i] != 0 {
                if FUN_004d6b90(l30, local_34, local_38) != 0 && any_key() {
                    local_20[i] = 2;
                }
            }
            i += 1;
            FUN_004d64a0(l30, 0, 0x1c);
            if i >= 7 {
                break;
            }
        }
        FUN_004daba0(l20);
        FUN_004d6460(l30, 0x16, 0xaa, 0x2a, 0xc2);
        let snd = &raw mut DAT_0052ad74;
        let mut pc = FUN_004ddc10(0x126);
        let mut r = FUN_004da780(
            l30,
            &raw mut DAT_0052c4f0,
            &raw mut DAT_0052c4fc,
            pc,
            0xc,
            0,
            0xd,
            snd,
        );
        if r != 0 {
            DAT_00665d2c = DAT_00665d2c.wrapping_sub(1);
            if DAT_00665d2c == -2 {
                DAT_00665d68 = DAT_00665d68.wrapping_sub(1);
                DAT_00665d2c = 1;
                if DAT_00665d68 == -2 {
                    DAT_00665d68 = 1;
                    DAT_00665d6c = DAT_00665d6c.wrapping_sub(1);
                    if DAT_00665d6c == -2 {
                        DAT_00665d6c = 1;
                    }
                }
            }
            FUN_0042b010();
        }
        FUN_004d64a0(l30, 0, -0x1c);
        pc = FUN_004ddc10(0x124);
        FUN_004da780(
            l30,
            &raw mut DAT_0052c4d8,
            &raw mut DAT_0052c4e4,
            pc,
            0xc,
            0,
            0xc,
            snd,
        );
        if FUN_004d6b90(l30, local_34, local_38) != 0 {
            if FUN_004dc1e0() == 0 {
                if FUN_004dc200() != 0 {
                    DAT_00665d64 = DAT_00665d64 + _DAT_005182ec;
                    if (_DAT_00518300 as f32) <= DAT_00665d64 {
                        DAT_00665d64 = 0.0;
                    }
                }
            } else {
                DAT_00665d64 = DAT_00665d64 - _DAT_005182ec;
                if DAT_00665d64 <= (_DAT_005182f0 as f32) {
                    DAT_00665d64 = 360.0;
                }
            }
        }
        FUN_004d64a0(l30, 0, -0x1c);
        pc = FUN_004ddc10(0x125);
        FUN_004da780(
            l30,
            &raw mut DAT_0052c4c0,
            &raw mut DAT_0052c4cc,
            pc,
            0xc,
            0,
            0xb,
            snd,
        );
        if FUN_004d6b90(l30, local_34, local_38) != 0 {
            if FUN_004dc1e0() == 0 {
                if FUN_004dc200() != 0 {
                    DAT_00665d74 = DAT_00665d74 + _DAT_005182ec;
                    if (_DAT_00518300 as f32) <= DAT_00665d74 {
                        DAT_00665d74 = 0.0;
                    }
                }
            } else {
                DAT_00665d74 = DAT_00665d74 - _DAT_005182ec;
                if DAT_00665d74 <= (_DAT_005182f0 as f32) {
                    DAT_00665d74 = 360.0;
                }
            }
        }
        FUN_004d6460(l30, 0x16, 0x14e, 0x2a, 0x166);
        pc = FUN_004ddc10(0x128);
        r = FUN_004da780(
            l30,
            &raw mut DAT_0052c4a8,
            &raw mut DAT_0052c4b4,
            pc,
            0xc,
            0,
            0x11,
            snd,
        );
        if r != 0 {
            DAT_00665d28 = DAT_00665d28.wrapping_sub(1);
            if DAT_00665d28 < 0 {
                DAT_00665d28 = DAT_00aa60fc.wrapping_sub(1);
            }
            let p = &raw mut DAT_00665d38 as *mut u32;
            for k in 0..11 {
                p.add(k).write(0);
            }
            let base = (DAT_00665d28 as u32)
                .wrapping_mul(0x44)
                .wrapping_add(DAT_00aa60f4 as u32);
            let kind_addr = (DAT_00aa60f4 as u32)
                .wrapping_add(0x40)
                .wrapping_add((DAT_00665d28 as u32).wrapping_mul(0x44));
            p.write((kind_addr as *const u32).read_unaligned());
            FUN_00426a00(p);
            FUN_00429e70(base as *mut i8);
            let kind_addr = (DAT_00aa60f4 as u32)
                .wrapping_add(0x40)
                .wrapping_add((DAT_00665d28 as u32).wrapping_mul(0x44));
            let k = (kind_addr as *const i32).read_unaligned();
            let v = if k == 6 || k == 7 { 100 } else { 0 };
            FUN_0042b0e0(0, v, 0);
        }
        FUN_004d64a0(l30, 0, -0x1c);
        pc = FUN_004ddc10(0x127);
        r = FUN_004da780(
            l30,
            &raw mut DAT_0052c490,
            &raw mut DAT_0052c49c,
            pc,
            0xc,
            0,
            0x10,
            snd,
        );
        if r != 0 {
            DAT_00665d28 = DAT_00665d28.wrapping_add(1);
            if DAT_00aa60fc <= DAT_00665d28 {
                DAT_00665d28 = 0;
            }
            let p = &raw mut DAT_00665d38 as *mut u32;
            for k in 0..11 {
                p.add(k).write(0);
            }
            let base = (DAT_00665d28 as u32)
                .wrapping_mul(0x44)
                .wrapping_add(DAT_00aa60f4 as u32);
            let kind_addr = (DAT_00aa60f4 as u32)
                .wrapping_add(0x40)
                .wrapping_add((DAT_00665d28 as u32).wrapping_mul(0x44));
            p.write((kind_addr as *const u32).read_unaligned());
            FUN_00426a00(p);
            FUN_00429e70(base as *mut i8);
            let kind_addr = (DAT_00aa60f4 as u32)
                .wrapping_add(0x40)
                .wrapping_add((DAT_00665d28 as u32).wrapping_mul(0x44));
            let k = (kind_addr as *const i32).read_unaligned();
            if k != 6 && k != 7 {
                FUN_0042b0e0(0, 0, 0);
                return 1;
            }
            FUN_0042b0e0(0, 100, 0);
        }
        1
    }
}
