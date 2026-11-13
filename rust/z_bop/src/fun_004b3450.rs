// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b3450(param_1: *mut i32, param_2: *mut i32, param_3: u32) -> u32 {
    if param_3 == 0 {
        return 0;
    }
    let mut p2 = param_2;
    let mut u_var3: u32 = 0;
    let mut u_var2: u32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: the pointers refer to the caller's records, read where the
    // original reads; the callee is the game's own.
    unsafe {
        let i_var1 = param_1.read_unaligned();
        loop {
            let mut u_var4: u32 = 0;
            if p2.read_unaligned() == i_var1 {
                u_var4 = 1;
                if p2.add(1).read_unaligned() == param_1.add(1).read_unaligned() {
                    u_var4 = 2;
                    if i_var1 == 1 {
                        if param_1.add(2).read_unaligned() == p2.add(2).read_unaligned()
                            && p2.add(3).read_unaligned() == param_1.add(3).read_unaligned()
                            && p2.add(4).read_unaligned() == param_1.add(4).read_unaligned()
                        {
                            FUN_004079e0();
                            return u_var3;
                        }
                    } else {
                        if i_var1 != 2 {
                            FUN_004079e0();
                            return u_var3;
                        }
                        if p2.read_unaligned() == 2 {
                            u_var4 = 3;
                        }
                        if param_1.add(2).read_unaligned() == p2.add(2).read_unaligned()
                            && p2.add(3).read_unaligned() == param_1.add(3).read_unaligned()
                            && p2.add(4).read_unaligned() == param_1.add(4).read_unaligned()
                            && p2.add(0xb).read_unaligned() == param_1.add(0xb).read_unaligned()
                        {
                            FUN_004079e0();
                            return u_var3;
                        }
                    }
                }
            }
            if u_var2 < u_var4 {
                u_var2 = u_var4;
                local_4 = u_var3;
            }
            p2 = p2.add(0x29);
            u_var3 = u_var3.wrapping_add(1);
            if u_var3 >= param_3 {
                break;
            }
        }
        FUN_004079e0();
    }
    local_4
}
