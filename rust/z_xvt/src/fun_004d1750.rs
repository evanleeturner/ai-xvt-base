// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d41: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3570(param_1: *mut i8) -> i32;
    fn FUN_004d1790(param_1: *mut i32, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1750(param_1: *mut i8, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: the callees and the global are the game's own; the table entry
    // address is computed as the original does and read as a pointer.
    unsafe {
        let i_var1 = FUN_004d3570(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let addr = DAT_00758d41
            .wrapping_add(0x40)
            .wrapping_add(i_var1.wrapping_mul(0x44));
        let p = (addr as usize as *const *mut i32).read_unaligned();
        FUN_004d1790(p, param_2, param_3)
    }
}
