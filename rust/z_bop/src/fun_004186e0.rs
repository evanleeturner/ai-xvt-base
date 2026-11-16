// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8db0: [i32; 0];
    static mut DAT_009ed670: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004186e0(param_1: i32, param_2: *mut u32) {
    // SAFETY: the globals are the game's own arrays, indexed as the original
    // does; param_2 and the derived pointers address the game's records, read
    // and written where the original does.
    unsafe {
        let cnt = (&raw mut DAT_009a8db0)
            .cast::<i32>()
            .wrapping_offset(param_1 as isize);
        if cnt.read() != 0 {
            let mut p3 = (&raw mut DAT_009ed670)
                .cast::<u32>()
                .wrapping_offset(param_1.wrapping_mul(0x7e9) as isize);
            if (p3 as usize) <= (param_2 as usize) {
                let i_var2 = cnt.read().wrapping_sub(1);
                let mut i_var1: i32 = 0;
                cnt.write(i_var2);
                if i_var2 > 0 {
                    loop {
                        if (param_2 as usize) <= (p3 as usize) {
                            let t = p3.byte_add(0x12).read_unaligned();
                            p3.write_unaligned(t);
                            let t = p3.byte_add(0x16).read_unaligned();
                            p3.add(1).write_unaligned(t);
                            let t = p3.byte_add(0x1a).read_unaligned();
                            p3.add(2).write_unaligned(t);
                            let t = p3.byte_add(0x1e).read_unaligned();
                            p3.add(3).write_unaligned(t);
                            let t2 = p3.byte_add(0x22).cast::<u16>().read_unaligned();
                            p3.add(2).cast::<u16>().write_unaligned(t2);
                        }
                        i_var1 = i_var1.wrapping_add(1);
                        p3 = p3.byte_add(0x12);
                        if !(i_var1 < cnt.read()) {
                            break;
                        }
                    }
                }
            }
        }
    }
}
