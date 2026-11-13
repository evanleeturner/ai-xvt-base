// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d41: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_2: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004d3570(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d0500(param_1: *mut i8, param_2: *mut i32) -> i32 {
    // SAFETY: the callees and the global are the game's own; param_1 points at
    // a C string and the table entry holds a pointer to two ints, read as the
    // original does.
    unsafe {
        if param_1.read() == 0 {
            FUN_004dcfd0(param_2, 0, 0, 0, 0);
            return 0;
        }
        let i_var2 = FUN_004d3570(param_1);
        if i_var2 == -1 {
            FUN_004dcfd0(param_2, 0, 0, 0, 0);
            return 0;
        }
        let addr = DAT_00758d41
            .wrapping_add(0x40)
            .wrapping_add(i_var2.wrapping_mul(0x44));
        let p_var1 = (addr as *const i32).read_unaligned() as *const i32;
        let c = p_var1.read_unaligned();
        let d = p_var1.add(1).read_unaligned();
        FUN_004dcfd0(param_2, 0, 0, c, d);
        1
    }
}
