// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: *mut u8;
    static mut DAT_006d9d56: *mut u8;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4980: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00732904: i32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d9d70: u16;
    static mut DAT_006da9f0: u16;
    static mut DAT_00732e40: u16;
    static mut DAT_0071b704: u16;
    static mut DAT_006db580: u32;
    static mut DAT_00525cd0: [u32; 0];
    static mut DAT_00525cdc: [u8; 0];
    static mut DAT_00525bf7: [i8; 0];
    static mut DAT_00525a7e: [u16; 0];
    static mut DAT_0052b0b4: [u8; 0];
    static mut DAT_00703ca0: [i8; 0];
    static mut s_disableldr1pln_00525028: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00429a20(a: u32) -> u8;
    fn FUN_0042a140(a: u32, b: u32);
    fn FUN_004440e0(a: u32) -> u32;
    fn FUN_00478320(a: u32, b: u32) -> u32;
    fn FUN_004490f0(a: u32) -> i32;
    fn FUN_00449790(a: u32, b: u32) -> u32;
    fn FUN_00422ab0(a: u32) -> u32;
    fn FUN_00447740(a: u32, b: u32);
}

unsafe fn r16(p: *const u8) -> u16 {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn rp(p: *const u8) -> *mut u8 {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.cast::<*mut u8>().read_unaligned() }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

fn ad(p: *mut u8, o: u32) -> *mut u8 {
    p.wrapping_add(o as usize)
}

unsafe fn rb(p: *const u8) -> u8 {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.read() }
}

unsafe fn scan_eq(mut a: *const u8, mut b: *const u8, init: bool) -> bool {
    let mut z = init;
    let mut n: i32 = 0xf;
    loop {
        if n == 0 {
            break;
        }
        n -= 1;
        // SAFETY: both pointers address the game's NUL-padded name strings.
        unsafe {
            z = a.read() == b.read();
        }
        a = a.wrapping_add(1);
        b = b.wrapping_add(1);
        if !z {
            break;
        }
    }
    z
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0044cdb0() -> u32 {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed in the same order and widths as the original.
    unsafe {
        let mut local_28 = [0u32; 10];
        let mut local_38: u32;
        let mut local_2c: u32 = 0;
        let mut local_46: u16 = 0;
        let mut local_44: u16 = 0;
        let mut u_stack_3c: u16;
        let mut local_3a: u16;
        let mut psv6: *mut u8 = core::ptr::null_mut();
        let name_a = (&raw mut DAT_00703ca0).cast::<u8>();
        let name_b = (&raw mut s_disableldr1pln_00525028).cast::<u8>();
        let t_cd0 = (&raw mut DAT_00525cd0).cast::<u32>();
        let t_cdc = (&raw mut DAT_00525cdc).cast::<u8>();
        let t_bf7 = (&raw mut DAT_00525bf7).cast::<i8>();
        let t_a7e = (&raw mut DAT_00525a7e).cast::<u16>();
        let t_0b4 = (&raw mut DAT_0052b0b4).cast::<u8>();

        if r16(ad(DAT_00732e54, 0x1df)) as i16 == 0 {
            return (DAT_00732e54 as usize as u32) & 0xffff0000;
        }
        let mut u_var19: u16 = r16(ad(DAT_006d9d56, 0x18));
        local_28[0] &= 0xffff0000;
        let psv5: i32 = u_var19 as i32;
        let b_var7 = FUN_00429a20(psv5 as u32);
        if b_var7 == 0 {
            local_38 = DAT_00732e54 as usize as u32;
            u_var19 = 0;
            if rb(ad(DAT_00732e54, 0x211)) != 0 {
                loop {
                    *ad(DAT_00732e54, (u_var19 as u32).wrapping_add(0x216)) = 0;
                    local_38 = u_var19.wrapping_add(1) as u32;
                    u_var19 = u_var19.wrapping_add(1);
                    if !(local_38 < rb(ad(DAT_00732e54, 0x211)) as u32) {
                        break;
                    }
                }
            }
        } else {
            let mut u_var20: u32 = *t_cd0.add(DAT_006d9d70 as usize);
            if psv5 < DAT_00732a38 {
                let rec = ad(DAT_006d4988, psv5.wrapping_mul(0x23) as u32);
                let pu = rp(ad(rec, 0x1f));
                if !pu.is_null() {
                    let mut u16a = r16(ad(pu, 0x79));
                    if 0x19 < u16a {
                        u16a = r16(ad(
                            DAT_006d4988,
                            (DAT_006d9d50 as u32).wrapping_mul(0x23).wrapping_add(0x10),
                        ));
                        let u16b = r16(ad(rec, 0x10));
                        let mut u_var8 = u16a.wrapping_sub(u16b);
                        if 0x7fff < u_var8 {
                            u_var8 = 0u16.wrapping_sub(u_var8);
                        }
                        if u_var8 < 0x2000 {
                            u_var20 = u_var20.wrapping_sub(0x4000);
                        } else if u_var8 < 0x5000 {
                            u_var20 = u_var20.wrapping_sub(0x2000);
                        }
                    }
                    if rb(ad(rec, 2)) == 5 || rb(ad(rec, 2)) == 4 {
                        u_var20 = u_var20.wrapping_add(0x6000);
                    }
                }
            }
            FUN_0042a140(DAT_006d9d50 as u32, psv5 as u32);
            let mut u16a = r16(ad(
                DAT_006d4988,
                (DAT_006d9d50 as u32).wrapping_mul(0x23).wrapping_add(0x10),
            ));
            u_stack_3c = DAT_006da9f0.wrapping_sub(u16a);
            if 0x7fff < u_stack_3c {
                u_stack_3c = 0u16.wrapping_sub(u_stack_3c);
            }
            u16a = r16(ad(DAT_00732e54, 0xd));
            local_3a = DAT_00732e40.wrapping_sub(u16a);
            if 0x7fff < local_3a {
                local_3a = 0u16.wrapping_sub(local_3a);
            }
            let b_var14: u8;
            if u_stack_3c < 0x800 && local_3a < 0x800 && DAT_006db580 < u_var20 {
                let mut b = 3u8;
                if 0x1fff < (DAT_006db580 as i32) {
                    b = (((DAT_006db580 as i32) < 0x4000) as u8).wrapping_add(1);
                }
                local_28[0] =
                    (local_28[0] & 0xffff0000) | (*t_cdc.add(DAT_006d9d70 as usize) as u32);
                b_var14 = b;
            } else {
                b_var14 = 0;
            }
            let b_var1 = rb(ad(DAT_00732e54, 0x211));
            let b_var2 = rb(ad(DAT_006d9d56, 0xb));
            let u_var15: u16 = b_var2 as u16;
            let mut u_var8: u16 = 0;
            if b_var1 != 0 {
                loop {
                    let mut b_var10: u8 = 0;
                    if b_var14 != 0 {
                        b_var10 = b_var14;
                        if rb(ad(DAT_00732e54, (u_var8 as u32).wrapping_add(0x214))) as i8 == -0x73
                        {
                            let z = scan_eq(
                                ad(name_a, (u_var15 as u32).wrapping_mul(0x55)),
                                name_b,
                                u_var15 == 0,
                            );
                            if !z || rb(ad(DAT_006d9d56, 0x1e)) != 1 {
                                b_var10 = 0;
                            }
                        } else if rb(ad(DAT_006d9d56, 0x1e)) != 0 {
                            let mut n: i32 = 0xf;
                            let mut lt = false;
                            let mut i_var16: i32 = 0;
                            let mut eq = true;
                            let mut pa =
                                ad(name_a, (u_var15 as u32).wrapping_mul(0x55)) as *const u8;
                            let mut pb = name_b as *const u8;
                            loop {
                                if n == 0 {
                                    break;
                                }
                                n -= 1;
                                lt = pa.read() < pb.read();
                                eq = pa.read() == pb.read();
                                pa = pa.wrapping_add(1);
                                pb = pb.wrapping_add(1);
                                if !eq {
                                    break;
                                }
                            }
                            if !eq {
                                i_var16 = 1i32
                                    .wrapping_sub(lt as i32)
                                    .wrapping_sub((lt as i32 != 0) as i32);
                            }
                            b_var10 = ((((i_var16 == 0) as u32).wrapping_sub(1)) & (b_var14 as u32))
                                as u8;
                        }
                    }
                    let u_var18 = u_var8.wrapping_add(1);
                    *ad(DAT_00732e54, (u_var8 as u32).wrapping_add(0x216)) = b_var10;
                    *ad(DAT_00732e54, (u_var8 as u32).wrapping_add(0x218)) = local_28[0] as u8;
                    u_var8 = u_var18;
                    if !(u_var18 < b_var1 as u16) {
                        break;
                    }
                }
            }
            local_38 = psv5 as u32;
            let mut cond = psv5 < DAT_006d4980;
            if cond {
                local_38 = FUN_004440e0(u_var19 as u32);
                psv6 = DAT_00732e54;
                cond = (local_38 as i8) == 0;
            }
            if cond {
                let i_var11: i32 = psv5.wrapping_mul(0x23);
                let rec = ad(DAT_006d4988, i_var11 as u32);
                let mut c_var3: i8 =
                    rb(ad(DAT_006d4988, (2i32.wrapping_add(i_var11)) as u32)) as i8;
                if c_var3 == 4 || c_var3 == 5 || c_var3 == 3 {
                    c_var3 = 2;
                    local_44 = 6;
                    local_46 = 0x10;
                    let m: u32 = if DAT_006d9d70 == 2 { 0xffff_ffff } else { 0 };
                    local_28[0] = (m & 0x9f12).wrapping_add(0x31b5a);
                } else {
                    local_28[0] = 0x18dad;
                    local_46 = 2;
                    c_var3 = 1;
                    local_44 = 1;
                }
                local_38 = 0;
                let pu = rp(ad(rec, 0x1f));
                let pq = rp(ad(pu, 0xa7));
                local_2c = (r16(ad(pq, 0x20b)) as i16) as i32 as u32;
                let z = scan_eq(
                    ad(name_a, (u_var15 as u32).wrapping_mul(0x55)),
                    name_b,
                    u_var15 == 0,
                );
                if z {
                    let u16a = r16(ad(pq, 199));
                    u_var20 = (u16a as u32) / 10;
                    let u16b = r16(ad(pq, 0xc3));
                    if (u_var20 as u16) < u16b {
                        local_2c = local_2c.wrapping_add(u_var20);
                    }
                    u_var20 = 0;
                    if rb(ad(DAT_00732e54, 0x228)) != 0 {
                        loop {
                            let u_var13 = rb(ad(DAT_00732e54, u_var20.wrapping_add(0x229))) as u32;
                            if *t_bf7.add(u_var13 as usize) == c_var3 {
                                let mut u_v8 = *t_a7e.add(u_var13 as usize);
                                let c_var4 = rb(ad(rec, 2)) as i8;
                                if c_var4 == 4 || c_var4 == 5 {
                                    u_v8 >>= 4;
                                }
                                if c_var4 == 3 {
                                    u_v8 >>= 2;
                                }
                                local_38 = local_38.wrapping_add(u_v8 as u32);
                            }
                            u_var20 = (u_var20 as u16).wrapping_add(1) as u32;
                            if !(u_var20 < rb(ad(DAT_00732e54, 0x228)) as u32) {
                                break;
                            }
                        }
                    }
                } else {
                    let u16a = r16(ad(pq, 199));
                    let u16b = r16(ad(pq, 0xc3));
                    local_2c = local_2c.wrapping_add((u16a as u32).wrapping_sub(u16b as u32));
                }
                let mut u_var13: u32 = DAT_0071b704 as u32;
                let mut u_v8: u16 = DAT_0071b704;
                u_var20 = u_var13;
                while (u_var20 as i32) < DAT_00732904 {
                    let i_var17 = ad(DAT_006d4988, (u_v8 as u32).wrapping_mul(0x23));
                    let b_v14 = rb(ad(i_var17, 3));
                    if 0x88 < b_v14 {
                        let pu = rp(ad(i_var17, 0x1f));
                        let pq = rp(ad(pu, 0xa3));
                        let u16a = r16(ad(pq, 4));
                        if rb(ad(pq, 1)) != 0 && u16a == u_var19 {
                            let c_var4 = rb(ad(rec, 2)) as i8;
                            let mut u_v15 = *t_a7e.add(b_v14 as usize);
                            if c_var4 == 4 || c_var4 == 5 {
                                u_v15 >>= 4;
                            }
                            if c_var4 == 3 {
                                u_v15 >>= 2;
                            }
                            local_38 = local_38.wrapping_add(u_v15 as u32);
                        }
                    }
                    u_v8 = u_v8.wrapping_add(1);
                    u_var20 = u_v8 as u32;
                }
                if local_38 < local_2c {
                    local_38 &= 0xffff0000;
                    u_v8 = DAT_0071b704;
                    while (u_var13 as i32) < DAT_00732904 {
                        let i_var16 = ad(DAT_006d4988, (u_v8 as u32).wrapping_mul(0x23));
                        let b_v14 = rb(ad(i_var16, 3));
                        if b_v14 != 0 && *t_bf7.add(b_v14 as usize) == c_var3 {
                            let pu = rp(ad(i_var16, 0x1f));
                            let pq = rp(ad(pu, 0xa3));
                            let u16a = r16(ad(pq, 4));
                            if rb(ad(pq, 1)) != 0 && u16a == u_var19 {
                                local_38 = (local_38 & 0xffff0000)
                                    | ((local_38 as u16).wrapping_add(1) as u32);
                            }
                        }
                        u_v8 = u_v8.wrapping_add(1);
                        u_var13 = u_v8 as u32;
                    }
                    let mut c2 = (local_38 as u16) < local_46;
                    if c2 {
                        local_38 = DAT_006d9d56 as usize as u32;
                        c2 = rb(ad(DAT_006d9d56, 0x34)) as i8 == 0x17;
                    }
                    if c2 {
                        local_38 = DAT_00732e54 as usize as u32;
                        c2 = r16(ad(DAT_00732e54, 0x1e1)) as i16 == 0;
                    }
                    if c2 {
                        let u32a = r32(ad(DAT_00732e54, 0x1d));
                        if u32a == 0 {
                            local_38 = local_44 as u32;
                            if (rb(ad(DAT_00732e54, 0x78)) as u32) < local_38 {
                                if u_stack_3c < 0x300
                                    && local_3a < 0x300
                                    && DAT_006db580 < local_28[0]
                                {
                                    u_var19 =
                                        ((DAT_006d9d70 as u32).wrapping_add(1).wrapping_mul(0x1d8))
                                            as u16;
                                    if DAT_006d9d70 == 2 {
                                        let v = r32(ad(DAT_006d9d56, 0xe));
                                        let mut u16a = r16(ad(DAT_00732e54, 0x231));
                                        u16a = u16a.wrapping_add(v as u16);
                                        w16(ad(DAT_00732e54, 0x231), u16a);
                                        let a = r16(ad(DAT_00732e54, 0xc5));
                                        let b = r16(ad(DAT_00732e54, 0xc3));
                                        if a <= b {
                                            u_var19 = 0x2c4;
                                        }
                                    } else {
                                        let v = r32(ad(DAT_006d9d56, 0xe));
                                        let r = FUN_00478320(v, 0xc000);
                                        let mut u16a = r16(ad(psv6, 0x231));
                                        u16a = u16a.wrapping_add(r as u16);
                                        w16(ad(psv6, 0x231), u16a);
                                    }
                                    local_38 = DAT_00732e54 as usize as u32;
                                    let s = r16(ad(DAT_00732e54, 0x231)) as i16;
                                    let mut c3 = (u_var19 as i32) <= (s as i32);
                                    if c3 {
                                        u_var19 = 0;
                                        c3 = rb(ad(DAT_00732e54, 0x228)) != 0;
                                    }
                                    if c3 {
                                        loop {
                                            psv6 = DAT_00732e54;
                                            let u20 = u_var19 as u32;
                                            let s = r16(ad(
                                                DAT_00732e54,
                                                u20.wrapping_mul(2).wrapping_add(0x22d),
                                            ))
                                                as i16;
                                            if s == 0
                                                && *t_bf7.add(rb(ad(
                                                    DAT_00732e54,
                                                    u20.wrapping_add(0x229),
                                                ))
                                                    as usize)
                                                    == c_var3
                                            {
                                                let a = r16(ad(DAT_00732e54, 0xc5));
                                                let b = r16(ad(DAT_00732e54, 0xc3));
                                                if DAT_006d9d70 == 2 && a <= b {
                                                    *ad(DAT_00732e54, u20.wrapping_add(0x22b)) = 3;
                                                } else {
                                                    let v13 = *t_0b4.add(
                                                        u20.wrapping_add(
                                                            (rb(ad(DAT_00732e54, 4)) as u32)
                                                                .wrapping_mul(0xef),
                                                        )
                                                            as usize,
                                                    )
                                                        as u32;
                                                    let b14 = rb(ad(
                                                        DAT_00732e54,
                                                        v13.wrapping_mul(4).wrapping_add(0x346),
                                                    ));
                                                    let b1 = rb(ad(
                                                        DAT_00732e54,
                                                        v13.wrapping_mul(4).wrapping_add(0x34a),
                                                    ));
                                                    *ad(DAT_00732e54, u20.wrapping_add(0x22b)) = 1;
                                                    if b14 < b1 {
                                                        *ad(psv6, u20.wrapping_add(0x22b)) = 0x81;
                                                    }
                                                }
                                                let mut u8v: u16 = 0;
                                                local_28[0] &= 0xffffff00;
                                                if psv5 < DAT_006d4980 {
                                                    let mut u13: u32 = 0;
                                                    let rt = rb(ad(
                                                        DAT_006d4988,
                                                        (3i32.wrapping_add(i_var11)) as u32,
                                                    ))
                                                        as u32;
                                                    let i16v = FUN_004490f0(rt);
                                                    if 0 < i16v {
                                                        loop {
                                                            let rt2 = rb(ad(
                                                                DAT_006d4988,
                                                                (3i32.wrapping_add(i_var11)) as u32,
                                                            ))
                                                                as u32;
                                                            let u9 = FUN_00449790(rt2, u13);
                                                            if (u9 as i16) == 1 || (u9 as i16) == 3
                                                            {
                                                                *local_28
                                                                    .as_mut_ptr()
                                                                    .cast::<u8>()
                                                                    .add(u8v as usize) = u13 as u8;
                                                                u8v = u8v.wrapping_add(1);
                                                            }
                                                            u13 =
                                                                (u13 as u16).wrapping_add(1) as u32;
                                                            if !((u13 as i32) < i16v) {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                    u8v = FUN_00422ab0(u8v as u32) as u16;
                                                    let val = *local_28
                                                        .as_mut_ptr()
                                                        .cast::<u8>()
                                                        .add(u8v as usize)
                                                        as u16;
                                                    w16(ad(DAT_006d9d56, 0x1c), val);
                                                } else {
                                                    w16(ad(DAT_006d9d56, 0x1c), 0);
                                                }
                                                FUN_00447740(DAT_006d9d50 as u32, u20);
                                                let p78 = ad(DAT_00732e54, 0x78);
                                                *p78 = (*p78).wrapping_add(1);
                                                let c4 = rb(ad(
                                                    DAT_006d4988,
                                                    (2i32.wrapping_add(i_var11)) as u32,
                                                ))
                                                    as i8;
                                                if c4 == 0 || c4 == 1 {
                                                    w16(ad(DAT_00732e54, 0x231), 0);
                                                }
                                            }
                                            u_var19 = u_var19.wrapping_add(1);
                                            if !(u_var19 < rb(ad(DAT_00732e54, 0x228)) as u16) {
                                                break;
                                            }
                                        }
                                        return 0;
                                    }
                                } else {
                                    let v = r32(ad(DAT_006d9d56, 0xe));
                                    let mut u16a = r16(ad(DAT_00732e54, 0x231));
                                    u16a = u16a.wrapping_sub(v as u16);
                                    w16(ad(DAT_00732e54, 0x231), u16a);
                                    local_38 = ad(DAT_00732e54, 0x231) as usize as u32;
                                    if (u16a as i16) < 0 {
                                        w16(ad(DAT_00732e54, 0x231), 0);
                                        return local_38 & 0xffff0000;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        local_38 & 0xffff0000
    }
}
