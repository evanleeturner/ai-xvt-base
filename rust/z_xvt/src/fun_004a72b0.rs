// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00657c00: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a72b0(param_1: *mut i32) {
    let mut local_64: [i32; 20] = [0; 20];
    local_64[0] = 100;
    loop {
        // SAFETY: param_1 is a COM-like object whose vtable pointer sits at its start;
        // the slot at offset 0x14 is called with the original's arguments.
        let i_var1: i32 = unsafe {
            let vtbl = param_1.read_unaligned() as usize;
            let f = (vtbl + 0x14) as *const *const core::ffi::c_void;
            let f: unsafe extern "C" fn(*mut i32, i32, i32, i32, i32, *mut i32) -> i32 =
                core::mem::transmute(f.read_unaligned());
            f(param_1, 0, 0, 0, 0x400, local_64.as_mut_ptr())
        };
        if i_var1 == 0 {
            return;
        }
        if i_var1 == 0x887601c2u32 as i32 {
            // SAFETY: DAT_00657c00 is the game's global object pointer, with a vtable at its start;
            // slot 0x6c is called with the object itself.
            let i_var2: i32 = unsafe {
                let obj = DAT_00657c00;
                let vtbl = obj.read_unaligned() as usize;
                let f = (vtbl + 0x6c) as *const *const core::ffi::c_void;
                let f: unsafe extern "C" fn(*mut i32) -> i32 =
                    core::mem::transmute(f.read_unaligned());
                f(obj)
            };
            if i_var2 != 0 {
                break;
            }
        }
        if i_var1 != 0x8876021cu32 as i32 {
            return;
        }
    }
}
