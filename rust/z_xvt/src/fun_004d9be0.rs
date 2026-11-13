// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f788: i32;
    static mut DAT_0064f678: i32;
    static mut DAT_0064f790: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df760(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004dd3e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
    fn FUN_004dd000(a: *mut i32, b: *mut i32);
    fn FUN_004dd070(a: *mut i32, b: i32, c: i32);
    fn FUN_004da920(a: i32, b: *mut i8, c: *mut i32, d: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9be0(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: u32,
    param_5: i8,
) {
    let _ = param_4;
    let mut ai_stack_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's rect, read where the original reads it.
    unsafe {
        if DAT_0064f788 == 0 {
            DAT_0064f788 = 1;
            DAT_0064f678 = FUN_004df760(5, 99, 0x6d);
            DAT_0064f790 = FUN_004df760(99, 0xe7, 0xf7);
        }
        let u_var1: u32;
        if param_5 == 0 {
            FUN_004dd3e0(param_1, 0, 0, DAT_0064f678 as i16, 1);
            FUN_004dd000(ai_stack_10.as_mut_ptr(), param_1);
            let n = if param_1.add(3).read().wrapping_sub(param_1.add(1).read()) < 0xf {
                1
            } else {
                3
            };
            FUN_004dd070(ai_stack_10.as_mut_ptr(), n, n);
            FUN_004dd3e0(ai_stack_10.as_mut_ptr(), 0, 0, DAT_0064f790 as i16, 1);
            u_var1 = DAT_0064f678 as u32;
        } else {
            FUN_004dd3e0(param_1, 0, 0, DAT_0064f790 as i16, 1);
            FUN_004dd000(ai_stack_10.as_mut_ptr(), param_1);
            let n = if param_1.add(3).read().wrapping_sub(param_1.add(1).read()) < 0xf {
                1
            } else {
                3
            };
            FUN_004dd070(ai_stack_10.as_mut_ptr(), n, n);
            FUN_004dd3e0(ai_stack_10.as_mut_ptr(), 0, 0, DAT_0064f678 as i16, 1);
            u_var1 = DAT_0064f790 as u32;
        }
        FUN_004da920(param_3 as i32, param_2, param_1, u_var1);
    }
}
