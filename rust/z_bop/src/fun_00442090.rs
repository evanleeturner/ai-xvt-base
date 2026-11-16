// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052747c: u16;
    static mut DAT_009a8074: u32;
    static mut DAT_009a7bac: i32;
    static mut DAT_00527e90: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040b520();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442090(
    mut param_1: *mut u8,
    param_2: u16,
    param_3: i16,
    param_4: i16,
) {
    let mut local_63: [u8; 99] = [0; 99];
    // SAFETY: the globals and callee are the game's own; the pointers walk the
    // caller's run-length buffers and the game's output buffer as the original does.
    unsafe {
        let mut pb4: *mut u8 = (DAT_0052747c as u32).wrapping_add(DAT_009a8074) as *mut u8;
        if param_3 != 0 {
            let mut local_68: i16 = param_3;
            loop {
                let mut b1: u8 = *param_1;
                if param_4 == 0 {
                    let mut u9: u16 = 0;
                    *pb4 = b1;
                    let mut pb3 = param_1.wrapping_add(1);
                    pb4 = pb4.wrapping_add(1);
                    param_1 = pb3;
                    if param_2 != 0 {
                        loop {
                            b1 = *pb3;
                            param_1 = pb3.wrapping_add(1);
                            'lab: {
                                if b1 == 0 {
                                    *pb4 = 0;
                                    b1 = *param_1;
                                    let mut pb5 = pb4.wrapping_add(1);
                                    param_1 = pb3.wrapping_add(2);
                                    let mut u8v = u9.wrapping_add(0xff);
                                    if DAT_009a7bac != 0x140 {
                                        if b1 == 0 {
                                            *pb5 = 0;
                                            b1 = *param_1;
                                            param_1 = pb3.wrapping_add(3);
                                            pb5 = pb4.wrapping_add(2);
                                            u8v = u9.wrapping_add(0x1ff);
                                        } else if b1 == 0xff {
                                            u9 = u9.wrapping_add(0x1ff);
                                            *pb5 = 0;
                                            pb4 = pb4.wrapping_add(2);
                                            b1 = 0;
                                            break 'lab;
                                        }
                                    }
                                    u9 = u8v;
                                    pb4 = pb5;
                                    b1 = b1.wrapping_add(1);
                                }
                            }
                            *pb4 = b1;
                            pb4 = pb4.wrapping_add(1);
                            u9 = u9.wrapping_add(b1 as u16);
                            pb3 = param_1;
                            if !(u9 < param_2) {
                                break;
                            }
                        }
                    }
                } else {
                    let mut u9: u16 = 0;
                    param_1 = param_1.wrapping_add(1);
                    let mut pb3: *mut u8 = local_63.as_mut_ptr();
                    let mut pb5: *mut u8 = param_1;
                    if param_2 != 0 {
                        loop {
                            let mut b2 = *pb5;
                            param_1 = pb5.wrapping_add(1);
                            'lab2: {
                                if b2 == 0 {
                                    *pb3 = 0;
                                    b2 = *param_1;
                                    let mut pb10 = pb3.wrapping_add(1);
                                    param_1 = pb5.wrapping_add(2);
                                    let mut u8v = u9.wrapping_add(0xff);
                                    if DAT_009a7bac != 0x140 {
                                        if b2 == 0 {
                                            *pb10 = 0;
                                            b2 = *param_1;
                                            param_1 = pb5.wrapping_add(3);
                                            pb10 = pb3.wrapping_add(2);
                                            u8v = u9.wrapping_add(0x1ff);
                                        } else if b2 == 0xff {
                                            u9 = u9.wrapping_add(0x1ff);
                                            *pb10 = 0;
                                            pb3 = pb3.wrapping_add(2);
                                            b2 = 0;
                                            break 'lab2;
                                        }
                                    }
                                    u9 = u8v;
                                    pb3 = pb10;
                                    b2 = b2.wrapping_add(1);
                                }
                            }
                            *pb3 = b2;
                            pb3 = pb3.wrapping_add(1);
                            u9 = u9.wrapping_add(b2 as u16);
                            pb5 = param_1;
                            if !(u9 < param_2) {
                                break;
                            }
                        }
                    }
                    let mut u8v: u16 = 0;
                    pb3 = local_63.as_mut_ptr();
                    *pb4 = b1;
                    u9 = 0;
                    pb5 = pb3;
                    if param_2 != 0 {
                        loop {
                            let mut u6 = *pb5 as u16;
                            pb3 = pb5.wrapping_add(1);
                            let mut u7 = u8v;
                            if *pb5 == 0 {
                                u7 = u8v.wrapping_add(0xff);
                                b1 = *pb3;
                                u6 = b1 as u16;
                                *pb5 = b1;
                                pb3 = pb5.wrapping_add(2);
                                *pb5.wrapping_add(1) = 0;
                                if b1 == 0 {
                                    u7 = u8v.wrapping_add(0x1ff);
                                    u6 = *pb3 as u16;
                                    *pb5 = *pb3;
                                    pb3 = pb5.wrapping_add(3);
                                    *pb5.wrapping_add(2) = 0;
                                }
                            }
                            u9 = u9.wrapping_add(1);
                            u8v = u7.wrapping_add(u6);
                            pb5 = pb3;
                            if !(u8v < param_2) {
                                break;
                            }
                        }
                    }
                    if (u9 & 1) == 0 {
                        *pb4 = (*pb4).wrapping_neg();
                    }
                    pb3 = pb3.wrapping_sub(1);
                    pb4 = pb4.wrapping_add(1);
                    while u9 != 0 {
                        b1 = *pb3;
                        let mut pb10 = pb3.wrapping_sub(1);
                        *pb4 = b1;
                        let mut pb5b = pb4.wrapping_add(1);
                        if b1 == 0 {
                            b1 = *pb10;
                            pb10 = pb3.wrapping_sub(2);
                            *pb5b = b1;
                            pb5b = pb4.wrapping_add(2);
                            if b1 == 0 {
                                b1 = *pb10;
                                pb10 = pb3.wrapping_sub(3);
                                *pb5b = b1;
                                pb5b = pb4.wrapping_add(3);
                            }
                        }
                        pb3 = pb10;
                        pb4 = pb5b;
                        u9 = u9.wrapping_sub(1);
                    }
                }
                local_68 = local_68.wrapping_sub(1);
                if local_68 == 0 {
                    break;
                }
            }
        }
        if DAT_00527e90 != 0 {
            FUN_0040b520();
        }
    }
}
