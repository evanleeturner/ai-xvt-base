// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e8f: u8;
    static mut DAT_006db7b4: i32;
    static mut DAT_00732f3c: i32;
    static mut DAT_00525518: i32;
    static mut DAT_00703ae0: u8;
    static mut DAT_007091b0: u8;
    static mut DAT_006db5c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a6a80();
    fn FUN_004774e0(a: i32, b: i32, c: i32, d: i32, e: f32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00465eb0(
    param_1: *mut u8,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    let mut param_1 = param_1;
    let mut param_4 = param_4;
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's pixel buffer, read and written where the original does.
    unsafe {
        if DAT_00732e8f == 0 {
            FUN_004a6a80();
        }
        let f_var3: f32 = DAT_006db7b4 as f32;
        let u_var2: f32 = DAT_00732f3c as f32;
        if DAT_00525518 == 2 {
            while param_4 < param_6 {
                if param_3 < param_5 {
                    let mut i_var1: i32 = param_3.wrapping_sub(param_5);
                    let mut i_var4: i32 = param_3;
                    let mut i_var5: i32 = param_3;
                    let mut pb6: *mut u8 = param_1;
                    let mut skip_top = param_5 <= param_3;
                    loop {
                        if !skip_top {
                            if param_1.add(1).read() != 0x80 {
                                param_1 = param_1.wrapping_add(2);
                                i_var4 = i_var4.wrapping_add(1);
                                if i_var4 < param_5 {
                                    continue;
                                }
                            }
                            i_var1 = i_var4.wrapping_sub(param_5);
                            i_var5 = i_var4;
                            pb6 = param_1;
                        }
                        skip_top = false;
                        param_1 = pb6;
                        if i_var5 == param_5 {
                            break;
                        }
                        i_var4 = i_var5;
                        let mut pb7 = pb6;
                        if (i_var5 < param_5) != (i_var1 < 0) {
                            loop {
                                param_1 = pb7;
                                if pb7.add(1).read() != 0x80 {
                                    break;
                                }
                                param_1 = pb7.wrapping_add(2);
                                i_var4 = i_var4.wrapping_add(1);
                                let idx = pb7.read() as usize;
                                pb7.add(1).write((&raw mut DAT_00703ae0).add(idx).read());
                                pb7.write((&raw mut DAT_007091b0).add(idx).read());
                                pb7 = param_1;
                                if !(i_var4 < param_5) {
                                    break;
                                }
                            }
                        }
                        FUN_004774e0(
                            pb6 as usize as i32,
                            i_var5,
                            i_var4,
                            param_4,
                            u_var2 / f_var3,
                        );
                        if !(i_var4 < param_5) {
                            break;
                        }
                    }
                }
                param_1 = param_1.wrapping_offset(param_2 as isize);
                param_4 = param_4.wrapping_add(1);
            }
        } else {
            while param_4 < param_6 {
                if param_3 < param_5 {
                    let mut i_var1: i32 = param_3.wrapping_sub(param_5);
                    let mut i_var4: i32 = param_3;
                    let mut i_var5: i32 = param_3;
                    let mut pb6: *mut u8 = param_1;
                    let mut skip_top = param_5 <= param_3;
                    loop {
                        if !skip_top {
                            if 0x3f < param_1.read() {
                                param_1 = param_1.wrapping_add(1);
                                i_var4 = i_var4.wrapping_add(1);
                                if i_var4 < param_5 {
                                    continue;
                                }
                            }
                            i_var1 = i_var4.wrapping_sub(param_5);
                            i_var5 = i_var4;
                            pb6 = param_1;
                        }
                        skip_top = false;
                        param_1 = pb6;
                        if i_var5 == param_5 {
                            break;
                        }
                        i_var4 = i_var5;
                        let mut pb7 = pb6;
                        if (i_var5 < param_5) != (i_var1 < 0) {
                            loop {
                                param_1 = pb7;
                                if 0x3f < pb7.read() {
                                    break;
                                }
                                param_1 = pb7.wrapping_add(1);
                                i_var4 = i_var4.wrapping_add(1);
                                let idx = pb7.read() as usize;
                                pb7.write((&raw mut DAT_006db5c0).add(idx).read());
                                pb7 = param_1;
                                if !(i_var4 < param_5) {
                                    break;
                                }
                            }
                        }
                        FUN_004774e0(
                            pb6 as usize as i32,
                            i_var5,
                            i_var4,
                            param_4,
                            u_var2 / f_var3,
                        );
                        if !(i_var4 < param_5) {
                            break;
                        }
                    }
                }
                param_1 = param_1.wrapping_offset(param_2 as isize);
                param_4 = param_4.wrapping_add(1);
            }
        }
        if DAT_00732e8f == 0 {
            FUN_004a6f90();
        }
    }
}
