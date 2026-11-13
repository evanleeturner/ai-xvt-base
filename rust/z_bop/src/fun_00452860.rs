// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00452860(param_1: i8, param_2: *mut i8) {
    let mut p = param_2;
    // SAFETY: param_2 points at a NUL-terminated string in the caller's
    // buffer with room for two more bytes; read and written as the original does.
    unsafe {
        let mut c = *p;
        while c != 0 {
            p = p.add(1);
            c = *p;
        }
        *p = param_1;
        *p.add(1) = 0;
    }
}
