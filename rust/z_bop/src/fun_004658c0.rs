// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8c2c: i32;
    static mut DAT_00556f08: i32;
    static mut DAT_00557360: i32;
    static mut DAT_00a60a18: *mut u8;
    static mut DAT_00a60a1c: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
    fn FUN_00463d00();
    fn FUN_004ac760() -> i32;
    fn FUN_0046d300(param_1: *mut u32, param_2: u32);
    fn FUN_00448f90(param_1: i32, param_2: *mut u8, param_3: i32);
    fn FUN_0046f660(param_1: i32) -> i32;
    fn FUN_00463c50(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004658c0(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: all callees and globals are the game's own; the table at
    // DAT_00556f08 is read as the original does.
    unsafe {
        let mut b_var1: i32 = 0;
        let mut local_4: i32 = 0;
        let mut local_8: i32 = 0;
        let mut local_c: i32 = 0x14;
        let mut i_var3: i32;
        loop {
            let mut i_var7: i32 = 0;
            i_var3 = local_8;
            loop {
                if i_var7 > 0xeb {
                    break;
                }
                i_var3 = FUN_004aa7f0();
                if i_var3 != 0 && FUN_004aa870() == 0x1b {
                    i_var7 = 0xec;
                    local_c = 1;
                    i_var3 = local_8;
                    break;
                }
                let i_var2 = DAT_009a8c2c;
                FUN_00463d00();
                let r = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(r);
                i_var7 = i_var7.wrapping_sub(i_var2).wrapping_add(DAT_009a8c2c);
                i_var3 = 0;
                if param_2 > 0 {
                    let mut p = &raw mut DAT_00556f08;
                    loop {
                        if p.read() == 0 {
                            break;
                        }
                        p = p.add(1);
                        i_var3 = i_var3.wrapping_add(1);
                        if !(i_var3 < param_2) {
                            break;
                        }
                    }
                }
                DAT_009a8c2c = i_var2;
                if i_var3 == param_2 {
                    break;
                }
            }
            if i_var3 == param_2 {
                break;
            }
            local_4 = local_4.wrapping_add(i_var7);
            if local_4 > 0xeb {
                local_4 = 0;
                DAT_00557360 = 0x1d;
                FUN_0046d300(&raw mut DAT_00557360 as *mut u32, 4);
                b_var1 = (b_var1 == 0) as i32;
                let mut pb = DAT_00a60a18;
                if b_var1 != 0 {
                    pb = DAT_00a60a1c;
                }
                FUN_00448f90(3, pb, 0x30);
            }
            if local_8 == i_var3 {
                local_c = local_c.wrapping_sub(1);
            } else {
                local_c = 0x14;
            }
            local_8 = i_var3;
            if local_c == 0 {
                break;
            }
        }
        if local_c != 0 {
            return 1;
        }
        let r = FUN_0046f660(param_1);
        FUN_00463c50(r);
        0
    }
}
