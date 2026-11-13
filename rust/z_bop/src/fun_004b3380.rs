// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c58: i32;
    static mut DAT_00a91120: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b3380(param_1: *mut i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own; param_1 points at
    // the caller's record, and the table at DAT_00a91120 holds entries of 0x8b ints.
    unsafe {
        if DAT_00528c58 == 0 {
            return 0;
        }
        let mut u_var4: u32 = 0;
        let mut pi_var1: *mut i32 = &raw mut DAT_00a91120;
        let mut u_var3: u32 = 0;
        let mut local_4: u32 = 0;
        if DAT_00528c58 != 0 {
            loop {
                let mut u_var2: u32 = 0;
                let p1 = param_1.byte_add(4).read_unaligned();
                if p1 == 0 || pi_var1.byte_add(4).read_unaligned() == p1 {
                    u_var2 = 1;
                    let p2 = param_1.byte_add(8).read_unaligned();
                    if p2 == 0 || pi_var1.byte_add(8).read_unaligned() == p2 {
                        u_var2 = 2;
                        if (pi_var1.byte_add(36).read_unaligned()
                            & param_1.byte_add(36).read_unaligned())
                            != 0
                        {
                            u_var2 = 3;
                            if pi_var1.read_unaligned() == param_1.read_unaligned() {
                                FUN_004079e0();
                                return u_var3;
                            }
                        }
                    }
                }
                if u_var4 < u_var2 {
                    u_var4 = u_var2;
                    local_4 = u_var3;
                }
                pi_var1 = pi_var1.wrapping_add(0x8b);
                u_var3 = u_var3.wrapping_add(1);
                if u_var3 >= DAT_00528c58 as u32 {
                    break;
                }
            }
        }
        FUN_004079e0();
        local_4
    }
}
