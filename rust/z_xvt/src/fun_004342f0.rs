// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da4f8: i32;
    static mut DAT_006db59c: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments)]
pub unsafe extern "stdcall" fn FUN_004342f0() {
    // SAFETY: the list nodes and globals are the game's own memory; nodes are
    // read and written at the offsets the original uses (value at 0, next at 8).
    unsafe {
        let mut local_8: *mut i32 = core::ptr::null_mut();
        let mut local_4: i32;
        let mut i_var1: i32;
        let mut pi2: *mut i32;
        let mut pi3: *mut i32;
        let mut pi4: *mut i32;
        let mut pi6: *mut i32;
        let mut pi7: *mut i32;
        let mut pi8: *mut i32;

        let mut i_var5: i32 = 1;
        if 1 < DAT_006da4f8 {
            loop {
                pi6 = core::ptr::null_mut();
                local_4 = 0;
                pi3 = DAT_006db59c;
                pi8 = local_8;
                'd7: {
                    if 0 < DAT_006da4f8 {
                        loop {
                            i_var1 = 0;
                            pi4 = pi3;
                            local_8 = pi8;
                            if 0 < i_var5 {
                                loop {
                                    pi8 = pi4;
                                    if pi8.is_null() {
                                        break 'd7;
                                    }
                                    i_var1 = i_var1.wrapping_add(1);
                                    pi4 = pi8.add(2).read_unaligned() as usize as *mut i32;
                                    local_8 = pi8;
                                    if !(i_var1 < i_var5) {
                                        break;
                                    }
                                }
                            }
                            local_8 = pi8;
                            if pi4.is_null() {
                                break;
                            }
                            i_var1 = 0;
                            let mut joined = false;
                            pi2 = core::ptr::null_mut();
                            if 0 < i_var5 {
                                loop {
                                    loop {
                                        pi2 = pi3;
                                        pi3 = pi2;
                                        pi7 = pi6;
                                        if pi4.read_unaligned() < pi2.read_unaligned() {
                                            break;
                                        }
                                        pi3 = pi2.add(2).read_unaligned() as usize as *mut i32;
                                        pi6 = pi2;
                                        pi7 = pi2;
                                        if !(pi8 != pi2) {
                                            break;
                                        }
                                    }
                                    pi2 = pi4;
                                    pi6 = pi8;
                                    if pi8 == pi7 {
                                        joined = true;
                                        break;
                                    }
                                    pi8.add(2).write_unaligned(pi4.add(2).read_unaligned());
                                    if pi7.is_null() {
                                        DAT_006db59c = pi4;
                                        pi4.add(2).write_unaligned(pi3 as usize as i32);
                                        pi6 = DAT_006db59c;
                                    } else {
                                        pi7.add(2).write_unaligned(pi4 as usize as i32);
                                        pi4.add(2).write_unaligned(pi3 as usize as i32);
                                        pi6 = pi4;
                                    }
                                    pi4 = pi8.add(2).read_unaligned() as usize as *mut i32;
                                    if pi4.is_null() {
                                        break;
                                    }
                                    i_var1 = i_var1.wrapping_add(1);
                                    if !(i_var1 < i_var5) {
                                        break;
                                    }
                                }
                            }
                            if !joined {
                                let b_var9 = pi8 == pi6;
                                pi6 = pi8;
                                pi2 = pi4;
                                if b_var9 {
                                    joined = true;
                                }
                            }
                            if joined {
                                loop {
                                    pi4 = pi2;
                                    if !(i_var1 < i_var5 && !pi4.is_null()) {
                                        break;
                                    }
                                    i_var1 = i_var1.wrapping_add(1);
                                    pi6 = pi4;
                                    pi2 = pi4.add(2).read_unaligned() as usize as *mut i32;
                                }
                            }
                            local_8 = pi6;
                            if pi4.is_null() {
                                break;
                            }
                            local_4 = local_4.wrapping_add(i_var5.wrapping_mul(2));
                            pi3 = pi4;
                            pi8 = pi6;
                            if DAT_006da4f8 <= local_4 {
                                break;
                            }
                        }
                    }
                }
                i_var5 = i_var5.wrapping_mul(2);
                if !(i_var5 < DAT_006da4f8) {
                    break;
                }
            }
        }
    }
}
