// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645778: *mut i8;
    static mut DAT_00645a84: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_006db7a4: i32;
    static mut DAT_00703ac0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__memcpy(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: u32,
    ) -> *mut core::ffi::c_void;
}

unsafe fn zero_fill(mut p: *mut i8, diff: u32) {
    let mut words = diff >> 2;
    // SAFETY: the caller's buffer is the game's own; zeroing exactly the span the original clears.
    unsafe {
        while words != 0 {
            p.cast::<u32>().write_unaligned(0);
            p = p.wrapping_add(4);
            words -= 1;
        }
        let mut rest = diff & 3;
        while rest != 0 {
            *p = 0;
            p = p.wrapping_add(1);
            rest -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049a2f0(param_1: *mut u8) {
    // SAFETY: globals and the memcpy are the game's own; param_1 and the
    // buffer at DAT_00645778 are read and written exactly where the original does.
    unsafe {
        let mut pcvar7: *mut i8 = DAT_00645778.wrapping_offset(DAT_00645a84 as isize);
        let ivar1: i32 = param_1.cast::<i32>().read_unaligned();
        let mut local_c: i32 = 0;
        let mut local_10: i32 = 0;
        let mut pcvar8: *mut i8 = DAT_00645778;
        let mut param_1: *mut u8 = param_1.wrapping_add(4);
        if 0 < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32 {
            loop {
                let mut pcvar10: *mut i8 = pcvar8;
                if local_10 < DAT_006db7a4 || DAT_00703ac0 <= local_10 {
                    let mut bvar5: u8;
                    if local_c == 0 {
                        if ivar1 <= local_10 {
                            break;
                        }
                        bvar5 = *param_1;
                        param_1 = param_1.wrapping_add(1);
                        if (bvar5 & 0x80) != 0 {
                            let uvar6 = bvar5 as u16;
                            bvar5 = 0;
                            local_c = ((uvar6 & 0x7f) as i32).wrapping_sub(1);
                        }
                    } else {
                        local_c = local_c.wrapping_sub(1);
                        bvar5 = 0;
                    }
                    let mut pcvar9: *mut i8 = pcvar8.wrapping_add(1);
                    pcvar10 = pcvar9;
                    if *pcvar8 == 0 {
                        if (bvar5 & 1) != 0 {
                            pcvar10 = pcvar8.wrapping_add(0x24);
                            let s = (pcvar7 as u32).wrapping_sub(pcvar9 as u32);
                            pcvar7 = pcvar7.wrapping_add(0x23);
                            FID_conflict__memcpy(pcvar10.cast(), pcvar9.cast(), s);
                            zero_fill(pcvar9, (pcvar10 as u32).wrapping_sub(pcvar9 as u32));
                        }
                    } else if (bvar5 & 1) == 0 {
                        let src = pcvar8.wrapping_add(0x24);
                        FID_conflict__memcpy(
                            pcvar9.cast(),
                            src.cast(),
                            (pcvar7 as u32).wrapping_sub(src as u32),
                        );
                        pcvar7 = pcvar7.wrapping_sub(0x23);
                    } else {
                        pcvar9 = pcvar8.wrapping_add(0x24);
                        pcvar10 = pcvar9;
                        if pcvar8.wrapping_add(0x20).cast::<i32>().read_unaligned() == 0 {
                            if (bvar5 & 2) != 0 {
                                pcvar10 = pcvar8.wrapping_add(0xd3);
                                let s = (pcvar7 as u32).wrapping_sub(pcvar9 as u32);
                                pcvar7 = pcvar7.wrapping_add(0xaf);
                                FID_conflict__memcpy(pcvar10.cast(), pcvar9.cast(), s);
                                zero_fill(pcvar9, (pcvar10 as u32).wrapping_sub(pcvar9 as u32));
                            }
                        } else if (bvar5 & 2) == 0 {
                            let src = pcvar8.wrapping_add(0xd3);
                            FID_conflict__memcpy(
                                pcvar9.cast(),
                                src.cast(),
                                (pcvar7 as u32).wrapping_sub(src as u32),
                            );
                            pcvar7 = pcvar7.wrapping_sub(0xaf);
                        } else {
                            pcvar9 = pcvar8.wrapping_add(0xd3);
                            pcvar10 = pcvar9;
                            let mut tmp: i32 =
                                pcvar8.wrapping_add(0xcb).cast::<i32>().read_unaligned();
                            if tmp == 0 {
                                if (bvar5 & 4) != 0 {
                                    pcvar10 = pcvar8.wrapping_add(0x509);
                                    let s = (pcvar7 as u32).wrapping_sub(pcvar9 as u32);
                                    pcvar7 = pcvar7.wrapping_add(0x436);
                                    FID_conflict__memcpy(pcvar10.cast(), pcvar9.cast(), s);
                                    zero_fill(pcvar9, (pcvar10 as u32).wrapping_sub(pcvar9 as u32));
                                }
                            } else if (bvar5 & 4) == 0 {
                                let src = pcvar8.wrapping_add(0x509);
                                FID_conflict__memcpy(
                                    pcvar9.cast(),
                                    src.cast(),
                                    (pcvar7 as u32).wrapping_sub(src as u32),
                                );
                                pcvar7 = pcvar7.wrapping_sub(0x436);
                            } else {
                                pcvar10 = pcvar8.wrapping_add(0x509);
                            }
                            pcvar9 = pcvar10;
                            tmp = pcvar8.wrapping_add(199).cast::<i32>().read_unaligned();
                            if tmp == 0 {
                                if (bvar5 & 8) != 0 {
                                    pcvar9 = pcvar10.wrapping_add(8);
                                    let s = (pcvar7 as u32).wrapping_sub(pcvar10 as u32);
                                    pcvar7 = pcvar7.wrapping_add(8);
                                    FID_conflict__memcpy(pcvar9.cast(), pcvar10.cast(), s);
                                    zero_fill(
                                        pcvar10,
                                        (pcvar9 as u32).wrapping_sub(pcvar10 as u32),
                                    );
                                }
                            } else if (bvar5 & 8) == 0 {
                                let src = pcvar10.wrapping_add(8);
                                FID_conflict__memcpy(
                                    pcvar10.cast(),
                                    src.cast(),
                                    (pcvar7 as u32).wrapping_sub(src as u32),
                                );
                                pcvar7 = pcvar7.wrapping_sub(8);
                            } else {
                                pcvar9 = pcvar10.wrapping_add(8);
                            }
                            pcvar10 = pcvar9;
                            tmp = pcvar8.wrapping_add(0xcf).cast::<i32>().read_unaligned();
                            if tmp == 0 {
                                if (bvar5 & 0x10) != 0 {
                                    pcvar10 = pcvar9.wrapping_add(0x4c);
                                    let s = (pcvar7 as u32).wrapping_sub(pcvar9 as u32);
                                    pcvar7 = pcvar7.wrapping_add(0x4c);
                                    FID_conflict__memcpy(pcvar10.cast(), pcvar9.cast(), s);
                                    zero_fill(pcvar9, (pcvar10 as u32).wrapping_sub(pcvar9 as u32));
                                }
                            } else if (bvar5 & 0x10) == 0 {
                                let src = pcvar9.wrapping_add(0x4c);
                                FID_conflict__memcpy(
                                    pcvar9.cast(),
                                    src.cast(),
                                    (pcvar7 as u32).wrapping_sub(src as u32),
                                );
                                pcvar7 = pcvar7.wrapping_sub(0x4c);
                            } else {
                                pcvar10 = pcvar9.wrapping_add(0x4c);
                            }
                        }
                    }
                }
                local_10 = local_10.wrapping_add(1);
                pcvar8 = pcvar10;
                if !(local_10 < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32) {
                    break;
                }
            }
        }
        DAT_00645a84 = (pcvar7 as u32).wrapping_sub(DAT_00645778 as u32) as i32;
    }
}
