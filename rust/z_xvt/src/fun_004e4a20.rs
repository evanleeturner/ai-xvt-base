// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bfba: i32;
    static mut DAT_00757cc4: i32;
    static mut DAT_005330fc: i32;
    static mut DAT_00859c2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004daee0(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: u32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_004d9010(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e4a20() -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // live local array passed as the rectangle buffer the callees expect.
    unsafe {
        let p = local_10.as_mut_ptr();
        FUN_004dcfd0(p, 0x58, 0x8a, 0x1ae, 0x99);
        let u_var3: u32 = if DAT_0081bfba == 2 {
            0x1c9
        } else if DAT_0081bfba == 4 {
            0x1ca
        } else {
            0xb4
        };
        let pc_var1 = FUN_004dcfa0(u_var3);
        FUN_004daac0(0xf, pc_var1, p, 0, 1, 0xffff);
        FUN_004dcfd0(p, 0x58, 0x9d, 0x1a4, 0x12d);
        let i_var2 = FUN_004daee0(0xc, DAT_00757cc4, p, 0xffff, 4, 0x400);
        let mut u_var6: u32 = 0x1ae;
        if 9 < i_var2.wrapping_add(1) {
            FUN_004dcfd0(p, 0x1a5, 0x9d, 0x1ae, 0x12d);
            DAT_005330fc = FUN_004d9010(
                p,
                DAT_005330fc,
                i_var2.wrapping_add(1),
                0,
                5,
                DAT_00859c2c,
                9,
            );
            u_var6 = 0x1a4;
        }
        FUN_004dcfd0(p, 0x58, 0x9d, u_var6 as i32, 0x12d);
        FUN_004daee0(0xc, DAT_00757cc4, p, 0xffff, 4, DAT_005330fc);
        1
    }
}
