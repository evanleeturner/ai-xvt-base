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
    // Declared as returning f64: the original leaves its result in st0, which __ftol then converts.
    fn FUN_004adcc0(param_1: i32) -> f64;
    fn FUN_004ac740(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae460(param_1: i32, mut param_2: i32) -> i32 {
    // SAFETY: all globals, callees and pointers belong to the game; reads and
    // writes occur in the original's order at the original's offsets.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<i8>()
            .byte_offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flags as i32) & 1 == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .byte_offset((param_1 as isize).wrapping_mul(2))
            .read_unaligned();
        let arg = (param_1.wrapping_mul(5) & (0xFFFF0000u32 as i32)) | ((s as i32) & 0xFFFF);
        let pvar1 = FUN_004ac720(arg);
        if (pvar1.read_unaligned() as u32) != (pvar1 as usize as u32) {
            FUN_00471ff0(pvar1);
        }
        let field_6 = pvar1.byte_add(6).cast::<i32>().read_unaligned();
        let field_10 = pvar1.byte_add(10).cast::<i32>().read_unaligned();
        let base = field_10 as u32 as usize as *const i32;
        if base.byte_add(4).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }
        let elem = base
            .byte_offset((param_2 as isize).wrapping_mul(4))
            .read_unaligned();
        let v = FUN_004adcc0(elem);
        let l_var2 = (v as i64) as i32;
        FUN_004ac740(l_var2);
        l_var2
    }
}
