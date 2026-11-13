// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e522: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_00703ad4: i16;
    static mut DAT_0074110c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004063f0(param_1: *mut i32, param_2: i32, param_3: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own; param_1 points at the caller's
    // records of three ints, and param_2/param_3 are addresses of the
    // caller's buffers, accessed where the original does.
    unsafe {
        loop {
            let mut i_var4: i32 = (DAT_0071e522 as i32).wrapping_add(param_1.read_unaligned());
            let mut i_var2: i32 = param_1.byte_add(8).read_unaligned().wrapping_add(i_var4);
            let i_var1: i32 = DAT_0071e518 as i32;
            let i_var3: i32 = DAT_00703ad4 as i32;
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
                        let i_var4b = pi_var5.read_unaligned();
                        pi_var5 = pi_var5.byte_add(4);
                        let dst = param_2.wrapping_add(i_var4b) as *mut u8;
                        dst.write(i_var1 as u8);
                        param_2
                            .wrapping_add(1)
                            .wrapping_add(i_var4b)
                            .cast_unsigned();
                        (param_2.wrapping_add(1).wrapping_add(i_var4b) as *mut u8).write(0x80);
                        i_var2 = i_var2.wrapping_sub(1);
                        if i_var2 == 0 {
                            break;
                        }
                    }
                }
            }
            DAT_0074110c = DAT_0074110c.wrapping_add(-1);
            param_1 = param_1.byte_add(12);
            if DAT_0074110c == 0 {
                break;
            }
        }
    }
}
