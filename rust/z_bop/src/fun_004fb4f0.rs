// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052b12c: *const i8;
    static mut DAT_0052d180: *const i8;
    static mut DAT_0052bf74: *const i8;
    static mut DAT_0052d160: *const i8;
    static mut DAT_0052d178: *const i8;
    static mut DAT_0052d13c: *const i8;
    static mut DAT_0052d154: *const i8;
    static mut DAT_0052d120: *const i8;
    static mut DAT_0052d134: *const i8;
    static mut DAT_0052d104: *const i8;
    static mut DAT_0052d118: *const i8;
    static mut DAT_0052d0e8: *const i8;
    static mut DAT_0052d0fc: *const i8;
    static mut DAT_0052d0cc: *const i8;
    static mut DAT_0052d0e0: *const i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4dd0();
    fn FUN_004de110();
    fn FUN_004d4bf0();
    fn FUN_004d4f60();
    fn FUN_004d3330();
    fn FUN_004d2e50();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4c00(param_1: i32);
    fn FUN_004dada0(param_1: i32);
    fn FUN_004df700(param_1: *const i8);
    fn FUN_004b4a60(param_1: *const i8, param_2: *const i8);
    fn FUN_004b4b20(param_1: *const i8, param_2: *const i8, param_3: i32, param_4: i32);
    fn FUN_004d34a0(param_1: i32);
    fn FUN_004d3020(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fb4f0() -> u32 {
    // SAFETY: all callees and globals are the game's own; the string
    // pointers are read by value in the original's order.
    unsafe {
        FUN_004d4dd0();
        FUN_004d4c00(0);
        FUN_004de110();
        FUN_004d4bf0();
        FUN_004d4f60();
        FUN_004dada0(0xf);
        FUN_004df700(DAT_0052b12c);
        FUN_004b4a60(DAT_0052d180, DAT_0052bf74);
        FUN_004b4b20(DAT_0052d160, DAT_0052d178, 0, 0);
        FUN_004b4b20(DAT_0052d13c, DAT_0052d154, 0, 0);
        FUN_004b4a60(DAT_0052d120, DAT_0052d134);
        FUN_004b4a60(DAT_0052d104, DAT_0052d118);
        FUN_004b4a60(DAT_0052d0e8, DAT_0052d0fc);
        FUN_004b4a60(DAT_0052d0cc, DAT_0052d0e0);
        FUN_004d3330();
        FUN_004d2e50();
        FUN_004d34a0(0x8000);
        FUN_004d3020(4, 0, 0);
    }
    0
}
