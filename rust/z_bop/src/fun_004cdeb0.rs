// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebc5: *mut u32;
    static mut DAT_00acebe1: u32;
    static mut DAT_00acebe5: u32;
    static mut DAT_00acebe9: u32;
    static mut DAT_00acebed: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cdf60(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdeb0(param_1: u32, param_2: *mut u32) -> u32 {
    let mut param_2 = param_2;
    // SAFETY: the callee and globals are the game's own; param_2 points at
    // the caller's (or callee-provided) four u32 values; the object in
    // DAT_00acebc5 is a COM-like object whose vtable slot 0x60 is called.
    unsafe {
        if param_2.is_null() {
            param_2 = FUN_004cdf60(param_1) as usize as *mut u32;
            if param_2.is_null() {
                return 0;
            }
        }
        let mut local_50: [u32; 20] = [0; 20];
        local_50[0] = 0x50;
        local_50[1] = 0x40;
        local_50[2] = param_2.read_unaligned();
        local_50[3] = param_2.byte_add(4).read_unaligned();
        let obj = DAT_00acebc5;
        let vtbl = obj.read_unaligned() as usize as *const u8;
        let f = vtbl
            .byte_add(0x60)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut u32, *mut u32, u32) -> i32 = core::mem::transmute(f);
        let r = f(obj, local_50.as_mut_ptr(), 1);
        if r != 0 {
            return 0;
        }
        DAT_00acebe1 = param_2.read_unaligned();
        DAT_00acebe5 = param_2.byte_add(4).read_unaligned();
        DAT_00acebe9 = param_2.byte_add(8).read_unaligned();
        DAT_00acebed = param_2.byte_add(12).read_unaligned();
        1
    }
}
