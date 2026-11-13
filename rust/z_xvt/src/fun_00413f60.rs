// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(param_1: u32, param_2: u32, param_3: u32, param_4: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413f60(param_1: u32, param_2: u32) -> u32 {
    let u_var3: u32 = param_1.wrapping_mul(0x100);
    let u_var2: u32 = (param_1 >> 0x18).wrapping_add((0xffffff7fu32 < u_var3) as u32);
    if u_var2 < param_2 {
        let u_var4: u64 = ((u_var2 as u64) << 32) | (u_var3.wrapping_add(0x80) as u64);
        if param_2 == 0 || (u_var4 >> 32) >= (param_2 as u64) {
            // SAFETY: RaiseException is the OS import; the arguments match its
            // signature and a null argument array is permitted with count 0.
            unsafe {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            0
        } else {
            (u_var4 / (param_2 as u64)) as u32
        }
    } else {
        0x7fffff00
    }
}
