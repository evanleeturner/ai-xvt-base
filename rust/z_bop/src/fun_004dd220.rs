// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665808: i32;
    static mut DAT_0066580c: i32;
    static mut DAT_00665890: i32;
    static mut DAT_00665894: i32;
    static mut DAT_00665898: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dad90() -> i32;
    fn FUN_004dad60();
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dc3b0(param_1: i32, param_2: *mut i32);
    fn FUN_004dc170();
    fn FUN_004dad40();
    fn FUN_004dd2c0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd220(param_1: *mut i32) -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's buffer, written where the original writes.
    unsafe {
        let i_var1 = FUN_004dad90();
        FUN_004dad60();
        FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
        FUN_004de090(&raw mut DAT_00665808, &raw mut DAT_0066580c);
        let p = &raw mut DAT_00665890;
        let mut i: usize = 0;
        while i < 0x40 {
            p.add(i).write_volatile(0);
            i += 1;
        }
        FUN_004dc3b0(FUN_004dd2c0 as usize as i32, local_10.as_mut_ptr());
        FUN_004dc170();
        if i_var1 != 0 {
            FUN_004dad40();
        }
        param_1.write_unaligned(DAT_00665890);
        param_1.add(1).write_unaligned(DAT_00665894);
        param_1.add(2).write_unaligned(DAT_00665898);
        param_1.add(3).cast::<u8>().write(0);
    }
    1
}
