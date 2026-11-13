// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_00427d30(param_1: i16, param_2: i16, param_3: i32);
}

#[inline(always)]
fn term(a: i16, b: i16) -> u32 {
    let p = (a as i64).wrapping_mul(b as i64);
    let lo = (p as u32) >> 0xf;
    let hi = (((p as u64) >> 0x20) as i32).wrapping_shl(0x11);
    lo | (hi as u32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004035d0(param_1: i32, param_2: i16, param_3: i16, param_4: i16) {
    // SAFETY: param_1 is the address of the game's object; all offsets read
    // and the globals written are the game's own memory, as in the original.
    unsafe {
        let ptr_base = ((param_1 as u32).wrapping_add(0x1f) as *const u32).read_unaligned();
        let base = ptr_base as usize;

        if *((base.wrapping_add(0x92)) as *const u8) != 0 {
            let v10 = ((param_1 as u32).wrapping_add(0x12) as *const i16).read_unaligned();
            let v11 = ((param_1 as u32).wrapping_add(0x10) as *const i16).read_unaligned();
            FUN_00427bd0(v10, v11, param_1);
            let v12 = ((param_1 as u32).wrapping_add(0x14) as *const i16).read_unaligned();
            FUN_00427d30(v12, 0, param_1);
        }

        let rd = |off: usize| -> i16 { (base.wrapping_add(off) as *const i16).read_unaligned() };

        let s99 = rd(0x99);
        let s9f = rd(0x9f);
        let s93 = rd(0x93);
        let a = term(param_3, s9f)
            .wrapping_add(term(param_2, s99))
            .wrapping_add(term(param_4, s93));
        DAT_009d8a58 = a as i32;

        let s9b = rd(0x9b);
        let sa1 = rd(0xa1);
        let s95 = rd(0x95);
        let b = term(param_3, sa1)
            .wrapping_add(term(param_2, s9b))
            .wrapping_add(term(param_4, s95));
        DAT_009d8a54 = b as i32;

        let s9d = rd(0x9d);
        let sa3 = rd(0xa3);
        let s97 = rd(0x97);
        let c = term(param_3, sa3)
            .wrapping_add(term(param_2, s9d))
            .wrapping_add(term(param_4, s97));
        DAT_009d8b60 = c as i32;
    }
}
