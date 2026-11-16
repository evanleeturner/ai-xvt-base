// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d682c: u16;
    static mut DAT_009a1fe4: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004aba50();
    fn FUN_004377b0(a: i32, b: i32, c: i32, d: u8);
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00437860(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u8,
) {
    // SAFETY: globals and callees are the game's own; values are read in the original's order.
    unsafe {
        let i_var1 = param_2.wrapping_add(param_4);
        if !(0 < i_var1
            && 0 < param_1.wrapping_add(param_3)
            && param_1 < DAT_009d682c as i32
            && param_2 < DAT_009a1fe4 as i32
            && 0 < param_4
            && 0 < param_3)
        {
            return;
        }
        let mut local_8 = param_4 >> 3;
        let mut i_var5 = param_3 >> 3;
        if i_var5 < 3 {
            i_var5 = 3;
        }
        if local_8 < 3 {
            local_8 = 3;
        }
        if param_3 < i_var5 {
            i_var5 = param_3;
        }
        if param_4 < local_8 {
            local_8 = param_4;
        }
        FUN_004aba50();
        if -1 < param_2 {
            let mut u_var3 = param_1.wrapping_add(i_var5);
            if 0 < u_var3 && param_1 < DAT_009d682c as i32 {
                let u_var2 = DAT_009d682c as i32;
                let mut i_var4 = param_1;
                if param_1 < 0 {
                    i_var4 = 0;
                }
                if u_var2 < u_var3 {
                    u_var3 = u_var2;
                }
                FUN_004377b0(i_var4, u_var3, param_2, param_5);
            }
            let mut i_var4 = param_3.wrapping_sub(i_var5).wrapping_add(param_1);
            if 0 < param_1.wrapping_add(param_3) && i_var4 < DAT_009d682c as i32 {
                let u_var3 = DAT_009d682c as i32;
                if i_var4 < 0 {
                    i_var4 = 0;
                }
                let mut u_var2 = param_1.wrapping_add(param_3);
                if u_var3 < param_1.wrapping_add(param_3) {
                    u_var2 = u_var3;
                }
                FUN_004377b0(i_var4, u_var2, param_2, param_5);
            }
        }
        if i_var1 <= DAT_009a1fe4 as i32 {
            let mut u_var3 = param_1.wrapping_add(i_var5);
            if 0 < u_var3 && param_1 < DAT_009d682c as i32 {
                let u_var2 = DAT_009d682c as i32;
                let mut i_var4 = param_1;
                if param_1 < 0 {
                    i_var4 = 0;
                }
                if u_var2 < u_var3 {
                    u_var3 = u_var2;
                }
                FUN_004377b0(i_var4, u_var3, i_var1.wrapping_sub(1), param_5);
            }
            let mut i_var6 = param_3.wrapping_sub(i_var5).wrapping_add(param_1);
            if 0 < param_1.wrapping_add(param_3) && i_var6 < DAT_009d682c as i32 {
                let u_var3 = DAT_009d682c as i32;
                if i_var6 < 0 {
                    i_var6 = 0;
                }
                let mut u_var2 = param_1.wrapping_add(param_3);
                if u_var3 < param_1.wrapping_add(param_3) {
                    u_var2 = u_var3;
                }
                FUN_004377b0(i_var6, u_var2, i_var1.wrapping_sub(1), param_5);
            }
        }
        let mut i_var1b = 1i32;
        if 1 < local_8 {
            loop {
                let i_var5b = param_2.wrapping_add(i_var1b);
                if -1 < i_var5b && i_var5b < DAT_009a1fe4 as i32 {
                    if -1 < param_1 {
                        FUN_004377b0(param_1, param_1.wrapping_add(1), i_var5b, param_5);
                    }
                    if param_1.wrapping_add(param_3) <= DAT_009d682c as i32 {
                        FUN_004377b0(
                            param_1.wrapping_sub(1).wrapping_add(param_3),
                            param_1.wrapping_add(param_3),
                            i_var5b,
                            param_5,
                        );
                    }
                }
                i_var1b = i_var1b.wrapping_add(1);
                if !(i_var1b < local_8) {
                    break;
                }
            }
        }
        let mut i_var1c = param_4.wrapping_sub(local_8);
        while i_var1c < param_4.wrapping_sub(1) {
            let i_var5b = param_2.wrapping_add(i_var1c);
            if local_8 <= i_var1c && -1 < i_var5b && i_var5b < DAT_009a1fe4 as i32 {
                if -1 < param_1 {
                    FUN_004377b0(param_1, param_1.wrapping_add(1), i_var5b, param_5);
                }
                if param_1.wrapping_add(param_3) <= DAT_009d682c as i32 {
                    FUN_004377b0(
                        param_1.wrapping_sub(1).wrapping_add(param_3),
                        param_1.wrapping_add(param_3),
                        i_var5b,
                        param_5,
                    );
                }
            }
            i_var1c = i_var1c.wrapping_add(1);
        }
        FUN_004abe50();
    }
}
