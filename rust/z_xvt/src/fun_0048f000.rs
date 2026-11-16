// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4870: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048f000(param_1: *mut i8) {
    // SAFETY: the global buffer is the game's own memory; param_1, when
    // non-null, points at a NUL-terminated string. Reads and writes happen
    // in the same order as the original.
    unsafe {
        let mut p_var3: *mut i8 = (&raw mut DAT_006d4870).cast::<i8>();
        let mut c_var2: i8 = p_var3.read();
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
