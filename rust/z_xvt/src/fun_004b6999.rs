// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ee44: *mut core::ffi::c_void;
    static mut DAT_0064eecc: *mut core::ffi::c_void;
    static mut DAT_0064ee50: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b51f0(param_1: i32, param_2: i32, param_3: i32) -> *mut i8;
    fn FUN_0045bbe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6999(param_1: u8) -> i32 {
    let mut local_6c: [u32; 20];
    let mut local_8: i32;

    if (param_1 & 1) != 0 {
        // SAFETY: the globals and callees are the game's own; the object at
        // DAT_0064ee44 is a COM-like object whose vtable slot 0x30 is called.
        unsafe {
            let obj = DAT_0064ee44;
            let vt = obj.cast::<*const *const core::ffi::c_void>().read();
            let f = vt.add(0x30 / 4).read();
            let f = core::mem::transmute::<
                *const core::ffi::c_void,
                unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    i32,
                    *mut core::ffi::c_void,
                    i32,
                ) -> i32,
            >(f);
            local_8 = f(obj, 1, (&raw mut DAT_0064ee50).cast(), 1);
            if local_8 != 0 {
                FUN_004b51f0(local_8, 0x52f210, 0x79);
                FUN_0045bbe0();
                return 0;
            }
        }
    }
    if (param_1 & 2) != 0 {
        local_6c = [0u32; 20];
        local_6c[0] = 100;
        // SAFETY: the globals and callees are the game's own; the object at
        // DAT_0064eecc has a vtable whose slot 0x14 is called with a local buffer.
        unsafe {
            let obj = DAT_0064eecc;
            let vt = obj.cast::<*const *const core::ffi::c_void>().read();
            let f = vt.add(0x14 / 4).read();
            let f = core::mem::transmute::<
                *const core::ffi::c_void,
                unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    i32,
                    i32,
                    i32,
                    u32,
                    *mut core::ffi::c_void,
                ) -> i32,
            >(f);
            local_8 = f(obj, 0, 0, 0, 0x3000000, local_6c.as_mut_ptr().cast());
            if local_8 != 0 {
                FUN_004b51f0(local_8, 0x52f210, 0x79);
                FUN_0045bbe0();
                return 0;
            }
        }
    }
    1
}
