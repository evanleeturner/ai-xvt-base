// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_007579b0: i32;
    static mut DAT_00757cee: i32;
    static mut DAT_00757e3e: i32;
    static mut DAT_00757a28: i32;
    static mut DAT_0075788c: i32;
    static mut DAT_00757870: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004cde00();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f0890() {
    // SAFETY: all globals and the callee belong to the game; pointers stay
    // within the game's tables exactly as the original walks them.
    unsafe {
        if DAT_0081b850 != 2 {
            FUN_004cde00();
        }
        let mut pi_stack_8: *mut i32 = &raw mut DAT_007579b0;
        loop {
            let mut i_var6: i32 = 0;
            let mut pi_var2: *mut i32 = &raw mut DAT_00757cee;
            loop {
                let v = pi_var2.read_unaligned();
                if v != 0 && pi_stack_8.read() == v {
                    break;
                }
                pi_var2 = pi_var2.byte_add(0x2a);
                i_var6 = i_var6.wrapping_add(1);
                if !(pi_var2 < (&raw mut DAT_00757e3e)) {
                    break;
                }
            }

            if i_var6 == 8 {
                if DAT_00757a28 > 0 {
                    let mut i_var3: i32 = 0;
                    let mut i_stack_4: i32 = DAT_00757a28;
                    let i_var6b: i32 = pi_stack_8.read();
                    let mut puvar4: *mut i32 = &raw mut DAT_0075788c;
                    let base: *mut i32 = &raw mut DAT_00757870;
                    loop {
                        let mut i_var7: i32 = 0;
                        loop {
                            let p = base.offset(i_var3.wrapping_add(i_var7) as isize);
                            if p.read() == i_var6b {
                                if i_var7 < 7 {
                                    let mut i_var5: i32 = 7 - i_var7;
                                    let mut puvar1 = p;
                                    loop {
                                        i_var5 = i_var5.wrapping_sub(1);
                                        puvar1.write(puvar1.add(1).read());
                                        puvar1 = puvar1.add(1);
                                        if i_var5 == 0 {
                                            break;
                                        }
                                    }
                                }
                                puvar4.write(0);
                            }
                            i_var7 = i_var7.wrapping_add(1);
                            if !(i_var7 < 8) {
                                break;
                            }
                        }
                        puvar4 = puvar4.add(8);
                        i_var3 = i_var3.wrapping_add(8);
                        i_stack_4 = i_stack_4.wrapping_sub(1);
                        if i_stack_4 == 0 {
                            break;
                        }
                    }
                }
                pi_stack_8.write(0);
            }
            pi_stack_8 = pi_stack_8.add(1);
            if (0x7579cf_usize) < (pi_stack_8 as usize) {
                return;
            }
        }
    }
}
