// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536ef8: i32;
    static mut DAT_00536f05: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004286f0(param_1: i32) {
    // SAFETY: param_1 is the address of the game's node structure and the
    // globals are the game's own; reads and writes follow the original's
    // offsets and order.
    unsafe {
        let flags = *((param_1.wrapping_add(2)) as *const u8);
        if flags != 0 {
            let mut u_var1: u32 = 0;
            loop {
                if ((1u32.wrapping_shl(u_var1 & 0x1f)) & (flags as u32)) != 0 {
                    let child = ((param_1.wrapping_add(0x32) as u32)
                        .wrapping_add(u_var1.wrapping_mul(4)))
                        as *const i32;
                    FUN_004286f0(child.read_unaligned());
                }
                u_var1 = u_var1.wrapping_add(1);
                if u_var1 >= 8 {
                    break;
                }
            }
        }
        if ((param_1.wrapping_add(10)) as *const i32).read_unaligned() != 0 {
            let l_var2 = __ftol();
            let a = DAT_00536ef8.wrapping_mul(9).wrapping_add(DAT_00536f05);
            (a as *mut u8).write(l_var2 as u8);
            let l_var2 = __ftol();
            let a = DAT_00536ef8
                .wrapping_mul(9)
                .wrapping_add(1)
                .wrapping_add(DAT_00536f05);
            (a as *mut u8).write(l_var2 as u8);
            let l_var2 = __ftol();
            let a = DAT_00536ef8
                .wrapping_mul(9)
                .wrapping_add(2)
                .wrapping_add(DAT_00536f05);
            (a as *mut u8).write(l_var2 as u8);
            ((param_1.wrapping_add(6)) as *mut i32).write_unaligned(DAT_00536ef8);
            DAT_00536ef8 = DAT_00536ef8.wrapping_add(1);
        }
    }
}
