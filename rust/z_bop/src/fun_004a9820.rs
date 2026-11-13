// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1ed0: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9820(param_1: *mut i8) {
    // SAFETY: the global is the game's own buffer; param_1 is the caller's
    // NUL-terminated string, read and appended where the original does.
    unsafe {
        let mut p_var3: *mut i8 = &raw mut DAT_009a1ed0;
        let mut c_var2: i8 = DAT_009a1ed0;
        while c_var2 != 0 {
            p_var3 = p_var3.add(1);
            c_var2 = p_var3.read();
        }
        if !param_1.is_null() {
            let mut src = param_1;
            c_var2 = src.read();
            while c_var2 != 0 {
                p_var3.write(c_var2);
                p_var3 = p_var3.add(1);
                src = src.add(1);
                c_var2 = src.read();
            }
        }
        p_var3.write(0);
    }
}
