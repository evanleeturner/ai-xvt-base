// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ed04: u16;
    static mut DAT_0071ed02: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00419000(param_1: u32, param_2: u32, param_3: *mut u16, param_4: *mut u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419100(param_1: u32, param_2: u32) -> u32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut local_4: u16 = 0;
    let mut local_2: u16 = 0;

    // SAFETY: the globals and the callee are the game's own; the out
    // pointers refer to live locals.
    unsafe {
        let neg1 = (param_1 as i32) < 0;
        if neg1 {
            param_1 = 0u32.wrapping_sub(param_1);
        }
        DAT_0071ed04 = neg1 as u16;
        let neg2 = (param_2 as i32) < 0;
        if neg2 {
            param_2 = 0u32.wrapping_sub(param_2);
        }
        DAT_0071ed02 = neg2 as u16;
        let extraout = FUN_00419000(param_2, param_1, &raw mut local_2, &raw mut local_4);
        if DAT_0071ed04 != 0 {
            local_2 = 0u32.wrapping_sub(local_2 as u32) as u16;
        }
        if DAT_0071ed02 != 0 {
            local_2 = 0x8000u32.wrapping_sub(local_2 as u32) as u16;
        }
        (extraout & 0xFFFF0000) | (local_2 as u32)
    }
}
