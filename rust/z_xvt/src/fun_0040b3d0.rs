// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b75d: [u8; 0];
    static mut DAT_0071b75f: [u8; 0];
    static mut DAT_00525550: i32;
    static mut DAT_0070a189: i8;
    static mut DAT_0085e793: i8;
    static mut DAT_0085e79c: i8;
    static mut DAT_0085e79a: u8;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_0073ba30: [u8; 0];
    static mut DAT_0052686c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040acd0();
    fn FUN_0040ae30();
    fn FUN_0040b190();
    fn FUN_00439830(a: i32, b: i32) -> u32;
    fn FUN_004398b0(a: i32);
    fn FUN_004396b0(a: i32, b: i32, c: u32);
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_00490a50(a: *mut i8, b: i32, c: i32, d: u32, e: i32, f: i32);
    fn FUN_00412810(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040a1a0(a: u32, b: u32, c: i32);
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: caller supplies a game-memory address, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: caller supplies a game-memory address, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: caller supplies a game-memory address, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040b3d0() {
    // SAFETY: all globals, callees and the addresses read are the game's own memory,
    // accessed in the same order as the original.
    unsafe {
        let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
        let u_var3 =
            (((&raw mut DAT_0071b730) as *mut u8 as u32).wrapping_add(off)) as usize as *const u32;
        let u_var3 = u_var3.read_unaligned();
        if u_var3 == 0xffff_ffff {
            return;
        }
        FUN_0040acd0();
        FUN_0040ae30();
        FUN_0040b190();
        if !(DAT_00525550 == 0 && DAT_0070a189 != 0 && DAT_0085e793 != 0 && DAT_0085e79c != 0) {
            return;
        }
        let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
        let u_var9 = rd16(((&raw mut DAT_0071b75d) as *mut u8 as u32).wrapping_add(off));
        if u_var9 == 0xffff {
            if FUN_00439830(0x4e, 0x100) != 0 {
                FUN_004398b0(0x4e);
            }
            if FUN_00439830(0x4f, 0x100) != 0 {
                FUN_004398b0(0x4f);
            }
        } else {
            let c_var1 = rd8((u_var9 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32)) as i8;
            let mut i_var10: i32 = 0x4f;
            if c_var1 == b'3' as i8
                || c_var1 == b'4' as i8
                || c_var1 == b'5' as i8
                || c_var1 == b'6' as i8
            {
                i_var10 = 0x4e;
            }
            let tmp16 = rd16(((&raw mut DAT_0071b75f) as *mut u8 as u32).wrapping_add(off));
            let mut u_var4 = (tmp16 as u32)
                .wrapping_mul(DAT_0085e79a as u32)
                .wrapping_mul(4)
                / 10;
            if 0x7f < u_var4 {
                u_var4 = 0x7f;
            }
            let u_var5 = FUN_00439830(i_var10, 0x100);
            if u_var5 == 0 {
                FUN_004a1d70(0xdc, DAT_0071ecf4);
                FUN_00490a50(
                    ((&raw mut DAT_0073ba30) as *mut u8 as u32)
                        .wrapping_add((i_var10 as u32).wrapping_mul(6)) as usize
                        as *mut i8,
                    1,
                    1,
                    u_var4,
                    0x41,
                    0x40,
                );
            } else {
                FUN_004396b0(i_var10, 0x600, u_var4);
            }
        }
        let mut u_var4: u32 = 0;
        if 0 < DAT_006d4980 {
            let mut i_var10: i32 = 0;
            loop {
                if u_var3 != u_var4 {
                    let p_ent = (DAT_006d4988 as u32).wrapping_add(i_var10 as u32);
                    let b_var2 = rd8(p_ent.wrapping_add(3));
                    if b_var2 != 0 {
                        let p_obj = rd32(p_ent.wrapping_add(0x1f));
                        let p_sub = rd32(p_obj.wrapping_add(0xa7));
                        let c7 = rd8(p_sub.wrapping_add(7)) as i8;
                        let s1 = rd16(p_sub.wrapping_add(0x1df)) as i16;
                        let s2 = rd16(p_obj.wrapping_add(0x79)) as i16;
                        if c7 == 0 && s1 != 0 && s2 != 0 {
                            let mut u_var9: u16 = 0xffff;
                            match b_var2 {
                                1 | 4 | 0xe | 0xf => u_var9 = 0x4a,
                                2 => u_var9 = 0x4b,
                                3 | 0xd => u_var9 = 0x4c,
                                5 | 6 | 7 | 8 | 9 => u_var9 = 0x48,
                                0xc | 0x10 => u_var9 = 0x49,
                                0x11..=0x19 | 0x1e => u_var9 = 0x49,
                                0x26 | 0x27 => u_var9 = 0x4d,
                                _ => {}
                            }
                            if u_var9 != 0xffff {
                                let p_other =
                                    (DAT_006d4988 as u32).wrapping_add(u_var3.wrapping_mul(0x23));
                                let a0 = rd32(p_ent.wrapping_add(4));
                                let b0 = rd32(p_other.wrapping_add(4));
                                let a1 = rd32(p_ent.wrapping_add(8));
                                let b1 = rd32(p_other.wrapping_add(8));
                                let a2 = rd32(p_ent.wrapping_add(0xc));
                                let b2 = rd32(p_other.wrapping_add(0xc));
                                let i_var6 = FUN_00412810(
                                    a0.wrapping_sub(b0) as i32,
                                    a1.wrapping_sub(b1) as i32,
                                    a2.wrapping_sub(b2) as i32,
                                );
                                let u_c = rd32(
                                    (DAT_006d4988 as u32)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(i_var10 as u32),
                                );
                                let u_o = rd32(
                                    (DAT_006d4988 as u32)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(u_var3.wrapping_mul(0x23)),
                                );
                                let a0 = rd32(u_c.wrapping_add(6));
                                let b0 = rd32(u_o.wrapping_add(6));
                                let a1 = rd32(u_c.wrapping_add(10));
                                let b1 = rd32(u_o.wrapping_add(10));
                                let a2 = rd32(u_c.wrapping_add(0xe));
                                let b2 = rd32(u_o.wrapping_add(0xe));
                                let i_var7 = FUN_00412810(
                                    a0.wrapping_sub(b0) as i32,
                                    a1.wrapping_sub(b1) as i32,
                                    a2.wrapping_sub(b2) as i32,
                                );
                                let thr = (rd16(
                                    ((&raw mut DAT_0052686c) as *mut u8 as u32)
                                        .wrapping_add((b_var2 as u32).wrapping_mul(0x16)),
                                ) as u32)
                                    .wrapping_add(0x400)
                                    as i32;
                                if i_var6 < thr && thr <= i_var7 {
                                    FUN_0040a1a0(u_var9 as u32, u_var4, DAT_0071ecf4);
                                }
                            }
                        }
                    }
                }
                i_var10 = i_var10.wrapping_add(0x23);
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
    }
}
