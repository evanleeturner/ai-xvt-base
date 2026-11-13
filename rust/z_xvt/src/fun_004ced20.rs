// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f070: i32;
    static mut DAT_0064f074: i32;
    static mut DAT_0064f430: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ced20(param_1: i32) -> u32 {
    let mut i_var2: i32 = 0;
    // SAFETY: the globals are the game's own table of 0x18-byte entries;
    // reads stay within it exactly as the original does.
    unsafe {
        let mut pi_var3: *mut i32 = &raw mut DAT_0064f070;
        loop {
            if pi_var3.read() == param_1 {
                let u_var1: u32 = ((0x64f084i32.wrapping_add(i_var2.wrapping_mul(0x18))) as usize
                    as *const u32)
                    .read_unaligned();
                if u_var1 == 0 {
                    return 1;
                }
                let base: *mut i32 = &raw mut DAT_0064f074;
                let v = base
                    .offset(i_var2.wrapping_mul(6) as isize)
                    .read_unaligned() as u32;
                return v / u_var1;
            }
            pi_var3 = pi_var3.wrapping_add(6);
            i_var2 = i_var2.wrapping_add(1);
            if !(pi_var3 < &raw mut DAT_0064f430) {
                break;
            }
        }
    }
    0
}
