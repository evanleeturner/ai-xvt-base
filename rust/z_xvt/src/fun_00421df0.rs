// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004490f0(param_1: u32) -> u32;
    fn FUN_0044a600(param_1: u32, param_2: u32) -> u32;
    fn FUN_00421ff0(param_1: u32, param_2: i8) -> u32;
    fn FUN_00478470() -> u32;
}

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *mut u8).write(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421df0(param_1: u32, param_2: i16) {
    // SAFETY: the globals and callees are the game's own; all addresses
    // computed here point into the game's records, as in the original.
    unsafe {
        let i_var5: u32 = (param_1 & 0xffff).wrapping_mul(0x23);
        let b = DAT_006d4988 as u32;
        let u_var10: u32 = FUN_004490f0(rd8(b.wrapping_add(3).wrapping_add(i_var5)) as u32);
        let u_var8: u16 = u_var10 as u16;
        if 1 < u_var8 {
            let mut local_10: u16 = 0;
            let b = DAT_006d4988 as u32;
            let i_var3: u32 =
                rd32(rd32(b.wrapping_add(0x1f).wrapping_add(i_var5)).wrapping_add(0xa7));
            loop {
                let u_var7: u32 = local_10 as u32;
                if rd8(u_var7.wrapping_add(0x2ca).wrapping_add(i_var3)) == 0 {
                    let b = DAT_006d4988 as u32;
                    let t =
                        FUN_0044a600(rd8(b.wrapping_add(3).wrapping_add(i_var5)) as u32, u_var7);
                    if t != 0 {
                        let u_var11: u32 = FUN_00421ff0(param_1, local_10 as i8);
                        if (u_var11 as i16) != -1 {
                            let mut s14: i16 =
                                (((FUN_00478470() as u16) & 0x3fff).wrapping_add(0x4000)) as i16;
                            let mut s13: i16 =
                                (((FUN_00478470() as u16) & 0x7ff).wrapping_add(0x400)) as i16;
                            let mut s15: i16 =
                                (((FUN_00478470() as u16) & 0xfff).wrapping_add(0x400)) as i16;
                            let u_var12 = FUN_00478470();
                            if (u_var12 & 1) != 0 {
                                s14 = s14.wrapping_neg();
                                s13 = s13.wrapping_neg();
                            }
                            let u_var12 = FUN_00478470();
                            if (u_var12 & 1) != 0 {
                                s15 = s15.wrapping_neg();
                            }
                            let i_var6: u32 = (u_var11 & 0xffff).wrapping_mul(0x23);
                            let b = DAT_006d4988 as u32;
                            wr16(
                                rd32(b.wrapping_add(0x1f).wrapping_add(i_var6)).wrapping_add(0x77),
                                s14 as u16,
                            );
                            let a10 = b.wrapping_add(0x10).wrapping_add(i_var6);
                            wr16(a10, rd16(a10).wrapping_add(s13 as u16));
                            let a12 = b.wrapping_add(0x12).wrapping_add(i_var6);
                            wr16(a12, rd16(a12).wrapping_add(s15 as u16));
                            let u_var9 = rd16(a12);
                            if 0x7fff < u_var9 {
                                wr16(a12, u_var9.wrapping_neg());
                                wr16(a10, rd16(a10).wrapping_add(0x8000));
                            }
                            let p = rd32(b.wrapping_add(0x1f).wrapping_add(i_var6));
                            wr8(p.wrapping_add(0x90), 1);
                            let i_var4 = rd32(b.wrapping_add(0x1f).wrapping_add(i_var6));
                            wr8(i_var4.wrapping_add(0x89), rd8(i_var4.wrapping_add(0x90)));
                            let u_var9 = FUN_00478470() as u16;
                            let v =
                                ((((u_var9 & 1) as u32).wrapping_add(1)).wrapping_mul(0xec)) as u16;
                            wr16(
                                rd32(b.wrapping_add(0x1f).wrapping_add(i_var6)).wrapping_add(0x7f),
                                v,
                            );
                            wr8(b.wrapping_add(0x1a).wrapping_add(i_var6), 2);
                            wr8(u_var7.wrapping_add(0x2ca).wrapping_add(i_var3), 4);
                            wr8(
                                (u_var10 & 0xffff).wrapping_add(0x2ca).wrapping_add(i_var3),
                                2,
                            );
                            if param_2 == 0 {
                                return;
                            }
                        }
                    }
                }
                local_10 = local_10.wrapping_add(1);
                if !(local_10 < u_var8) {
                    break;
                }
            }
        }
    }
}
