// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ef64: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004da920(param_1: u32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004b9010(param_1: i32);
    fn FUN_004b90b0(param_1: i32);
    fn FUN_004b9140(param_1: i32);
    fn FUN_004b91c0(param_1: i32);
    fn FUN_004b9240(param_1: i32);
    fn FUN_004b92c0(param_1: i32);
    fn FUN_004b9340(param_1: i32);
    fn FUN_004b93c0(param_1: i32);
    fn FUN_004b9440(param_1: i32);
    fn FUN_004b94c0(param_1: i32);
    fn FUN_004b9540(param_1: i32);
    fn FUN_004b9600(param_1: i32);
    fn FUN_004b9680(param_1: i32);
    fn FUN_004b9760(param_1: i32);
    fn FUN_004b97e0(param_1: i32);
    fn FUN_004b9870(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b8ed0() {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed where the original passes it.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x54, 0x5a, 0x25c, 0x6a);
        let piv = local_10.as_mut_ptr();
        let u_var4: u32 = 0xffff;
        let u_var2: u32 = if DAT_0064ef64 == 1 { 0x273 } else { 0xce };
        let pc = FUN_004dcfa0(u_var2);
        FUN_004da920(0xf, pc, piv, u_var4);
        FUN_004b9010(DAT_0064ef64.wrapping_add(-1));
        FUN_004b90b0(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9140(DAT_0064ef64.wrapping_add(-1));
        FUN_004b91c0(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9240(DAT_0064ef64.wrapping_add(-1));
        FUN_004b92c0(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9340(DAT_0064ef64.wrapping_add(-1));
        FUN_004b93c0(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9440(DAT_0064ef64.wrapping_add(-1));
        FUN_004b94c0(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9540(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9600(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9680(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9760(DAT_0064ef64.wrapping_add(-1));
        FUN_004b97e0(DAT_0064ef64.wrapping_add(-1));
        FUN_004b9870(DAT_0064ef64.wrapping_add(-1));
    }
}
