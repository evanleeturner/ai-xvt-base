// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a330: i8;
    static mut DAT_00533080: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004be160(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004be1f0(param_1: *mut core::ffi::c_void) -> i32;
    fn _fgets(buffer: *mut i8, count: i32, stream: *mut core::ffi::c_void) -> *mut i8;
    fn _fscanf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
    fn FUN_004d0360(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dfaf0(param_1: *mut i8) -> i32 {
    let mut local_204: i32 = 0;
    let mut local_200: [i8; 256] = [0; 256];
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the callees and globals are the game's own; the buffers are
    // local and sized as the original's, and the file handle is closed on
    // every exit path.
    unsafe {
        let file = FUN_004be160(param_1, &raw mut DAT_0052a330);
        if file.is_null() {
            return 0;
        }
        let p = _fgets(local_200.as_mut_ptr(), 0xff, file);
        if p.is_null() {
            FUN_004be1f0(file);
            return 1;
        }
        loop {
            let r = _fscanf(
                file,
                (&raw const DAT_00533080).cast::<i8>(),
                local_200.as_mut_ptr(),
                local_100.as_mut_ptr(),
                &raw mut local_204,
            );
            if r == -1 {
                FUN_004be1f0(file);
                return 1;
            }
            if r != 3 {
                break;
            }
            FUN_004d0360(local_200.as_mut_ptr(), local_100.as_mut_ptr(), 0, local_204);
        }
        FUN_004be1f0(file);
        0
    }
}
