// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00452830(param_1: *mut i8, param_2: *mut i8) {
    let mut p1 = param_1;
    let mut p2 = param_2;
    // SAFETY: both pointers are the caller's NUL-terminated strings; the
    // destination has room for the appended bytes, as in the original strcat.
    unsafe {
        let mut c = p2.read();
        while c != 0 {
            p2 = p2.add(1);
            c = p2.read();
        }
        c = p1.read();
        while c != 0 {
            c = p1.read();
            p1 = p1.add(1);
            p2.write(c);
            p2 = p2.add(1);
            c = p1.read();
        }
        p2.write(0);
    }
}
