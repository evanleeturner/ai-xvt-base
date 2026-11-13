// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [i8; 0];
    static mut DAT_009a7ed0: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004ae1a0(param_1: i32, param_2: i32) -> i32;
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae820(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals, callees and the object pointers are the game's own
    // memory; reads and calls happen in the original's order. __ftol consumes
    // the x87 top-of-stack the previous callee left, as in the original.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<i8>()
            .byte_offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flags & 1) == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .byte_offset((param_1 as isize).wrapping_mul(2))
            .read_unaligned() as u16;
        let arg = ((param_1.wrapping_mul(5) as u32) >> 16) | (w as u32);
        let p1 = FUN_004ac720(arg as i32);
        if p1.read_unaligned() != (p1 as usize as i32) {
            FUN_00471ff0(p1);
        }
        let base = p1.byte_add(0xA).cast::<*mut i32>().read_unaligned();
        let inner = base.read_unaligned() as usize as *mut i32;
        if inner.byte_add(4).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let val_6 = p1.byte_add(6).read_unaligned();
        if val_6 <= param_2 {
            param_2 = val_6.wrapping_sub(1);
        }
        let mut u_var3: i32 = 0;
        let base2 = p1.byte_add(0xA).cast::<*mut i32>().read_unaligned();
        let elem = base2
            .byte_offset((param_2 as isize).wrapping_mul(4))
            .read_unaligned();
        let i_var2 = FUN_004ae1a0(elem, p1 as usize as i32);
        if i_var2 != 0 {
            let l_var4 = __ftol();
            u_var3 = l_var4 as i32;
        }
        FUN_004ac740();
        u_var3
    }
}
