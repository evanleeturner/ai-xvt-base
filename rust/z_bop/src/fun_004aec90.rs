// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [i8; 0];
    static mut DAT_009a7ed0: [i16; 0];
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
pub unsafe extern "C" fn FUN_004aec90(param_1: i32, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals, callees and the object pointers are the game's own
    // memory; reads and calls happen in the original's order.
    unsafe {
        let flag_idx = (param_1 as u32).wrapping_mul(0x1a) as usize;
        let flag = (&raw mut DAT_00521f19).cast::<i8>().add(flag_idx).read();
        if (flag as u8) & 1 == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16;
        let arg = ((((param_1 as u32).wrapping_mul(5)) >> 0x10) << 16) | u32::from(s);
        let pi_var1 = FUN_004ac720(arg as i32);
        if pi_var1.read() != (pi_var1 as usize as i32) {
            FUN_00471ff0(pi_var1);
        }
        let local_ptr_10: *mut i32 = pi_var1
            .cast::<u8>()
            .byte_add(10)
            .cast::<*mut i32>()
            .read_unaligned();
        if local_ptr_10.add(1).read() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let local_val_6: i32 = pi_var1
            .cast::<u8>()
            .byte_add(6)
            .cast::<i32>()
            .read_unaligned();
        if local_val_6 <= param_2 {
            param_2 = local_val_6.wrapping_sub(1);
        }
        let i_var2 = FUN_004ae1a0(local_ptr_10.offset(param_2 as isize).read(), pi_var1);
        let u_var3: i32;
        if i_var2 == 0 {
            u_var3 = 0;
        } else if ((i_var2 as u32).wrapping_add(0x38) as usize as *const i32).read_unaligned() == 0
        {
            u_var3 = __ftol() as i32;
        } else {
            u_var3 = __ftol() as i32;
        }
        FUN_004ac740();
        u_var3
    }
}
