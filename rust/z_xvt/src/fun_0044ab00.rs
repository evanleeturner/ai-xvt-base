// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [u16; 0];
    static mut DAT_00525cc8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_0044a830(param_1: i32, param_2: i32, param_3: i32) -> *mut core::ffi::c_void;
    fn __ftol() -> i64;
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044ab00(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: the callees and globals are the game's own; the pointers come
    // from the game's own structures and are read where the original reads.
    unsafe {
        let p = (&raw mut DAT_006da850) as *mut u16;
        let idx = p.offset(param_1 as isize).read_unaligned();
        let pi_var1 = FUN_0048f4f0(idx as i32) as *mut i32;
        if pi_var1.read_unaligned() != pi_var1 as i32 {
            FUN_00485fa0(pi_var1);
        }
        let v10: i32 = pi_var1.byte_add(10).cast::<i32>().read_unaligned();
        let v6: i32 = pi_var1.byte_add(6).cast::<i32>().read_unaligned();
        if ((v10 as u32).wrapping_add(4) as *const i32).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if v6 <= param_2 {
            param_2 = v6.wrapping_sub(1);
        }
        let mut i_var3: i32 = 0;
        DAT_00525cc8 = 0;
        let entry = ((v10 as u32).wrapping_add((param_2 as u32).wrapping_mul(4)) as *const i32)
            .read_unaligned();
        let p_var2 = FUN_0044a830(entry, pi_var1 as i32, param_3);
        if !p_var2.is_null() {
            let l_var4 = __ftol();
            i_var3 = l_var4 as i32;
        }
        FUN_0048f510();
        i_var3.wrapping_neg()
    }
}
