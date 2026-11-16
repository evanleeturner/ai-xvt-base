// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
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
pub unsafe extern "C" fn FUN_004498f0(param_1: i32, mut param_2: i32) -> i32 {
    // SAFETY: all pointers come from the game's own tables and objects; reads
    // happen at the offsets and in the order the original uses. __ftol takes
    // its value from the x87 stack as in the original.
    unsafe {
        let rd = |addr: i32| -> i32 { (addr as usize as *const i32).read_unaligned() };

        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16;
        let p_var2 = FUN_0048f4f0(idx as i32) as usize as *mut i32;
        if p_var2.read_unaligned() != p_var2 as usize as i32 {
            FUN_00485fa0(p_var2);
        }
        let base = p_var2 as usize as i32;
        let ptr_at_10 = rd(base.wrapping_add(10));
        let val_at_ptr10_plus_4 = rd(ptr_at_10.wrapping_add(4));
        if val_at_ptr10_plus_4 == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let field_6 = rd(base.wrapping_add(6));
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }
        let arr_val = rd(base.wrapping_add(10).wrapping_add(param_2.wrapping_mul(4)));
        let i_var1 = arr_val;
        if i_var1 != 0 && rd(i_var1.wrapping_add(4)) != 3 {
            let mut i_var5: i32 = 0;
            let mut i_var4: i32 = 0;
            if 0 < rd(i_var1.wrapping_add(8)) {
                loop {
                    let i_var3 = rd(rd(i_var1.wrapping_add(0xc)).wrapping_add(i_var5));
                    if i_var3 != 0 {
                        let r = FUN_00449140(i_var3);
                        if r != 0 {
                            break;
                        }
                    }
                    i_var5 = i_var5.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4 < rd(i_var1.wrapping_add(8))) {
                        break;
                    }
                }
            }
        }
        let l_var6 = __ftol();
        FUN_0048f510(l_var6 as i32);
        l_var6 as i32
    }
}
