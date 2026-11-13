// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _free(ptr: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00443c30(param_1: *mut core::ffi::c_void) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's structure; the fields are read
    // where the original reads them and freed through the game's own _free.
    unsafe {
        let p = param_1 as *mut u8;
        for off in [0x1014usize, 0x1018, 0x1044, 0x1048, 0x104c] {
            let v = p
                .byte_add(off)
                .cast::<*mut core::ffi::c_void>()
                .read_unaligned();
            if !v.is_null() {
                _free(v);
            }
        }
        for off in [0x1082usize, 0x1086, 0x109a, 0x18ae, 0x18b2] {
            let v = p
                .byte_add(off)
                .cast::<*mut core::ffi::c_void>()
                .read_unaligned();
            if !v.is_null() {
                _free(v);
            }
        }
        _free(param_1);
    }
}
