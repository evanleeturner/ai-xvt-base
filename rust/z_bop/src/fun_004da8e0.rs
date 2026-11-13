// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665570: i32;
    static mut DAT_00665460: i32;
    static mut DAT_00665578: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6e30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004d67e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
    fn FUN_004d6480(a: *mut i32, b: *mut i32);
    fn FUN_004d64c0(a: *mut i32, b: i32, c: i32);
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004da8e0(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: i8,
) {
    let _ = param_4;
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's rect of four ints, local_10 is a local rect.
    unsafe {
        if DAT_00665570 == 0 {
            DAT_00665570 = 1;
            DAT_00665460 = FUN_004d6e30(5, 99, 0x6d);
            DAT_00665578 = FUN_004d6e30(99, 0xe7, 0xf7);
        }
        let u_var1: u32;
        if param_5 == 0 {
            FUN_004d67e0(param_1, 0, 0, DAT_00665460 as i16, 1);
            FUN_004d6480(local_10.as_mut_ptr(), param_1);
            let n = if param_1.add(3).read().wrapping_sub(param_1.add(1).read()) < 0xf {
                1
            } else {
                3
            };
            FUN_004d64c0(local_10.as_mut_ptr(), n, n);
            FUN_004d67e0(local_10.as_mut_ptr(), 0, 0, DAT_00665578 as i16, 1);
            u_var1 = DAT_00665460 as u32;
        } else {
            FUN_004d67e0(param_1, 0, 0, DAT_00665578 as i16, 1);
            FUN_004d6480(local_10.as_mut_ptr(), param_1);
            let n = if param_1.add(3).read().wrapping_sub(param_1.add(1).read()) < 0xf {
                1
            } else {
                3
            };
            FUN_004d64c0(local_10.as_mut_ptr(), n, n);
            FUN_004d67e0(local_10.as_mut_ptr(), 0, 0, DAT_00665460 as i16, 1);
            u_var1 = DAT_00665578 as u32;
        }
        FUN_004db640(param_3, param_2, param_1, u_var1);
    }
}
