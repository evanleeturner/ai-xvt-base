// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664e9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db640(param_1: u32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004b84e0(param_1: i32);
    fn FUN_004b8570(param_1: i32);
    fn FUN_004b8600(param_1: i32);
    fn FUN_004b86e0(param_1: i32);
    fn FUN_004b8760(param_1: i32);
    fn FUN_004b87e0(param_1: i32);
    fn FUN_004b8860(param_1: i32);
    fn FUN_004b88e0(param_1: i32);
    fn FUN_004b8960(param_1: i32);
    fn FUN_004b89e0(param_1: i32);
    fn FUN_004b8a60(param_1: i32);
    fn FUN_004b8b20(param_1: i32);
    fn FUN_004b8ba0(param_1: i32);
    fn FUN_004b8c80(param_1: i32);
    fn FUN_004b8d00(param_1: i32);
    fn FUN_004b8da0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b83a0() {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all callees and the global are the game's own; local_10 is a
    // valid 4-int buffer for the callees to fill and read.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x54, 0x5a, 0x25c, 0x6a);
        let u_var2: u32 = if DAT_00664e9c == 1 { 0xdb } else { 0x281 };
        let pc_var1 = FUN_004ddc10(u_var2);
        FUN_004db640(0xf, pc_var1, local_10.as_mut_ptr(), 0xffff);
        FUN_004b84e0(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8570(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8600(DAT_00664e9c.wrapping_sub(1));
        FUN_004b86e0(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8760(DAT_00664e9c.wrapping_sub(1));
        FUN_004b87e0(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8860(DAT_00664e9c.wrapping_sub(1));
        FUN_004b88e0(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8960(DAT_00664e9c.wrapping_sub(1));
        FUN_004b89e0(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8a60(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8b20(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8ba0(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8c80(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8d00(DAT_00664e9c.wrapping_sub(1));
        FUN_004b8da0(DAT_00664e9c.wrapping_sub(1));
    }
}
