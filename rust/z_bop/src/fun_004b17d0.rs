// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn _free(p: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_004b17d0(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's 0x36-dword object, which is
    // cleared and freed here as the original does; the vtable call and
    // _free belong to the game.
    unsafe {
        if param_1.read() == 1 {
            let p_var2 = param_1.add(0x19).read() as usize as *mut i32;
            if !p_var2.is_null() {
                let vtable = p_var2.read() as usize;
                let f = ((vtable + 8) as *const usize).read();
                let func: unsafe extern "stdcall" fn(*mut i32) = core::mem::transmute(f);
                func(p_var2);
            }
        } else {
            _free(param_1.add(0x17).read() as usize as *mut core::ffi::c_void);
        }
        let mut p = param_1;
        let mut i = 0x36i32;
        while i != 0 {
            p.write(0);
            p = p.add(1);
            i -= 1;
        }
        _free(param_1 as *mut core::ffi::c_void);
    }
}
