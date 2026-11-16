// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006691d0: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a606: [u8; 4];
    static mut DAT_0052c208: i32;
    static mut DAT_00b6a250: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5e10: [i32; 1];

    static mut s_jewelsound_0052ad74: [i8; 1];
    static mut s_clearu_0052c8f4: [i8; 1];
    static mut s_cleard_0052c8fc: [i8; 1];
    static mut s_assignu_0052c8e4: [i8; 1];
    static mut s_assignd_0052c8ec: [i8; 1];
    static mut s_team2d_0052c8dc: [i8; 1];
    static mut s_team2u_0052c8d4: [i8; 1];
    static mut s_team1d_0052c8cc: [i8; 1];
    static mut s_team1u_0052c8c4: [i8; 1];
    static mut s_frontres_soloteam_bmp_0052c890: [i8; 1];
    static mut s_background_0052bf74: [i8; 1];
    static mut s_frame_0052a6d4: [i8; 1];
    static mut s_clientactive_0052b69c: [i8; 1];
    static mut s_allactive_0052b6ac: [i8; 1];
    static mut s_chatbox_0052bfac: [i8; 1];
    static mut s_teamoverlay_0052c884: [i8; 1];
    static mut s_captoverlay_0052c878: [i8; 1];
    static mut s_slotoverlay_0052c86c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfee0() -> i32;
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004dc1e0() -> u8;
    fn FUN_004dc200() -> u8;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004daba0(a: *mut i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004da780(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: i32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004f46c0();
    fn FUN_004f4580();
    fn FUN_004daa20(a: *mut i32, b: *mut i8, c: *mut i8, d: i32);
    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f4010() -> u32 {
    // SAFETY: all globals, strings and callees are the game's own memory and
    // code; locals are passed by pointer as the original does.
    unsafe {
        let mut local_38: [i32; 4] = [0; 4];
        let mut local_28: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_20: [i32; 8] = [0; 8];
        let l38 = local_38.as_mut_ptr();
        let l20 = local_20.as_mut_ptr();

        let s_jewel = (&raw mut s_jewelsound_0052ad74).cast::<i8>();
        let s_clearu = (&raw mut s_clearu_0052c8f4).cast::<i8>();
        let s_cleard = (&raw mut s_cleard_0052c8fc).cast::<i8>();
        let s_assignu = (&raw mut s_assignu_0052c8e4).cast::<i8>();
        let s_assignd = (&raw mut s_assignd_0052c8ec).cast::<i8>();
        let s_team2d = (&raw mut s_team2d_0052c8dc).cast::<i8>();
        let s_team2u = (&raw mut s_team2u_0052c8d4).cast::<i8>();
        let s_team1d = (&raw mut s_team1d_0052c8cc).cast::<i8>();
        let s_team1u = (&raw mut s_team1u_0052c8c4).cast::<i8>();
        let s_solo = (&raw mut s_frontres_soloteam_bmp_0052c890).cast::<i8>();
        let s_bg = (&raw mut s_background_0052bf74).cast::<i8>();
        let s_frame = (&raw mut s_frame_0052a6d4).cast::<i8>();
        let s_client = (&raw mut s_clientactive_0052b69c).cast::<i8>();
        let s_all = (&raw mut s_allactive_0052b6ac).cast::<i8>();
        let s_chat = (&raw mut s_chatbox_0052bfac).cast::<i8>();
        let s_teamov = (&raw mut s_teamoverlay_0052c884).cast::<i8>();
        let s_capt = (&raw mut s_captoverlay_0052c878).cast::<i8>();
        let s_slot = (&raw mut s_slotoverlay_0052c86c).cast::<i8>();

        let mut b_var1: u8;
        let mut i_var3 = FUN_004cfee0();
        if i_var3 != 0 || {
            b_var1 = 0;
            DAT_00b69e30 == 2
        } {
            b_var1 = 1;
        }
        let tmp_5aa = (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned();
        let tmp_606 = (&raw const DAT_00b6a606).cast::<i32>().read_unaligned();
        if tmp_5aa == 3 && tmp_606 == 1 {
            local_20[0] = 1;
            local_20[1] = 1;
            l20.offset(DAT_006691d0 as isize).write(2);
        } else {
            local_20[0] = 0;
            local_20[1] = 0;
        }
        local_20[2] = 0;
        local_20[3] = 0;
        local_20[4] = 0;
        if b_var1 != 0 && DAT_006691d0 == 0 {
            local_20[5] = 1;
        } else {
            local_20[5] = 0;
        }
        local_20[7] = 0;
        local_20[6] = local_20[5];
        FUN_004de090(&raw mut local_24, &raw mut local_28);
        FUN_004de090(&raw mut local_24, &raw mut local_28);
        if b_var1 != 0 {
            FUN_004d6460(l38, 0x16, 0x132, 0x2a, 0x14a);
            let mut p = l20.add(5);
            loop {
                i_var3 = FUN_004d6b90(l38, local_24, local_28);
                if i_var3 != 0 {
                    'lab: {
                        if FUN_004dc1e0() == 0
                            && FUN_004dc200() == 0
                            && FUN_004dc220() == 0
                            && FUN_004dc240() == 0
                        {
                            break 'lab;
                        }
                        p.write(2);
                    }
                }
                p = p.add(1);
                FUN_004d64a0(l38, 0, 0x1c);
                if !(p < l20.add(7)) {
                    break;
                }
            }
        }
        FUN_004daba0(l20);
        if !(DAT_006691d0 != 0 || b_var1 == 0) {
            FUN_004d6460(l38, 0x16, 0x14e, 0x2a, 0x166);
            let pc = FUN_004ddc10(0xd7);
            FUN_004da780(l38, s_clearu, s_cleard, pc, 0xc, 0, 0x10, s_jewel);
            i_var3 = FUN_004d6b90(l38, local_24, local_28);
            if i_var3 != 0 {
                if FUN_004dc220() != 0 || FUN_004dc240() != 0 {
                    FUN_004f46c0();
                }
            }
            FUN_004d64a0(l38, 0, -0x1c);
            let pc = FUN_004ddc10(0xd6);
            FUN_004da780(l38, s_assignu, s_assignd, pc, 0xc, 0, 0x11, s_jewel);
            i_var3 = FUN_004d6b90(l38, local_24, local_28);
            if i_var3 != 0 {
                if FUN_004dc220() != 0 || FUN_004dc240() != 0 {
                    FUN_004f4580();
                }
            }
        }
        FUN_004d6460(l38, 0x16, 0x8e, 0x2a, 0xa6);
        if local_20[1] != 0 {
            if DAT_006691d0 == 1 {
                let pc = FUN_004ddc10(0x31e);
                FUN_004daa20(l38, s_team2d, pc, 0xc);
            } else {
                let pc = FUN_004ddc10(0x31e);
                i_var3 = FUN_004da780(l38, s_team2u, s_team2u, pc, 0xc, 0, 0xc, s_jewel);
                if i_var3 != 0 {
                    DAT_006691d0 = 1;
                    DAT_0052c208 = 0;
                    FUN_004b4a60(s_solo, s_bg);
                    FUN_004d4e20();
                    FUN_004b5de0(s_bg, 0, 0);
                    FUN_004b5570(s_frame, 0, 0);
                    let pc = if DAT_00b6a250 == 0 { s_client } else { s_all };
                    FUN_004b5570(pc, 0, 0);
                    if DAT_00b69e30 != 2 {
                        FUN_004b4d40(s_chat, 0, 0);
                    }
                    FUN_004b4d40(s_teamov, 0, 0);
                    FUN_004d4ec0(1);
                }
            }
        }
        FUN_004d64a0(l38, 0, -0x1c);
        if local_20[0] != 0 {
            if DAT_006691d0 == 0 {
                let pc = FUN_004ddc10(0x31d);
                FUN_004daa20(l38, s_team1d, pc, 0xc);
            } else {
                let pc = FUN_004ddc10(0x31d);
                i_var3 = FUN_004da780(l38, s_team1u, s_team1u, pc, 0xc, 0, 0xb, s_jewel);
                if i_var3 != 0 {
                    DAT_006691d0 = 0;
                    FUN_004b4a60(s_solo, s_bg);
                    FUN_004d4e20();
                    FUN_004b5de0(s_bg, 0, 0);
                    FUN_004b5570(s_frame, 0, 0);
                    let pc = if DAT_00b6a250 == 0 { s_client } else { s_all };
                    FUN_004b5570(pc, 0, 0);
                    if DAT_00b69e30 != 2 {
                        FUN_004b4d40(s_chat, 0, 0);
                    }
                    FUN_004b4d40(s_teamov, 0, 0);
                    i_var3 = 0;
                    FUN_004d6460(l38, 0x58, 0xcf, 0x100, 0xdd);
                    let arr = (&raw mut DAT_00aa5e10).cast::<i32>();
                    if 0 < DAT_00aa5e68 {
                        loop {
                            if (DAT_00aa5e68 == 8 || DAT_00aa5e68 == 7) && i_var3 == 4 {
                                FUN_004d6460(l38, 0x104, 0xcf, 0x1ac, 0xdd);
                            }
                            let mut i_var7: i32 = 0;
                            FUN_004d64a0(l38, 0, 0xf);
                            if 0 < arr.offset(i_var3 as isize).read() {
                                loop {
                                    let pc = if i_var7 == 0 { s_capt } else { s_slot };
                                    i_var7 = i_var7.wrapping_add(1);
                                    FUN_004b4d40(pc, *l38, *l38.add(1));
                                    FUN_004d64a0(l38, 0, 0xf);
                                    if !(i_var7 < arr.offset(i_var3 as isize).read()) {
                                        break;
                                    }
                                }
                            }
                            i_var3 = i_var3.wrapping_add(1);
                            if !(i_var3 < DAT_00aa5e68) {
                                break;
                            }
                        }
                    }
                    FUN_004d4ec0(1);
                }
            }
        }
        1
    }
}
