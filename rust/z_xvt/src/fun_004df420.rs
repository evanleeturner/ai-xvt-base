// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee8: [i8; 1];
    static mut DAT_0064fca8: i32;
    static mut DAT_0077dc66: i32;
    static mut DAT_0077dc42: [i32; 1];
    static mut DAT_0064fcac: u16;
    static mut DAT_00532c3c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd3e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u16, param_5: i32);
    fn sprintf(buffer: *mut i8, format: *const i8, ...);
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: i32);
    fn FUN_004df260(param_1: i32) -> u8;
    fn FUN_004df290(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004dbea0();
    fn FUN_004dbb40();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004df420() -> i32 {
    let mut ac_stack_64 = [0i8; 100];
    // SAFETY: the globals and callees are the game's own memory and code;
    // the buffer is local and large enough for the original's usage.
    unsafe {
        let flag = (&raw const DAT_00758ee8)
            .cast::<i8>()
            .offset(DAT_0064fca8 as isize)
            .read();
        let mut skip = false;
        if flag != 0 {
            let i_var2 = DAT_0077dc66.wrapping_mul(0x440);
            let base = (&raw mut DAT_0077dc42)
                .cast::<u8>()
                .byte_offset(i_var2 as isize)
                .cast::<i32>();
            FUN_004dd3e0(base, 0, 0, DAT_0064fcac, 1);
            sprintf(
                ac_stack_64.as_mut_ptr(),
                (&raw const DAT_00532c3c).cast::<i8>(),
                DAT_0064fca8.wrapping_add(1),
            );
            FUN_004da920(0x14, ac_stack_64.as_mut_ptr(), base, 0xff);
            if FUN_004df260(DAT_0064fca8) == 0 && FUN_004df290(DAT_0064fca8) == 0 {
                skip = true;
            }
        }
        if !skip {
            DAT_0064fca8 = DAT_0064fca8.wrapping_add(1);
        }
        if DAT_0064fca8 > 1 {
            FUN_004dbea0();
            FUN_004dbb40();
        }
    }
    0
}
