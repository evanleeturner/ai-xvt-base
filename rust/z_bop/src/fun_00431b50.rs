// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_00431c10(a: u16, b: u16, c: u16, d: u16, e: i16, f: u16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00431b50(param_1: *mut u8, param_2: i16) -> u32 {
    // SAFETY: param_1 points at the caller's record of at least 11 bytes;
    // the callee is the game's own.
    unsafe {
        let mut u_var2: u16 = 10;
        if param_1.byte_add(4).read() == 0x27 && param_1.byte_add(5).read() == 0xc {
            u_var2 = param_1.byte_add(6).read() as u16;
        }
        let u_var3: u32 = (FUN_00431c10(
            param_1.read() as u16,
            param_1.byte_add(1).read() as u16,
            param_1.byte_add(2).read() as u16,
            param_1.byte_add(3).read() as u16,
            param_2,
            u_var2,
        ) as u32)
            & 0xffff;
        let u_var4: u32 = if param_1.byte_add(4).read() == 0x27 {
            0
        } else {
            (FUN_00431c10(
                param_1.byte_add(4).read() as u16,
                param_1.byte_add(5).read() as u16,
                param_1.byte_add(6).read() as u16,
                param_1.byte_add(7).read() as u16,
                param_2,
                10,
            ) as u32)
                & 0xffff
        };
        if param_1.byte_add(10).read() != 1 && param_1.byte_add(4).read() != 0x27 {
            return u_var4 & u_var3;
        }
        u_var4 | u_var3
    }
}
