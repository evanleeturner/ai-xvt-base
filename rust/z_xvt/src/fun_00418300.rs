// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418300(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut u32,
) -> i32 {
    // SAFETY: the caller passes a pointer to a RIFF buffer and optional
    // out-pointers; all are read and written where the original does.
    unsafe {
        if !param_2.is_null() {
            param_2.write_unaligned(0);
        }
        if !param_3.is_null() {
            param_3.write_unaligned(0);
        }
        if !param_4.is_null() {
            param_4.write_unaligned(0);
        }
        let i_var1 = param_1.byte_add(4).read_unaligned();
        let mut pi_var4: u32 = (param_1 as u32).wrapping_add(12);
        if param_1.read_unaligned() == 0x46464952
            && param_1.byte_add(8).read_unaligned() == 0x45564157
        {
            let end: u32 = (param_1 as u32).wrapping_add(i_var1 as u32).wrapping_add(8);
            while pi_var4 < end {
                let p4 = pi_var4 as *const i32;
                let u_var2 = p4.byte_add(4).read_unaligned() as u32;
                let pi_var3: u32 = pi_var4.wrapping_add(8);
                let tag = p4.read_unaligned();
                if tag == 0x20746d66 {
                    if !param_2.is_null() && param_2.read_unaligned() == 0 {
                        if u_var2 < 0xe {
                            return 0;
                        }
                        param_2.write_unaligned(pi_var3 as i32);
                        if (param_3.is_null() || param_3.read_unaligned() != 0)
                            && (param_4.is_null() || param_4.read_unaligned() != 0)
                        {
                            return 1;
                        }
                    }
                } else if tag == 0x61746164
                    && ((!param_3.is_null() && param_3.read_unaligned() == 0)
                        || (!param_4.is_null() && param_4.read_unaligned() == 0))
                {
                    if !param_3.is_null() {
                        param_3.write_unaligned(pi_var3 as i32);
                    }
                    if !param_4.is_null() {
                        param_4.write_unaligned(u_var2);
                    }
                    if param_2.is_null() || param_2.read_unaligned() != 0 {
                        return 1;
                    }
                }
                pi_var4 = pi_var3.wrapping_add(u_var2.wrapping_add(1) & 0xffff_fffe);
            }
        }
        0
    }
}
