// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
    static mut DAT_00732e54: u32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b704: u32;
    static mut DAT_006db798: u32;
    static mut DAT_0052b0a8: [u8; 0];
    static mut DAT_0052b0aa: [u8; 0];
    static mut DAT_0052b0ac: [u8; 0];
    static mut DAT_0070ad44: [u8; 0];
    static mut DAT_0070b215: [u8; 0];
    static mut DAT_0071b74a: [u8; 0];
    static mut DAT_0071b7b5: [u8; 0];
    static mut DAT_0071b7b9: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_004a1d70(param_1: i32, param_2: i32);
    fn FUN_00447cf0(param_1: i32, param_2: u32, param_3: u32) -> u32;
    fn FUN_004a7aa0(param_1: i32) -> u32;
    fn FUN_0040a3c0(param_1: u32, param_2: i32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn w32(a: u32, v: u32) {
    // SAFETY: address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00447240(param_1: i32, param_2: i32) {
    // SAFETY: all globals, callees and addresses belong to the game; the
    // original's reads and writes are reproduced in order.
    unsafe {
        let a8 = (&raw mut DAT_0052b0a8) as usize as u32;
        let aa = (&raw mut DAT_0052b0aa) as usize as u32;
        let ac = (&raw mut DAT_0052b0ac) as usize as u32;
        let p2 = param_2 as u32;

        let mut local_12: u16 = 0;
        let mut u_stack_e: u16 = 0;
        let mut u_stack_c: u16 = 0;
        let mut local_16: u32;

        let i_var9 = DAT_006d4988.wrapping_add((param_1 as u32).wrapping_mul(0x23));
        let i_var5 = r32(i_var9.wrapping_add(0x1b)) as i32;
        let i_var14 = r32(i_var9.wrapping_add(0x1f));
        DAT_00732e54 = r32(i_var14.wrapping_add(0xa7));
        let i_var9 = FUN_0042b150(DAT_00732e54 as i32);
        let b_var3 = r8(DAT_00732e54.wrapping_add(4));
        let u_var6 = b_var3 as u16;

        if r8(DAT_00732e54.wrapping_add(0x29)) == 0 {
            let mut u_var18: u32 = 0;
            let i_var14 = DAT_00732e54.wrapping_add(p2);
            let c_var12 = r8(i_var14.wrapping_add(0x216)) as i8;
            let u_var13: u16;
            if c_var12 == 1 {
                let i_var16 = (u_var6 as u32).wrapping_mul(0xef).wrapping_add(p2);
                let pa = i_var14.wrapping_add(0x21a);
                if r8(pa) < r8(aa.wrapping_add(i_var16)) || r8(ac.wrapping_add(i_var16)) < r8(pa) {
                    w8(pa, r8(aa.wrapping_add(i_var16)));
                }
                let q = DAT_00732e54.wrapping_add(0x21a).wrapping_add(p2);
                let b_var4 = r8(q);
                u_stack_e = b_var4 as u16;
                w8(q, b_var4.wrapping_add(1));
                if r8(ac.wrapping_add(i_var16)) < r8(q) {
                    w8(q, r8(aa.wrapping_add(i_var16)));
                }
                local_16 = 1u32 << 16;
                u_stack_c = 1;
                u_var13 = u_stack_e;
            } else if c_var12 == 2 {
                let mut p15 = i_var14.wrapping_add(0x21a);
                let i_var14 = (u_var6 as u32).wrapping_mul(0xef).wrapping_add(p2);
                let pb1 = aa.wrapping_add(i_var14);
                if r8(p15) < r8(pb1) || r8(ac.wrapping_add(i_var14)) < r8(p15) {
                    w8(p15, r8(pb1));
                }
                p15 = DAT_00732e54.wrapping_add(0x21a).wrapping_add(p2);
                let b_var4 = r8(p15);
                w8(p15, b_var4 ^ 1);
                p15 = DAT_00732e54.wrapping_add(0x21a).wrapping_add(p2);
                if r8(ac.wrapping_add(i_var14)) < r8(p15) {
                    w8(p15, r8(pb1));
                }
                u_stack_e = r8(ac.wrapping_add(i_var14)) as u16;
                u_stack_c = 2;
                let t = ((u_stack_e as u32)
                    .wrapping_sub(r8(pb1) as u32)
                    .wrapping_add(1)) as i32
                    / 2;
                local_16 = ((t as u16) as u32) << 16;
                u_var13 = b_var4 as u16;
            } else {
                let mut v = u_stack_c;
                if c_var12 == 3 {
                    let i_var14 = p2.wrapping_add((u_var6 as u32).wrapping_mul(0xef));
                    u_stack_c = 1;
                    u_stack_e = r8(ac.wrapping_add(i_var14)) as u16;
                    let t = (u_stack_e as i32)
                        .wrapping_sub(r8(aa.wrapping_add(i_var14)) as i32)
                        .wrapping_add(1);
                    local_16 = ((t as u16) as u32) << 16;
                    v = r8(aa.wrapping_add(i_var14)) as u16;
                } else {
                    local_16 = 0;
                }
                u_var13 = v;
            }
            local_16 = (local_16 & 0xffff0000) | u_var13 as u32;
            let mut u_var13 = u_var13;
            while u_var13 <= u_stack_e {
                let u_var19 = local_16 & 0xffff;
                let d = DAT_00732e54;
                if r8(d.wrapping_add(0x344).wrapping_add(u_var19.wrapping_mul(4))) != 0 {
                    let c_var12 =
                        r8(d.wrapping_add(u_var19.wrapping_mul(4)).wrapping_add(0x345)) as i8;
                    if 0 < c_var12 {
                        local_12 = r8(
                            a8.wrapping_add(p2.wrapping_add((u_var6 as u32).wrapping_mul(0xef)))
                        ) as u16;
                        if 0x3f < c_var12 {
                            local_12 = local_12.wrapping_add(1);
                        }
                        let u_var10 = FUN_00447cf0(param_1, u_var19, local_12 as u32);
                        if u_var10 != 0xffffffff {
                            let i_var14 = (r8(DAT_006d4988
                                .wrapping_add(0x16)
                                .wrapping_add((param_1 as u32).wrapping_mul(0x23)))
                                as u32)
                                .wrapping_mul(0x566);
                            let b1 =
                                ((&raw mut DAT_0070ad44) as usize as u32).wrapping_add(i_var14);
                            let b2 =
                                ((&raw mut DAT_0070b215) as usize as u32).wrapping_add(i_var14);
                            if r8(b1) != 0x15 && r8(b2) != 0x15 {
                                let ca = DAT_00732e54
                                    .wrapping_add(0x345)
                                    .wrapping_add(u_var19.wrapping_mul(4));
                                let nv: i8;
                                if i_var5 == -1 {
                                    nv = (r8(ca) as i8).wrapping_sub(1);
                                } else {
                                    let u_var11 = FUN_004a7aa0(5);
                                    let hit = if (u_var11 as u16) == u_var6 {
                                        true
                                    } else {
                                        let u_var11 = FUN_004a7aa0(7);
                                        (u_var11 as u16) == u_var6
                                    };
                                    if hit {
                                        nv = (r8(DAT_00732e54
                                            .wrapping_add(0x345)
                                            .wrapping_add(u_var19.wrapping_mul(4)))
                                            as i8)
                                            .wrapping_sub(3);
                                    } else {
                                        nv = (r8(DAT_00732e54
                                            .wrapping_add(0x345)
                                            .wrapping_add(u_var19.wrapping_mul(4)))
                                            as i8)
                                            .wrapping_sub(4);
                                    }
                                }
                                w8(
                                    DAT_00732e54
                                        .wrapping_add(0x345)
                                        .wrapping_add(u_var19.wrapping_mul(4)),
                                    nv as u8,
                                );
                            }
                            let call_it = if (local_16 >> 16) < 2 {
                                (u_var18 as u16) < 2
                            } else {
                                (u_var18 & 1) == 0
                            };
                            if call_it {
                                FUN_0040a3c0(u_var10, i_var5);
                            }
                            let cb = DAT_00732e54
                                .wrapping_add(0x345)
                                .wrapping_add(u_var19.wrapping_mul(4));
                            if (r8(cb) as i8) < 0 {
                                w8(cb, 0);
                            }
                            let i_var14 = u_var10.wrapping_sub(DAT_0071b704).wrapping_mul(8);
                            let u_var8: u16 = if i_var5 == -1 {
                                r16((i_var9 as u32).wrapping_add(0x18))
                            } else {
                                r16(((&raw mut DAT_0071b74a) as usize as u32)
                                    .wrapping_add((i_var5 as u32).wrapping_mul(0x5bd)))
                            };
                            u_var18 = (u_var18 as u16).wrapping_add(1) as u32;
                            w16(i_var14.wrapping_add(4).wrapping_add(DAT_006db798), u_var8);
                            w8(i_var14.wrapping_add(DAT_006db798), i_var5 as u8);
                        }
                    }
                }
                if ((local_16 >> 16) as u16).wrapping_sub(1) == 0 {
                    break;
                }
                u_var13 = (local_16 as u16).wrapping_add(u_stack_c);
                local_16 =
                    ((((local_16 >> 16) as u16).wrapping_sub(1) as u32) << 16) | u_var13 as u32;
            }
            let s_var17 = u_var18 as i16;
            let d = DAT_00732e54;
            if local_12 == 0x8d || local_12 == 0x8e {
                w16(
                    d.wrapping_add(0x246),
                    r16(d.wrapping_add(0x246)).wrapping_add(s_var17 as u16),
                );
                if i_var5 != -1 {
                    let a = ((&raw mut DAT_0071b7b9) as usize as u32)
                        .wrapping_add((i_var5 as u32).wrapping_mul(0x5bd));
                    let v = r16(a).wrapping_add(s_var17 as u16);
                    w16(a, v);
                }
            } else {
                w16(
                    d.wrapping_add(0x242),
                    r16(d.wrapping_add(0x242)).wrapping_add(s_var17 as u16),
                );
                if i_var5 != -1 {
                    let a = ((&raw mut DAT_0071b7b5) as usize as u32)
                        .wrapping_add((i_var5 as u32).wrapping_mul(0x5bd));
                    let v = r16(a).wrapping_add(s_var17 as u16);
                    w16(a, v);
                }
            }
            let ps = DAT_00732e54
                .wrapping_add(0x21c)
                .wrapping_add(p2.wrapping_mul(2));
            let add = (s_var17 as i32).wrapping_mul(0x2f).wrapping_add(2);
            w16(ps, (r16(ps) as i32).wrapping_add(add) as u16);
            let pl = DAT_00732e54
                .wrapping_add(0x220)
                .wrapping_add(p2.wrapping_mul(4));
            w32(
                pl,
                r32(pl)
                    .wrapping_add(u_var18.wrapping_mul(0x2f))
                    .wrapping_add(2),
            );
        } else if i_var5 == DAT_0071ecf4 {
            FUN_004a1d70(0x81, DAT_0071ecf4);
            return;
        }
    }
}
