// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut in_EAX: i32;
    static mut DAT_006da850: [i16; 1];
    static mut DAT_00525cc8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> *mut i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_0044a830(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
    ) -> *mut core::ffi::c_void;
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044ab90(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's own structures and are read where the original reads.
    unsafe {
        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((in_EAX as u32) & 0xFFFF_0000) | (idx as u16 as u32);
        let pi_var1 = FUN_0048f4f0(arg as i32);
        if (*pi_var1) as u32 != pi_var1 as u32 {
            FUN_00485fa0(pi_var1);
        }
        let ptr_at_a: i32 = pi_var1
            .cast::<u8>()
            .byte_add(10)
            .cast::<i32>()
            .read_unaligned();
        if ((ptr_at_a as u32 as usize + 4) as *const i32).read_unaligned() != 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let val_at_6: i32 = pi_var1
            .cast::<u8>()
            .byte_add(6)
            .cast::<i32>()
            .read_unaligned();
        if param_2 >= val_at_6 {
            param_2 = val_at_6.wrapping_sub(1);
        }
        let mut u_var3: i32 = 0;
        DAT_00525cc8 = 0;
        let elem = (ptr_at_a as u32 as usize as *const *mut core::ffi::c_void)
            .offset(param_2 as isize)
            .read();
        let p_var2 = FUN_0044a830(elem, pi_var1 as i32, param_3);
        if !p_var2.is_null() {
            // __ftol: truncate st0 toward zero into a 64-bit integer.
            let mut old: u16 = 0;
            let mut newcw: u16 = 0;
            let mut out: i64 = 0;
            core::arch::asm!(
                "fnstcw word ptr [{o}]",
                "movzx {t:e}, word ptr [{o}]",
                "or {t:e}, 0xC00",
                "mov word ptr [{n}], {t:x}",
                "fldcw word ptr [{n}]",
                "fistp qword ptr [{r}]",
                "fldcw word ptr [{o}]",
                o = in(reg) &raw mut old,
                n = in(reg) &raw mut newcw,
                r = in(reg) &raw mut out,
                t = out(reg) _,
                options(nostack)
            );
            u_var3 = out as i32;
        }
        FUN_0048f510();
        u_var3
    }
}
