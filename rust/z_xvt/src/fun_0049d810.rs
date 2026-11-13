// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049d8b0(a: u16, b: u16, c: u16, d: u16, e: i16, f: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049d810(param_1: *mut u8, param_2: i16) -> u32 {
    // SAFETY: param_1 points at the caller's record; the callee is the game's own.
    unsafe {
        if param_1.byte_add(4).read() == 0x27 {
            param_1.byte_add(5).write(0xc);
        }

        let u_var2 = FUN_0049d8b0(
            u16::from(param_1.read()),
            u16::from(param_1.byte_add(1).read()),
            u16::from(param_1.byte_add(2).read()),
            u16::from(param_1.byte_add(3).read()),
            param_2,
            10,
        ) & 0xffff;

        let u_var3: u32 = if param_1.byte_add(4).read() == 0x27 {
            0
        } else {
            FUN_0049d8b0(
                u16::from(param_1.byte_add(4).read()),
                u16::from(param_1.byte_add(5).read()),
                u16::from(param_1.byte_add(6).read()),
                u16::from(param_1.byte_add(7).read()),
                param_2,
                10,
            ) & 0xffff
        };

        if param_1.byte_add(10).read() == 1 {
            return u_var3 | u_var2;
        }
        u_var3 & u_var2
    }
}
