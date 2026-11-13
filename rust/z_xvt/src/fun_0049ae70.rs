// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525568: i32;
    static mut DAT_0071f720: i32;
    static mut DAT_0071b740: i8;
    static mut DAT_006db730: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071e528: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049ae70() {
    // SAFETY: all globals are the game's own memory; the pointers walk the
    // game's tables exactly as the original does.
    unsafe {
        if DAT_00525568 == 0 {
            let mut i_stack_4: i32 = 0;
            let mut pi_stack_c: *mut i32 = &raw mut DAT_0071f720;
            let mut pc_stack_8: *mut i8 = &raw mut DAT_0071b740;
            let mut pi_var5: *mut i32 = &raw mut DAT_006db730;
            let end = (&raw mut DAT_0071e528) as usize;
            loop {
                if *pc_stack_8 != 0 && i_stack_4 != DAT_0071ecf4 {
                    let mut i_var3: i32 = 0;
                    let mut pi_var4: *mut i32 = pi_stack_c;
                    if 0 < pi_var5.read_unaligned() {
                        loop {
                            if pi_var4.read_unaligned() == 0 && pi_var4.add(1).read_unaligned() == 2
                            {
                                if pi_var5.read_unaligned() != 0
                                    && (pi_stack_c as usize) <= (pi_var4 as usize)
                                {
                                    let i_var1 = pi_var5.read_unaligned().wrapping_sub(1);
                                    let mut i_var2: i32 = 0;
                                    pi_var5.write_unaligned(i_var1);
                                    let mut pi_var6: *mut i32 = pi_stack_c;
                                    if 0 < i_var1 {
                                        loop {
                                            if (pi_var4 as usize) <= (pi_var6 as usize) {
                                                pi_var6.write_unaligned(
                                                    pi_var6.byte_add(0x12).read_unaligned(),
                                                );
                                                pi_var6.byte_add(4).write_unaligned(
                                                    pi_var6.byte_add(0x16).read_unaligned(),
                                                );
                                                pi_var6.byte_add(8).write_unaligned(
                                                    pi_var6.byte_add(0x1a).read_unaligned(),
                                                );
                                                pi_var6.byte_add(12).write_unaligned(
                                                    pi_var6.byte_add(0x1e).read_unaligned(),
                                                );
                                                pi_var6.byte_add(16).cast::<u16>().write_unaligned(
                                                    pi_var6
                                                        .byte_add(0x22)
                                                        .cast::<u16>()
                                                        .read_unaligned(),
                                                );
                                            }
                                            i_var2 = i_var2.wrapping_add(1);
                                            pi_var6 = pi_var6.byte_add(0x12);
                                            if !(i_var2 < pi_var5.read_unaligned()) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            } else {
                                pi_var4 = pi_var4.byte_add(0x12);
                                i_var3 = i_var3.wrapping_add(1);
                            }
                            if !(i_var3 < pi_var5.read_unaligned()) {
                                break;
                            }
                        }
                    }
                }
                pi_var5 = pi_var5.add(1);
                pi_stack_c = pi_stack_c.wrapping_add(0x7e9);
                pc_stack_8 = pc_stack_8.wrapping_add(0x5bd);
                i_stack_4 = i_stack_4.wrapping_add(1);
                if !((pc_stack_8 as usize) < end) {
                    break;
                }
            }
        }
    }
}
