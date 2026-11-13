// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c01c: i32;
    static mut DAT_0052c020: i32;
    static mut DAT_00665468: i32;
    static mut DAT_0066546c: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d6e30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004daca0(param_1: *mut i32, param_2: *mut i8);
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004de130(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d67e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d6950(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16);
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004daa20(
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

    // SAFETY: the callees and globals are the game's own; all pointers passed
    // are either the caller's or point at locals that outlive the calls.
    unsafe {
        FUN_004b5570(param_2, 0, 0);
        if DAT_0052c01c == 0 {
            DAT_0052c01c = 1;
            DAT_0052c020 = FUN_004d6e30(0x60, 0x60, 0x60);
        }
        FUN_004de090(&raw mut local_1c, &raw mut local_20);
        if DAT_00665468 != 0 {
            FUN_004daca0(param_1, DAT_0066546c);
        }
        if !param_3.is_null() {
            let r = FUN_004d6b90(param_1, local_1c, local_20);
            if r != 0 {
                let i_var1 = FUN_004dbfa0(param_3, param_4);
                FUN_004de130(&raw mut local_18, &raw mut local_14);
                local_1c = local_1c.wrapping_add(local_18);
                local_20 = local_20.wrapping_add(local_14);
                if 0x27f < local_1c.wrapping_add(i_var1).wrapping_add(5) {
                    local_1c = 0x27a_i32.wrapping_sub(i_var1);
                }
                if 0x1df < (local_20.wrapping_add(param_4).wrapping_add(5)) as u32 {
                    local_20 = 0x1da_i32.wrapping_sub(param_4);
                }
                let p = local_10.as_mut_ptr();
                FUN_004d6460(
                    p,
                    local_1c,
                    local_20,
                    local_1c.wrapping_add(i_var1).wrapping_add(5),
                    local_20.wrapping_add(param_4).wrapping_add(3),
                );
                FUN_004d67e0(p, 0, 0, 0xffff, 1);
                FUN_004d6950(p, 0, 0, DAT_0052c020 as i16);
                FUN_004db640(param_4, param_3, p, 0);
            }
        }
    }
}
