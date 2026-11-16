// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4988: u32;
    static mut DAT_006d9d56: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00418cd0(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042a3a0() {
    // SAFETY: the globals and callee are the game's own; the addresses
    // computed from them point at the game's data, read where the original does.
    unsafe {
        let i_var1: u32 = (DAT_006d9d50 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988);

        let base = DAT_006d9d56;
        let val_1f = ((base.wrapping_add(0x1f)) as *const i32).read_unaligned();
        let val_23 = ((base.wrapping_add(0x23)) as *const i32).read_unaligned();
        let val_27 = ((base.wrapping_add(0x27)) as *const i32).read_unaligned();

        let val_i4 = (i_var1.wrapping_add(4) as *const i32).read_unaligned();
        let val_i8 = (i_var1.wrapping_add(8) as *const i32).read_unaligned();
        let val_ic = (i_var1.wrapping_add(0xc) as *const i32).read_unaligned();

        FUN_00418cd0(
            val_1f.wrapping_sub(val_i4),
            val_23.wrapping_sub(val_i8),
            val_27.wrapping_sub(val_ic),
        );
    }
}
