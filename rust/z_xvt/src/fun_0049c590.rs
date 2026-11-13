// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071f714: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb1b: i32;
    static mut DAT_0071bb1f: i32;
    static mut DAT_0071bb23: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> u32;
    fn FUN_00419100(a: u32, b: u32) -> u32;
    fn FUN_004832a0(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_00486310(a: i32);
}

#[inline(always)]
unsafe fn r32(a: i32) -> i32 {
    // SAFETY: address within the game's own memory, as in the original.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn r16(a: i32) -> i16 {
    // SAFETY: address within the game's own memory, as in the original.
    unsafe { (a as usize as *const i16).read_unaligned() }
}

#[inline(always)]
unsafe fn w16(a: i32, v: i16) {
    // SAFETY: address within the game's own memory, as in the original.
    unsafe { (a as usize as *mut i16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049c590(param_1: u16) {
    // SAFETY: globals and callees are the game's own; all pointers derive
    // from the game's tables and are accessed where the original does.
    unsafe {
        DAT_0071f714 = param_1;
        let i_var8: i32 = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988 as u32) as i32;
        let off = DAT_0071ecf4.wrapping_mul(0x5bd);
        let a1 = (&raw mut DAT_0071bb1b as usize as i32).wrapping_add(off);
        let a2 = (&raw mut DAT_0071bb1f as usize as i32).wrapping_add(off);
        let a3 = (&raw mut DAT_0071bb23 as usize as i32).wrapping_add(off);
        let i_var9 = r32(a1).wrapping_sub(r32(i_var8.wrapping_add(4)));
        let mut p = r32(i_var8.wrapping_add(0x1f));
        let i_var6 = r32(a2).wrapping_sub(r32(i_var8.wrapping_add(8)));
        let i_var7 = r32(a3).wrapping_sub(r32(i_var8.wrapping_add(0xc)));
        let s97 = r16(p.wrapping_add(0x97));
        let s99 = r16(p.wrapping_add(0x99));
        let s9b = r16(p.wrapping_add(0x9b));
        let u_var3 = FUN_00413cc0(s97 as i32, s99 as i32, s9b as i32, i_var9, i_var6, i_var7);
        p = r32(i_var8.wrapping_add(0x1f));
        let s9d = r16(p.wrapping_add(0x9d));
        let s9f = r16(p.wrapping_add(0x9f));
        let sa1 = r16(p.wrapping_add(0xa1));
        let u_var4 = FUN_00413cc0(s9d as i32, s9f as i32, sa1 as i32, i_var9, i_var6, i_var7);
        let s_var1 = r16(i_var8.wrapping_add(0x14));
        let u_var5 = FUN_00419100(u_var4, u_var3);
        w16(
            i_var8.wrapping_add(0x14),
            (u_var5 as i16).wrapping_add(s_var1),
        );
        let t = r16(i_var8.wrapping_add(0x14));
        w16(i_var8.wrapping_add(0x14), t.wrapping_add(-0x4000));
        p = r32(i_var8.wrapping_add(0x1f));
        (p.wrapping_add(0x90) as usize as *mut u8).write(1);
        FUN_004832a0(
            r16(i_var8.wrapping_add(0x14)) as i32,
            r16(i_var8.wrapping_add(0x12)) as i32,
            r16(i_var8.wrapping_add(0x10)) as i32,
            0,
            i_var8,
        );
        FUN_00486310(i_var8);
        w16(i_var8.wrapping_add(0x14), s_var1);
        p = r32(i_var8.wrapping_add(0x1f));
        (p.wrapping_add(0x90) as usize as *mut u8).write(1);
    }
}
