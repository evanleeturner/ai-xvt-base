// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5cb0: [i32; 0];
    static mut DAT_00b6a5a6: i32;
    static mut DAT_00aa5e10: [i32; 0];
    static mut DAT_00b69e30: i32;
    static mut DAT_00669790: i32;
    static mut DAT_006691e6: i16;
    static mut DAT_006696e0: i16;
    static mut DAT_00669218: i32;
    static mut DAT_0066921c: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_00b6a250: i32;
    static mut DAT_00b6a2a4: i32;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00a91d24: i8;
    static mut DAT_00bb73a7: i8;
    static mut DAT_00669220: u32;

    static mut s_jewelsound_0052ad74: [i8; 0];
    static mut s_clearu_0052c8f4: [i8; 0];
    static mut s_cleard_0052c8fc: [i8; 0];
    static mut s_assignu_0052c8e4: [i8; 0];
    static mut s_assignd_0052c8ec: [i8; 0];
    static mut s_play2d_0052d090: [i8; 0];
    static mut s_play2u_0052d088: [i8; 0];
    static mut s_play1d_0052d080: [i8; 0];
    static mut s_play1u_0052d078: [i8; 0];
    static mut s_background_0052bf74: [i8; 0];
    static mut s_frame_0052a6d4: [i8; 0];
    static mut s_clientactive_0052b69c: [i8; 0];
    static mut s_allactive_0052b6ac: [i8; 0];
    static mut s_chatbox_0052bfac: [i8; 0];
    static mut s_mapassignoverlay_0052cf8c: [i8; 0];
    static mut s_fgslotoverlay_0052cf7c: [i8; 0];
    static mut s_mapoverlay_0052cf70: [i8; 0];
    static mut s_map4u_0052d068: [i8; 0];
    static mut s_map4d_0052d070: [i8; 0];
    static mut s_map2d_0052d060: [i8; 0];
    static mut s_map2u_0052d058: [i8; 0];
    static mut s_map1d_0052d050: [i8; 0];
    static mut s_map1u_0052d048: [i8; 0];
    static mut s_clear2u_0052d038: [i8; 0];
    static mut s_clear2d_0052d040: [i8; 0];
    static mut s_assign2u_0052d020: [i8; 0];
    static mut s_assign2d_0052d02c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d11e0() -> i32;
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dc1e0() -> u8;
    fn FUN_004dc200() -> u8;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004daba0(param_1: *mut i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004daa20(param_1: *mut i32, param_2: *mut i8, param_3: *mut i8, param_4: i32);
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
    fn FUN_004d4e20();
    fn FUN_004b5de0(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004b4d40(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d4ec0(param_1: i32);
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut u32);
    fn FUN_004d6550(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004de0b0(param_1: i32, param_2: i32);
    fn FUN_004fa690();
    fn FUN_004fa570();
    fn FUN_004f7380();
    fn FUN_004de5a0(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004f7480();
}

macro_rules! s {
    ($n:ident) => {
        (&raw mut $n).cast::<i8>()
    };
}

macro_rules! hit {
    () => {
        (FUN_004dc1e0() != 0 || FUN_004dc200() != 0 || FUN_004dc220() != 0 || FUN_004dc240() != 0)
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f8e40() -> u32 {
    // l[0..4] = local_38, l[4] = local_28, l[5..14] = local_24 (contiguous stack layout)
    let mut l: [i32; 14] = [0; 14];
    macro_rules! q {
        ($i:expr) => {
            l[(5 + ($i)) as usize]
        };
    }
    // SAFETY: all globals, strings and callees are the game's own; the local
    // buffer is passed to callees exactly as the original passes its stack.
    unsafe {
        let a_cb0 = (&raw mut DAT_00aa5cb0).cast::<i32>();
        let a_e10 = (&raw mut DAT_00aa5e10).cast::<i32>();
        let p38 = l.as_mut_ptr();
        let p24 = l.as_mut_ptr().add(5);
        let p28 = l.as_mut_ptr().add(4);

        let i_var2 = FUN_004d11e0();
        let b_var6: i32 =
            (i_var2 == a_cb0.offset((DAT_00b6a5a6.wrapping_mul(8)) as isize).read()) as i32;
        let b_var7: i32 = (DAT_00b69e30 == 2) as i32;
        let b_var8: i32 = (DAT_00669790 != 0) as i32;
        let b78 = b_var7 != 0 || b_var6 != 0;
        if b_var8 != 0 {
            q!(5) = b78 as i32;
            q!(8) = q!(5);
        } else {
            q!(5) = 0;
        }
        q!(4) = (b_var8 == 0) as i32;
        q!(2) = (b_var8 == 0) as i32;
        q!(1) = (b_var8 == 0) as i32;
        q!(3) = 0;
        q!(6) = 0;
        q!(7) = 0;
        let lvl = a_e10.offset(DAT_00b6a5a6 as isize).read();
        if lvl < 5 {
            if b78 && 1 < a_e10.offset(DAT_00b6a5a6 as isize).read() {
                q!(6) = 1;
                q!(7) = 1;
            }
        } else {
            q!(6) = 1;
            q!(7) = 1;
            q!(DAT_00669790.wrapping_add(6)) = 2;
        }
        if DAT_00669790 == 0 {
            let idx = (((DAT_006691e6 as i32) ^ 1) as i16 as i32).wrapping_add(1);
            q!(idx) = 2;
        }
        FUN_004d6460(p38, 0x16, 0xc6, 0x2a, 0xde);
        FUN_004de090(p24, p28);
        if DAT_00669790 == 0 && FUN_004d6b90(p38, q!(0), l[4]) != 0 && hit!() {
            q!(4) = 2;
        }
        if a_e10.offset(DAT_00b6a5a6 as isize).read() < 5 {
            if b78 && q!(6) != 0 {
                FUN_004d6460(p38, 0x16, 0x132, 0x2a, 0x14a);
                let mut k = 6;
                loop {
                    let r = FUN_004d6b90(p38, q!(0), l[4]);
                    if r != 0 && hit!() {
                        q!(k) = 2;
                    }
                    k += 1;
                    FUN_004d64a0(p38, 0, 0x1c);
                    if !(k < 8) {
                        break;
                    }
                }
            }
        } else if DAT_00669790 == 1 && b78 {
            let mut i_var2 = 0;
            FUN_004d6460(p38, 0x16, 0xe2, 0x2a, 0xfa);
            loop {
                let r = FUN_004d6b90(p38, q!(0), l[4]);
                if r != 0 && hit!() {
                    if i_var2 == 0 {
                        q!(6) = 2;
                    } else {
                        q!(7) = 2;
                    }
                }
                i_var2 += 1;
                FUN_004d64a0(p38, 0, 0x1c);
                if !(i_var2 < 2) {
                    break;
                }
            }
        }
        FUN_004daba0(p24.add(1));
        if a_e10.offset(DAT_00b6a5a6 as isize).read() < 5 {
            if b78 && q!(6) != 0 {
                FUN_004d6460(p38, 0x16, 0x14e, 0x2a, 0x166);
                let pc = FUN_004ddc10(0xd7);
                let r = FUN_004da780(
                    p38,
                    s!(s_clearu_0052c8f4),
                    s!(s_cleard_0052c8fc),
                    pc,
                    0xc,
                    0,
                    0x11,
                    s!(s_jewelsound_0052ad74),
                );
                if r != 0 {
                    FUN_004fa690();
                }
                FUN_004d64a0(p38, 0, -0x1c);
                let pc = FUN_004ddc10(0xd6);
                let r = FUN_004da780(
                    p38,
                    s!(s_assignu_0052c8e4),
                    s!(s_assignd_0052c8ec),
                    pc,
                    0xc,
                    0,
                    0x10,
                    s!(s_jewelsound_0052ad74),
                );
                if r != 0 {
                    FUN_004fa570();
                }
            }
        } else {
            FUN_004d6460(p38, 0x16, 0x14e, 0x2a, 0x166);
            if DAT_00669790 == 1 {
                let pc = FUN_004ddc10(799);
                FUN_004daa20(p38, s!(s_play2d_0052d090), pc, 0xc);
            } else {
                let pc = FUN_004ddc10(799);
                let r = FUN_004da780(
                    p38,
                    s!(s_play2u_0052d088),
                    s!(s_play2u_0052d088),
                    pc,
                    0xc,
                    0,
                    0x11,
                    s!(s_jewelsound_0052ad74),
                );
                if r != 0 {
                    DAT_00669790 = 1;
                    DAT_0052c208 = 0;
                    FUN_004d4e20();
                    FUN_004b5de0(s!(s_background_0052bf74), 0, 0);
                    FUN_004b5570(s!(s_frame_0052a6d4), 0, 0);
                    let pc = if DAT_00b6a250 == 0 {
                        s!(s_clientactive_0052b69c)
                    } else {
                        s!(s_allactive_0052b6ac)
                    };
                    FUN_004b5570(pc, 0, 0);
                    if DAT_00b69e30 != 2 {
                        FUN_004b4d40(s!(s_chatbox_0052bfac), 0, 0);
                    }
                    let mut i_var2: i32 = 0;
                    FUN_004b4d40(s!(s_mapassignoverlay_0052cf8c), 0, 0);
                    q!(0) = 0xe6;
                    l[4] = 0x11c;
                    if 0 < DAT_00aa5aac {
                        let mut pc = &raw mut DAT_00a91d24;
                        loop {
                            if pc.read() != 0
                                && (pc.byte_sub(0xc).cast::<u8>().read() as i32) == DAT_00b6a5a6
                            {
                                FUN_004b4d40(s!(s_fgslotoverlay_0052cf7c), q!(0), l[4]);
                                l[4] = l[4].wrapping_add(0x11);
                            }
                            pc = pc.byte_add(0x562);
                            i_var2 += 1;
                            if !(i_var2 < DAT_00aa5aac as i32) {
                                break;
                            }
                        }
                    }
                    FUN_004d4ec0(1);
                }
            }
            FUN_004d64a0(p38, 0, -0x1c);
            if DAT_00669790 == 0 {
                let pc = FUN_004ddc10(800);
                FUN_004daa20(p38, s!(s_play1d_0052d080), pc, 0xc);
            } else {
                let pc = FUN_004ddc10(800);
                let r = FUN_004da780(
                    p38,
                    s!(s_play1u_0052d078),
                    s!(s_play1u_0052d078),
                    pc,
                    0xc,
                    0,
                    0x10,
                    s!(s_jewelsound_0052ad74),
                );
                if r != 0 {
                    DAT_00669790 = 0;
                    FUN_004d4e20();
                    FUN_004b5de0(s!(s_background_0052bf74), 0, 0);
                    FUN_004b5570(s!(s_frame_0052a6d4), 0, 0);
                    let pc = if DAT_00b6a250 == 0 {
                        s!(s_clientactive_0052b69c)
                    } else {
                        s!(s_allactive_0052b6ac)
                    };
                    FUN_004b5570(pc, 0, 0);
                    if DAT_00b69e30 != 2 {
                        FUN_004b4d40(s!(s_chatbox_0052bfac), 0, 0);
                    }
                    FUN_004b4d40(s!(s_mapoverlay_0052cf70), 0, 0);
                    FUN_004d6480(p38, &raw mut DAT_00669220);
                    FUN_004d64a0(p38, 0x54, 0x60);
                    l[1] = l[3].wrapping_sub(0x1b);
                    FUN_004d6550(p38, 0, 0, DAT_00b6a2a4);
                    FUN_004d4ec0(1);
                    FUN_004de0b0(0x25, 0x1bd);
                }
            }
        }
        if DAT_00669790 == 0 {
            FUN_004d6460(p38, 0x16, 0xc6, 0x2a, 0xde);
            let pc = FUN_004ddc10(0xd2);
            let r = FUN_004da780(
                p38,
                s!(s_map4u_0052d068),
                s!(s_map4d_0052d070),
                pc,
                0xc,
                0,
                0xe,
                s!(s_jewelsound_0052ad74),
            );
            if r != 0 {
                DAT_00669218 = 0;
                DAT_0066921c = 0;
                FUN_004f7380();
            }
            FUN_004d64a0(p38, 0, -0x38);
            if DAT_006691e6 == 0 {
                let pc = FUN_004ddc10(0xd0);
                FUN_004daa20(p38, s!(s_map2d_0052d060), pc, 0xc);
            } else {
                let pc = FUN_004ddc10(0xd0);
                let r = FUN_004da780(
                    p38,
                    s!(s_map2u_0052d058),
                    s!(s_map2u_0052d058),
                    pc,
                    0xc,
                    0,
                    0xc,
                    s!(s_jewelsound_0052ad74),
                );
                if r != 0 {
                    DAT_006691e6 = 0;
                }
            }
            FUN_004d64a0(p38, 0, -0x1c);
            if DAT_006691e6 != 0 {
                let r = FUN_004d6b90(p38, q!(0), l[4]);
                if r != 0 && (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0(s!(s_jewelsound_0052ad74), 1, 0, 0xff);
                    }
                    FUN_004f7480();
                    if DAT_006696e0 as i32 == DAT_00669218 {
                        DAT_00669218 = 0;
                        DAT_0066921c = 0;
                        FUN_004f7380();
                    }
                }
                let pc = FUN_004ddc10(0xd1);
                FUN_004daa20(p38, s!(s_map1d_0052d050), pc, 0xc);
                return 0;
            }
            let pc = FUN_004ddc10(0x231);
            let r = FUN_004da780(
                p38,
                s!(s_map1u_0052d048),
                s!(s_map1u_0052d048),
                pc,
                0xc,
                0,
                0xb,
                s!(s_jewelsound_0052ad74),
            );
            if r != 0 {
                DAT_006691e6 = 1;
                return 0;
            }
        } else if b78 && q!(5) != 0 {
            FUN_004d6460(p38, 0x16, 0xfe, 0x2a, 0x116);
            let pc = FUN_004ddc10(0xd7);
            let r = FUN_004da780(
                p38,
                s!(s_clear2u_0052d038),
                s!(s_clear2d_0052d040),
                pc,
                0xc,
                0,
                0xc,
                s!(s_jewelsound_0052ad74),
            );
            if r != 0 {
                FUN_004fa690();
            }
            FUN_004d64a0(p38, 0, -0x1c);
            let pc = FUN_004ddc10(0xd6);
            let r = FUN_004da780(
                p38,
                s!(s_assign2u_0052d020),
                s!(s_assign2d_0052d02c),
                pc,
                0xc,
                0,
                0xb,
                s!(s_jewelsound_0052ad74),
            );
            if r != 0 {
                FUN_004fa570();
            }
        }
        0
    }
}
