// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db5a8: i32;
    static mut DAT_00536f78: i32;
    static mut DAT_005373d0: u32;
    static mut DAT_005373d4: i32;
    static mut DAT_006a89b8: *mut u8;
    static mut DAT_006a89bc: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004a53f0() -> i32;
    fn FUN_004a5470() -> u32;
    fn FUN_0048f530() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042c710();
    fn FUN_004310c0(param_1: *mut u32, param_2: u32);
    fn FUN_00438c30(param_1: i32, param_2: *mut u8, param_3: i32);
    fn FUN_00432c10(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e660(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the scan reads the game's int array at DAT_00536f78 as the original does.
    unsafe {
        let mut local_c: i32 = 0x14;
        let mut b_var1: i32 = 0;
        let mut local_4: i32 = 0;
        let mut local_8: i32 = 0;
        loop {
            let mut i_var7: i32 = 0;
            let mut i_var3: i32 = local_8;
            loop {
                if 0xeb < i_var7 {
                    break;
                }
                i_var3 = FUN_004a53f0();
                if i_var3 != 0 && FUN_004a5470() == 0x1b {
                    i_var7 = 0xec;
                    local_c = 1;
                    i_var3 = local_8;
                    break;
                }
                let i_var2 = DAT_006db5a8;
                FUN_0042c710();
                let r = FUN_0048f530();
                DAT_006db5a8 = DAT_006db5a8.wrapping_add(r);
                i_var7 = i_var7.wrapping_sub(i_var2).wrapping_add(DAT_006db5a8);
                i_var3 = 0;
                if 0 < param_2 {
                    let mut p: *mut i32 = &raw mut DAT_00536f78;
                    loop {
                        if p.read_unaligned() == 0 {
                            break;
                        }
                        p = p.add(1);
                        i_var3 = i_var3.wrapping_add(1);
                        if !(i_var3 < param_2) {
                            break;
                        }
                    }
                }
                DAT_006db5a8 = i_var2;
                if i_var3 == param_2 {
                    break;
                }
            }
            if i_var3 == param_2 {
                break;
            }
            local_4 = local_4.wrapping_add(i_var7);
            if 0xeb < local_4 {
                local_4 = 0;
                DAT_005373d0 = 0x1d;
                FUN_004310c0(&raw mut DAT_005373d0, 4);
                b_var1 = (b_var1 == 0) as i32;
                let mut pb_var6 = DAT_006a89b8;
                if b_var1 != 0 {
                    pb_var6 = DAT_006a89bc;
                }
                FUN_00438c30(3, pb_var6, 0x30);
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
        DAT_005373d4 = FUN_00432c10(param_1);
        DAT_005373d0 = 0x15;
        FUN_004310c0(&raw mut DAT_005373d0, 8);
        0
    }
}
