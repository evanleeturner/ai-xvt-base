// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449140(param_1: i32) -> i32;
    fn __ftol() -> i64;
    fn FUN_0048f510(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004499b0(param_1: i32, mut param_2: i32) -> i32 {
    // SAFETY: all pointers are the game's own structures; reads happen in the
    // same order as the original, and __ftol consumes the x87 state left by it.
    unsafe {
        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read_unaligned() as u16 as i32;
        let pi_var2 = FUN_0048f4f0(idx) as usize as *mut i32;
        if pi_var2.read_unaligned() != pi_var2 as usize as i32 {
            FUN_00485fa0(pi_var2);
        }
        let mut p_table: i32 = pi_var2.byte_add(10).read_unaligned();
        if ((p_table.wrapping_add(4)) as usize as *const i32).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let val_6: i32 = pi_var2.byte_add(6).read_unaligned();
        if val_6 <= param_2 {
            param_2 = val_6.wrapping_sub(1);
        }
        p_table = pi_var2.byte_add(10).read_unaligned();
        let i_var1: i32 =
            (p_table.wrapping_add(param_2.wrapping_mul(4)) as usize as *const i32).read_unaligned();
        if i_var1 != 0 && ((i_var1.wrapping_add(4)) as usize as *const i32).read_unaligned() != 3 {
            let mut i_var5: i32 = 0;
            let mut i_var4: i32 = 0;
            if 0 < ((i_var1.wrapping_add(8)) as usize as *const i32).read_unaligned() {
                loop {
                    let base = ((i_var1.wrapping_add(0xc)) as usize as *const i32).read_unaligned();
                    let mut i_var3 =
                        (base.wrapping_add(i_var5) as usize as *const i32).read_unaligned();
                    if i_var3 != 0 {
                        i_var3 = FUN_00449140(i_var3);
                        if i_var3 != 0 {
                            break;
                        }
                    }
                    i_var5 = i_var5.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4
                        < ((i_var1.wrapping_add(8)) as usize as *const i32).read_unaligned())
                    {
                        break;
                    }
                }
            }
        }
        let l_var6 = __ftol();
        FUN_0048f510(0);
        l_var6 as i32
    }
}
