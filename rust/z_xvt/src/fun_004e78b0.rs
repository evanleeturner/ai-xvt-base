// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_0081bfba: i32;
    static mut DAT_0081fb96: i32;
    static mut DAT_00533304: *mut i8;
    static mut DAT_005332f0: *mut i8;
    static mut DAT_005332dc: *mut i8;
    static mut DAT_005332c8: *mut i8;
    static mut DAT_005332b4: *mut i8;
    static mut DAT_005332a0: *mut i8;
    static mut DAT_0053328c: *mut i8;
    static mut DAT_0075766c: i32;
    static mut DAT_007438e4: i8;
    static mut DAT_007438d7: i8;
    static mut DAT_00533260: *mut i8;
    static mut DAT_0053324c: *mut i8;
    static mut DAT_00533274: *mut i8;
    static mut DAT_00532a4c: *mut i8;
    static mut DAT_00531358: *mut i8;
    static mut DAT_0081bc68: i32;
    static mut DAT_00531eac: *mut i8;
    static mut DAT_00531ebc: *mut i8;
    static mut DAT_00532a84: *mut i8;
    static mut DAT_00533240: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0290(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_004d52b0();
    fn FUN_004d1750(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d0580(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d5350(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e78b0() -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the indexed reads stay where the original reads them.
    unsafe {
        let pc: *mut i8 = 'sel: {
            if DAT_0081b850 == 2 {
                if DAT_0081bfba == 0 {
                    if DAT_0081fb96 == 0 {
                        break 'sel DAT_00533304;
                    } else {
                        break 'sel DAT_005332f0;
                    }
                }
                if DAT_0081bfba == 3 {
                    if DAT_0081fb96 == 0 {
                        break 'sel DAT_005332dc;
                    } else {
                        break 'sel DAT_005332c8;
                    }
                }
                if DAT_0081bfba == 4 {
                    if DAT_0081fb96 == 0 {
                        break 'sel DAT_005332b4;
                    } else {
                        break 'sel DAT_005332a0;
                    }
                }
                if DAT_0081bfba == 1 {
                    break 'sel DAT_0053328c;
                }
            } else {
                if DAT_0081bfba == 0 {
                    let mut i_var1: i32 = 0;
                    if 0 < DAT_0075766c {
                        let mut p: *mut i8 = &raw mut DAT_007438e4;
                        loop {
                            if p.read() != 0 {
                                break;
                            }
                            p = p.byte_add(0x562);
                            i_var1 = i_var1.wrapping_add(1);
                            if !(i_var1 < DAT_0075766c) {
                                break;
                            }
                        }
                    }
                    let v = (&raw mut DAT_007438d7)
                        .byte_offset((i_var1.wrapping_mul(0x562)) as isize)
                        .read();
                    if v == 0 {
                        break 'sel DAT_00533304;
                    } else {
                        break 'sel DAT_005332f0;
                    }
                }
                if DAT_0081bfba == 3 {
                    break 'sel DAT_00533260;
                }
                if DAT_0081bfba == 4 {
                    break 'sel DAT_0053324c;
                }
                if DAT_0081bfba == 1 {
                    break 'sel DAT_0053328c;
                }
            }
            DAT_00533274
        };
        FUN_004d0290(pc, DAT_00532a4c);
        FUN_004d52b0();
        FUN_004d1750(DAT_00532a4c, 0, 0);
        FUN_004d0e60(DAT_00531358, 0, 0);
        let pc2: *mut i8 = if DAT_0081bc68 == 0 {
            DAT_00531eac
        } else {
            DAT_00531ebc
        };
        FUN_004d0e60(pc2, 0, 0);
        if DAT_0081b850 != 2 {
            FUN_004d0580(DAT_00532a84, 0, 0);
        }
        FUN_004d0580(DAT_00533240, 0, 0);
        FUN_004d5350(1);
        1
    }
}
