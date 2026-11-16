// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005569d4: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044b790(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: *mut *mut i32,
        param_5: *mut i32,
    ) -> i32;
    fn FUN_0044bad0(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> i32;
    fn _free(ptr: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b650(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: i32,
) -> *mut i32 {
    let mut local_1c: *mut i32 = core::ptr::null_mut();
    let mut local_18: *mut i32 = core::ptr::null_mut();
    // buf[0] = local_14, buf[1] = local_10, buf[2..5] = local_c[0..3]
    let mut buf: [i32; 5] = [0; 5];
    // SAFETY: the callees and the global are the game's own; param_1 points at
    // a COM-like object whose first word is a vtable, and local_1c is the
    // object returned by that vtable call.
    unsafe {
        DAT_005569d4 = core::ptr::null_mut();
        let buf_ptr = buf.as_mut_ptr();
        let iv = FUN_0044b790(
            0,
            param_2,
            buf_ptr.add(4),
            &raw mut local_18,
            buf_ptr.add(2),
        );
        if iv != 0 {
            buf[0] = 0x14;
            buf[1] = 0xc2;
            if param_3 == 0 {
                buf[1] = 0xea;
            }
            let vt = (param_1.read() as u32 as usize) as *const *const core::ffi::c_void;
            let f: unsafe extern "C" fn(*mut i32, *mut i32, *mut *mut i32, i32) -> i32 =
                core::mem::transmute(vt.add(3).read());
            let r = f(param_1, buf_ptr, &raw mut local_1c, 0);
            if r > -1 {
                let r2 = FUN_0044bad0(local_1c, local_18, buf[2]);
                if r2 != 0 {
                    if !DAT_005569d4.is_null() {
                        _free(DAT_005569d4);
                    }
                    return local_1c;
                }
                let vt2 = (local_1c.read() as u32 as usize) as *const *const core::ffi::c_void;
                let g: unsafe extern "C" fn(*mut i32) = core::mem::transmute(vt2.add(2).read());
                g(local_1c);
            }
            local_1c = core::ptr::null_mut();
        }
        if !DAT_005569d4.is_null() {
            _free(DAT_005569d4);
        }
    }
    local_1c
}
