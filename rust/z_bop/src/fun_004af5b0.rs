// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 1];
    static mut DAT_009a7ed0: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004ae1a0(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af5b0(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the pointers read
    // are the game's records, accessed where the original accesses them.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flags & 1) == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let hi = ((param_1.wrapping_mul(5) as u32) >> 16) << 16;
        let arg = hi | ((s as i32) as u32);
        let pi_var2 = FUN_004ac720(arg as i32);
        if pi_var2.read_unaligned() != (pi_var2 as usize as i32) {
            FUN_00471ff0(pi_var2);
        }
        let mut i_var5: i32 = 0;
        let mut i_var4: i32 = 0;
        let field_6: i32 = pi_var2
            .cast::<u8>()
            .byte_add(6)
            .cast::<i32>()
            .read_unaligned();
        if 0 < field_6 {
            loop {
                let field_10: i32 = pi_var2
                    .cast::<u8>()
                    .byte_add(10)
                    .cast::<i32>()
                    .read_unaligned();
                let i_var1: i32 =
                    (field_10.wrapping_add(i_var5) as usize as *const i32).read_unaligned();
                if i_var1 != 0
                    && ((i_var1 as usize as *const i32).byte_add(4).read_unaligned()) != 0x14
                {
                    let pi_var3 =
                        FUN_004ae1a0(i_var1, pi_var2 as usize as i32) as usize as *mut i32;
                    if !pi_var3.is_null() && pi_var3.read_unaligned() == 3 {
                        FUN_004ac740();
                        return 1;
                    }
                }
                i_var5 = i_var5.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < field_6) {
                    break;
                }
            }
        }
        FUN_004ac740();
        0
    }
}
