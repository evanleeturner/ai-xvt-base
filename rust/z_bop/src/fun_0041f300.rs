// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425d10() -> u32;
    fn FUN_0042e4d0(param_1: u32, param_2: u32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041f300(param_1: u32, param_2: u8) {
    // SAFETY: the globals and callees are the game's own; the entry table
    // and the record pointers it holds are read and written at the same
    // offsets as the original.
    unsafe {
        let i_var2: u32 = param_1.wrapping_mul(0x23);
        let base: u32 = DAT_009a1fe8;

        ((base.wrapping_add(3).wrapping_add(i_var2)) as *mut u8).write(param_2);
        ((base.wrapping_add(2).wrapping_add(i_var2)) as *mut u8).write(0xd);

        let mut ptr_val: u32 =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val as *mut u8).write(5);

        ((DAT_009a1fe8.wrapping_add(0x19).wrapping_add(i_var2)) as *mut u8).write(2);

        ptr_val =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val.wrapping_add(1) as *mut u8).write(0);

        ptr_val =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val.wrapping_add(0x79) as *mut u16).write_unaligned(0);

        ptr_val =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val.wrapping_add(0x83) as *mut u16).write_unaligned(0);

        ptr_val =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val.wrapping_add(0x81) as *mut u16).write_unaligned(0);

        (DAT_009a1fe8.wrapping_add(0x14).wrapping_add(i_var2) as *mut u16).write_unaligned(0);

        ptr_val =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val.wrapping_add(0x77) as *mut u16).write_unaligned(0);

        ptr_val =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var2) as *const u32).read_unaligned();
        (ptr_val.wrapping_add(0x92) as *mut u8).write(1);

        let i_var2b: i32 = DAT_009ecc34;
        let u_var1: u32 = FUN_00425d10();
        FUN_0042e4d0((u_var1 & 3).wrapping_add(0x16), param_1, i_var2b);
    }
}
