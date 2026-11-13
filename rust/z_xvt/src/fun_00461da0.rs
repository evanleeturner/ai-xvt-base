// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006cca80: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00461da0() -> u8 {
    // SAFETY: DAT_006cca80 is the game's own pointer to a live object; the
    // field at offset 0xc is read as the original does.
    unsafe {
        let p = DAT_006cca80;
        (p.byte_add(0xc).cast::<i32>().read_unaligned() == 5) as u8
    }
}
