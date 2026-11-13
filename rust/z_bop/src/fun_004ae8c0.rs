// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: i8;
    static mut DAT_009a7ed0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> i32;
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
pub unsafe extern "C" fn FUN_004ae8c0(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all globals and callees are the game's own memory and code;
    // piVar1 and the arrays it points to are the game's structures, read
    // at the same offsets as the original.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .byte_offset((param_1 as isize).wrapping_mul(0x1a))
            .read();
        if (flag as u8) & 1 == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0).offset(param_1 as isize).read() as u16 as u32;
        let hi = ((param_1 as u32).wrapping_mul(5) >> 16) << 16;
        let arg = (hi | s) as i32;
        let pi = FUN_004ac720(arg) as usize as *mut i32;
        if pi.read_unaligned() != pi as usize as i32 {
            FUN_00471ff0(pi);
        }
        let arr = pi.byte_add(10).cast::<i32>().read_unaligned() as usize as *mut i32;
        let first = arr.read_unaligned();
        let t = ((first as usize).wrapping_add(4) as *const i32).read_unaligned();
        if t == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let count = pi.byte_add(6).cast::<i32>().read_unaligned();
        if count <= param_2 {
            param_2 = count.wrapping_sub(1);
        }
        let mut u_var3: i32 = 0;
        let elem = arr.offset(param_2 as isize).read_unaligned();
        let i_var2 = FUN_004ae1a0(elem, pi);
        if i_var2 != 0 {
            let l = __ftol();
            u_var3 = l as i32;
        }
        FUN_004ac740();
        u_var3
    }
}
