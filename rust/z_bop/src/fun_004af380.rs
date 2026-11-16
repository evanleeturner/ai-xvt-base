// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: u8;
    static mut DAT_009a7ed0: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004af150(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af380(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: globals and callees are the game's own; pointers returned by the
    // game are read at the offsets the original uses; the asm truncates st0
    // left by the callee, as __ftol does.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flag & 1 != 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0).offset(param_1 as isize).read() as u32;
        let arg = ((((param_1.wrapping_mul(5)) as u32) >> 16) << 16) | w;
        let pvar1 = FUN_004ac720(arg as i32) as *mut i32;
        if pvar1.read() != pvar1 as i32 {
            FUN_00471ff0(pvar1);
        }
        let list = pvar1.byte_add(10).cast::<i32>().read_unaligned() as *const i32;
        let first = list.read_unaligned() as *const i32;
        if first.byte_add(4).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let count = pvar1.byte_add(6).cast::<i32>().read_unaligned();
        if count <= param_2 {
            param_2 = count.wrapping_sub(1);
        }
        let mut i_var3: i32 = 0;
        let elem = list.offset(param_2 as isize).read_unaligned() as *mut i32;
        let i_var2 = FUN_004af150(elem, pvar1 as i32, param_3);
        if i_var2 != 0 {
            let mut old: u16 = 0;
            let mut new: u16 = 0;
            let mut out: i64 = 0;
            core::arch::asm!(
                "fnstcw word ptr [{old}]",
                "mov {t:x}, word ptr [{old}]",
                "or {t:x}, 0x0c00",
                "mov word ptr [{new}], {t:x}",
                "fldcw word ptr [{new}]",
                "fistp qword ptr [{out}]",
                "fldcw word ptr [{old}]",
                old = in(reg) &raw mut old,
                new = in(reg) &raw mut new,
                out = in(reg) &raw mut out,
                t = out(reg) _,
                options(nostack),
            );
            i_var3 = out as i32;
        }
        FUN_004ac740();
        i_var3.wrapping_neg()
    }
}
