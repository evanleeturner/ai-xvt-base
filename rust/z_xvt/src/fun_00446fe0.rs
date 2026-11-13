// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525ade: i16;
    static mut DAT_00525b0e: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478320(param_1: u32, param_2: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00446fe0(param_1: i32) -> i16 {
    // SAFETY: the globals and the callee are the game's own; the offsets
    // index the game's tables exactly as the original does.
    unsafe {
        let off = param_1.wrapping_mul(2) as isize;
        let s_var1 = (&raw mut DAT_00525ade)
            .cast::<u8>()
            .wrapping_byte_offset(off)
            .cast::<i16>()
            .read_unaligned();
        let v = (&raw mut DAT_00525b0e)
            .cast::<u8>()
            .wrapping_byte_offset(off)
            .cast::<i16>()
            .read_unaligned();
        let arg = ((param_1 as u32) & 0xFFFF0000) | (v as u16 as u32);
        let u_var2 = FUN_00478320(arg, 0xec);
        (u_var2 as i16).wrapping_add(s_var1.wrapping_mul(0xec))
    }
}
