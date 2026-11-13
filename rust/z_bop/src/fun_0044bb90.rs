// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044bb90(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut u32,
) -> i32 {
    // SAFETY: the caller supplies a RIFF buffer and optional out pointers;
    // all reads and writes follow the original's accesses.
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
        let base = param_1 as usize as u32;
        let mut cur: u32 = base.wrapping_add(12);
        let limit: u32 = base.wrapping_add(i_var1 as u32).wrapping_add(8);
        if param_1.read_unaligned() == 0x46464952
            && param_1.byte_add(8).read_unaligned() == 0x45564157
        {
            while cur < limit {
                let p4 = cur as usize as *const i32;
                let u_var2: u32 = p4.byte_add(4).cast::<u32>().read_unaligned();
                let pi3: u32 = cur.wrapping_add(8);
                let id = p4.read_unaligned();
                if id == 0x20746d66 {
                    if !param_2.is_null() && param_2.read_unaligned() == 0 {
                        if u_var2 < 0xe {
                            return 0;
                        }
                        param_2.write_unaligned(pi3 as i32);
                        if (param_3.is_null() || param_3.read_unaligned() != 0)
                            && (param_4.is_null() || param_4.read_unaligned() != 0)
                        {
                            return 1;
                        }
                    }
                } else if id == 0x61746164
                    && ((!param_3.is_null() && param_3.read_unaligned() == 0)
                        || (!param_4.is_null() && param_4.read_unaligned() == 0))
                {
                    if !param_3.is_null() {
                        param_3.write_unaligned(pi3 as i32);
                    }
                    if !param_4.is_null() {
                        param_4.write_unaligned(u_var2);
                    }
                    if param_2.is_null() || param_2.read_unaligned() != 0 {
                        return 1;
                    }
                }
                cur = pi3.wrapping_add(u_var2.wrapping_add(1) & 0xfffffffe);
            }
        }
        0
    }
}
