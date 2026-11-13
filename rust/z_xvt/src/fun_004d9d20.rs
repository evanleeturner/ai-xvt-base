// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532af4: i32;
    static mut DAT_00532af8: i32;
    static mut DAT_0064f680: i32;
    static mut DAT_0064f684: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004df760(param_1: i8, param_2: i8, param_3: i8) -> i32;
    fn FUN_004ddc70(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d9f90(param_1: *mut i32, param_2: *mut i8);
    fn FUN_004dd7d0(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004ddd10(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dd3e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dd570(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16);
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9d20(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: *mut i8,
    param_4: i32,
) {
    let mut local_20: i32 = 0;
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; locals are passed
    // by pointer exactly as the original does.
    unsafe {
        FUN_004d0e60(param_2, 0, 0);
        if DAT_00532af4 == 0 {
            DAT_00532af4 = 1;
            DAT_00532af8 = FUN_004df760(0x60, 0x60, 0x60);
        }
        FUN_004ddc70(&raw mut local_1c, &raw mut local_20);
        if DAT_0064f680 != 0 {
            FUN_004d9f90(param_1, DAT_0064f684);
        }
        if !param_3.is_null() {
            let i_var1 = FUN_004dd7d0(param_1, local_1c, local_20);
            if i_var1 != 0 {
                let i_var1 = FUN_004db2f0(param_3, param_4);
                FUN_004ddd10(&raw mut local_18, &raw mut local_14);
                local_1c = local_1c.wrapping_add(local_18);
                local_20 = local_20.wrapping_add(local_14);
                if 0x27f < local_1c.wrapping_add(i_var1).wrapping_add(5) {
                    local_1c = 0x27a_i32.wrapping_sub(i_var1);
                }
                if 0x1df < local_20.wrapping_add(param_4).wrapping_add(5) {
                    local_20 = 0x1da_i32.wrapping_sub(param_4);
                }
                FUN_004dcfd0(
                    local_10.as_mut_ptr(),
                    local_1c,
                    local_20,
                    local_1c.wrapping_add(i_var1).wrapping_add(5),
                    local_20.wrapping_add(param_4).wrapping_add(3),
                );
                FUN_004dd3e0(local_10.as_mut_ptr(), 0, 0, 0xffff, 1);
                FUN_004dd570(local_10.as_mut_ptr(), 0, 0, DAT_00532af8 as i16);
                FUN_004da920(param_4, param_3, local_10.as_mut_ptr(), 0);
            }
        }
    }
}
