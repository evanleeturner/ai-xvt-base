// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005247e8: u8;
    static mut DAT_005247e9: u8;
    static mut DAT_00524860: u8;
    static mut DAT_0052486c: u8;
    static mut DAT_0052686a: u8;
    static mut DAT_0052686b: u8;
    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0070ad47: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad49: u8;
    static mut DAT_0070ad4f: u8;
    static mut DAT_0070ad92: u8;
    static mut DAT_0070b216: u8;
    static mut DAT_0070b252: u8;
}

#[inline(always)]
unsafe fn at(p: *mut u8, i: usize) -> u8 {
    // SAFETY: the caller indexes the game's own tables as the original does.
    unsafe { p.byte_add(i).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049ec30(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    // SAFETY: all globals are the game's own tables, indexed as the original does.
    unsafe {
        let i_var1: i32 = (param_1 as u32).wrapping_mul(0x566) as i32;
        let idx = i_var1 as usize;
        let p3 = param_3 as usize;
        let u_var3: u32 = at(
            &raw mut DAT_005247e8,
            at(&raw mut DAT_0070ad42, idx) as usize,
        ) as u32;
        let row = u_var3.wrapping_mul(0x16) as usize;
        match param_2 {
            0 => 0,
            1 => (param_3 == param_1) as u16,
            2 => (at(&raw mut DAT_005247e9, p3) as u32 == u_var3) as u16,
            3 => (at(&raw mut DAT_0052686b, row) == at(&raw mut DAT_00524860, p3)) as u16,
            4 => (at(&raw mut DAT_0052686a, row) == at(&raw mut DAT_0052486c, p3)) as u16,
            5 => (at(&raw mut DAT_0070ad47, idx) as u16 == param_3) as u16,
            6 => (at(&raw mut DAT_0070ad92, idx) as u16 == param_3) as u16,
            7 => {
                let local_val: i32 = (&raw mut DAT_0070b252)
                    .byte_add(idx)
                    .cast::<i32>()
                    .read_unaligned();
                if param_3 == 9 {
                    (local_val != -1) as u16
                } else if param_3 != 10 {
                    1
                } else {
                    (local_val == -1) as u16
                }
            }
            8 => (at(&raw mut DAT_0070ad4f, idx) as u16 == param_3) as u16,
            9 => (at(&raw mut DAT_0070ad49, idx) as u16 == param_3) as u16,
            10 => (at(&raw mut DAT_0070ad44, idx) as u16 == param_3) as u16,
            0xb => 1,
            0xc => (at(&raw mut DAT_0070ad48, idx) as u16 == param_3) as u16,
            0xf => (param_3 != param_1) as u16,
            0x10 => (at(&raw mut DAT_005247e9, p3) as u32 != u_var3) as u16,
            0x11 => (at(&raw mut DAT_0052686b, row) != at(&raw mut DAT_00524860, p3)) as u16,
            0x12 => (at(&raw mut DAT_0052686a, row) != at(&raw mut DAT_0052486c, p3)) as u16,
            0x13 => (at(&raw mut DAT_0070ad47, idx) as u16 != param_3) as u16,
            0x14 => (at(&raw mut DAT_0070ad4f, idx) as u16 != param_3) as u16,
            0x15 => (at(&raw mut DAT_0070ad48, idx) as u16 != param_3) as u16,
            0x17 => (at(&raw mut DAT_0070b216, idx) as u16 == param_3) as u16,
            0x18 => (at(&raw mut DAT_0070b216, idx) as u16 != param_3) as u16,
            _ => 0,
        }
    }
}
