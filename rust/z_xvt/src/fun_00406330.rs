// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e522: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_00703ad4: i16;
    static mut _DAT_0074110c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406330(param_1: *mut i32, param_2: i32, param_3: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own; param_1, param_2 and param_3
    // point at the caller's span records, destination buffer and offset
    // table, read and written exactly where the original does.
    unsafe {
        loop {
            let mut i_var4 = (DAT_0071e522 as i32).wrapping_add(param_1.read_unaligned());
            let mut i_var2 = param_1.byte_add(8).read_unaligned().wrapping_add(i_var4);
            let i_var1 = DAT_0071e518 as i32;
            let i_var3 = DAT_00703ad4 as i32;
            if i_var4 < i_var1 {
                i_var4 = i_var1;
            }
            if i_var4 <= i_var3 && i_var1 <= i_var2 {
                if i_var3 < i_var2 {
                    i_var2 = i_var3;
                }
                i_var2 = i_var2.wrapping_sub(i_var4);
                if i_var2 != 0 {
                    let i_var1 = param_1.byte_add(4).read_unaligned();
                    let mut pi_var5 = i_var4.wrapping_mul(4).wrapping_add(param_3) as *const i32;
                    loop {
                        let v = pi_var5.read_unaligned();
                        pi_var5 = pi_var5.byte_add(4);
                        (param_2.wrapping_add(v) as *mut u8).write(i_var1 as u8);
                        i_var2 = i_var2.wrapping_sub(1);
                        if i_var2 == 0 {
                            break;
                        }
                    }
                }
            }
            _DAT_0074110c = _DAT_0074110c.wrapping_sub(1);
            param_1 = param_1.byte_add(12);
            if _DAT_0074110c == 0 {
                break;
            }
        }
    }
}
