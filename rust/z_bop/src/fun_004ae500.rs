// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 0];
    static mut DAT_009a7ed0: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> i32;
    fn FUN_00471ff0(param_1: *mut i32);
    // Returns its value in st0, which the original's __ftol converts.
    fn FUN_004adcc0(param_1: i32) -> f64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae500(param_1: i32, mut param_2: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's tables and are read where the original reads them.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flags & 1 == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((param_1.wrapping_mul(5) as u32) & 0xFFFF0000) | (s as u16 as u32);
        let pi_var1 = FUN_004ac720(arg as i32) as *mut i32;
        if pi_var1.read_unaligned() != pi_var1 as i32 {
            FUN_00471ff0(pi_var1);
        }
        let ptr_at_10 = pi_var1.byte_add(10).read_unaligned() as *mut i32;
        let val_at_6 = pi_var1.byte_add(6).read_unaligned();
        if ptr_at_10.byte_add(4).read_unaligned() != 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if val_at_6 <= param_2 {
            param_2 = val_at_6.wrapping_sub(1);
        }
        let item = ptr_at_10.offset(param_2 as isize).read_unaligned();
        let f = FUN_004adcc0(item);
        let l_var2 = f as i64;
        FUN_004ac740();
        l_var2 as i32
    }
}
