// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099f6e0: u32;
    static mut DAT_00999be8: i32;
    static mut DAT_00999bec: i32;
    static mut DAT_00999bf0: i32;
    static mut DAT_00999bf4: i32;
    static mut DAT_00999bf8: i32;
    static mut DAT_00999bfc: i32;
    static mut DAT_00999c00: i32;
    static mut DAT_00999c04: i32;
    static mut DAT_00999c08: i32;
    static mut DAT_00999e08: i32;
    static mut DAT_00999e0c: i32;
    static mut DAT_00999e10: i32;
    static mut DAT_00999e14: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046fee0(param_1: i32) -> u32 {
    // SAFETY: all globals are the game's own memory, accessed by address at
    // the same strides and offsets as the original; timeGetTime is the system API.
    unsafe {
        let mut u_var4: u32 = 0;
        if DAT_0099f6e0 != 0 {
            let mut pi_var2: *mut i32 = &raw mut DAT_00999be8;
            loop {
                if pi_var2.read_unaligned() == param_1 {
                    return u_var4;
                }
                pi_var2 = pi_var2.wrapping_add(0x8e);
                u_var4 = u_var4.wrapping_add(1);
                if !(u_var4 < DAT_0099f6e0) {
                    break;
                }
            }
        }
        if DAT_0099f6e0 == u_var4 && u_var4 < 0x28 {
            let idx = u_var4.wrapping_mul(0x8e) as usize;
            let base = u_var4.wrapping_mul(0x238) as usize;
            (&raw mut DAT_00999be8)
                .wrapping_add(idx)
                .write_unaligned(param_1);
            (&raw mut DAT_00999bec).byte_add(base).write_unaligned(0x7f);
            (&raw mut DAT_00999bf4).byte_add(base).write_unaligned(0x7f);
            (&raw mut DAT_00999bfc).byte_add(base).write_unaligned(0x7f);
            (&raw mut DAT_00999bf0)
                .wrapping_add(idx)
                .write_unaligned(0x7f);
            (&raw mut DAT_00999bf8)
                .wrapping_add(idx)
                .write_unaligned(0x7f);
            (&raw mut DAT_00999c00)
                .wrapping_add(idx)
                .write_unaligned(0x7f);
            (&raw mut DAT_00999c04).wrapping_add(idx).write_unaligned(0);
            (&raw mut DAT_00999c08).byte_add(base).write_unaligned(0x39);
            (&raw mut DAT_00999e08).wrapping_add(idx).write_unaligned(1);
            let d_var3 = timeGetTime();
            (&raw mut DAT_00999e0c)
                .wrapping_add(idx)
                .write_unaligned(d_var3 as i32);
            (&raw mut DAT_00999e10).wrapping_add(idx).write_unaligned(0);
            (&raw mut DAT_00999e14).wrapping_add(idx).write_unaligned(0);
            DAT_0099f6e0 = DAT_0099f6e0.wrapping_add(1);
        }
        u_var4
    }
}
