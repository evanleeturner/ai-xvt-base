// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758620: u32;
    static mut DAT_00757870: u32;
    static mut DAT_00758624: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cb830(param_1: u32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f0db0() -> u32 {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the copy moves 0x50 dwords from DAT_00757870 to DAT_00758624 as the original does.
    unsafe {
        DAT_00758620 = 0x48;
        let mut src: *const u32 = &raw mut DAT_00757870;
        let mut dst: *mut u32 = &raw mut DAT_00758624;
        let mut count: u32 = 0x50;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            count -= 1;
        }
        FUN_004cb830(0, &raw mut DAT_00758620, 0x144);
    }
    1
}
