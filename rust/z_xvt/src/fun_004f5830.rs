// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_007579b0: i32;
    static mut DAT_00757cee: i32;
    static mut DAT_00757e3e: i32;
    static mut DAT_00757a28: i32;
    static mut DAT_00757870: i32;
    static mut DAT_00757874: i32;
    static mut DAT_00757680: i32;
    static mut DAT_00757684: i32;
    static mut DAT_0075788c: i32;
    static mut DAT_0075769c: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004cde00();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f5830() {
    // SAFETY: all globals and the callee are the game's own memory; pointer
    // arithmetic mirrors the original, including its 4-byte-scaled offsets
    // on the DAT_00757680/DAT_00757684/DAT_0075769c accesses.
    unsafe {
        if DAT_0081b850 != 2 {
            FUN_004cde00();
        }
        let mut p_stack_8: *mut i32 = &raw mut DAT_007579b0;
        loop {
            let mut i_var5: i32 = 0;
            let mut pi_var2: *mut u8 = (&raw mut DAT_00757cee).cast::<u8>();
            let end: usize = (&raw mut DAT_00757e3e) as usize;
            let i_var1: i32 = p_stack_8.read_unaligned();
            loop {
                if pi_var2.cast::<i32>().read_unaligned() == i_var1 {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(0x2a);
                i_var5 = i_var5.wrapping_add(1);
                if (pi_var2 as usize) >= end {
                    break;
                }
            }
            if i_var5 == 8 {
                if 0 < DAT_00757a28 {
                    let mut i_var5: i32 = 0;
                    let mut i_stack_4: i32 = DAT_00757a28;
                    loop {
                        let mut i_var7: i32 = 0;
                        let mut i_var4: i32 = i_var5;
                        loop {
                            let a870 = (&raw mut DAT_00757870)
                                .cast::<u8>()
                                .wrapping_offset(i_var4 as isize);
                            if a870.cast::<i32>().read_unaligned() == i_var1 {
                                if i_var7 < 7 {
                                    let mut i_var6: i32 = 7i32.wrapping_sub(i_var7);
                                    let mut i_var3: i32 = i_var4;
                                    loop {
                                        i_var6 = i_var6.wrapping_sub(1);
                                        let dst = (&raw mut DAT_00757870)
                                            .cast::<u8>()
                                            .wrapping_offset(i_var3 as isize);
                                        let src = (&raw mut DAT_00757874)
                                            .cast::<u8>()
                                            .wrapping_offset(i_var3 as isize);
                                        dst.cast::<i32>()
                                            .write_unaligned(src.cast::<i32>().read_unaligned());
                                        let dst2 = (&raw mut DAT_00757680)
                                            .wrapping_offset(i_var3 as isize);
                                        let src2 = (&raw mut DAT_00757684)
                                            .wrapping_offset(i_var3 as isize);
                                        dst2.write_unaligned(src2.read_unaligned());
                                        i_var3 = i_var3.wrapping_add(4);
                                        if i_var6 == 0 {
                                            break;
                                        }
                                    }
                                }
                                (&raw mut DAT_0075788c)
                                    .cast::<u8>()
                                    .wrapping_offset(i_var5 as isize)
                                    .cast::<i32>()
                                    .write_unaligned(0);
                                (&raw mut DAT_0075769c)
                                    .wrapping_offset(i_var5 as isize)
                                    .write_unaligned(-1);
                            }
                            i_var4 = i_var4.wrapping_add(4);
                            i_var7 = i_var7.wrapping_add(1);
                            if !(i_var7 < 8) {
                                break;
                            }
                        }
                        i_var5 = i_var5.wrapping_add(0x20);
                        i_stack_4 = i_stack_4.wrapping_sub(1);
                        if i_stack_4 == 0 {
                            break;
                        }
                    }
                }
                p_stack_8.write_unaligned(0);
            }
            p_stack_8 = p_stack_8.wrapping_add(1);
            if 0x7579cfusize < (p_stack_8 as usize) {
                return;
            }
        }
    }
}
