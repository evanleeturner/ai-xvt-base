// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00870: i32;
    static mut DAT_00a00874: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ae2a0(param_1: i32, param_2: i32) -> i32;
    fn FUN_004aeb40(param_1: i32, param_2: i32) -> i32;
    fn FUN_004adc40(param_1: i32) -> i32;
}

unsafe fn lookup(p1: i32, p2: i32) -> i32 {
    if p1 < 0x49 {
        if p2 < 0 {
            0
        } else {
            let mut i = p2;
            // SAFETY: the game's own table memory, indexed as the original does.
            unsafe {
                let n = (&raw mut DAT_00a00870)
                    .wrapping_offset(p1.wrapping_mul(0x65) as isize)
                    .read_unaligned();
                if n <= p2 {
                    i = n.wrapping_sub(1);
                }
                (&raw mut DAT_00a00874)
                    .wrapping_offset(p1.wrapping_mul(0x65).wrapping_add(i) as isize)
                    .read_unaligned()
            }
        }
    } else {
        // SAFETY: the callee is the game's own function.
        unsafe { FUN_004ae2a0(p1, p2) }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00484e10(param_1: i32, param_2: i32) -> u8 {
    // SAFETY: the globals and callees are the game's own.
    unsafe {
        let i_var5 = lookup(param_1, param_2);

        if i_var5 == 0x12 || i_var5 == 0x13 {
            return 0;
        }

        let i_var1 = FUN_004aeb40(param_1, param_2);
        if i_var1 == 0 {
            return 1;
        }
        if i_var1 == 1 && i_var5 != 1 && i_var5 != 3 {
            return 1;
        }

        let i_var2 = if param_1 < 0x49 {
            (&raw mut DAT_00a00870)
                .wrapping_offset(param_1.wrapping_mul(0x65) as isize)
                .read_unaligned()
        } else {
            FUN_004adc40(param_1)
        };

        let mut i_var4: i32 = 0;
        if 0 < i_var2 {
            loop {
                let i_var3 = FUN_004aeb40(param_1, i_var4);
                if i_var3 == i_var1 {
                    let i_var3 = lookup(param_1, i_var4);
                    if i_var5 == i_var3 {
                        return (i_var4 == param_2) as u8;
                    }
                }
                i_var4 = i_var4.wrapping_add(1);
                if i_var4 >= i_var2 {
                    break;
                }
            }
        }
        0
    }
}
