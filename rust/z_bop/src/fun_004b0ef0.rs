// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006644f8: i32;
    static mut DAT_006644fc: i32;
    static mut DAT_00664504: i32;
    static mut DAT_0066450c: i32;
    static mut DAT_00664510: i32;
    static mut DAT_00664514: i32;
    static mut DAT_00664518: i32;
    static mut DAT_0066451c: i32;
    static mut DAT_00664520: i32;
    static mut DAT_00664524: i32;
    static mut DAT_00664528: i32;
    static mut DAT_0066452c: i32;
    static mut DAT_00664530: i32;
    static mut DAT_00664534: i32;
    static mut DAT_00664538: i32;
    static mut DAT_0066453c: i32;
    static mut DAT_00664540: i32;
    static mut DAT_00664508: i32;
    static mut DAT_00664500: i32;
    static mut DAT_00a90be0: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b0ef0(param_1: u32, param_2: i32, param_3: i32) {
    // SAFETY: all globals are the game's own memory; the zeroing loop clears
    // the 19-dword block starting at DAT_006644f8 exactly as the original does.
    unsafe {
        let mut p = (&raw mut DAT_006644f8).cast::<u32>();
        let mut i: i32 = 0x13;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        DAT_00a90be0 = &raw mut DAT_006644f8;
        DAT_006644f8 = param_1 as i32;
        DAT_006644fc = param_2;
        DAT_00664504 = param_3;
        DAT_0066450c = 1;
        DAT_00664510 = 0x10;
        DAT_00664514 = 5;
        DAT_00664518 = 6;
        DAT_0066451c = 5;
        DAT_00664520 = 0xb;
        DAT_00664524 = 5;
        DAT_00664528 = 0;
        DAT_0066452c = 3;
        DAT_00664530 = 2;
        DAT_00664534 = 3;
        DAT_00664538 = 0;
        DAT_0066453c = 0;
        DAT_00664540 = 0;
        DAT_00664508 = param_3 / 2;
        DAT_00664500 = param_2.wrapping_mul(param_3);
    }
}
