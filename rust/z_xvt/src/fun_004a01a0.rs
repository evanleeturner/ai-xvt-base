// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d49a0: i32;
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0070ad40: [i8; 0];
    static mut DAT_0070ad49: [u8; 0];
    static mut DAT_0070ad42: [u8; 0];
    static mut DAT_0070b252: [u8; 0];
    static mut DAT_0071b738: [u8; 0];
    static mut DAT_0071b73c: [u8; 0];
    static mut DAT_0071b7a5: [u8; 0];
    static mut DAT_0071b9e9: [u8; 0];
    static mut DAT_0071ba49: [u8; 0];
    static mut DAT_0071e522: [u8; 0];
    static mut DAT_0072f508: [i8; 0];
    static mut DAT_0052b07f: [u8; 0];
    static mut DAT_005247e8: [u8; 0];
    static mut DAT_0052ad08: [u8; 0];
    static mut DAT_00703ca0: [i8; 0];
    static mut DAT_00732800: [i8; 0];
    static mut DAT_00524948: [i8; 0];
    static mut s_stationaryldrpln_00524934: [i8; 0];
    static mut DAT_005250b4: [i8; 0];
    static mut DAT_00525b80: [i8; 0];
    static mut s_formevadeldr1pln_005250a0: [i8; 0];
    static mut s_formevadeflw1pln_00525b6c: [i8; 0];
    static mut s_exithangarpln_00523c08: [i8; 0];
    static mut s_enterhangarpln_00523be4: [i8; 0];
    static mut s_disabledpln_00524914: [i8; 0];
    static mut s_selfdestroypln_00528360: [i8; 0];
    static mut s_Rating_points_awarded___d_to_pla_0052ad48: [i8; 0];
    static mut s_Rating_points_awarded___d_to_pla_0052ad84: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478420(a: u32, b: u32) -> u32;
    fn FUN_004a0880(a: u32, b: u32, c: u32, d: i32, e: i32, f: u32);
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_004a7aa0(a: u32) -> u32;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004a1400(a: u32, b: u32, c: u32);
    fn FUN_004a0cb0(a: u32, b: u32, c: u32, d: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

unsafe fn cmpn(a: u32, b: u32, n: i32) -> bool {
    let mut n = n;
    let mut a = a;
    let mut b = b;
    let mut eq = false;
    loop {
        if n == 0 {
            break;
        }
        n = n.wrapping_sub(1);
        // SAFETY: byte compare of the game's own strings, as the original.
        eq = unsafe { r8(a) == r8(b) };
        a = a.wrapping_add(1);
        b = b.wrapping_add(1);
        if !eq {
            break;
        }
    }
    eq
}

fn tier(v: u16) -> u32 {
    if v < 0xaaaa {
        if v < 0x5999 { (0xccb < v) as u32 } else { 2 }
    } else {
        3
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a01a0(param_1: u32, param_2: u32) {
    // SAFETY: all globals, callees and addresses are the game's own memory;
    // reads and writes happen in the original's order.
    unsafe {
        let mut local_28: [i32; 10] = [0; 10];
        let mut local_4c: u32 = 0;
        let u_var4 = param_2 & 0xffff;
        let i_var2 = u_var4.wrapping_mul(0x23);
        let base = DAT_006d4988 as u32;
        let mut i_var8 = r32(base.wrapping_add(i_var2).wrapping_add(0x1f));
        if i_var8 != 0 {
            local_4c = r32(i_var8.wrapping_add(0xa7));
        }
        let mut t32: u32 = 0;
        if local_4c != 0 {
            t32 = r32(local_4c.wrapping_add(0xcd));
        }
        if local_4c == 0 || t32 == 0 {
            let i_var6 = (param_1 & 0xffff).wrapping_mul(0x23);
            i_var8 = r32(base.wrapping_add(0x1b).wrapping_add(i_var6));
            if i_var8 as i32 != -1 {
                FUN_004a0880(param_2, 0, 3, i_var8 as i32, -1, 0);
                if r8(base.wrapping_add(2).wrapping_add(i_var2)) == 8 {
                    let s32 = r32(base.wrapping_add(0x1b).wrapping_add(i_var6));
                    let p = (&raw mut DAT_0071b7a5 as usize as u32)
                        .wrapping_add(s32.wrapping_mul(0x5bd));
                    let t = r32(p).wrapping_add(4);
                    w32(p, t);
                }
            }
            let idx = (r8(base.wrapping_add(0x16).wrapping_add(i_var6)) as u32).wrapping_mul(0x566);
            FUN_004a0cb0(
                param_2,
                0,
                3,
                r8((&raw mut DAT_0070ad48 as usize as u32).wrapping_add(idx)) as u32,
            );
            return;
        }
        let i_var8 = (r8(base.wrapping_add(i_var2).wrapping_add(0x16)) as u32).wrapping_mul(0x566);
        let local_2c: u32 = (r8((&raw mut DAT_0070ad40 as usize as u32).wrapping_add(i_var8))
            == r8(local_4c.wrapping_add(0xb0))) as u32;
        let mut local_50: i32 = -1;
        let t32a = r32(local_4c.wrapping_add(0xd1));
        let t32b = r32(local_4c.wrapping_add(0xcd));
        let u_var5 = FUN_00478420(t32a, t32b);
        if 0x7fff < (u_var5 as u16) {
            local_50 = r32((&raw mut DAT_0070b252 as usize as u32).wrapping_add(i_var8)) as i32;
        }
        let local_34: u32 = if local_50 == -1 {
            let idx = (r8(base.wrapping_add(0x16).wrapping_add(i_var2)) as u32).wrapping_mul(0x566);
            r8((&raw mut DAT_0070ad49 as usize as u32).wrapping_add(idx)) as u32
        } else {
            r16((&raw mut DAT_0071b738 as usize as u32)
                .wrapping_add((local_50 as u32).wrapping_mul(0x5bd))) as u32
        };
        let mut p11: u32 = &raw mut DAT_0071b73c as usize as u32;
        let mut local_40: i32 = 0;
        let mut local_30: u32 = local_4c.wrapping_add(0x1a1);
        let mut u_var5: u32;
        loop {
            if r8(p11.wrapping_add(4)) != 0 {
                let ta = r32(local_30);
                let tb = r32(local_4c.wrapping_add(0xcd));
                let r = FUN_00478420(ta, tb);
                u_var5 = tier(r as u16);
                if u_var5 != 0 {
                    FUN_004a0880(param_2, local_2c, u_var5, local_40, local_50, local_34);
                    let v0 = r16(p11) as u32;
                    let idx = (r8(base.wrapping_add(0x16).wrapping_add(i_var2)) as u32)
                        .wrapping_mul(0x566);
                    let sel = r8((&raw mut DAT_0070ad48 as usize as u32).wrapping_add(idx)) as u32;
                    let off = (&raw mut DAT_0072f508 as usize as u32)
                        .wrapping_add(v0.wrapping_mul(0x1e5))
                        .wrapping_add(sel);
                    if r8(off) == 0 {
                        let i_var6 = FUN_0042b150(local_4c as i32) as u32;
                        let name = (&raw mut DAT_00703ca0 as usize as u32)
                            .wrapping_add((r8(i_var6.wrapping_add(0xb)) as u32).wrapping_mul(0x55));
                        let matched = cmpn(name, &raw mut DAT_00524948 as usize as u32, 8)
                            || cmpn(
                                name,
                                &raw mut s_stationaryldrpln_00524934 as usize as u32,
                                0x11,
                            )
                            || cmpn(name, &raw mut DAT_005250b4 as usize as u32, 0xc)
                            || cmpn(name, &raw mut DAT_00525b80 as usize as u32, 0xc)
                            || cmpn(
                                name,
                                &raw mut s_formevadeldr1pln_005250a0 as usize as u32,
                                0x11,
                            )
                            || cmpn(
                                name,
                                &raw mut s_formevadeflw1pln_00525b6c as usize as u32,
                                0x11,
                            )
                            || cmpn(name, &raw mut s_exithangarpln_00523c08 as usize as u32, 0xe)
                            || cmpn(
                                name,
                                &raw mut s_enterhangarpln_00523be4 as usize as u32,
                                0xf,
                            )
                            || cmpn(name, &raw mut s_disabledpln_00524914 as usize as u32, 0xc)
                            || cmpn(
                                name,
                                &raw mut s_selfdestroypln_00528360 as usize as u32,
                                0xf,
                            );
                        let mut b17: bool = if matched { local_50 == -1 } else { false };
                        let a1 = ((base & 0xffffff00)
                            | r8(base.wrapping_add(3).wrapping_add(i_var2)) as u32)
                            & 0xffff00ff;
                        let mut u_var7 = FUN_004a7aa0(a1);
                        let mut local_44: u32 = r8((&raw mut DAT_0052b07f as usize as u32)
                            .wrapping_add((u_var7 & 0xffff).wrapping_mul(0xef)))
                            as u32;
                        if local_44 == 0 {
                            b17 = true;
                        }
                        let v1 = r16(p11.wrapping_add(2)) as u32;
                        let i1 = r8((&raw mut DAT_0070ad42 as usize as u32)
                            .wrapping_add(v1.wrapping_mul(0x566)))
                            as u32;
                        u_var7 = FUN_004a7aa0(r8(
                            (&raw mut DAT_005247e8 as usize as u32).wrapping_add(i1)
                        ) as u32);
                        let mut u_var16: u32 = if local_50 == -1 {
                            let k = r8((&raw mut DAT_0070ad49 as usize as u32).wrapping_add(i_var8))
                                as u32;
                            r32((&raw mut DAT_0052ad08 as usize as u32)
                                .wrapping_add(k.wrapping_mul(4)))
                        } else {
                            r16((&raw mut DAT_0071b738 as usize as u32)
                                .wrapping_add((local_50 as u32).wrapping_mul(0x5bd)))
                                as u32
                        };
                        let vm2 = r16(p11.wrapping_sub(4));
                        let mut i_var6: i32 = u_var16.wrapping_sub(vm2 as u32) as i32;
                        let mut go_a = b17;
                        if b17 {
                            u_var16 = 1;
                            if local_44 == 0 {
                                local_44 = 1;
                            }
                        } else if i_var6 < -4 {
                            go_a = true;
                        }
                        let i_var9: i32;
                        let fmt: u32;
                        if go_a {
                            if u_var16 == 0 {
                                u_var16 = 1;
                            }
                            let divisor = r8((&raw mut DAT_0052b07f as usize as u32)
                                .wrapping_add((u_var7 & 0xffff).wrapping_mul(0xef)))
                                as u32;
                            if divisor == 0 {
                                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                            } else {
                                i_var6 = (u_var16.wrapping_mul(local_44) as i32) / (divisor as i32);
                            }
                            if u_var5 == 2 {
                                i_var6 /= 2;
                            } else if u_var5 == 1 {
                                i_var6 /= 10;
                            }
                            if i_var6 == 0 {
                                i_var6 = 1;
                            }
                            let s32 = r32(p11.wrapping_add(0x69));
                            i_var9 = s32.wrapping_add(i_var6 as u32) as i32;
                            w32(p11.wrapping_add(0x69), i_var9 as u32);
                            fmt = &raw mut s_Rating_points_awarded___d_to_pla_0052ad48 as usize
                                as u32;
                        } else {
                            i_var6 = i_var6.wrapping_add(4);
                            let vm2 = r16(p11.wrapping_sub(4));
                            if vm2 < 0xf {
                                if i_var6 < 4 {
                                    i_var6 = 4;
                                }
                            } else if i_var6 < 3 {
                                i_var6 = 3;
                            }
                            let divisor = r8((&raw mut DAT_0052b07f as usize as u32)
                                .wrapping_add((u_var7 & 0xffff).wrapping_mul(0xef)))
                                as u32;
                            if divisor == 0 {
                                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                            } else {
                                i_var6 = (u_var16.wrapping_mul(i_var6 as u32).wrapping_mul(local_44)
                                    as i32)
                                    / (divisor as i32);
                            }
                            if u_var5 == 2 {
                                i_var6 /= 2;
                            } else if u_var5 == 1 {
                                i_var6 /= 10;
                            }
                            let s32 = r32(p11.wrapping_add(0x65));
                            i_var9 = s32.wrapping_add(i_var6 as u32) as i32;
                            w32(p11.wrapping_add(0x65), i_var9 as u32);
                            fmt = &raw mut s_Rating_points_awarded___d_to_pla_0052ad84 as usize
                                as u32;
                        }
                        sprintf(
                            &raw mut DAT_00732800 as *mut i8,
                            fmt as usize as *const i8,
                            i_var6,
                            local_40,
                            i_var9,
                        );
                        if u_var5 == 2 || u_var5 == 3 {
                            let v1 = r16(p11.wrapping_add(2)) as u32;
                            FUN_004a1400(v1, u_var4, u_var5);
                            let v0 = r16(p11) as usize;
                            let ptr = local_28.as_mut_ptr().wrapping_add(v0);
                            let cur = ptr.read();
                            ptr.write(
                                cur.wrapping_add(2u32.wrapping_sub((u_var5 == 2) as u32) as i32),
                            );
                        }
                    }
                }
            }
            p11 = p11.wrapping_add(0x5bd);
            local_30 = local_30.wrapping_add(4);
            local_40 = local_40.wrapping_add(1);
            if (&raw mut DAT_0071e522 as usize as u32).wrapping_add(1) < p11 {
                break;
            }
        }
        u_var5 = 0;
        loop {
            let mut u_var16: u32 = 0;
            let mut u_var7: u32 = 0;
            let mut i_var8: i32 = 0;
            let mut local_48: i32 = 0;
            if 0 < DAT_006d49a0 {
                let mut pb = &raw mut DAT_0070ad48 as usize as u32;
                let mut p12 = local_4c.wrapping_add(0xe1);
                loop {
                    if r8(pb) as u32 == u_var5 {
                        let u_var1 = r32(p12);
                        u_var16 = u_var16.wrapping_add(u_var1);
                        if u_var7 < u_var1 {
                            u_var7 = u_var1;
                            local_48 = i_var8;
                        }
                    }
                    p12 = p12.wrapping_add(4);
                    pb = pb.wrapping_add(0x566);
                    i_var8 = i_var8.wrapping_add(1);
                    if !(i_var8 < DAT_006d49a0) {
                        break;
                    }
                }
            }
            let tb = r32(local_4c.wrapping_add(0xcd));
            let r = FUN_00478420(u_var16, tb);
            let u_var7 = tier(r as u16);
            if u_var7 != 0 {
                FUN_004a0cb0(param_2, local_2c, u_var7, u_var5);
            }
            if (u_var7 == 2 || u_var7 == 3) && local_28[u_var5 as usize] < 2 {
                let i_var8 =
                    2u32.wrapping_sub((u_var7 == 2) as u32)
                        .wrapping_sub(local_28[u_var5 as usize] as u32) as i32;
                if i_var8 == 2 {
                    FUN_004a1400(local_48 as u32, u_var4, 3);
                    if local_50 != -1 {
                        let p = (&raw mut DAT_0071b9e9 as usize as u32)
                            .wrapping_add((local_48 as u32).wrapping_mul(2))
                            .wrapping_add((local_50 as u32).wrapping_mul(0x5bd));
                        let t = r16(p).wrapping_add(1);
                        w16(p, t);
                    }
                } else if i_var8 == 1 {
                    FUN_004a1400(local_48 as u32, u_var4, 2);
                    if local_50 != -1 {
                        let p = (&raw mut DAT_0071ba49 as usize as u32)
                            .wrapping_add((local_48 as u32).wrapping_mul(2))
                            .wrapping_add((local_50 as u32).wrapping_mul(0x5bd));
                        let t = r16(p).wrapping_add(1);
                        w16(p, t);
                    }
                }
            }
            u_var5 = u_var5.wrapping_add(1);
            if !((u_var5 as i32) < 10) {
                break;
            }
        }
    }
}
