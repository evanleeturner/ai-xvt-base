// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_0085e7bb: i8;
    static mut DAT_00757c9c: i32;
    static mut DAT_00757cb4: i32;
    static mut DAT_00757c80: i32;
    static mut DAT_00757c88: i32;
    static mut DAT_00757c8c: i32;
    static mut DAT_00757cb8: i32;
    static mut DAT_00757c90: i32;
    static mut DAT_00757c84: i32;
    static mut DAT_00757cb0: i32;
    static mut DAT_00757ca4: i32;
    static mut DAT_00757c98: u32;
    static mut DAT_00757c94: u32;
    static mut DAT_00757cac: i32;
    static mut DAT_00757ca0: i32;
    static mut DAT_0064ff9c: i32;
    static mut DAT_007438d3: [u8; 0];
    static mut DAT_007438e1: [u8; 0];
    static mut DAT_00743dcc: [u8; 0];
    static mut DAT_00743dd6: [u8; 0];
    static mut DAT_00743dc2: [u8; 0];
    static mut DAT_007438d2: [u8; 0];
    static mut DAT_005333fc: [u32; 0];
    static mut DAT_0053340c: [u32; 0];
    static mut DAT_00533428: [i32; 0];
    static mut DAT_0081c012: [u8; 4];
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081d326: [u8; 4];
    static mut DAT_0081fb96: [u8; 4];
    static mut DAT_00758620: u32;
    static mut DAT_00758624: u32;
    static mut DAT_00758628: u32;
    static mut DAT_0075862c: u32;
    static mut DAT_00758630: u32;
    static mut DAT_00758634: u32;
    static mut DAT_00758638: u32;
    static mut DAT_0075863c: u32;
    static mut DAT_00758640: u32;

    static mut s_jewelsound_00531990: [i8; 0];
    static mut s_craft5u_00533668: [i8; 0];
    static mut s_craft5d_00533670: [i8; 0];
    static mut s_craft4u_00533658: [i8; 0];
    static mut s_craft4d_00533660: [i8; 0];
    static mut s_craft3u_00533648: [i8; 0];
    static mut s_craft3d_00533650: [i8; 0];
    static mut s_craft2u_00533638: [i8; 0];
    static mut s_craft2d_00533640: [i8; 0];
    static mut s_craft1u_00533628: [i8; 0];
    static mut s_craft1d_00533630: [i8; 0];
    static mut s_background_00532a4c: [i8; 0];
    static mut s_frontres_craftsi_bmp_005335c0: [i8; 0];
    static mut s_frontres_craftmi_bmp_005335a8: [i8; 0];
    static mut s_frontres_craftsr_bmp_005335d8: [i8; 0];
    static mut s_frontres_craftmr_bmp_00533590: [i8; 0];
    static mut s_frame_00531358: [i8; 0];
    static mut s_allactive_00531ebc: [i8; 0];
    static mut s_chatbox_00532a84: [i8; 0];
    static mut s_regoverlay_00531e94: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc810() -> i32;
    fn FUN_004ecce0(a: i32) -> u32;
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dd7d0(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004dd030(a: *mut i32, b: i32, c: i32);
    fn FUN_004d9ea0(a: *mut i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004d9a80(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: i32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004526c0(a: *mut i8);
    fn FUN_004d0430(a: *mut i8);
    fn FUN_004d0290(a: *mut i8, b: *mut i8);
    fn FUN_004d52b0();
    fn FUN_004d1750(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004cb830(a: i32, b: *mut u32, c: i32);
}

macro_rules! p {
    ($s:ident) => {
        (&raw mut $s).cast::<i8>()
    };
}

macro_rules! rd {
    ($s:ident, $t:ty, $i:expr) => {
        (&raw mut $s).cast::<$t>().offset(($i) as isize).read()
    };
}

unsafe fn rd32(p: *mut [u8; 4]) -> i32 {
    // SAFETY: p points at the game's packed 4-byte global.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn after_move() {
    // SAFETY: all callees and globals are the game's own.
    unsafe {
        let u = FUN_004ecce0(-1);
        FUN_004526c0(
            DAT_00757ca0.wrapping_add(rd!(DAT_00533428, i32, u).wrapping_mul(0x44)) as *mut i8,
        );
        let bfba = rd32(&raw mut DAT_0081bfba);
        if bfba != 1 && bfba != 2 {
            return;
        }
        let u = FUN_004ecce0(-1);
        let a = (u as i32) < 1 || (3 < (u as i32) && u != 0xe);
        if a {
            if DAT_0064ff9c == 0 {
                FUN_004d0430(p!(s_background_00532a4c));
                DAT_0064ff9c = 1;
                let pc = if DAT_0081b850 == 2 {
                    p!(s_frontres_craftsi_bmp_005335c0)
                } else {
                    p!(s_frontres_craftmi_bmp_005335a8)
                };
                FUN_004d0290(pc, p!(s_background_00532a4c));
            }
        } else if DAT_0064ff9c == 1 {
            FUN_004d0430(p!(s_background_00532a4c));
            DAT_0064ff9c = 0;
            let pc = if DAT_0081b850 == 2 {
                p!(s_frontres_craftsr_bmp_005335d8)
            } else {
                p!(s_frontres_craftmr_bmp_00533590)
            };
            FUN_004d0290(pc, p!(s_background_00532a4c));
        }
        FUN_004d52b0();
        FUN_004d1750(p!(s_background_00532a4c), 0, 0);
        FUN_004d0e60(p!(s_frame_00531358), 0, 0);
        FUN_004d0e60(p!(s_allactive_00531ebc), 0, 0);
        if DAT_0081b850 != 2 {
            FUN_004d0580(p!(s_chatbox_00532a84), 0, 0);
        }
        FUN_004d0580(p!(s_regoverlay_00531e94), 0, 0);
        FUN_004d5350(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ebad0() -> u32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // all pointers passed are to locals or the game's globals.
    unsafe {
        let b4: i32;
        let mut b6: i32 = 0;
        let b5: i32;
        let mut a34: [i32; 4] = [0; 4];
        let mut a24: [i32; 8] = [0; 8];
        let mut i_stack4: i32 = 0;

        if DAT_0081b850 == 2 {
            b4 = 1;
        } else if FUN_004cc810() != 0 {
            b4 = 1;
        } else {
            b4 = 0;
        }
        if DAT_0081b850 == 2 {
            b5 = 1;
        } else if DAT_0085e7bb == 0 {
            b5 = 0;
        } else if DAT_0085e7bb == 1 || (DAT_0085e7bb == 2 && b4 != 0) {
            b5 = 1;
        } else {
            b5 = 0;
        }
        if (DAT_00757c9c < 2 && DAT_00757cb4 < 2) || b5 == 0 {
            a24[0] = 0;
        } else {
            a24[0] = 1;
        }
        if DAT_00757c80 < 2 || b5 == 0 {
            a24[2] = 0;
        } else {
            a24[2] = 1;
        }
        a24[1] = a24[0];
        if DAT_00757c88 < 2 || b5 == 0 {
            a24[3] = 0;
        } else {
            a24[3] = 1;
            let u = FUN_004ecce0(-1);
            match u {
                1 | 2 | 3 | 5 | 0xe => a24[3] = 0,
                _ => {}
            }
        }
        if DAT_00757c8c < 2 || b5 == 0 {
            a24[4] = 0;
        } else {
            a24[4] = 1;
        }
        a24[5] = 0;
        a24[6] = 0;
        FUN_004ddc70(&raw mut i_stack4, a24.as_mut_ptr().add(7));
        FUN_004dcfd0(a34.as_mut_ptr(), 0x16, 0x72, 0x2a, 0x8a);
        let mut idx: usize = 0;
        loop {
            if a24[idx] != 0 && FUN_004dd7d0(a34.as_mut_ptr(), i_stack4, a24[7]) != 0 {
                if FUN_004db530() != 0
                    || FUN_004db550() != 0
                    || FUN_004db570() != 0
                    || FUN_004db590() != 0
                {
                    a24[idx] = 2;
                }
            }
            idx += 1;
            FUN_004dd030(a34.as_mut_ptr(), 0, 0x1c);
            if idx >= 5 {
                break;
            }
        }
        FUN_004d9ea0(a24.as_mut_ptr());
        FUN_004dcfd0(a34.as_mut_ptr(), 0x16, 0xe2, 0x2a, 0xfa);

        if a24[4] != 0 {
            let pc = FUN_004dcfa0(0x102);
            let r = FUN_004d9a80(
                a34.as_mut_ptr(),
                p!(s_craft5u_00533668),
                p!(s_craft5d_00533670),
                pc,
                0xc,
                0,
                0xf,
                p!(s_jewelsound_00531990),
            );
            if r != 0 {
                if FUN_004db570() == 0 {
                    if DAT_00757cb8 == 0 {
                        DAT_00757cb8 = DAT_00757c8c.wrapping_sub(1);
                    } else {
                        DAT_00757cb8 = DAT_00757cb8.wrapping_sub(1);
                    }
                } else {
                    DAT_00757cb8 = DAT_00757cb8.wrapping_add(1);
                    if DAT_00757c8c <= DAT_00757cb8 {
                        DAT_00757cb8 = 0;
                    }
                }
                b6 = 1;
            }
        }
        FUN_004dd030(a34.as_mut_ptr(), 0, -0x1c);
        if a24[3] != 0 {
            let pc = FUN_004dcfa0(0x103);
            let r = FUN_004d9a80(
                a34.as_mut_ptr(),
                p!(s_craft4u_00533658),
                p!(s_craft4d_00533660),
                pc,
                0xc,
                0,
                0xe,
                p!(s_jewelsound_00531990),
            );
            if r != 0 {
                if FUN_004db570() == 0 {
                    if DAT_00757c90 == 0 {
                        DAT_00757c90 = DAT_00757c88.wrapping_sub(1);
                    } else {
                        DAT_00757c90 = DAT_00757c90.wrapping_sub(1);
                    }
                } else {
                    DAT_00757c90 = DAT_00757c90.wrapping_add(1);
                    if DAT_00757c88 <= DAT_00757c90 {
                        DAT_00757c90 = 0;
                    }
                }
                b6 = 1;
            }
        }
        FUN_004dd030(a34.as_mut_ptr(), 0, -0x1c);
        if a24[2] != 0 {
            let pc = FUN_004dcfa0(0x101);
            let r = FUN_004d9a80(
                a34.as_mut_ptr(),
                p!(s_craft3u_00533648),
                p!(s_craft3d_00533650),
                pc,
                0xc,
                0,
                0xd,
                p!(s_jewelsound_00531990),
            );
            if r != 0 {
                if FUN_004db570() == 0 {
                    if DAT_00757c84 == 0 {
                        DAT_00757c84 = DAT_00757c80.wrapping_sub(1);
                    } else {
                        DAT_00757c84 = DAT_00757c84.wrapping_sub(1);
                    }
                } else {
                    DAT_00757c84 = DAT_00757c84.wrapping_add(1);
                    if DAT_00757c80 <= DAT_00757c84 {
                        DAT_00757c84 = 0;
                    }
                }
                b6 = 1;
            }
        }
        FUN_004dd030(a34.as_mut_ptr(), 0, -0x1c);
        if a24[1] != 0 {
            let pc = FUN_004dcfa0(0x100);
            let r = FUN_004d9a80(
                a34.as_mut_ptr(),
                p!(s_craft2u_00533638),
                p!(s_craft2d_00533640),
                pc,
                0xc,
                0,
                0xc,
                p!(s_jewelsound_00531990),
            );
            if r != 0 {
                b6 = 1;
                let mut i8v = DAT_00757cb0;
                let mut i13 = DAT_00757cb4;
                loop {
                    let mut again = 0;
                    let b2: u8;
                    let b3: u8;
                    if i13 == 0 {
                        DAT_00757ca4 = DAT_00757ca4.wrapping_sub(1);
                        if DAT_00757ca4 < 0 {
                            DAT_00757ca4 = DAT_00757c9c.wrapping_sub(1);
                        }
                        if DAT_00757ca4 == 0 {
                            let o = i8v.wrapping_mul(0x562);
                            b2 = rd!(DAT_007438d3, u8, o);
                            b3 = rd!(DAT_007438e1, u8, o);
                        } else {
                            let i1 = i8v.wrapping_mul(0x562).wrapping_add(DAT_00757ca4);
                            b2 = rd!(DAT_00743dcc, u8, i1);
                            b3 = rd!(DAT_00743dd6, u8, i1);
                        }
                        DAT_00757c98 = b2 as u32;
                        DAT_00757c94 = b3 as u32;
                    } else {
                        DAT_00757cac = DAT_00757cac.wrapping_sub(1);
                        if DAT_00757cac < 0 {
                            DAT_00757cac = i13.wrapping_sub(1);
                        }
                        if DAT_00757cac != 0 {
                            let i1 = i8v.wrapping_mul(0x562);
                            let bb = rd!(DAT_00743dc2, u8, i1);
                            if bb != 0 {
                                if bb < 3 {
                                    if rd!(DAT_005333fc, u32, DAT_00757cac)
                                        == rd!(DAT_007438d2, u8, i1) as u32
                                    {
                                        DAT_00757cac = DAT_00757cac.wrapping_sub(1);
                                    }
                                } else if bb == 3
                                    && rd!(DAT_0053340c, u32, DAT_00757cac)
                                        == rd!(DAT_007438d2, u8, i1) as u32
                                {
                                    DAT_00757cac = DAT_00757cac.wrapping_sub(1);
                                }
                            }
                        }
                    }
                    let c012 = rd32(&raw mut DAT_0081c012);
                    let bfba = rd32(&raw mut DAT_0081bfba);
                    let d326 = rd32(&raw mut DAT_0081d326);
                    if c012 == 1 && (bfba == 1 || bfba == 2) && d326 != 0 {
                        let u = FUN_004ecce0(-1);
                        i8v = DAT_00757cb0;
                        i13 = DAT_00757cb4;
                        let fb96 = rd32(&raw mut DAT_0081fb96);
                        if (u as i32) < 1 || (3 < (u as i32) && u != 0xe) {
                            if fb96 == 0 {
                                again = 1;
                            }
                        } else if fb96 == 1 {
                            again = 1;
                        }
                    }
                    if again == 0 {
                        break;
                    }
                }
                after_move();
            }
        }
        FUN_004dd030(a34.as_mut_ptr(), 0, -0x1c);
        if a24[0] != 0 {
            let pc = FUN_004dcfa0(0xff);
            let r = FUN_004d9a80(
                a34.as_mut_ptr(),
                p!(s_craft1u_00533628),
                p!(s_craft1d_00533630),
                pc,
                0xc,
                0,
                0xb,
                p!(s_jewelsound_00531990),
            );
            if r != 0 {
                b6 = 1;
                let mut i8v = DAT_00757cb0;
                loop {
                    let mut again = 0;
                    let b2: u8;
                    let b3: u8;
                    if DAT_00757cb4 == 0 {
                        DAT_00757ca4 = DAT_00757ca4.wrapping_add(1);
                        if DAT_00757c9c <= DAT_00757ca4 {
                            DAT_00757ca4 = 0;
                        }
                        if DAT_00757ca4 == 0 {
                            let o = i8v.wrapping_mul(0x562);
                            b2 = rd!(DAT_007438d3, u8, o);
                            b3 = rd!(DAT_007438e1, u8, o);
                        } else {
                            let i13 = i8v.wrapping_mul(0x562).wrapping_add(DAT_00757ca4);
                            b2 = rd!(DAT_00743dcc, u8, i13);
                            b3 = rd!(DAT_00743dd6, u8, i13);
                        }
                        DAT_00757c98 = b2 as u32;
                        DAT_00757c94 = b3 as u32;
                    } else {
                        DAT_00757cac = DAT_00757cac.wrapping_add(1);
                        if DAT_00757cb4 <= DAT_00757cac {
                            DAT_00757cac = 0;
                        }
                        if DAT_00757cac != 0 {
                            let i13 = i8v.wrapping_mul(0x562);
                            let bb = rd!(DAT_00743dc2, u8, i13);
                            if bb != 0 {
                                if bb < 3 {
                                    if rd!(DAT_005333fc, u32, DAT_00757cac)
                                        == rd!(DAT_007438d2, u8, i13) as u32
                                    {
                                        DAT_00757cac = DAT_00757cac.wrapping_add(1);
                                    }
                                } else if bb == 3
                                    && rd!(DAT_0053340c, u32, DAT_00757cac)
                                        == rd!(DAT_007438d2, u8, i13) as u32
                                {
                                    DAT_00757cac = DAT_00757cac.wrapping_add(1);
                                }
                            }
                        }
                        if DAT_00757cb4 <= DAT_00757cac {
                            DAT_00757cac = 0;
                        }
                    }
                    let c012 = rd32(&raw mut DAT_0081c012);
                    let bfba = rd32(&raw mut DAT_0081bfba);
                    let d326 = rd32(&raw mut DAT_0081d326);
                    if c012 == 1 && (bfba == 1 || bfba == 2) && d326 != 0 {
                        let u = FUN_004ecce0(-1);
                        i8v = DAT_00757cb0;
                        let fb96 = rd32(&raw mut DAT_0081fb96);
                        if (u as i32) < 1 || (3 < (u as i32) && u != 0xe) {
                            if fb96 == 0 {
                                again = 1;
                            }
                        } else if fb96 == 1 {
                            again = 1;
                        }
                    }
                    if again == 0 {
                        break;
                    }
                }
                after_move();
            }
        }
        if b6 != 0 && DAT_0081b850 != 2 {
            DAT_00758620 = 0x62;
            DAT_00758628 = DAT_00757cac as u32;
            DAT_0075862c = DAT_00757ca4 as u32;
            DAT_00758630 = DAT_00757c84 as u32;
            DAT_00758634 = DAT_00757c90 as u32;
            DAT_0075863c = DAT_00757c94;
            DAT_00758624 = rd!(DAT_00743dc2, u8, DAT_00757cb0.wrapping_mul(0x562)) as u32;
            DAT_00758640 = DAT_00757c98;
            DAT_00758638 = DAT_00757cb8 as u32;
            FUN_004cb830(0, &raw mut DAT_00758620, 0x24);
        }
        0
    }
}
