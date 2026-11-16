// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 1];
    static mut DAT_009a7ed0: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut core::ffi::c_void);
    fn FUN_004adcc0(param_1: i32);
    fn FUN_004ac740(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae3c0(param_1: i32, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: the globals, callees and the object pointers they return are the
    // game's own memory, read in the same order as the original.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag & 1) != 0 {
            return 0;
        }

        let w = (&raw mut DAT_009a7ed0)
            .cast::<u16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((param_1.wrapping_mul(5) as u32) & 0xFFFF_0000) | (w as u32);
        let pv1 = FUN_004ac720(arg as i32).cast::<i32>();

        if pv1.read_unaligned() != (pv1 as usize as u32 as i32) {
            FUN_00471ff0(pv1.cast());
        }

        let field_a = pv1
            .cast::<u8>()
            .byte_add(0xa)
            .cast::<i32>()
            .read_unaligned();
        let field_base = (field_a as u32 as usize as *const i32).read_unaligned();
        let field_val = ((field_base as u32 as usize) as *const u8)
            .byte_add(4)
            .cast::<i32>()
            .read_unaligned();
        if field_val != 0x14 {
            param_2 = param_2.wrapping_add(1);
        }

        let field_6 = pv1.cast::<u8>().byte_add(6).cast::<i32>().read_unaligned();
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }

        let field_a = pv1
            .cast::<u8>()
            .byte_add(0xa)
            .cast::<i32>()
            .read_unaligned();
        let field_base = (field_a as u32 as usize as *const i32).read_unaligned();
        let field_val = ((field_base as u32 as usize) as *const u8)
            .byte_add(param_2.wrapping_mul(4) as u32 as usize)
            .cast::<i32>()
            .read_unaligned();
        FUN_004adcc0(field_val);

        let l_var2 = (param_2 as f32) as i64;
        FUN_004ac740(l_var2 as i32);
        l_var2 as i32
    }
}
