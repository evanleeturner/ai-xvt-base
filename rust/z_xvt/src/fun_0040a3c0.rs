// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: u32;
    static mut DAT_0070a189: u8;
    static mut DAT_0085e791: u8;
    static mut DAT_0085e79a: u8;
    static mut DAT_006d4988: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040a1a0(param_1: u32, param_2: u32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040a3c0(param_1: u32, param_2: i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own; the table
    // pointer is read at the same computed address as the original.
    unsafe {
        if param_2 != DAT_0071ecf4 as i32 {
            return 0;
        }
        if DAT_0070a189 == 0 {
            return 0;
        }
        if DAT_0085e791 == 0 {
            return 0;
        }
        if DAT_0085e79a != 0 {
            let addr = param_1
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988);
            let u_var1 = u32::from((addr as *const u8).read());
            return match u_var1 {
                0x89..=0x93 => FUN_0040a1a0(u_var1.wrapping_sub(0x85), param_1, param_2),
                0x94 | 0x95 => FUN_0040a1a0(u_var1.wrapping_sub(0x8a), param_1, param_2),
                0x96 | 0x97 => FUN_0040a1a0(u_var1.wrapping_sub(0x87), param_1, param_2),
                0x98..=0x9b => FUN_0040a1a0(0x11, param_1, param_2),
                _ => u_var1,
            };
        }
        0
    }
}
