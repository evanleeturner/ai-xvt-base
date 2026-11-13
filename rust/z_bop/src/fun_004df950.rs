// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051c4bc: i8;
    static mut s__s__d_0052c178: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004cc590(param_1: *mut core::ffi::c_void) -> i32;
    fn _fgets(ptr: *mut i8, num: i32, stream: *mut core::ffi::c_void) -> *mut i8;
    fn _fscanf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
    fn FUN_004b4bf0(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df950(param_1: *mut i8) -> i32 {
    let mut local_204: i32 = 0;
    let mut local_200: [i8; 256] = [0; 256];
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the callees and globals are the game's own; the local buffers
    // outlive every call that receives a pointer to them.
    unsafe {
        let file = FUN_004cc4a0(param_1, &raw mut DAT_0051c4bc);
        if file.is_null() {
            return 0;
        }
        let p = _fgets(local_200.as_mut_ptr(), 0xff, file);
        if p.is_null() {
            FUN_004cc590(file);
            return 1;
        }
        loop {
            let r = _fscanf(
                file,
                (&raw const s__s__d_0052c178).cast::<i8>(),
                local_200.as_mut_ptr(),
                local_100.as_mut_ptr(),
                &raw mut local_204,
            );
            if r == -1 {
                FUN_004cc590(file);
                return 1;
            }
            if r == 3 {
                break;
            }
            FUN_004b4bf0(local_100.as_mut_ptr());
        }
        FUN_004cc590(file);
        0
    }
}
