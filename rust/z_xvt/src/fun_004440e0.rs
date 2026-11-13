// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004440e0(param_1: u16) -> u32 {
    if param_1 == 0xffff {
        return 0;
    }
    // SAFETY: the globals are the game's own memory; the pointers computed
    // from them address the game's own structures, read where the original does.
    unsafe {
        if (param_1 as u32) >= DAT_006d4980 {
            return DAT_006d4980 & 0xffffff00;
        }
        let u_var2: u32 = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988);
        let local_1b: i32 = ((u_var2.wrapping_add(0x1b)) as *const i32).read_unaligned();
        if local_1b == -1 {
            return u_var2 & 0xffffff00;
        }
        let local_1f: i32 = ((u_var2.wrapping_add(0x1f)) as *const i32).read_unaligned();
        let local_a7: i32 = ((local_1f.wrapping_add(0xa7)) as u32 as *const i32).read_unaligned();
        let i_var1: i32 = local_a7;
        if i_var1 == 0 {
            return 0;
        }
        let u_var3: u32 = (i_var1 as u32) >> 8;
        let base = i_var1 as u32;
        if (((base.wrapping_add(0x1e0)) as *const u8).read() & 1) != 0
            && ((base.wrapping_add(0x237)) as *const i8).read() != 0
            && ((base.wrapping_add(0x233)) as *const i8).read() == 3
            && ((base.wrapping_add(0x238)) as *const i16).read_unaligned() != 0
        {
            return (u_var3 << 8) | 1;
        }
        u_var3 << 8
    }
}
