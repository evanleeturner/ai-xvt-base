// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066e700: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ac460() -> i32 {
    // SAFETY: the global is the game's own memory; it holds an address of an
    // object whose slot at offset 0x6c is a function pointer, called as the
    // original does.
    unsafe {
        let vtable = DAT_0066e700;
        let func = ((vtable as u32).wrapping_add(0x6c) as *const i32).read_unaligned();
        let f: unsafe extern "stdcall" fn(i32) -> i32 =
            core::mem::transmute(func as usize as *const core::ffi::c_void);
        let result = f(DAT_0066e700);
        (result == 0) as i32
    }
}
