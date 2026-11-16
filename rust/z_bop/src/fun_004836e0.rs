// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(param_1: u32, param_2: u32);
    fn FUN_004ae5a0(param_1: u32, param_2: i32) -> i32;
    fn FUN_004ae6e0(param_1: u32, param_2: i32) -> i32;
    fn FUN_004ae640(param_1: u32, param_2: i32) -> i32;
    fn FUN_004037b0(param_1: *mut core::ffi::c_void, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_0046a8d0(param_1: u32, param_2: u32, param_3: u32);
    fn FUN_0041f410(param_1: u32, param_2: u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004836e0(param_1: u32, param_2: u32, param_3: i32) {
    // SAFETY: the globals and callees are the game's own; the table at
    // DAT_009a1fe8 holds 0x23-byte records read where the original reads.
    unsafe {
        let mut i_var8: i32 = ((param_1 & 0xffff) as i32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        let i_var1: i32 = ((i_var8.wrapping_add(4)) as *const i32).read_unaligned();
        let i_var2: i32 = ((i_var8.wrapping_add(8)) as *const i32).read_unaligned();
        i_var8 = ((i_var8.wrapping_add(0xc)) as *const i32).read_unaligned();
        FUN_00458e20(param_2 & 0xffff, 0);
        let i_var6 = DAT_00a07cdc;
        let i_var5 = DAT_00a07cd8;
        let i_var4 = DAT_00a07cd4;
        let i_var3: i32 = ((param_2 & 0xffff) as i32).wrapping_mul(0x23);
        let u_var9: u32 =
            u32::from(((DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var3)) as *const u8).read());
        DAT_009d8a58 = FUN_004ae5a0(u_var9, param_3);
        DAT_009d8a54 = FUN_004ae6e0(u_var9, param_3);
        let i_var7 = FUN_004ae640(u_var9, param_3);
        DAT_009d8b60 = i_var7.wrapping_neg();
        FUN_004037b0(
            DAT_009a1fe8.wrapping_add(i_var3) as *mut core::ffi::c_void,
            DAT_009d8a58,
            DAT_009d8a54,
            DAT_009d8b60,
        );
        let u_var10 = (i_var6.wrapping_add(DAT_009d8a58).wrapping_sub(i_var1)) as u32;
        let u_var11 = (i_var5.wrapping_add(DAT_009d8a54).wrapping_sub(i_var2)) as u32;
        let u_var9 = (i_var4.wrapping_add(DAT_009d8b60).wrapping_sub(i_var8)) as u32;
        FUN_0046a8d0(u_var10, u_var11, u_var9);
        FUN_0041f410(u_var10, u_var11, u_var9);
    }
}
