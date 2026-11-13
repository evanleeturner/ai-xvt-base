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
pub unsafe extern "C" fn FUN_004ae6e0(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's own structures and are read where the original reads.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<i8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag & 1) == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16;
        let arg = ((param_1.wrapping_mul(5) as u32) >> 16) | (s as u32);
        let pi = FUN_004ac720(arg as i32);
        if pi.read() != (pi as usize as i32) {
            FUN_00471ff0(pi);
        }
        let arr = pi
            .cast::<u8>()
            .byte_add(10)
            .cast::<*mut i32>()
            .read_unaligned();
        let first = arr.read_unaligned();
        let v = (first as u32 as usize as *const u8)
            .byte_add(4)
            .cast::<i32>()
            .read_unaligned();
        if v == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let count = pi.cast::<u8>().byte_add(6).cast::<i32>().read_unaligned();
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
