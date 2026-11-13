// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5cb0: u32;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cef70(param_1: u32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f4710() -> u32 {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the copy runs over 0x50 consecutive dwords from DAT_00aa5cb0 into the
    // buffer at DAT_00aa6af4, as the original does.
    unsafe {
        DAT_00aa6af0 = 0x48;
        let mut src: *mut u32 = &raw mut DAT_00aa5cb0;
        let mut dst: *mut u32 = &raw mut DAT_00aa6af4;
        let mut count: u32 = 0x50;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            count -= 1;
        }
        FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x144);
    }
    1
}
