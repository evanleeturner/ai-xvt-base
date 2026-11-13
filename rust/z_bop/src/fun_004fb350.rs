// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00b69cdc: i32;
    static mut DAT_00aa6144: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a2c8: i32;
    static mut DAT_00b6a2fe: i32;
    static mut DAT_00669798: u32;
    static mut _DAT_00669794: u32;
    static mut DAT_0052d0b4: i8;
    static mut DAT_0052bf74: i8;
    static mut DAT_0052a6d4: i8;
    static mut DAT_0052a6cc: i8;
    static mut LAB_00506690: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de110() -> i32;
    fn FUN_004cfee0() -> i32;
    fn FUN_004d1400() -> i32;
    fn GetTickCount() -> u32;
    fn FUN_004b4a60(a: *const i8, b: *const i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *const i8, b: i32, c: i32);
    fn FUN_004b5570(a: *const i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc170();
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004e6310(a: i32);
    fn FUN_004b4bf0(a: *const i8);
    fn FUN_004dc330(a: unsafe extern "C" fn() -> i32, b: *mut i32);
    fn FUN_005066a0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004fb350(param_1: i32) -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all callees and globals are the game's own; local_10 is a
    // valid buffer for the rect the callee fills.
    unsafe {
        if param_1 == 0 {
            let mut i_var2: i32 = 1;
            FUN_004de110();
            _DAT_00b69cdc = 1;
            DAT_00aa6144 = FUN_004cfee0();
            if DAT_00b69e30 == 2 {
                DAT_00aa6144 = 1;
            } else {
                i_var2 = FUN_004d1400();
            }
            DAT_00b6a2c8 = i_var2;
            DAT_00b6a2fe = i_var2;
            DAT_00669798 = GetTickCount();
            FUN_004b4a60(&raw const DAT_0052d0b4, &raw const DAT_0052bf74);
            FUN_004d4e20();
            FUN_004b5de0(&raw const DAT_0052bf74, 0, 0);
            FUN_004b5570(&raw const DAT_0052a6d4, 0, 0);
            FUN_004b5570(&raw const DAT_0052a6cc, 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc170();
        }
        FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x27f, 0x1df);
        let pi_var3 = local_10.as_mut_ptr();
        let pc_var1 = FUN_004ddc10(0xcd);
        FUN_004db640(0xf, pc_var1, pi_var3, 0xffff);
        if DAT_00aa6144 != 0 {
            _DAT_00669794 = GetTickCount();
            if (DAT_00669798.wrapping_add(2000) as i32) < (_DAT_00669794 as i32) {
                FUN_004e6310(0);
                _DAT_00b69cdc = 1;
                FUN_004b4bf0(&raw const DAT_0052bf74);
                FUN_004dc330(FUN_005066a0, &raw mut LAB_00506690);
                return 0;
            }
        }
        _DAT_00669794 = GetTickCount();
        if (DAT_00669798.wrapping_add(4000) as i32) < (_DAT_00669794 as i32) {
            _DAT_00b69cdc = 1;
            FUN_004b4bf0(&raw const DAT_0052bf74);
            FUN_004dc330(FUN_005066a0, &raw mut LAB_00506690);
            return 0;
        }
        0
    }
}
