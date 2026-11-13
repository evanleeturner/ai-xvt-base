// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7bc0: u8;
    static mut DAT_009a7bc1: u8;
    static mut DAT_009a7bc2: u8;
    static mut DAT_009a7a30: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00449410(param_1: u32, param_2: u32, param_3: u32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e5e0(mut param_1: *mut u8, mut param_2: u16, param_3: u32) {
    let u_var2: u32 = param_2 as u32;
    let mut u_var3: u32 = u_var2;
    // SAFETY: the globals and the callee are the game's own memory; param_1
    // points at the caller's RGB triples, read and written where the original does.
    unsafe {
        while u_var3 < u_var2.wrapping_add(param_3 & 0xffff) {
            let idx = (param_2 as u32).wrapping_mul(3) as usize;
            (&raw mut DAT_009a7bc0).byte_add(idx).write(param_1.read());
            (&raw mut DAT_009a7bc1)
                .byte_add(idx)
                .write(param_1.byte_add(1).read());
            (&raw mut DAT_009a7bc2)
                .byte_add(idx)
                .write(param_1.byte_add(2).read());
            u_var3 = param_2.wrapping_add(1) as u32;
            param_1 = param_1.byte_add(3);
            param_2 = param_2.wrapping_add(1);
        }
        if DAT_009a7a30 == 2 {
            FUN_00449410(0x9a7bc0, 0xa00530, u_var2, param_3 & 0xffff);
        }
    }
}
