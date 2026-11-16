// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a700: [i8; 0];
    static mut DAT_0052a708: [i8; 0];
    static mut DAT_0052a71c: [i8; 0];
    static mut DAT_00b69ce0: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00664e98: i32;
    static mut DAT_00b69cc4: i32;
    static mut DAT_0052c000: i32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00bb72d0: i8;
    static mut DAT_00bb73a7: i8;
    static mut DAT_00bb736f: i8;
    static mut DAT_00bb7370: i8;
    static mut DAT_00bb7311: [i8; 0];
    static mut DAT_00bb72d1: [i8; 0];
    static mut DAT_00bb735f: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004b4cc0(param_1: *mut i8, param_2: *mut i32);
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d4e20();
    fn FUN_004b4d40(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d4ec0(param_1: i32);
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004de5a0(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d64c0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d6550(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004da380(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i8,
    ) -> i32;
    fn FUN_004bc1c0(param_1: *mut i8, param_2: *mut i32, param_3: i32);
    fn FUN_004bc2f0(param_1: *mut i8, param_2: *mut i32, param_3: i32);
    fn FUN_004b8ea0(param_1: *mut i8, param_2: *mut i32, param_3: i32);
    fn FUN_004b9090(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bb680() {
    let mut local_29: i8;
    let mut local_28: [i32; 4] = [0; 4];
    let mut local_18: [i32; 4] = [0; 4];
    let mut local_8: i32 = 0;
    let mut local_4: i32 = 0;
    // SAFETY: all callees and globals are the game's own; pointers passed
    // refer to live locals or the game's static buffers.
    unsafe {
        let p28 = local_28.as_mut_ptr();
        let p18 = local_18.as_mut_ptr();
        let slot = (&raw mut DAT_0052a71c) as *mut i8;
        let btn = (&raw mut DAT_0052a700) as *mut i8;
        let snd = (&raw mut DAT_0052a708) as *mut i8;

        FUN_004d6460(p18, 0x54, 0x5a, 0x25c, 0x6a);
        let pc = FUN_004ddc10(0x130);
        FUN_004db640(0xf, pc, p18, 0xffff);
        let pc = FUN_004ddc10(0x134);
        let i3 = FUN_004dbfa0(pc, 0xc);
        let pc = FUN_004ddc10(0x135);
        let mut i8v = FUN_004dbfa0(pc, 0xc);
        if i8v < i3 {
            i8v = i3;
        }
        FUN_004b4cc0(slot, p28);
        let iv3 = local_28[2].wrapping_sub(local_28[0]).wrapping_add(1);
        FUN_004de090(&raw mut local_4, &raw mut local_8);
        FUN_004d6460(p18, 0x58, 0x6f, 0x14c, 0x7d);
        let u7: i32 = if DAT_00b69ce0 == 0 && DAT_00b69e30 != 2 {
            0x2b4
        } else {
            0x1dd
        };
        let pc = FUN_004ddc10(u7);
        FUN_004db7d0(0xc, pc, p18, 0, 1, 0xffff);
        FUN_004d64a0(p18, 0, 0x14);
        local_18[2] = local_18[0].wrapping_add(0x7f);
        FUN_004d6480(p28, p18);
        local_28[2] = local_28[0].wrapping_add(iv3);
        if DAT_00664e98 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
            FUN_004d4e20();
            FUN_004b4d40(slot, local_28[0], local_28[1]);
            FUN_004d4ec0(0);
            FUN_004b4d40(slot, local_28[0], local_28[1]);
        }
        let i4 = FUN_004d6b90(p28, local_4, local_8);
        'l1: {
            if i4 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
                if FUN_004dc220() == 0 {
                    if FUN_004dc240() == 0 {
                        break 'l1;
                    }
                }
                if DAT_00bb72d0 != 0 {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0(snd, 1, 0, 0xff);
                    }
                    if DAT_00bb72d0 != 0 {
                        DAT_00bb736f = 0;
                    }
                }
                DAT_00bb72d0 = 0;
            }
        }
        if DAT_00bb72d0 == 0 {
            FUN_004b5570(btn, local_28[0], local_28[1]);
        }
        local_28[0] = local_28[2].wrapping_add(5);
        local_28[2] = local_28[2].wrapping_add(0x69);
        let pc = FUN_004ddc10(0x131);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        FUN_004d64a0(p18, 0, 0x19);
        local_18[2] = local_18[0].wrapping_add(0x7f);
        FUN_004d6480(p28, p18);
        local_28[2] = local_28[0].wrapping_add(iv3);
        if DAT_00664e98 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
            FUN_004d4e20();
            FUN_004b4d40(slot, local_28[0], local_28[1]);
            FUN_004d4ec0(0);
            FUN_004b4d40(slot, local_28[0], local_28[1]);
        }
        let i4 = FUN_004d6b90(p28, local_4, local_8);
        'l2: {
            if i4 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
                if FUN_004dc220() == 0 {
                    if FUN_004dc240() == 0 {
                        break 'l2;
                    }
                }
                if DAT_00bb72d0 != 1 {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0(snd, 1, 0, 0xff);
                    }
                    if DAT_00bb72d0 != 1 {
                        DAT_00bb736f = 1;
                    }
                }
                DAT_00bb72d0 = 1;
            }
        }
        if DAT_00bb72d0 == 1 {
            FUN_004b5570(btn, local_28[0], local_28[1]);
        }
        local_28[0] = local_28[2].wrapping_add(5);
        local_28[2] = local_28[2].wrapping_add(0x69);
        let pc = FUN_004ddc10(0x132);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        FUN_004d64a0(p28, 100, 0);
        let pc = FUN_004ddc10(0x134);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        local_28[0] = local_28[0].wrapping_add(i8v).wrapping_add(0xf);
        local_28[2] = local_28[0].wrapping_add(200);
        FUN_004d64c0(p28, 0, -2);
        FUN_004d6550(p28, 0, 0, DAT_00b69cc4);
        let i4 = FUN_004da380(
            p28,
            (&raw mut DAT_00bb7311) as *mut i8,
            0x40,
            0,
            0xc,
            core::ptr::null_mut(),
        );
        if i4 != 0 {
            DAT_0052c000 = 1;
        }
        FUN_004d64a0(p18, 0, 0x19);
        local_18[2] = local_18[0].wrapping_add(0x7f);
        FUN_004d6480(p28, p18);
        local_28[2] = local_28[0].wrapping_add(iv3);
        if DAT_00664e98 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
            FUN_004d4e20();
            FUN_004b4d40(slot, local_28[0], local_28[1]);
            FUN_004d4ec0(0);
            FUN_004b4d40(slot, local_28[0], local_28[1]);
        }
        let i4 = FUN_004d6b90(p28, local_4, local_8);
        'l3: {
            if i4 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
                if FUN_004dc220() == 0 {
                    if FUN_004dc240() == 0 {
                        break 'l3;
                    }
                }
                if DAT_00bb72d0 != 2 {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0(snd, 1, 0, 0xff);
                    }
                    if DAT_00bb72d0 != 2 {
                        DAT_00bb736f = 0;
                    }
                }
                DAT_00bb72d0 = 2;
            }
        }
        if DAT_00bb72d0 == 2 {
            FUN_004b5570(btn, local_28[0], local_28[1]);
        }
        local_28[0] = local_28[2].wrapping_add(5);
        local_28[2] = local_28[2].wrapping_add(0x69);
        let pc = FUN_004ddc10(0x133);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        FUN_004d64a0(p28, 100, 0);
        let pc = FUN_004ddc10(0x135);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        local_28[0] = local_28[0].wrapping_add(i8v).wrapping_add(0xf);
        local_28[2] = local_28[0].wrapping_add(200);
        FUN_004d64c0(p28, 0, -2);
        FUN_004d6550(p28, 0, 0, DAT_00b69cc4);
        let i8r = FUN_004da380(
            p28,
            (&raw mut DAT_00bb72d1) as *mut i8,
            0x40,
            1,
            0xc,
            core::ptr::null_mut(),
        );
        if i8r != 0 {
            DAT_0052c000 = 2;
        }
        FUN_004d64a0(p18, 0, 0x19);
        local_18[2] = local_18[0].wrapping_add(0x7f);
        FUN_004d6480(p28, p18);
        local_28[2] = iv3.wrapping_add(local_28[0]);
        if DAT_00664e98 != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
            FUN_004d4e20();
            FUN_004b4d40(slot, local_28[0], local_28[1]);
            FUN_004d4ec0(0);
            FUN_004b4d40(slot, local_28[0], local_28[1]);
        }
        let i8r = FUN_004d6b90(p28, local_4, local_8);
        'l4: {
            if i8r != 0 && (DAT_00b69ce0 != 0 || DAT_00b69e30 == 2) {
                if FUN_004dc220() == 0 {
                    if FUN_004dc240() == 0 {
                        break 'l4;
                    }
                }
                if DAT_00bb72d0 != 3 {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0(snd, 1, 0, 0xff);
                    }
                    if DAT_00bb72d0 != 3 {
                        DAT_00bb736f = 0;
                    }
                }
                DAT_00bb72d0 = 3;
            }
        }
        if DAT_00bb72d0 == 3 {
            FUN_004b5570(btn, local_28[0], local_28[1]);
        }
        local_28[0] = local_28[2].wrapping_add(5);
        local_28[2] = local_28[2].wrapping_add(0x69);
        let pc = FUN_004ddc10(0x1c1);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        FUN_004d64a0(p18, 0, 0x23);
        let pc = FUN_004ddc10(0x1de);
        FUN_004db7d0(0xc, pc, p18, 0, 1, 0xffff);
        FUN_004d64a0(p18, 0, 0x14);
        FUN_004d6480(p28, p18);
        let pc = FUN_004ddc10(0x1e2);
        FUN_004db7d0(0xc, pc, p28, 0, 1, DAT_00b6a2c0);
        let pc = FUN_004ddc10(0x1e2);
        let i8r = FUN_004dbfa0(pc, 0xc);
        local_28[0] = local_28[0].wrapping_add(i8r).wrapping_add(0xf);
        local_28[2] = local_28[0].wrapping_add(200);
        FUN_004d64c0(p28, 0, -2);
        FUN_004d6550(p28, 0, 0, DAT_00b69cc4);
        let i8r = FUN_004da380(
            p28,
            (&raw mut DAT_00bb735f) as *mut i8,
            0x10,
            2,
            0xc,
            core::ptr::null_mut(),
        );
        if i8r != 0 {
            DAT_0052c000 = 0;
        }
        FUN_004d64a0(p18, 0, 0x23);
        let v736f = &raw mut DAT_00bb736f;
        if DAT_00b69e30 == 3 {
            let pc = FUN_004ddc10(0x2eb);
            FUN_004db7d0(0xc, pc, p18, 0, 1, 0xffff);
            FUN_004d64a0(p18, 0, 0x14);
            let pc = FUN_004ddc10(0x1df);
            FUN_004db7d0(0xc, pc, p18, 0, 1, DAT_00b6a2c0);
            FUN_004d64a0(p18, 0, 0x11);
            FUN_004bc1c0(v736f, p18, 0x1e0);
            FUN_004d64a0(p18, 0, 0x14);
            let pc = FUN_004ddc10(0x2e7);
            FUN_004db7d0(0xc, pc, p18, 0, 1, DAT_00b6a2c0);
            FUN_004d64a0(p18, 0, 0x11);
            local_29 = (((DAT_00bb7370 as i32) >> 1).wrapping_sub(2)) as i8;
            FUN_004bc2f0(&raw mut local_29, p18, 0x2e8);
            return;
        }
        let pc = FUN_004ddc10(0x2e6);
        FUN_004db7d0(0xc, pc, p18, 0, 1, 0xffff);
        FUN_004d64a0(p18, 0, 0x14);
        let pc = FUN_004ddc10(0x1df);
        FUN_004db7d0(0xc, pc, p18, 0, 1, DAT_00b6a2c0);
        FUN_004d64a0(p18, 0, 0x11);
        FUN_004b8ea0(v736f, p18, 0x1e0);
        FUN_004d64a0(p18, 0, 0x14);
        let pc = FUN_004ddc10(0x2e7);
        FUN_004db7d0(0xc, pc, p18, 0, 1, DAT_00b6a2c0);
        FUN_004d64a0(p18, 0, 0x11);
        local_29 = (((DAT_00bb7370 as i32) >> 1).wrapping_sub(2)) as i8;
        FUN_004b9090(&raw mut local_29, p18, 0x2e8);
        DAT_00bb7370 = ((local_29 as i32).wrapping_mul(2).wrapping_add(4)) as i8;
    }
}
