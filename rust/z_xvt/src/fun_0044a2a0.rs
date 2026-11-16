// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: i32) -> i32;
    fn FUN_0048f510(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a2a0(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all pointers and globals belong to the game; memory is read
    // at the same offsets and in the same order as the original.
    unsafe {
        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((param_1 as u32) >> 0x10) | (idx as u16 as u32);
        let pi2 = FUN_0048f4f0(arg as i32) as *mut i32;
        let base = pi2 as i32;
        if pi2.read_unaligned() != base {
            FUN_00485fa0(pi2);
        }
        let arr = ((base.wrapping_add(10)) as *const i32).read_unaligned();
        let first = (arr as *const i32).read_unaligned();
        if ((first.wrapping_add(4)) as *const i32).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let cnt = ((base.wrapping_add(6)) as *const i32).read_unaligned();
        if cnt <= param_2 {
            param_2 = cnt.wrapping_sub(1);
        }
        let i_var1 = ((arr.wrapping_add(param_2.wrapping_mul(4))) as *const i32).read_unaligned();
        if i_var1 != 0 && ((i_var1.wrapping_add(4)) as *const i32).read_unaligned() != 0x19 {
            let mut i_var5: i32 = 0;
            let mut i_var4: i32 = 0;
            if 0 < ((i_var1.wrapping_add(8)) as *const i32).read_unaligned() {
                loop {
                    let tbl = ((i_var1.wrapping_add(0xc)) as *const i32).read_unaligned();
                    let i_var3 = ((tbl.wrapping_add(i_var5)) as *const i32).read_unaligned();
                    if i_var3 != 0 {
                        let r = FUN_00449670(i_var3, base);
                        if r != 0 {
                            break;
                        }
                    }
                    i_var5 = i_var5.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4 < ((i_var1.wrapping_add(8)) as *const i32).read_unaligned()) {
                        break;
                    }
                }
            }
        }
        // __ftol(): truncating conversion of the x87 top of stack.
        let mut old: u16 = 0;
        let mut new: u16 = 0;
        let mut r: i64 = 0;
        core::arch::asm!(
            "fnstcw word ptr [{old}]",
            "movzx {tmp:e}, word ptr [{old}]",
            "or {tmp:e}, 0xC00",
            "mov word ptr [{new}], {tmp:x}",
            "fldcw word ptr [{new}]",
            "fistp qword ptr [{r}]",
            "fldcw word ptr [{old}]",
            old = in(reg) &raw mut old,
            new = in(reg) &raw mut new,
            r = in(reg) &raw mut r,
            tmp = out(reg) _,
            out("st(0)") _,
        );
        FUN_0048f510(0);
        r as i32
    }
}
