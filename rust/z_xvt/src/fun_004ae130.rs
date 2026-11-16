// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070a184: unsafe extern "stdcall" fn(i32, i16, i16, i16, i32);
    static mut DAT_00703c8c: unsafe extern "stdcall" fn(i32, i16, i16, i16, u32, i16);
    static mut DAT_00655184: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_006d4988: i32;
    static mut DAT_00656c90: [u8; 0];
    static mut DAT_00655ab0: [u8; 0];
    static mut DAT_0071f2f0: [u8; 0];
    static mut DAT_00656cd6: u16;
    static mut DAT_0052f120: [u8; 0];
    static mut DAT_0052f123: u8;
    static mut DAT_0052552c: i32;
    static mut DAT_0052f128: [u8; 0];
    static mut DAT_0052f150: [u8; 0];
    static mut DAT_00655c58: [u8; 0];
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address belongs to the game's own memory, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address belongs to the game's own memory, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address belongs to the game's own memory, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ae130() {
    // SAFETY: all globals and addresses are the game's own memory; reads,
    // writes and calls occur in the original's order.
    unsafe {
        let u_var12: u16 = DAT_00655184.wrapping_add(0x23);
        let b_71b730 = (&raw mut DAT_0071b730) as *mut u8 as u32;
        let b_656c90 = (&raw mut DAT_00656c90) as *mut u8 as u32;
        let b_655ab0 = (&raw mut DAT_00655ab0) as *mut u8 as u32;
        let b_71f2f0 = (&raw mut DAT_0071f2f0) as *mut u8 as u32;
        let b_52f120 = (&raw mut DAT_0052f120) as *mut u8 as u32;
        let b_52f128 = (&raw mut DAT_0052f128) as *mut u8 as u32;
        let b_52f150 = (&raw mut DAT_0052f150) as *mut u8 as u32;
        let b_655c58 = (&raw mut DAT_00655c58) as *mut u8 as u32;

        let mut t0 = r32(b_71b730.wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd) as u32));
        t0 = r32(t0
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32));
        let i_var3 = r32(t0.wrapping_add(0xa7));

        if (r8(i_var3.wrapping_add(0x1db)) & 0x10) != 0 {
            let mut u_var11: u16 = r16(i_var3.wrapping_add(0x235));
            if (u_var11 as i16) < 0 {
                u_var11 = 0;
            }
            let us = r16(i_var3.wrapping_add(0x1df));
            let u_var7 = us & 0x100;
            if u_var7 == 0 {
                u_var11 = 0;
            }
            let mut u_var5: u16 = (r8(i_var3.wrapping_add(0x237)) != 0) as u16;
            if u_var7 == 0 {
                u_var5 = 0;
            }
            let mut i3: i32 = (DAT_00655184 as i32).wrapping_add(0x5c);
            let p = b_656c90.wrapping_add(i3.wrapping_mul(2) as u32) as usize as *mut u16;
            if p.read_unaligned() != u_var5 {
                p.write_unaligned(u_var5);
                i3 = i3.wrapping_mul(0xc);
                let off = i3 as u32;
                let a0 = r32(b_71f2f0.wrapping_add(
                    (r16(b_655ab0.wrapping_add(4).wrapping_add(off)) as u32)
                        .wrapping_add(u_var5 as u32)
                        .wrapping_mul(4),
                )) as i32;
                let a1 = r16(b_655ab0.wrapping_add(off)) as i16;
                let a2 = r16(b_655ab0.wrapping_add(2).wrapping_add(off)) as i16;
                let a3 = r16(b_655ab0.wrapping_add(6).wrapping_add(off)) as i16;
                (DAT_0070a184)(a0, a1, a2, a3, 0);
            }
            let u_var4: u32 = (u_var11 as i16) as i32 as u32;
            if (DAT_00656cd6 as u32) != u_var4 {
                let mut local_e: i16 = 0;
                DAT_00656cd6 = u_var11;
                loop {
                    let i_var6: i32 = local_e as i32;
                    let i_var3b: i32 = (i_var6 as u32).wrapping_mul(5).wrapping_add(5) as i32;
                    let u_var9: u32 = ((i_var3b as u32).wrapping_mul(5) >> 8) & 0x00ff_ffff;
                    let u_var8: u32;
                    if (i_var3b as u32).wrapping_mul(200) as i32 > u_var4 as i32 {
                        u_var8 = (u_var9 << 8) | (DAT_0052f123 as u32);
                    } else if (u_var11 as i16) < 0 {
                        u_var8 = (u_var9 << 8) | (r8(b_52f120) as u32);
                    } else {
                        let mut u7 = u_var11;
                        if 1000 < (u_var11 as i16) {
                            u7 = 1000;
                        }
                        let idx = ((u7 as i16) as i32 / 0x14d) as u32;
                        u_var8 =
                            ((u_var9 << 8) | (r8(b_52f120.wrapping_add(idx)) as u32)) & 0xffff_00ff;
                    }
                    let mut s13: i16 =
                        r16(b_655ab0.wrapping_add((u_var12 as u32).wrapping_mul(0xc))) as i16;
                    let mut s10: i16 = r16(b_655ab0
                        .wrapping_add(2)
                        .wrapping_add((u_var12 as u32).wrapping_mul(0xc)))
                        as i16;
                    if DAT_0052552c == 0x101 {
                        let s1 = ((8i32.wrapping_sub(local_e as i32)).wrapping_mul(3)) as i16;
                        s13 = s13.wrapping_add(s1);
                        s10 = s10.wrapping_add(s1);
                    } else if DAT_0052552c == 0x112 {
                        let o = i_var6.wrapping_mul(4) as u32;
                        s13 = s13.wrapping_add(r16(b_52f128.wrapping_add(o)) as i16);
                        s10 =
                            s10.wrapping_add(r16(b_52f128.wrapping_add(2).wrapping_add(o)) as i16);
                    } else {
                        let o = i_var6.wrapping_mul(4) as u32;
                        s13 = s13.wrapping_add(r16(b_52f150.wrapping_add(o)) as i16);
                        s10 =
                            s10.wrapping_add(r16(b_52f150.wrapping_add(2).wrapping_add(o)) as i16);
                    }
                    let u_var2: i16 = if (r8(b_52f120) as u32) == (u_var8 & 0xff) {
                        0
                    } else {
                        r16(b_655c58
                            .wrapping_add(4)
                            .wrapping_add((DAT_00655184 as u32).wrapping_mul(0xc)))
                            as i16
                    };
                    u_var11 = u_var11.wrapping_sub(1000);
                    let base = (DAT_00655184 as u32).wrapping_mul(0xc);
                    let a0 = r32(b_71f2f0.wrapping_add(
                        (r16(b_655c58.wrapping_add(base)) as u32)
                            .wrapping_add(i_var6 as u32)
                            .wrapping_mul(4),
                    )) as i32;
                    let a3 = r16(b_655c58.wrapping_add(2).wrapping_add(base)) as i16;
                    (DAT_00703c8c)(a0, s13, s10, a3, u_var8 & 0xffff_00ff, u_var2);
                    local_e = local_e.wrapping_add(1);
                    if local_e >= 9 {
                        break;
                    }
                }
            }
        }
    }
}
