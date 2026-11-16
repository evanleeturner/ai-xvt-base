// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527e74: u16;
    static mut DAT_006da9f4: u32;
    static mut DAT_006da538: i32;
    static mut DAT_0052ae18: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045f5b0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b2f40(
    mut param_1: *mut u8,
    param_2: u16,
    param_3: i16,
    param_4: i16,
) {
    let mut local_63: [u8; 99] = [0; 99];
    // SAFETY: the globals and callee are the game's own; the pointers are the
    // caller's buffers and the local scratch, accessed as the original does.
    unsafe {
        let mut pb3: *mut u8 = (DAT_00527e74 as u32).wrapping_add(DAT_006da9f4) as usize as *mut u8;
        if param_3 != 0 {
            let mut local_68: i16 = param_3;
            loop {
                let mut b1: u8 = *param_1;
                if param_4 == 0 {
                    let mut u5: u16 = 0;
                    *pb3 = b1;
                    let mut pb10 = param_1.wrapping_add(1);
                    pb3 = pb3.wrapping_add(1);
                    param_1 = pb10;
                    if param_2 != 0 {
                        loop {
                            b1 = *pb10;
                            param_1 = pb10.wrapping_add(1);
                            'lab: {
                                if b1 == 0 {
                                    *pb3 = 0;
                                    b1 = *param_1;
                                    let mut pb4 = pb3.wrapping_add(1);
                                    param_1 = pb10.wrapping_add(2);
                                    let mut u9 = u5.wrapping_add(0xff);
                                    if DAT_006da538 != 0x140 {
                                        if b1 == 0 {
                                            *pb4 = 0;
                                            b1 = *param_1;
                                            param_1 = pb10.wrapping_add(3);
                                            pb4 = pb3.wrapping_add(2);
                                            u9 = u5.wrapping_add(0x1ff);
                                        } else if b1 == 0xff {
                                            u5 = u5.wrapping_add(0x1ff);
                                            *pb4 = 0;
                                            pb3 = pb3.wrapping_add(2);
                                            b1 = 0;
                                            break 'lab;
                                        }
                                    }
                                    u5 = u9;
                                    pb3 = pb4;
                                    b1 = b1.wrapping_add(1);
                                }
                            }
                            *pb3 = b1;
                            pb3 = pb3.wrapping_add(1);
                            u5 = u5.wrapping_add(b1 as u16);
                            pb10 = param_1;
                            if !(u5 < param_2) {
                                break;
                            }
                        }
                    }
                } else {
                    let mut u5: u16 = 0;
                    param_1 = param_1.wrapping_add(1);
                    let mut pb10: *mut u8 = local_63.as_mut_ptr();
                    let mut pb4 = param_1;
                    if param_2 != 0 {
                        loop {
                            let mut b2 = *pb4;
                            param_1 = pb4.wrapping_add(1);
                            'lab2: {
                                if b2 == 0 {
                                    *pb10 = 0;
                                    b2 = *param_1;
                                    let mut pb7 = pb10.wrapping_add(1);
                                    param_1 = pb4.wrapping_add(2);
                                    let mut u9 = u5.wrapping_add(0xff);
                                    if DAT_006da538 != 0x140 {
                                        if b2 == 0 {
                                            *pb7 = 0;
                                            b2 = *param_1;
                                            param_1 = pb4.wrapping_add(3);
                                            pb7 = pb10.wrapping_add(2);
                                            u9 = u5.wrapping_add(0x1ff);
                                        } else if b2 == 0xff {
                                            u5 = u5.wrapping_add(0x1ff);
                                            *pb7 = 0;
                                            pb10 = pb10.wrapping_add(2);
                                            b2 = 0;
                                            break 'lab2;
                                        }
                                    }
                                    u5 = u9;
                                    pb10 = pb7;
                                    b2 = b2.wrapping_add(1);
                                }
                            }
                            *pb10 = b2;
                            pb10 = pb10.wrapping_add(1);
                            u5 = u5.wrapping_add(b2 as u16);
                            pb4 = param_1;
                            if !(u5 < param_2) {
                                break;
                            }
                        }
                    }
                    let mut u9: u16 = 0;
                    pb10 = local_63.as_mut_ptr();
                    *pb3 = b1;
                    u5 = 0;
                    pb4 = pb10;
                    if param_2 != 0 {
                        loop {
                            let mut u6 = *pb4 as u16;
                            pb10 = pb4.wrapping_add(1);
                            let mut u8 = u9;
                            if *pb4 == 0 {
                                u8 = u9.wrapping_add(0xff);
                                b1 = *pb10;
                                u6 = b1 as u16;
                                *pb4 = b1;
                                pb10 = pb4.wrapping_add(2);
                                *pb4.wrapping_add(1) = 0;
                                if b1 == 0 {
                                    u8 = u9.wrapping_add(0x1ff);
                                    u6 = *pb10 as u16;
                                    *pb4 = *pb10;
                                    pb10 = pb4.wrapping_add(3);
                                    *pb4.wrapping_add(2) = 0;
                                }
                            }
                            u5 = u5.wrapping_add(1);
                            u9 = u8.wrapping_add(u6);
                            pb4 = pb10;
                            if !(u9 < param_2) {
                                break;
                            }
                        }
                    }
                    if (u5 & 1) == 0 {
                        *pb3 = 0u8.wrapping_sub(*pb3);
                    }
                    pb10 = pb10.wrapping_sub(1);
                    pb3 = pb3.wrapping_add(1);
                    while u5 != 0 {
                        b1 = *pb10;
                        let mut pb7 = pb10.wrapping_sub(1);
                        *pb3 = b1;
                        let mut pb4b = pb3.wrapping_add(1);
                        if b1 == 0 {
                            b1 = *pb7;
                            pb7 = pb10.wrapping_sub(2);
                            *pb4b = b1;
                            pb4b = pb3.wrapping_add(2);
                            if b1 == 0 {
                                b1 = *pb7;
                                pb7 = pb10.wrapping_sub(3);
                                *pb4b = b1;
                                pb4b = pb3.wrapping_add(3);
                            }
                        }
                        pb10 = pb7;
                        pb3 = pb4b;
                        u5 = u5.wrapping_sub(1);
                    }
                }
                local_68 = local_68.wrapping_sub(1);
                if local_68 == 0 {
                    break;
                }
            }
        }
        if DAT_0052ae18 != 0 {
            FUN_0045f5b0();
        }
    }
}
