// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00646e18: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6248(param_1: i32) -> i32 {
    // SAFETY: the global is the game's own object pointer; param_1 points at
    // the caller's 256-entry table of 32-bit values, and the vtable slot at
    // offset 0x18 is called with the same arguments as the original.
    unsafe {
        let obj = DAT_00646e18;
        if obj.is_null() {
            return 0;
        }
        let mut local_408 = [0u8; 1028];
        let mut i: i32 = 1;
        while i < 0x100 {
            let idx = (i as usize) * 4;
            let v =
                ((param_1.wrapping_add(i.wrapping_mul(4))) as usize as *const u32).read_unaligned();
            local_408[idx + 3] = 0;
            local_408[idx] = v as u8;
            local_408[idx + 1] = (v >> 8) as u8;
            local_408[idx + 2] = (v >> 16) as u8;
            i += 1;
        }
        let vtbl = (obj as *const *const core::ffi::c_void).read_unaligned();
        let fp = vtbl
            .byte_add(0x18)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(
            *mut core::ffi::c_void,
            i32,
            i32,
            i32,
            *mut core::ffi::c_void,
        ) -> i32 = core::mem::transmute(fp);
        let r = f(obj, 0, 0, 0x100, local_408.as_mut_ptr().cast());
        if r == 0 { 1 } else { 0 }
    }
}
