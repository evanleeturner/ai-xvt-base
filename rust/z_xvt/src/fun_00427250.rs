// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536ed0: i32;
    static mut DAT_00536ecc: i32;
    static mut DAT_00536ec8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00427250() {
    // SAFETY: the globals hold the game's own COM-style object pointers;
    // each vtable slot is called as the original does, in the same order.
    unsafe {
        if DAT_00536ed0 != 0 {
            let obj = DAT_00536ecc;
            let vtbl = (obj as *const i32).read_unaligned();
            let f: unsafe extern "stdcall" fn(i32) =
                core::mem::transmute(((vtbl + 0x20) as *const u32).read_unaligned() as usize);
            f(obj);
            DAT_00536ed0 = 0;
        }
        if DAT_00536ecc != 0 {
            let obj = DAT_00536ecc;
            let vtbl = (obj as *const i32).read_unaligned();
            let f: unsafe extern "stdcall" fn(i32) =
                core::mem::transmute(((vtbl + 8) as *const u32).read_unaligned() as usize);
            f(obj);
        }
        if DAT_00536ec8 != 0 {
            let obj = DAT_00536ec8;
            let vtbl = (obj as *const i32).read_unaligned();
            let f: unsafe extern "stdcall" fn(i32) =
                core::mem::transmute(((vtbl + 8) as *const u32).read_unaligned() as usize);
            f(obj);
        }
    }
}
