// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa5aac: i32;
    static mut DAT_00b6a250: i32;
    static mut DAT_00a91d24: [i8; 1];
    static mut DAT_00a91d17: [i8; 1];
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6d962: [u8; 4];

    static mut s_frontres_gametr_bmp_0052c468: [i8; 1];
    static mut s_frontres_gameti_bmp_0052c454: [i8; 1];
    static mut s_frontres_gamecr_bmp_0052c440: [i8; 1];
    static mut s_frontres_gameci_bmp_0052c42c: [i8; 1];
    static mut s_frontres_gamebr_bmp_0052c418: [i8; 1];
    static mut s_frontres_gamebi_bmp_0052c404: [i8; 1];
    static mut s_frontres_gamemn_bmp_0052c3f0: [i8; 1];
    static mut s_frontres_gamecar_bmp_0052c3d8: [i8; 1];
    static mut s_frontres_gamecai_bmp_0052c3c0: [i8; 1];
    static mut s_frontres_gamecn_bmp_0052c394: [i8; 1];
    static mut s_frontres_gamebn_bmp_0052c380: [i8; 1];
    static mut s_frontres_gametrn_bmp_0052c3a8: [i8; 1];
    static mut s_background_0052bf74: [i8; 1];
    static mut s_frame_0052a6d4: [i8; 1];
    static mut s_clientactive_0052b69c: [i8; 1];
    static mut s_allactive_0052b6ac: [i8; 1];
    static mut s_chatbox_0052bfac: [i8; 1];
    static mut s_gameoverlay_0052c374: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004e9560() -> u8;
    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
}

macro_rules! p {
    ($n:ident) => {
        (&raw mut $n).cast::<i8>()
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e9330() -> u32 {
    // SAFETY: all globals, strings and callees are the game's own memory
    // and functions; reads and writes happen where the original does.
    unsafe {
        let local_a: i32 = (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned();
        let local_b: i32 = (&raw const DAT_00b6d962).cast::<i32>().read_unaligned();

        let mut pc: *mut i8 = 'sel: {
            if DAT_00b69e30 == 2 {
                if local_a == 0 {
                    if local_b == 0 {
                        break 'sel p!(s_frontres_gametr_bmp_0052c468);
                    } else {
                        break 'sel p!(s_frontres_gameti_bmp_0052c454);
                    }
                }
                if local_a == 3 {
                    if local_b == 0 {
                        break 'sel p!(s_frontres_gamecr_bmp_0052c440);
                    } else {
                        break 'sel p!(s_frontres_gameci_bmp_0052c42c);
                    }
                }
                if local_a == 4 {
                    if local_b == 0 {
                        break 'sel p!(s_frontres_gamebr_bmp_0052c418);
                    } else {
                        break 'sel p!(s_frontres_gamebi_bmp_0052c404);
                    }
                }
                if local_a == 1 {
                    break 'sel p!(s_frontres_gamemn_bmp_0052c3f0);
                }
                if local_a == 5 {
                    if local_b == 0 {
                        break 'sel p!(s_frontres_gamecar_bmp_0052c3d8);
                    } else {
                        break 'sel p!(s_frontres_gamecai_bmp_0052c3c0);
                    }
                }
            } else {
                if local_a == 0 {
                    let mut i_var2: i32 = 0;
                    if 0 < DAT_00aa5aac {
                        let mut q: *mut i8 = p!(DAT_00a91d24);
                        loop {
                            if q.read() != 0 {
                                break;
                            }
                            q = q.byte_add(0x562);
                            i_var2 = i_var2.wrapping_add(1);
                            if !(i_var2 < DAT_00aa5aac) {
                                break;
                            }
                        }
                    }
                    let v = p!(DAT_00a91d17)
                        .offset(i_var2.wrapping_mul(0x562) as isize)
                        .read();
                    if v == 0 {
                        break 'sel p!(s_frontres_gametr_bmp_0052c468);
                    } else {
                        break 'sel p!(s_frontres_gameti_bmp_0052c454);
                    }
                }
                if local_a == 3 {
                    break 'sel p!(s_frontres_gamecn_bmp_0052c394);
                }
                if local_a == 4 {
                    break 'sel p!(s_frontres_gamebn_bmp_0052c380);
                }
                if local_a == 1 {
                    break 'sel p!(s_frontres_gamemn_bmp_0052c3f0);
                }
                if local_a == 5 {
                    let b = FUN_004e9560();
                    if b == 0 {
                        break 'sel p!(s_frontres_gamecai_bmp_0052c3c0);
                    } else {
                        break 'sel p!(s_frontres_gamecar_bmp_0052c3d8);
                    }
                }
            }
            p!(s_frontres_gametrn_bmp_0052c3a8)
        };

        FUN_004b4a60(pc, p!(s_background_0052bf74));
        FUN_004d4e20();
        FUN_004b5de0(p!(s_background_0052bf74), 0, 0);
        FUN_004b5570(p!(s_frame_0052a6d4), 0, 0);
        if DAT_00b6a250 == 0 {
            pc = p!(s_clientactive_0052b69c);
        } else {
            pc = p!(s_allactive_0052b6ac);
        }
        FUN_004b5570(pc, 0, 0);
        if DAT_00b69e30 != 2 {
            FUN_004b4d40(p!(s_chatbox_0052bfac), 0, 0);
        }
        FUN_004b4d40(p!(s_gameoverlay_0052c374), 0, 0);
        FUN_004d4ec0(1);
        1
    }
}
