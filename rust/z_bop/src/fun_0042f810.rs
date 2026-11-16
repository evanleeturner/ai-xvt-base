// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_00523410: i32;
    static mut DAT_009d80c9: i8;
    static mut DAT_00bb7373: i8;
    static mut DAT_00bb737c: i8;
    static mut DAT_00bb737a: u8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_00a0adf0: [i8; 0];
    static mut DAT_00521f1c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0042f110();
    fn FUN_0042f270();
    fn FUN_0042f5d0();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9300(param_1: i32, param_2: i32) -> u32;
    fn FUN_004a9380(param_1: i32);
    fn FUN_004a9180(param_1: i32, param_2: i32, param_3: u32);
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_0042d120(
        param_1: *mut i8,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_0041f410(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_0042e4d0(param_1: u32, param_2: u32, param_3: i32) -> u32;
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042f810() {
    // SAFETY: globals and callees are the game's own; all pointer reads
    // are of the game's tables at the offsets the original uses.
    unsafe {
        let pb_player: u32 = ((&raw mut DAT_009e9670) as *mut u8 as usize as u32)
            .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd));
        let u_var3: u32 = rd32(pb_player);
        if u_var3 == 0xffff_ffff {
            return;
        }
        FUN_0042f110();
        FUN_0042f270();
        FUN_0042f5d0();
        if !(DAT_00523410 == 0 && DAT_009d80c9 != 0 && DAT_00bb7373 != 0 && DAT_00bb737c != 0) {
            return;
        }
        let u_var9 = rd16(pb_player.wrapping_add(0x2d));
        if u_var9 == 0xffff {
            if FUN_004a9300(0x4e, 0x100) != 0 {
                FUN_004a9380(0x4e);
            }
            if FUN_004a9300(0x4f, 0x100) != 0 {
                FUN_004a9380(0x4f);
            }
        } else {
            let c_var1 = rd8((u_var9 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_009a1fe8 as u32)) as i8;
            let mut i_var10: i32 = 0x4f;
            if c_var1 == b'3' as i8
                || c_var1 == b'4' as i8
                || c_var1 == b'5' as i8
                || c_var1 == b'6' as i8
            {
                i_var10 = 0x4e;
            }
            let u_w = rd16(pb_player.wrapping_add(0x2f));
            let mut u_var4: u32 =
                ((u_w as u32).wrapping_mul(DAT_00bb737a as u32)).wrapping_mul(4) / 10;
            if 0x7f < u_var4 {
                u_var4 = 0x7f;
            }
            let u_var5 = FUN_004a9300(i_var10, 0x100);
            if u_var5 == 0 {
                FUN_00450650(0xdd, DAT_009ecc34);
                FUN_0042d120(
                    ((&raw mut DAT_00a0adf0) as *mut i8)
                        .byte_add((i_var10 as usize).wrapping_mul(6)),
                    1,
                    1,
                    u_var4,
                    0x41,
                    0x40,
                );
            } else {
                FUN_004a9180(i_var10, 0x600, u_var4);
            }
        }
        let mut u_var4: u32 = 0;
        if 0 < DAT_009a1fe0 {
            let mut i_var10: i32 = 0;
            loop {
                if u_var3 != u_var4 {
                    let pb_entry: u32 = (DAT_009a1fe8 as u32).wrapping_add(i_var10 as u32);
                    let b_var2 = rd8(pb_entry.wrapping_add(3));
                    if b_var2 != 0 {
                        let u_a = rd32(pb_entry.wrapping_add(0x1f));
                        let u_b = rd32(u_a.wrapping_add(0xa9));
                        let u_w = rd16(u_b.wrapping_add(0x1e5));
                        let u_9 = rd16(u_a.wrapping_add(0x79));
                        if rd8(u_b.wrapping_add(7)) == 0 && u_w != 0 && u_9 != 0 {
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
                                let pb_other: u32 =
                                    (DAT_009a1fe8 as u32).wrapping_add(u_var3.wrapping_mul(0x23));
                                let a = rd32(pb_entry.wrapping_add(4));
                                let b = rd32(pb_other.wrapping_add(4));
                                let c = rd32(pb_entry.wrapping_add(8));
                                let d = rd32(pb_other.wrapping_add(8));
                                let e = rd32(pb_entry.wrapping_add(0xc));
                                let f = rd32(pb_other.wrapping_add(0xc));
                                let i_var6 = FUN_0041f410(
                                    a.wrapping_sub(b) as i32,
                                    c.wrapping_sub(d) as i32,
                                    e.wrapping_sub(f) as i32,
                                );

                                let i_var7 = rd32(
                                    (DAT_009a1fe8 as u32)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(i_var10 as u32),
                                );
                                let i_var8 = rd32(
                                    (DAT_009a1fe8 as u32)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(u_var3.wrapping_mul(0x23)),
                                );
                                let a = rd32(i_var7.wrapping_add(6));
                                let b = rd32(i_var8.wrapping_add(6));
                                let c = rd32(i_var7.wrapping_add(10));
                                let d = rd32(i_var8.wrapping_add(10));
                                let e = rd32(i_var7.wrapping_add(0xe));
                                let f = rd32(i_var8.wrapping_add(0xe));
                                let i_var7b = FUN_0041f410(
                                    a.wrapping_sub(b) as i32,
                                    c.wrapping_sub(d) as i32,
                                    e.wrapping_sub(f) as i32,
                                );

                                let thresh_addr = ((&raw mut DAT_00521f1c) as *mut u8 as usize
                                    as u32)
                                    .wrapping_add((b_var2 as u32).wrapping_mul(0x1a));
                                let u_thresh = rd32(thresh_addr).wrapping_add(0x400) as i32;
                                if i_var6 < u_thresh && u_thresh <= i_var7b {
                                    FUN_0042e4d0(u_var9 as u32, u_var4, DAT_009ecc34);
                                }
                            }
                        }
                    }
                }
                i_var10 = i_var10.wrapping_add(0x23);
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
    }
}
