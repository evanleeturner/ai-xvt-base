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
    fn FUN_004af150(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn __ftol() -> i64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af2e0(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's own structures and are read where the original reads.
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
        let arg = ((param_1.wrapping_mul(5) as u32) & 0xFFFF0000) | (w as u32);
        let pvar1 = FUN_004ac720(arg as i32) as u32 as usize as *mut i32;
        if (pvar1.read() as u32 as usize as *mut i32) != pvar1 {
            FUN_00471ff0(pvar1);
        }

        let val_at_6: i32 = pvar1
            .cast::<u8>()
            .byte_add(6)
            .cast::<i32>()
            .read_unaligned();
        let ptr_at_10: *mut i32 = pvar1
            .cast::<u8>()
            .byte_add(10)
            .cast::<i32>()
            .read_unaligned() as u32 as usize as *mut i32;

        if ptr_at_10.offset(1).read() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if val_at_6 <= param_2 {
            param_2 = val_at_6.wrapping_sub(1);
        }
        let mut u_var3: i32 = 0;
        let first = ptr_at_10.offset(param_2 as isize).read() as u32 as usize as *mut i32;
        let i_var2 = FUN_004af150(first, pvar1 as usize as i32, param_3);
        if i_var2 != 0 {
            let l_var4 = __ftol();
            u_var3 = l_var4 as i32;
        }
        FUN_004ac740();
        u_var3
    }
}
