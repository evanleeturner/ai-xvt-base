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
    fn FUN_004ae1a0(param_1: i32, param_2: *mut i32) -> i32;
    fn __ftol() -> i64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae780(param_1: i32, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals, callees and the object pointed to by the returned
    // pointer are the game's own memory, read in the original's order.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flag & 1 == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16;
        let hi = (((param_1.wrapping_mul(5)) >> 16) as i16 as i32).wrapping_shl(16);
        let pi_var1 = FUN_004ac720(hi | (w as i32));
        if pi_var1.read() != pi_var1 as i32 {
            FUN_00471ff0(pi_var1);
        }
        let field_6 = pi_var1.byte_add(6).cast::<i32>().read_unaligned();
        let field_10 = pi_var1.byte_add(10).cast::<*mut i32>().read_unaligned();
        let inner = ((field_10.read()).wrapping_add(4) as usize) as *const i32;
        if inner.read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }
        let mut u_var3: i32 = 0;
        let i_var2 = FUN_004ae1a0(field_10.offset(param_2 as isize).read(), pi_var1);
        if i_var2 != 0 {
            let l_var4 = __ftol();
            u_var3 = l_var4 as i32;
        }
        FUN_004ac740();
        u_var3
    }
}
