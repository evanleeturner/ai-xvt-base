// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004979e0(param_1: i16, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00492a00() -> u16 {
    // SAFETY: the callee and the global are the game's own; DAT_00732e54
    // points at a game structure with at least 0xbf bytes, written where
    // the original does.
    unsafe {
        FUN_004979e0(-0x8000, 1);
        let local_val: u16 = 0xffff;
        DAT_00732e54
            .cast::<u8>()
            .byte_add(0xbd)
            .cast::<u16>()
            .write_unaligned(local_val);
    }
    0
}
