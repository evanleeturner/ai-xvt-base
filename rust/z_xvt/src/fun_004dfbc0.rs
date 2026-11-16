// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a330: core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004be160(param_1: *mut i8, param_2: *mut core::ffi::c_void) -> *mut i32;
    fn _fgets(buffer: *mut i8, size: i32, stream: *mut i32) -> *mut i8;
    fn FUN_004be1f0(stream: *mut i32);
    fn _fscanf(stream: *mut i32, format: *const i8, ...) -> i32;
    fn FUN_004d0430(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dfbc0(param_1: *mut i8) -> i32 {
    let mut local_204: i32 = 0;
    let mut local_200: [i8; 256] = [0; 256];
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the callees and the global are the game's own; the buffers are
    // local and large enough for the original's calls.
    unsafe {
        let file = FUN_004be160(param_1, &raw mut DAT_0052a330);
        if file.is_null() {
            return 0;
        }
        let p_var1 = _fgets(local_200.as_mut_ptr(), 0xff, file);
        if p_var1.is_null() {
            FUN_004be1f0(file);
            return 1;
        }
        loop {
            let i_var2 = _fscanf(
                file,
                b"%s %s %d\n\0".as_ptr().cast::<i8>(),
                local_200.as_mut_ptr(),
                local_100.as_mut_ptr(),
                &raw mut local_204,
            );
            if i_var2 == -1 {
                FUN_004be1f0(file);
                return 1;
            }
            if i_var2 != 3 {
                break;
            }
            FUN_004d0430(local_100.as_mut_ptr());
        }
        FUN_004be1f0(file);
        0
    }
}
