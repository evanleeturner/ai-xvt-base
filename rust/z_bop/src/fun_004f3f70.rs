// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa615e: i32;
    static mut DAT_00aa5df0: i32;
    static mut DAT_00aa5e10: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5cb4: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d1400() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f3f70() -> i32 {
    // SAFETY: all globals and the callee belong to the game; pointers are
    // derived from the game's own addresses and read where the original reads.
    unsafe {
        let mut i_var1: i32 = 1;
        if DAT_00b69e30 != 2 {
            i_var1 = FUN_004d1400();
        }
        let mut i_var2: i32 = 0;
        if 0 < i_var1 {
            let mut pi_var5: *mut i32 = &raw mut DAT_00aa615e;
            loop {
                let mut i_var3: i32 = 0;
                let mut pi_var4: *mut i32 = &raw mut DAT_00aa5df0;
                let end: *mut i32 = &raw mut DAT_00aa5e10;
                loop {
                    let v5 = pi_var5.read_unaligned();
                    if v5 != 0 && pi_var4.read_unaligned() == v5 {
                        break;
                    }
                    pi_var4 = pi_var4.wrapping_add(1);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(pi_var4 < end) {
                        break;
                    }
                }
                if i_var3 == 8 {
                    return 0;
                }
                pi_var5 = pi_var5.byte_add(0x2a);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < i_var1) {
                    break;
                }
            }
        }
        i_var1 = 0;
        if 0 < DAT_00aa5e68 {
            let mut pi_var5: *mut i32 = &raw mut DAT_00aa5cb4;
            loop {
                i_var2 = 1;
                let mut pi_var4: *mut i32 = pi_var5;
                loop {
                    if pi_var4.read_unaligned() != 0
                        && pi_var5.wrapping_sub(1).read_unaligned() == 0
                    {
                        return 0;
                    }
                    pi_var4 = pi_var4.wrapping_add(1);
                    i_var2 = i_var2.wrapping_add(1);
                    if !(i_var2 < 8) {
                        break;
                    }
                }
                pi_var5 = pi_var5.wrapping_add(8);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_00aa5e68) {
                    break;
                }
            }
        }
        1
    }
}
