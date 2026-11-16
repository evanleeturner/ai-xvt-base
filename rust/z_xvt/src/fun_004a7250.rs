// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658020: *mut *mut core::ffi::c_void;
    static mut DAT_00657c00: *mut *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a7250() {
    let mut local_64: [i32; 20] = [0; 20];
    local_64[0] = 100;
    loop {
        // SAFETY: the globals are the game's own COM-style objects; the
        // vtable slots are read and called exactly as the original does.
        unsafe {
            let obj = DAT_00658020;
            let vtbl = *obj as *const u8;
            let f = vtbl
                .byte_add(0x14)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(
                *mut *mut core::ffi::c_void,
                i32,
                i32,
                i32,
                i32,
                *mut i32,
            ) -> i32 = core::mem::transmute(f);
            let i_var1 = f(obj, 0, 0, 0, 0x400, local_64.as_mut_ptr());
            if i_var1 == 0 {
                return;
            }
            if i_var1 == -0x7789FE3E {
                let obj2 = DAT_00657c00;
                let vtbl2 = *obj2 as *const u8;
                let g = vtbl2
                    .byte_add(0x6C)
                    .cast::<*const core::ffi::c_void>()
                    .read_unaligned();
                let g: unsafe extern "stdcall" fn(*mut *mut core::ffi::c_void) -> i32 =
                    core::mem::transmute(g);
                let i_var2 = g(obj2);
                if i_var2 != 0 {
                    break;
                }
            }
            if i_var1 != -0x7789FDE4 {
                return;
            }
        }
    }
}
