// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebbd: u32;
    static mut DAT_00aa73f6: u32;
    static mut DAT_00aa7416: u32;
    static mut DAT_00aac111: u32;
    static mut DAT_00aa73a0: u8;
    static mut DAT_00aa73a1: u8;
    static mut DAT_00aa6cfc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004d48e0() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dc450(param_1: u32, param_2: *mut i32);
    fn FUN_004d5420() -> i32;
    fn FUN_004dcb30();
    fn FUN_004dc7e0();
    fn _exit(code: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dc3b0(param_1: u32, param_2: *mut i32) -> u32 {
    // SAFETY: the callees and globals are the game's own memory; the
    // zeroed ranges are 8 consecutive dwords at the addresses the original clears.
    unsafe {
        FUN_004d4970();
        FUN_004dc450(param_1, param_2);
        DAT_00acebbd = 0;
        let mut p = &raw mut DAT_00aa73f6;
        let mut i: i32 = 8;
        while i != 0 {
            p.write_unaligned(0);
            p = p.byte_add(4);
            i = i.wrapping_sub(1);
        }
        let mut p = &raw mut DAT_00aa7416;
        let mut i: i32 = 8;
        while i != 0 {
            p.write_unaligned(0);
            p = p.byte_add(4);
            i = i.wrapping_sub(1);
        }
        if DAT_00aac111 != 0 {
            DAT_00aac111 = DAT_00aac111.wrapping_sub(1);
        }
        DAT_00aa73a0 = 0;
        DAT_00aa73a1 = 0;
        loop {
            let i_var1 = FUN_004d5420();
            if i_var1 == 1 {
                FUN_004dcb30();
                FUN_004dc7e0();
                DAT_00aa6cfc = FUN_004d48e0();
                DAT_00aa73a0 = 0;
                DAT_00aa73a1 = 0;
                return 1;
            }
            if i_var1 == 2 {
                break;
            }
        }
        _exit(0);
        0
    }
}
