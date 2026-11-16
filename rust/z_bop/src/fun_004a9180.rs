// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0adf0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042d7c0(param_1: *mut i8, param_2: i32) -> u8;
    fn FUN_0042d940(param_1: *mut i8, param_2: i32) -> u8;
    fn FUN_0042dac0(param_1: *mut i8, param_2: i32) -> u8;
    fn FUN_0042db50(param_1: *mut i8, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9180(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    if param_1 < 4 || 0x345 < param_1 {
        return 0;
    }
    // SAFETY: the global buffer and the callees are the game's own; the
    // pointer is computed from the address of the global without a reference.
    unsafe {
        let p = (&raw mut DAT_00a0adf0).byte_add(param_1.wrapping_mul(0x18) as usize);
        if param_2 < 0x601 {
            if param_2 == 0x600 {
                return FUN_0042d7c0(p, param_3) as u32;
            }
            if param_2 != 0x500 {
                return 0;
            }
            return FUN_0042db50(p, param_3);
        }
        if param_2 == 0x700 {
            return FUN_0042d940(p, param_3) as u32;
        }
        if param_2 != 0x777 {
            return 0;
        }
        FUN_0042dac0(p, param_3) as u32
    }
}
