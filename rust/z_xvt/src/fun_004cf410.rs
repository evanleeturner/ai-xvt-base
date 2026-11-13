// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b5be: u32;
    static mut DAT_00815cee: [i32; 0];
    static mut DAT_0064f070: [i32; 0];
    static mut DAT_0064f078: [i32; 0];
    static mut DAT_0064f430: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cee90(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf410(param_1: i32) -> i32 {
    // SAFETY: the callee and the globals are the game's own memory; the
    // tables are read at the same offsets as the original, unaligned reads
    // being used since the addresses need not be aligned.
    unsafe {
        let u_var1 = FUN_004cee90(param_1);
        let i_var4: i32;
        if u_var1 < DAT_0081b5be && u_var1 < 0x28 {
            let base = (&raw mut DAT_00815cee) as *mut i32;
            i_var4 = base
                .wrapping_add(u_var1.wrapping_mul(0x8e) as usize)
                .read_unaligned();
        } else {
            i_var4 = 0;
        }
        let mut i_var2: i32 = 0;
        let mut pi_var3 = (&raw mut DAT_0064f070) as *mut i32;
        let end = (&raw mut DAT_0064f430) as *mut i32;
        let tab78 = (&raw mut DAT_0064f078) as *mut i32;
        loop {
            if pi_var3.read_unaligned() == param_1 {
                return tab78
                    .wrapping_add(i_var2.wrapping_mul(6) as usize)
                    .read_unaligned()
                    .wrapping_add(i_var4);
            }
            pi_var3 = pi_var3.wrapping_add(6);
            i_var2 = i_var2.wrapping_add(1);
            if !(pi_var3 < end) {
                break;
            }
        }
        i_var4
    }
}
