// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aabd11: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32 {
    if param_1.is_null() {
        return 0;
    }
    if -1 < param_2 && param_2 < 0x100 {
        // SAFETY: the global table is the game's own memory; the index is in 0..0x100.
        let i_var2: i32 = unsafe {
            (&raw mut DAT_00aabd11)
                .wrapping_add(param_2 as usize)
                .read_unaligned()
        };
        if i_var2 != 0 {
            let mut i_var3: i32 = 0;
            let mut i_var4: i32 = 0;
            // SAFETY: param_1 points at the caller's NUL-terminated string; the
            // table at i_var2 is the game's own memory, read where the original does.
            unsafe {
                let mut c_var1 = param_1.read();
                while c_var1 != 0 {
                    let b = param_1.byte_add(i_var4 as usize).cast::<u8>().read();
                    if 6 < b {
                        let a = (i_var2.wrapping_add(0x504).wrapping_add(b as i32) as usize)
                            as *const u8;
                        let c = (i_var2.wrapping_add(0x609) as usize) as *const u8;
                        i_var3 = i_var3
                            .wrapping_add(a.read() as i32)
                            .wrapping_add(c.read() as i32);
                    }
                    i_var4 = i_var4.wrapping_add(1);
                    c_var1 = param_1.byte_add(i_var4 as usize).read();
                }
                let c = (i_var2.wrapping_add(0x609) as usize) as *const u8;
                return i_var3.wrapping_sub(c.read() as i32);
            }
        }
        return 0;
    }
    0
}
