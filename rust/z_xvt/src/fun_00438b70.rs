// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525520: u32;
    static mut DAT_0052551c: u32;
    static mut DAT_006d4a4e: u8;
    static mut DAT_00525528: u32;
    static mut DAT_00525524: u32;
    static mut DAT_005257e0: u32;
    static mut DAT_0052ae54: u32;
    static mut DAT_00732a30: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ee20(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004a7040();
    fn FUN_004a6a80();
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00438b70() {
    // SAFETY: the callees and globals are the game's own memory; the function
    // pointer in DAT_00732a30 is the game's cdecl callback taking five u32s.
    unsafe {
        FUN_0048ee20(0);
        let u_var4: u32 = DAT_00525520.wrapping_sub(DAT_0052551c) >> 1;
        let u_var2: u32 = DAT_006d4a4e as u32;
        let u_var1: u32 = DAT_0052551c.wrapping_add(DAT_00525520);
        let u_var3: u32 = DAT_00525528.wrapping_add(DAT_00525524);
        if DAT_005257e0 != 0 {
            FUN_004a7040();
            DAT_0052ae54 = 0;
            FUN_004a6a80();
            let f: unsafe extern "C" fn(u32, u32, u32, u32, u32) =
                core::mem::transmute(DAT_00732a30);
            f(
                DAT_005257e0,
                ((u_var1 >> 1).wrapping_sub(u_var4 / 2).wrapping_sub(1)) & 0xffff,
                ((u_var3 >> 1)
                    .wrapping_sub(u_var2.wrapping_mul(5) / 2)
                    .wrapping_sub(1))
                    & 0xffff,
                u_var4.wrapping_add(2) & 0xffff,
                u_var2.wrapping_mul(5).wrapping_add(2),
            );
            FUN_004a6f90();
            DAT_0052ae54 = 1;
            FUN_004a7040();
        }
    }
}
