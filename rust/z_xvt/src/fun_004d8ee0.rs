// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532adc: u32;
    static mut DAT_0064f684: u32;
    static mut DAT_0064f688: u32;
    static mut DAT_0064f68c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d8ee0() -> u32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // copy and the indexed store hit the same addresses the original does.
    unsafe {
        let u_var1 = DAT_0064f688;
        let n = DAT_00532adc;
        if n == 0 {
            return 0;
        }
        _memcpy(
            (&raw mut DAT_0064f688).cast::<core::ffi::c_void>(),
            (&raw mut DAT_0064f68c)
                .cast::<core::ffi::c_void>()
                .cast_const(),
            n.wrapping_mul(4).wrapping_sub(4),
        );
        (&raw mut DAT_0064f684)
            .byte_add((n as usize).wrapping_mul(4))
            .write_unaligned(u_var1);
        1
    }
}
