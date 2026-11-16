// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00550880: *mut i8;
    static mut DAT_00550b8c: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a8e24: i32;
    static mut DAT_009cd274: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__memcpy(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00417a60(param_1: *mut u8) {
    // SAFETY: all globals and the buffers they point to belong to the game;
    // param_1 points at the caller's packed data, read where the original does.
    unsafe {
        let mut p7: *mut i8 = DAT_00550880.wrapping_offset(DAT_00550b8c as isize);
        let i_var1: i32 = (param_1 as *const i32).read_unaligned();
        let mut local_c: i32 = 0;
        let mut local_10: i32 = 0;
        let mut p8: *mut i8 = DAT_00550880;
        let mut src: *const u8 = param_1.wrapping_add(4);

        if 0 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            loop {
                let mut p10: *mut i8 = p8;
                if local_10 < DAT_009a8e24 || DAT_009cd274 <= local_10 {
                    let mut b5: u8;
                    if local_c == 0 {
                        if i_var1 <= local_10 {
                            break;
                        }
                        b5 = src.read();
                        src = src.wrapping_add(1);
                        if b5 & 0x80 != 0 {
                            local_c = ((b5 & 0x7f) as u32).wrapping_sub(1) as i32;
                            b5 = 0;
                        }
                    } else {
                        local_c = local_c.wrapping_sub(1);
                        b5 = 0;
                    }
                    let mut p9: *mut i8 = p8.wrapping_add(1);
                    p10 = p9;
                    if *p8 == 0 {
                        if b5 & 1 != 0 {
                            p10 = p8.wrapping_add(0x24);
                            let n = (p7 as u32).wrapping_sub(p9 as u32);
                            p7 = p7.wrapping_add(0x23);
                            FID_conflict__memcpy(p10.cast(), p9.cast(), n);
                            p9.write_bytes(0, (p10 as u32).wrapping_sub(p9 as u32) as usize);
                        }
                    } else if b5 & 1 == 0 {
                        let s = p8.wrapping_add(0x24);
                        FID_conflict__memcpy(
                            p9.cast(),
                            s.cast(),
                            (p7 as u32).wrapping_sub(s as u32),
                        );
                        p7 = p7.wrapping_sub(0x23);
                    } else {
                        p9 = p8.wrapping_add(0x24);
                        p10 = p9;
                        let mut i_tmp = (p8.wrapping_add(0x20) as *const i32).read_unaligned();
                        if i_tmp == 0 {
                            if b5 & 2 != 0 {
                                p10 = p8.wrapping_add(0xd5);
                                let n = (p7 as u32).wrapping_sub(p9 as u32);
                                p7 = p7.wrapping_add(0xb1);
                                FID_conflict__memcpy(p10.cast(), p9.cast(), n);
                                p9.write_bytes(0, (p10 as u32).wrapping_sub(p9 as u32) as usize);
                            }
                        } else if b5 & 2 == 0 {
                            let s = p8.wrapping_add(0xd5);
                            FID_conflict__memcpy(
                                p9.cast(),
                                s.cast(),
                                (p7 as u32).wrapping_sub(s as u32),
                            );
                            p7 = p7.wrapping_sub(0xb1);
                        } else {
                            p9 = p8.wrapping_add(0xd5);
                            p10 = p9;
                            i_tmp = (p8.wrapping_add(0xcd) as *const i32).read_unaligned();
                            if i_tmp == 0 {
                                if b5 & 4 != 0 {
                                    p10 = p8.wrapping_add(0x537);
                                    let n = (p7 as u32).wrapping_sub(p9 as u32);
                                    p7 = p7.wrapping_add(0x462);
                                    FID_conflict__memcpy(p10.cast(), p9.cast(), n);
                                    p9.write_bytes(
                                        0,
                                        (p10 as u32).wrapping_sub(p9 as u32) as usize,
                                    );
                                }
                            } else if b5 & 4 == 0 {
                                let s = p8.wrapping_add(0x537);
                                FID_conflict__memcpy(
                                    p9.cast(),
                                    s.cast(),
                                    (p7 as u32).wrapping_sub(s as u32),
                                );
                                p7 = p7.wrapping_sub(0x462);
                            } else {
                                p10 = p8.wrapping_add(0x537);
                            }
                            p9 = p10;
                            i_tmp = (p8.wrapping_add(0xc9) as *const i32).read_unaligned();
                            if i_tmp == 0 {
                                if b5 & 8 != 0 {
                                    p9 = p10.wrapping_add(10);
                                    let n = (p7 as u32).wrapping_sub(p10 as u32);
                                    p7 = p7.wrapping_add(10);
                                    FID_conflict__memcpy(p9.cast(), p10.cast(), n);
                                    p10.write_bytes(
                                        0,
                                        (p9 as u32).wrapping_sub(p10 as u32) as usize,
                                    );
                                }
                            } else if b5 & 8 == 0 {
                                let s = p10.wrapping_add(10);
                                FID_conflict__memcpy(
                                    p10.cast(),
                                    s.cast(),
                                    (p7 as u32).wrapping_sub(s as u32),
                                );
                                p7 = p7.wrapping_sub(10);
                            } else {
                                p9 = p10.wrapping_add(10);
                            }
                            p10 = p9;
                            i_tmp = (p8.wrapping_add(0xd1) as *const i32).read_unaligned();
                            if i_tmp == 0 {
                                if b5 & 0x10 != 0 {
                                    p10 = p9.wrapping_add(0x4c);
                                    let n = (p7 as u32).wrapping_sub(p9 as u32);
                                    p7 = p7.wrapping_add(0x4c);
                                    FID_conflict__memcpy(p10.cast(), p9.cast(), n);
                                    p9.write_bytes(
                                        0,
                                        (p10 as u32).wrapping_sub(p9 as u32) as usize,
                                    );
                                }
                            } else if b5 & 0x10 == 0 {
                                let s = p9.wrapping_add(0x4c);
                                FID_conflict__memcpy(
                                    p9.cast(),
                                    s.cast(),
                                    (p7 as u32).wrapping_sub(s as u32),
                                );
                                p7 = p7.wrapping_sub(0x4c);
                            } else {
                                p10 = p9.wrapping_add(0x4c);
                            }
                        }
                    }
                }
                local_10 = local_10.wrapping_add(1);
                p8 = p10;
                if !(local_10 < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }
        DAT_00550b8c = (p7 as i32).wrapping_sub(DAT_00550880 as i32);
    }
}
