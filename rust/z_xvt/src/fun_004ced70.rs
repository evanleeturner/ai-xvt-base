// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f070: i32;
    static mut DAT_0064f074: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ced70(param_1: i32, param_2: u32) -> u32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own table of 0x18-byte entries;
    // pointers are walked and written exactly where the original does.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_0064f070;
        while pi_var2.read_unaligned() != param_1 && pi_var2.read_unaligned() != 0 {
            pi_var2 = pi_var2.byte_add(0x18);
            i_var1 = i_var1.wrapping_add(1);
            if (pi_var2 as usize as u32) >= 0x64f430u32 {
                return 1;
            }
        }
        let off = (i_var1.wrapping_mul(0x18)) as isize;
        (&raw mut DAT_0064f070)
            .byte_offset(off)
            .write_unaligned(param_1);
        (&raw mut DAT_0064f074)
            .byte_offset(off)
            .write_unaligned(param_2);
        ((i_var1.wrapping_mul(0x18).wrapping_add(0x64f084)) as u32 as usize as *mut u32)
            .write_unaligned(1);
    }
    1
}
