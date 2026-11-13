// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665470: u32;
    static mut DAT_006654f0: u32;
    static mut DAT_0052c008: u32;
    static mut DAT_0052c004: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d9d10() -> u32 {
    // SAFETY: the globals are the game's own memory; the source and destination
    // blocks are 0x20 dwords each, copied in order as the original does.
    unsafe {
        let mut src = &raw mut DAT_00665470 as *const u32;
        let mut dst = &raw mut DAT_006654f0;
        let mut count: i32 = 0x20;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            count -= 1;
        }
        DAT_0052c008 = DAT_0052c004;
    }
    1
}
