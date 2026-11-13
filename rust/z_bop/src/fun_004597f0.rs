// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d1308: u32;
    static mut DAT_009a1fe8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004597f0() -> u32 {
    // SAFETY: the globals are the game's own; the table at DAT_009a1fe8 holds
    // 0x23-byte entries, and the byte read is the one the original reads.
    unsafe {
        let mut u_var3: u32 = DAT_00a07ce8 & 0xffff;
        let mut u_var1: u32 = u_var3;
        let sum: u32 = DAT_009d1308.wrapping_add(DAT_00a07ce8);

        loop {
            if (u_var1 as i32) < (sum as i32) {
                return 0xffff;
            }
            let addr = u_var3
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_009a1fe8);
            if *(addr as *const i8) == 0 {
                break;
            }
            u_var3 = ((u_var3 as u16).wrapping_add(1)) as u32;
            u_var1 = u_var3;
        }
        u_var3
    }
}
