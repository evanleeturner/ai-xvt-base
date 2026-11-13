// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae18: u32;
    static mut DAT_006a885e: u32;
    static mut DAT_006a886c: u32;
    static mut DAT_006a889a: u32;
    static mut DAT_006a889e: u32;
    static mut DAT_006a8850: u32;
    static mut DAT_006a8854: u32;
    static mut DAT_006a8862: i32;
    static mut DAT_006a8870: i32;
    static mut DAT_006a8894: u32;
    static mut DAT_0052a34c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045f100(param_1: *mut u32);
    fn FUN_004840f0(param_1: i32);
    fn FUN_00484970(param_1: *mut i32);
    fn FUN_004842b0(param_1: *mut i32);
    fn FUN_00484fd0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485db0(param_1: *mut u32) {
    // SAFETY: the globals and callees are the game's own; param_1 points at a
    // 0xd0-byte record, copied into the game's record array as the original does.
    unsafe {
        if DAT_0052ae18 != 0 {
            FUN_0045f100(param_1);
            return;
        }
        DAT_006a885e = 0;
        DAT_006a886c = 0;
        if DAT_006a889a != DAT_006a889e
            && param_1
                .byte_add(0x23 * 4)
                .read_unaligned()
                .wrapping_add(DAT_006a8850)
                <= DAT_006a8854
            && (param_1.byte_add(0x1e * 4).read_unaligned() as i32) <= DAT_006a8862
            && (param_1.byte_add(0x24 * 4).read_unaligned() as i32) <= DAT_006a8870
        {
            let addr = DAT_006a889e.wrapping_mul(0xd0).wrapping_add(DAT_006a8894);
            let mut src = param_1;
            let mut dst = addr as *mut u32;
            let mut n: i32 = 0x34;
            while n != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.byte_add(4);
                dst = dst.byte_add(4);
                n -= 1;
            }
            let pi_var2 = addr as *mut i32;
            FUN_004840f0(addr as i32);
            if pi_var2.byte_add(0x31 * 4).read_unaligned() != 0 {
                if DAT_0052a34c == 0 {
                    FUN_004842b0(pi_var2);
                } else {
                    FUN_00484970(pi_var2);
                }
                FUN_00484fd0(addr as i32);
                DAT_006a889e = DAT_006a889e.wrapping_add(1);
            }
        }
    }
}
