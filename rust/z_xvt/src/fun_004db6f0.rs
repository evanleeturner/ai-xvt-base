// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806f2: u32;
    static mut DAT_00758f2e: u32;
    static mut DAT_00758f4e: u32;
    static mut DAT_0075dc46: u32;
    static mut DAT_00758ee0: u8;
    static mut DAT_00758ee1: u8;
    static mut DAT_00758830: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
    fn FUN_004d4d00() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004db790(param_1: u32, param_2: *mut i32);
    fn FUN_004d58d0() -> i32;
    fn FUN_004dbea0();
    fn FUN_004dbb40();
    fn _exit(code: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db6f0(param_1: u32, param_2: *mut i32) -> u32 {
    // SAFETY: the callees and globals are the game's own memory; the
    // zeroed ranges are 8 consecutive u32 words at each address.
    unsafe {
        FUN_004d4d90();
        FUN_004db790(param_1, param_2);
        DAT_007806f2 = 0;
        let mut p = &raw mut DAT_00758f2e;
        let mut i = 8;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        let mut p = &raw mut DAT_00758f4e;
        let mut i = 8;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        if DAT_0075dc46 != 0 {
            DAT_0075dc46 = DAT_0075dc46.wrapping_sub(1);
        }
        DAT_00758ee0 = 0;
        DAT_00758ee1 = 0;
        loop {
            let r = FUN_004d58d0();
            if r == 1 {
                FUN_004dbea0();
                FUN_004dbb40();
                DAT_00758830 = FUN_004d4d00();
                DAT_00758ee0 = 0;
                DAT_00758ee1 = 0;
                return 1;
            }
            if r == 2 {
                break;
            }
        }
        _exit(0);
        0
    }
}
