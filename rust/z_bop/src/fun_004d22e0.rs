// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00acebc5: *mut core::ffi::c_void;
    static mut DAT_00aa6cfc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004d48e0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d22e0(param_1: u32, param_2: u32, param_3: u32) -> u8 {
    // SAFETY: the globals and callees are the game's own; DAT_00acebc5 is a
    // COM-like object whose first field is a vtable pointer, and the slot at
    // byte offset 0x78 holds a stdcall function, called as the original does.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        FUN_004d4970();
        let obj = DAT_00acebc5;
        if obj.is_null() {
            return 0;
        }
        let mut local: [u32; 4] = [0x10, 0, param_3, param_2];
        let vtable = obj.cast::<*mut core::ffi::c_void>().read_unaligned();
        let slot = vtable
            .byte_add(0x78)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, u32, *mut u32, u32) -> i32 =
            core::mem::transmute(slot);
        let i_var2 = f(obj, param_1, local.as_mut_ptr(), 0);
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        (i_var2 == 0) as u8
    }
}
