// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00491810(param_1: *mut i8, param_2: i32) -> u32;
    fn FUN_00491240(param_1: *mut i8, param_2: i32) -> u32;
    fn FUN_004914a0(param_1: *mut i8, param_2: i32) -> u32;
    fn FUN_00491700(param_1: *mut i8, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004396b0(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    if param_1 < 4 || 0x342 < param_1 {
        return 0;
    }
    let p = (0x73ba30i32.wrapping_add(param_1.wrapping_mul(0x18))) as usize as *mut i8;
    if param_2 < 0x601 {
        if param_2 == 0x600 {
            // SAFETY: the callee is the game's own; p addresses its record table.
            return unsafe { FUN_00491240(p, param_3) };
        }
        if param_2 != 0x500 {
            return 0;
        }
        // SAFETY: the callee is the game's own; p addresses its record table.
        return unsafe { FUN_00491810(p, param_3) };
    }
    if param_2 == 0x700 {
        // SAFETY: the callee is the game's own; p addresses its record table.
        return unsafe { FUN_004914a0(p, param_3) };
    }
    if param_2 != 0x777 {
        return 0;
    }
    // SAFETY: the callee is the game's own; p addresses its record table.
    unsafe { FUN_00491700(p, param_3) }
}
