// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: i32) -> i32;
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044ad00(param_1: i32) -> i32 {
    // SAFETY: all addresses are the game's own memory structures, read
    // exactly where the original reads them.
    unsafe {
        let entry = (&raw mut DAT_006da850)
            .cast::<i32>()
            .wrapping_offset(param_1 as isize)
            .read_unaligned();
        let pi_var3 = FUN_0048f4f0(((param_1 as u32) & 0xFFFF0000 | (entry as u32 & 0xFFFF)) as i32)
            as *mut i32;
        if pi_var3.read_unaligned() != pi_var3 as i32 {
            FUN_00485fa0(pi_var3);
        }
        let mut local_4: i32 = 0;
        let field_6: i32 = pi_var3
            .cast::<u8>()
            .byte_add(6)
            .cast::<i32>()
            .read_unaligned();
        if 0 < field_6 {
            let mut local_8: i32 = 0;
            let field_10: i32 = pi_var3
                .cast::<u8>()
                .byte_add(10)
                .cast::<i32>()
                .read_unaligned();
            loop {
                let i_var1: i32 =
                    (field_10.wrapping_add(local_8) as usize as *const i32).read_unaligned();
                if i_var1 != 0 && ((i_var1 as usize + 4) as *const i32).read_unaligned() != 0x14 {
                    let pi_var4: i32;
                    if ((i_var1 as usize + 4) as *const i32).read_unaligned() != 0x19 {
                        let mut i_var6: i32 = 0;
                        let mut i_var5: i32 = 0;
                        let mut found: i32 = 0;
                        if 0 < ((i_var1 as usize + 8) as *const i32).read_unaligned() {
                            loop {
                                let base = ((i_var1 as usize + 0xc) as *const i32).read_unaligned();
                                let i_var2 = (base.wrapping_add(i_var6) as usize as *const i32)
                                    .read_unaligned();
                                if i_var2 != 0 {
                                    let r = FUN_00449670(i_var2, pi_var3 as i32);
                                    if r != 0 {
                                        found = r;
                                        break;
                                    }
                                }
                                i_var6 = i_var6.wrapping_add(4);
                                i_var5 = i_var5.wrapping_add(1);
                                if !(i_var5
                                    < ((i_var1 as usize + 8) as *const i32).read_unaligned())
                                {
                                    break;
                                }
                            }
                        }
                        pi_var4 = found;
                    } else {
                        pi_var4 = ((i_var1 as usize + 0x14) as *const i32).read_unaligned();
                    }
                    if pi_var4 != 0 && (pi_var4 as usize as *const i32).read_unaligned() == 3 {
                        FUN_0048f510();
                        return 1;
                    }
                }
                local_8 = local_8.wrapping_add(4);
                local_4 = local_4.wrapping_add(1);
                if !(local_4 < field_6) {
                    break;
                }
            }
        }
        FUN_0048f510();
        0
    }
}
