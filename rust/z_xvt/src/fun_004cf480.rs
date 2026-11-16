// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b5be: u32;
    static mut DAT_00815cf2: [i32; 0];
    static mut DAT_0064f070: [i32; 0];
    static mut DAT_0064f07c: [i32; 0];
    static mut DAT_0064f430: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cee90(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf480(param_1: i32) -> i32 {
    // SAFETY: the callee and the globals are the game's own memory; the
    // tables are read where the original reads them, unaligned-safe.
    unsafe {
        let u_var1 = FUN_004cee90(param_1);
        let i_var4: i32;
        if u_var1 < (&raw mut DAT_0081b5be).read_unaligned() && u_var1 < 0x28 {
            i_var4 = (&raw mut DAT_00815cf2)
                .cast::<i32>()
                .add(u_var1.wrapping_mul(0x8e) as usize)
                .read_unaligned();
        } else {
            i_var4 = 0;
        }
        let mut i_var3: i32 = 0;
        let mut pi_var2: *mut i32 = (&raw mut DAT_0064f070).cast::<i32>();
        let end: *mut i32 = (&raw mut DAT_0064f430).cast::<i32>();
        loop {
            if pi_var2.read_unaligned() == param_1 {
                FUN_004cee90(param_1);
                let v = (&raw mut DAT_0064f07c)
                    .cast::<i32>()
                    .offset((i_var3.wrapping_mul(6)) as isize)
                    .read_unaligned();
                return v.wrapping_add(i_var4);
            }
            pi_var2 = pi_var2.wrapping_add(6);
            i_var3 = i_var3.wrapping_add(1);
            if pi_var2 >= end {
                break;
            }
        }
        i_var4
    }
}
