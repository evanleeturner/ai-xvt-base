// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0074244e: i16;
    static mut DAT_0074244c: i16;
    static mut unaff_EDI: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: u32) -> i32;
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051d220(param_1: i16) {
    // SAFETY: the globals and callees are the game's own; iVar2 is an address
    // returned by the game's function, read and written where the original does.
    unsafe {
        if DAT_0074244e == 0 {
            DAT_0074244e = param_1;
            DAT_0074244c = param_1;
            return;
        }
        let mut u_var3: u32 = (unaff_EDI & 0xFFFF0000) | (DAT_0074244e as i32 as u32);
        let mut b_var1: i32 = 1;
        loop {
            if (u_var3 & 0xFFFF) == 0 {
                break;
            }
            let i_var2 = FUN_0048f4f0(u_var3);
            let p = (i_var2 as u32).wrapping_add(0x48) as *mut i16;
            if p.read_unaligned() == 0 {
                p.write_unaligned(param_1);
                b_var1 = 0;
            }
            FUN_0048f510();
            u_var3 = (u_var3 & 0xFFFF0000) | (p.read_unaligned() as i32 as u32);
            if b_var1 == 0 {
                break;
            }
        }
        if DAT_0074244c == 0 {
            DAT_0074244c = param_1;
        }
    }
}
