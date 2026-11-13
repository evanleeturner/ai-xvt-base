// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0048c: i32;
    static mut DAT_00a00494: i32;
    static mut DAT_00a07c68: i32;
    static mut DAT_00a07c6c: i32;
    static mut DAT_00a07cc4: i32;
    static mut DAT_00a004d0: i32;
    static mut DAT_00a004b4: i32;
    static mut DAT_00a004ac: i32;
    static mut DAT_009a8da4: i32;
    static mut DAT_009a8d88: i32;
    static mut DAT_009a8d84: i32;
    static mut DAT_00a0049c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a6d0(param_1: i16) -> i64;
    fn FUN_0046a3c0(param_1: i16) -> i64;
}

#[inline(always)]
fn mulshift(a: i32, b: i32) -> i32 {
    let p = (a as i64).wrapping_mul(b as i64);
    let lo = (p as u32) >> 0xf;
    let hi = (((p as u64) >> 0x20) as i32).wrapping_shl(0x11);
    (lo | (hi as u32)) as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427bd0(param_1: i16, param_2: i16, param_3: i32) {
    // SAFETY: the callees and globals are the game's own; param_3 is the
    // address of a game structure, read and written where the original does.
    unsafe {
        let neg2 = (param_2 as i32).wrapping_neg() as i16;
        let ang = (-0x4000i32).wrapping_sub(param_1 as i32) as i16;
        let l3 = FUN_0046a6d0(neg2);
        let l4 = FUN_0046a6d0(ang);
        let l5 = FUN_0046a3c0(neg2);
        let i_var2 = (l3 as i16) as i32;
        let l3b = FUN_0046a3c0(ang);
        DAT_00a0048c = (l5 as i16) as i32;
        DAT_00a00494 = 0;
        let i_var1 = (l4 as i16) as i32;
        DAT_00a07c68 = mulshift(DAT_00a0048c.wrapping_neg(), i_var1);
        DAT_00a07c6c = mulshift(i_var2, i_var1);
        DAT_00a07cc4 = (l3b as i16) as i32;
        DAT_00a004d0 = mulshift(DAT_00a0048c, DAT_00a07cc4).wrapping_neg();
        DAT_00a004b4 = i_var1.wrapping_neg();
        DAT_00a004ac = mulshift(i_var2.wrapping_neg(), DAT_00a07cc4).wrapping_neg();
        DAT_009a8da4 = DAT_00a07c68.wrapping_neg();
        DAT_009a8d88 = DAT_00a07cc4.wrapping_neg();
        DAT_009a8d84 = DAT_00a07c6c.wrapping_neg();
        DAT_00a0049c = i_var2;

        let ptr_val = (param_3.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
        if param_3 != 0 {
            let base = ptr_val as usize as *mut u8;
            base.byte_add(0x8c)
                .cast::<i16>()
                .write_unaligned(DAT_009a8da4 as i16);
            base.byte_add(0x8e)
                .cast::<i16>()
                .write_unaligned(DAT_009a8d84 as i16);
            base.byte_add(0x90)
                .cast::<i16>()
                .write_unaligned(DAT_009a8d88 as i16);
            base.byte_add(0x8b).write(0);
        }
    }
}
