// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1c8: u32;
    static mut DAT_00742830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bbe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6bba(param_1: *mut i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own; param_1 and the
    // table at DAT_00742830 are read where the original reads them.
    unsafe {
        if DAT_0052f1c8 == 0 {
            return 0;
        }
        let mut local_8: u32 = 0;
        let mut local_10: u32 = 0;
        let mut local_c: *mut i32 = &raw mut DAT_00742830;
        let mut local_18: u32 = 0;
        while local_18 < DAT_0052f1c8 {
            let mut u_var1: u32 = 0;
            if local_c.add(1).read_unaligned() == param_1.add(1).read_unaligned() {
                u_var1 = 1;
                if local_c.read_unaligned() == param_1.read_unaligned() {
                    u_var1 = 2;
                    if param_1.read_unaligned() == 1 {
                        if local_c.add(2).read_unaligned() == param_1.add(2).read_unaligned()
                            && local_c.add(3).read_unaligned() == param_1.add(3).read_unaligned()
                            && local_c.add(4).read_unaligned() == param_1.add(4).read_unaligned()
                        {
                            FUN_0045bbe0();
                            return local_18;
                        }
                    } else {
                        if param_1.read_unaligned() != 2 {
                            FUN_0045bbe0();
                            return local_18;
                        }
                        if (param_1.add(0xb).read_unaligned() as u32)
                            <= (local_c.add(0xb).read_unaligned() as u32)
                        {
                            u_var1 = 3;
                            if local_c.add(2).read_unaligned() == param_1.add(2).read_unaligned()
                                && local_c.add(3).read_unaligned()
                                    == param_1.add(3).read_unaligned()
                                && local_c.add(4).read_unaligned()
                                    == param_1.add(4).read_unaligned()
                            {
                                FUN_0045bbe0();
                                return local_18;
                            }
                        }
                    }
                }
            }
            if local_8 < u_var1 {
                local_10 = local_18;
                local_8 = u_var1;
            }
            local_c = local_c.add(0x29);
            local_18 = local_18.wrapping_add(1);
        }
        FUN_0045bbe0();
        local_10
    }
}
