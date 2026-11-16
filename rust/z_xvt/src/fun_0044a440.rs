// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_0048f510(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a440(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all globals, callees and pointers are the game's own memory;
    // reads and writes follow the original's order and offsets.
    unsafe {
        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read_unaligned();
        let arg = ((((param_1 as u32) >> 0x10) << 16) as i32) | ((idx as i32) & 0xFFFF);
        let pi_var2 = FUN_0048f4f0(arg) as usize as *mut i32;
        if pi_var2.read_unaligned() != pi_var2 as usize as i32 {
            FUN_00485fa0(pi_var2);
        }
        let field_6: i32 = pi_var2.byte_add(6).cast::<i32>().read_unaligned();
        let field_10: i32 = pi_var2.byte_add(10).cast::<i32>().read_unaligned();
        if (field_10.wrapping_add(4) as usize as *const i32).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }
        let i_var1 = (field_10.wrapping_add(param_2.wrapping_mul(4)) as usize as *const i32)
            .read_unaligned();
        if i_var1 != 0 && (i_var1.wrapping_add(4) as usize as *const i32).read_unaligned() != 0x19 {
            let mut i_var5: i32 = 0;
            let mut i_var4: i32 = 0;
            if 0 < (i_var1.wrapping_add(8) as usize as *const i32).read_unaligned() {
                loop {
                    let base = (i_var1.wrapping_add(0xc) as usize as *const i32).read_unaligned();
                    let i_var3 =
                        (base.wrapping_add(i_var5) as usize as *const i32).read_unaligned();
                    if i_var3 != 0 {
                        let r = FUN_00449670(i_var3, pi_var2);
                        if r != 0 {
                            break;
                        }
                    }
                    i_var5 = i_var5.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4 < (i_var1.wrapping_add(8) as usize as *const i32).read_unaligned())
                    {
                        break;
                    }
                }
            }
        }
        // __ftol(): truncate st(0) toward zero into a 64-bit integer.
        let mut cw_old: u16 = 0;
        let mut cw_new: u16 = 0;
        let mut out: i64 = 0;
        core::arch::asm!(
            "fnstcw word ptr [{old}]",
            "mov ax, word ptr [{old}]",
            "or ax, 0x0c00",
            "mov word ptr [{new}], ax",
            "fldcw word ptr [{new}]",
            "fistp qword ptr [{out}]",
            "fldcw word ptr [{old}]",
            old = in(reg) &raw mut cw_old,
            new = in(reg) &raw mut cw_new,
            out = in(reg) &raw mut out,
            out("eax") _,
        );
        let l_var6 = out;
        FUN_0048f510(l_var6 as i32);
        l_var6 as i32
    }
}
