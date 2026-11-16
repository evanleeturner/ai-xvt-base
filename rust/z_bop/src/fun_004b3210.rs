// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528cb4: *mut i32;
    static mut DAT_00518ee0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments)]
pub unsafe extern "stdcall" fn FUN_004b3210() -> i32 {
    // SAFETY: the globals and the object they point to are the game's own;
    // the vtable slots are called with the signatures the original uses.
    unsafe {
        let pi_var2: *mut i32 = DAT_00528cb4;
        let local_8: i32 = 0;
        let mut local_10: *mut i32 = &raw mut DAT_00518ee0;
        let vtable: *const *const core::ffi::c_void =
            (pi_var2.read() as u32 as usize) as *const *const core::ffi::c_void;
        let f0: unsafe extern "stdcall" fn() -> i32 = core::mem::transmute(vtable.read());
        let i_var1 = f0();
        if i_var1 != 0 {
            return 0;
        }
        local_10 = 0x1000usize as *mut i32;
        let local_4: i32 = 0;
        let f17: unsafe extern "stdcall" fn(*mut i32, *mut *mut i32, i32, i32) -> i32 =
            core::mem::transmute(vtable.add(0x17).read());
        let i_var1 = f17(pi_var2, &raw mut local_10, local_8, local_4);
        if i_var1 != 0 {
            return 0;
        }
        let f2: unsafe extern "stdcall" fn(*mut i32) = core::mem::transmute(vtable.add(2).read());
        f2(pi_var2);
        1
    }
}
