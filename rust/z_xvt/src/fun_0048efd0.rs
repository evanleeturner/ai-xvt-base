// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4870: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048efd0(param_1: *mut i8) {
    // SAFETY: the global is the game's own buffer; param_1 points at the
    // caller's NUL-terminated string, copied byte by byte as the original does.
    unsafe {
        let mut p_var2: *mut i8 = &raw mut DAT_006d4870;
        let mut param_1 = param_1;
        if !param_1.is_null() {
            let mut c_var1 = param_1.read();
            while c_var1 != 0 {
                c_var1 = param_1.read();
                param_1 = param_1.add(1);
                p_var2.write(c_var1);
                p_var2 = p_var2.add(1);
                c_var1 = param_1.read();
            }
        }
        p_var2.write(0);
    }
}
