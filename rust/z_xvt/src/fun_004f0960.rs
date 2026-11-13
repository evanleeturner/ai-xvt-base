// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_00757cee: i32;
    static mut DAT_007579b0: i32;
    static mut DAT_007579d0: i32;
    static mut DAT_00757a28: i32;
    static mut DAT_00757874: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004cde00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f0960() -> i32 {
    // SAFETY: all globals and the callee are the game's own memory; pointers
    // walk the same tables, at the same strides, as the original.
    unsafe {
        let mut i_var1: i32 = 1;
        if DAT_0081b850 != 2 {
            i_var1 = FUN_004cde00();
        }
        let mut i_var3: i32 = 0;
        if 0 < i_var1 {
            let mut pi_var2: *mut i32 = &raw mut DAT_00757cee;
            loop {
                let mut i_var5: i32 = 0;
                let mut pi_var4: *mut i32 = &raw mut DAT_007579b0;
                let end = &raw mut DAT_007579d0 as usize;
                loop {
                    let v2 = pi_var2.read_unaligned();
                    if v2 != 0 && pi_var4.read_unaligned() == v2 {
                        break;
                    }
                    pi_var4 = pi_var4.add(1);
                    i_var5 = i_var5.wrapping_add(1);
                    if (pi_var4 as usize) >= end {
                        break;
                    }
                }
                if i_var5 == 8 {
                    return 0;
                }
                pi_var2 = pi_var2.byte_add(0x2a);
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= i_var1 {
                    break;
                }
            }
        }
        i_var1 = 0;
        if 0 < DAT_00757a28 {
            let mut pi_var2: *mut i32 = &raw mut DAT_00757874;
            loop {
                i_var3 = 1;
                let mut pi_var4: *mut i32 = pi_var2;
                loop {
                    if pi_var4.read_unaligned() != 0 && pi_var2.sub(1).read_unaligned() == 0 {
                        return 0;
                    }
                    pi_var4 = pi_var4.add(1);
                    i_var3 = i_var3.wrapping_add(1);
                    if i_var3 >= 8 {
                        break;
                    }
                }
                pi_var2 = pi_var2.add(8);
                i_var1 = i_var1.wrapping_add(1);
                if i_var1 >= DAT_00757a28 {
                    break;
                }
            }
        }
        1
    }
}
