// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00b69e38: i32;
    static mut DAT_00b6a2c0: i32;
    static mut DAT_00aa477b: i8;
    static mut DAT_00b69d20: i8;
    static mut DAT_00bb2810: i32;
    static mut DAT_00a91d18: u8;

    static mut DAT_0066d918: i32;
    static mut DAT_0066da70: i32;
    static mut DAT_0066da6c: i32;
    static mut DAT_0066da50: i32;
    static mut DAT_0066da54: i32;
    static mut DAT_0066da60: i32;
    static mut DAT_0066da64: i32;
    static mut DAT_0066d9e8: i32;
    static mut DAT_0066d958: i32;
    static mut DAT_0066da40: i32;
    static mut DAT_0066d948: i32;
    static mut DAT_0066d8c8: i32;
    static mut DAT_0066d908: i32;
    static mut DAT_0066d9b4: i32;
    static mut DAT_0066d9b0: i32;
    static mut DAT_0066d9e4: i32;
    static mut DAT_0066d9f4: i32;
    static mut DAT_0066da4c: i32;
    static mut DAT_0066d904: i32;
    static mut DAT_0066d900: i32;

    static mut DAT_00b6d5aa: i8;
    static mut DAT_00b6d86a: i8;
    static mut DAT_00b6a5aa: i8;
    static mut DAT_00b6a5a6: i8;
    static mut DAT_00b6a616: i8;
    static mut DAT_00b6da12: i8;
    static mut DAT_00b6da1a: i8;
    static mut DAT_00b6d962: i8;
    static mut DAT_00b6c13e: i8;
    static mut DAT_00b6c2ce: i8;
    static mut DAT_00b6bcc2: i8;
    static mut DAT_00b6bce2: i8;
    static mut DAT_00b6be82: i8;
    static mut DAT_00b6bea2: i8;
    static mut DAT_00b6ca9e: i8;
    static mut DAT_00b6cf4e: i8;
    static mut DAT_00b6d2d2: i8;
    static mut DAT_00b6d416: i8;
    static mut DAT_00b6d542: i8;
    static mut DAT_00b6d84e: i8;
    static mut DAT_00b6d856: i8;
    static mut DAT_00b6d5a6: i8;
    static mut DAT_00b6d5ae: i8;
    static mut DAT_00b6d5b2: i8;
    static mut DAT_00b6bc32: i8;
    static mut DAT_00b6c126: i8;
    static mut DAT_00b6d3aa: i8;
    static mut DAT_00b6d3b6: i8;
    static mut DAT_00b6d3c2: i8;
    static mut DAT_00b6d3ce: i8;
    static mut DAT_00b6d3da: i8;
    static mut DAT_00b6d3e6: i8;
    static mut DAT_00b6d3f2: i8;
    static mut DAT_00b6d3fe: i8;
    static mut DAT_00b6d40a: i8;

    static mut s__c_s__c_s__c_s__c_d__c_s_0052d4e8: i8;
    static mut s__c_s__c_s_0052a6b0: i8;
    static mut s__c_s__c_s__s__0052d4d8: i8;
    static mut s__c_s__c_d_0052c518: i8;
    static mut s_rank_d_0052b4e0: i8;
    static mut s_rcitlvl_d_0052b4b4: i8;
    static mut s_citlvl_d_0052b4a8: i8;
    static mut s_medlvl_d_0052b49c: i8;
    static mut s__s____s_0052d4d0: i8;
    static mut s__d___d__0052b464: i8;
    static mut s__d__d__0052b450: i8;
    static mut DAT_0052b46c: i8;
    static mut DAT_0052b460: i8;
    static mut DAT_0052b458: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d6460(rect: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(id: u32) -> *mut i8;
    fn FUN_004db640(a: i32, s: *mut i8, rect: *mut i32, col: u32);
    fn FUN_004d11e0() -> i32;
    fn FUN_004d9d70(rect: *mut i32, a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004db4e0(a: i32, s: *mut i8, x: i32, y: i32, col: u32);
    fn FUN_004c9bf0(a: i32);
    fn FUN_004b5570(s: *mut i8, x: i32, y: i32);
    fn FUN_004dbfa0(s: *mut i8, a: i32) -> i32;
}

unsafe fn ld(p: *const i8) -> i32 {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn ldi(p: *const i8, n: i32) -> i32 {
    // SAFETY: p points into the game's own memory; offset is n dwords.
    unsafe { ld(p.byte_offset(n.wrapping_mul(4) as isize)) }
}

unsafe fn in_(x: i32) -> bool {
    // SAFETY: reads the game's global.
    unsafe { DAT_0066da6c <= x && ((x as u32).wrapping_sub(DAT_0066da6c as u32) as i32) < 0x15 }
}

unsafe fn addto(p: *mut i32, off: i32, v: i32) {
    // SAFETY: p plus off bytes is a game global dword.
    unsafe {
        let q = p.byte_offset(off as isize);
        q.write(q.read().wrapping_add(v));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "stdcall" fn FUN_00501710() -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers walk the
    // game's tables exactly as the original does.
    unsafe {
        let buf: *mut i8 = &raw mut DAT_00b69d20;
        let mut local_24: [i32; 4] = [0; 4];
        let rect = local_24.as_mut_ptr();
        let mut local_14: [i8; 20] = [0; 20];
        let mut local_28: i32;
        let mut i_var8: i32;
        let mut i_var9: i32;
        let mut i_var10: i32 = 0;
        let mut i_var11: i32;
        let mut u_var19: u32;

        FUN_004d6460(rect, 0x54, 0x5a, 0x1b2, 0x6a);
        let p2 = FUN_004ddc10(0x138);
        FUN_004db640(0xf, p2, rect, 0xffff);
        local_28 = 0;
        if DAT_00b69e30 != 2 {
            let mut p12: *mut i8 = &raw mut DAT_00b6d5aa;
            loop {
                let r = FUN_004d11e0();
                if r == ld(p12) {
                    break;
                }
                p12 = p12.byte_add(0x58);
                local_28 += 1;
                if !(p12 < &raw mut DAT_00b6d86a) {
                    break;
                }
            }
        }
        if local_28 == 8 {
            return 0;
        }
        if DAT_0066d918 != 0 {
            DAT_0066da70 = 0xd;
            DAT_0066da6c = 0;
            DAT_0066d918 = 0;
            if DAT_00aa477b == 0 && ld(&raw mut DAT_00b6a5aa) != 1 && ld(&raw mut DAT_00b6a5aa) != 2
            {
                DAT_0066da70 = 0xe;
            }
            if DAT_00b69e30 != 2 {
                DAT_0066da70 = DAT_0066da70.wrapping_add(5);
            }
            if ld(&raw mut DAT_00b6a5aa) == 1 {
                DAT_0066da70 = DAT_0066da70.wrapping_add(1);
            }
            if ld(&raw mut DAT_00b6a616) != 0 {
                DAT_0066da70 = DAT_0066da70.wrapping_add(1);
            }
            let mut n = 4;
            let off = ld(&raw mut DAT_00b6d962)
                .wrapping_mul(0x4278)
                .wrapping_mul(4);
            let mut p12: *mut i8 = (&raw mut DAT_00b6da12).byte_offset(off as isize);
            loop {
                if ld(p12) != 0 {
                    DAT_0066da70 = DAT_0066da70.wrapping_add(1);
                }
                p12 = p12.byte_add(4);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
            DAT_0066da50 = 0;
            DAT_0066da60 = 0;
            DAT_0066d9e8 = 0;
            DAT_0066d958 = 0;
            DAT_0066da40 = 0;
            DAT_0066d948 = 0;
            DAT_0066d8c8 = 0;
            DAT_0066d908 = 0;
            DAT_0066d9b4 = 0;
            let mut p12: *mut i8 = &raw mut DAT_00b6c13e;
            loop {
                i_var8 = 1;
                DAT_0066d9f4 = 0;
                let mut p6 = p12;
                loop {
                    if ld(p6) != 0 || ldi(p6, 300) != 0 {
                        DAT_0066d9f4 = 1;
                        DAT_0066d9b4 = 1;
                    }
                    p6 = p6.byte_add(400);
                    i_var8 -= 1;
                    if i_var8 == 0 {
                        break;
                    }
                }
                if DAT_0066d9f4 != 0 {
                    DAT_0066da70 = DAT_0066da70.wrapping_add(1);
                }
                p12 = p12.byte_add(4);
                if !(p12 < &raw mut DAT_00b6c2ce) {
                    break;
                }
            }
            if DAT_0066d9b4 != 0 {
                DAT_0066da70 = DAT_0066da70.wrapping_add(2);
            }
            DAT_0066d9b0 = 0;
            if DAT_00b69e30 != 2 {
                let mut p12: *mut i8 = &raw mut DAT_00b6bcc2;
                loop {
                    if ld(p12) != 0 || ldi(p12, 8) != 0 {
                        DAT_0066d9b0 = 1;
                        DAT_0066da70 = DAT_0066da70.wrapping_add(1);
                    }
                    p12 = p12.byte_add(4);
                    if !(p12 < &raw mut DAT_00b6bce2) {
                        break;
                    }
                }
                if DAT_0066d9b0 != 0 {
                    DAT_0066da70 = DAT_0066da70.wrapping_add(2);
                }
            }
            DAT_0066d9e4 = 0;
            if DAT_00b69e30 != 2 {
                let mut p12: *mut i8 = &raw mut DAT_00b6be82;
                loop {
                    if ld(p12) != 0 || ldi(p12, 8) != 0 {
                        DAT_0066d9e4 = 1;
                        DAT_0066da70 = DAT_0066da70.wrapping_add(1);
                    }
                    p12 = p12.byte_add(4);
                    if !(p12 < &raw mut DAT_00b6bea2) {
                        break;
                    }
                }
                if DAT_0066d9e4 != 0 {
                    DAT_0066da70 = DAT_0066da70.wrapping_add(2);
                }
            }
            i_var8 = 0;
            let mut p12: *mut i8 = &raw mut DAT_00b6ca9e;
            loop {
                i_var9 = 100;
                loop {
                    let a = ld(p12);
                    let b = ldi(p12, -300);
                    p12 = p12.byte_add(4);
                    addto(&raw mut DAT_0066d908, i_var8, a);
                    addto(&raw mut DAT_0066d8c8, i_var8, b);
                    i_var9 -= 1;
                    if i_var9 == 0 {
                        break;
                    }
                }
                i_var8 += 4;
                if !(i_var8 < 4) {
                    break;
                }
            }
            i_var8 = 0;
            let mut p12: *mut i8 = &raw mut DAT_00b6cf4e;
            loop {
                i_var9 = 0x19;
                loop {
                    let a = ld(p12);
                    let b = ldi(p12, 0x4b);
                    p12 = p12.byte_add(4);
                    addto(&raw mut DAT_0066d948, i_var8, a);
                    addto(&raw mut DAT_0066da40, i_var8, b);
                    i_var9 -= 1;
                    if i_var9 == 0 {
                        break;
                    }
                }
                i_var8 += 4;
                if !(i_var8 < 4) {
                    break;
                }
            }
            i_var8 = 0;
            let mut p12: *mut i8 = &raw mut DAT_00b6d2d2;
            loop {
                i_var9 = 6;
                loop {
                    let a = ld(p12);
                    let b = ldi(p12, 0x12);
                    p12 = p12.byte_add(4);
                    addto(&raw mut DAT_0066d958, i_var8, a);
                    addto(&raw mut DAT_0066d9e8, i_var8, b);
                    i_var9 -= 1;
                    if i_var9 == 0 {
                        break;
                    }
                }
                i_var8 += 4;
                if !(i_var8 < 4) {
                    break;
                }
            }
            let mut p7: *mut i32 = &raw mut DAT_0066da60;
            let mut p6: *mut i8 = &raw mut DAT_00b6d416;
            loop {
                i_var8 = 0x19;
                loop {
                    let a = ld(p6);
                    p6 = p6.byte_add(4);
                    p7.write(p7.read().wrapping_add(a));
                    i_var8 -= 1;
                    if i_var8 == 0 {
                        break;
                    }
                }
                p7 = p7.add(1);
                if !(p7 < &raw mut DAT_0066da64) {
                    break;
                }
            }
            let mut p13: *mut i32 = &raw mut DAT_0066da50;
            let mut p12: *mut i8 = &raw mut DAT_00b6d542;
            loop {
                i_var8 = 6;
                loop {
                    let a = ld(p12);
                    p12 = p12.byte_add(4);
                    p13.write(p13.read().wrapping_add(a));
                    i_var8 -= 1;
                    if i_var8 == 0 {
                        break;
                    }
                }
                p13 = p13.add(1);
                if !(p13 < &raw mut DAT_0066da54) {
                    break;
                }
            }
        }
        FUN_004d6460(rect, 0x1a9, 0x6b, 0x1b2, 0x1b1);
        if 0x15 < DAT_0066da70 {
            DAT_0066da6c = FUN_004d9d70(rect, DAT_0066da6c, DAT_0066da70, 0, 5, DAT_00bb2810, 10);
        }
        i_var8 = 0x6f;
        if DAT_00b69e30 != 2 && ld(&raw mut DAT_00b6a5aa) == 1 {
            if DAT_0066da6c < 1 && -0x15 < DAT_0066da6c {
                u_var19 = if DAT_0066da4c == 0 { 0x2b3 } else { 0x170 };
                let p2 = FUN_004ddc10(u_var19);
                let u_var17: u32 = 4;
                let u_var15: u32 = 1;
                let u_var18: u32 = DAT_0066d904 as u32;
                let p3 = FUN_004ddc10(0x14f);
                let u_var14: u32 = 4;
                let p4 = FUN_004ddc10(DAT_0066d900.wrapping_add(0x13e) as u32);
                let u_var13: u32 = 1;
                let p5 = FUN_004ddc10(0x16f);
                _sprintf(
                    buf,
                    &raw const s__c_s__c_s__c_s__c_d__c_s_0052d4e8,
                    4i32,
                    p5,
                    u_var13,
                    p4,
                    u_var14,
                    p3,
                    u_var15,
                    u_var18,
                    u_var17,
                    p2,
                );
                i_var8 = 0x7e;
                FUN_004db4e0(0xc, buf, 0x58, 0x6f, 0xffff);
            }
            i_var10 = 1;
        }
        i_var9 = i_var8;
        'lab: {
            if DAT_00aa477b != 0 || ld(&raw mut DAT_00b6a5aa) == 1 || ld(&raw mut DAT_00b6a5aa) == 2
            {
                break 'lab;
            }
            if in_(i_var10) {
                u_var19 = ldi(
                    &raw mut DAT_00b6d84e,
                    ld(&raw mut DAT_00b6a5a6).wrapping_mul(7),
                ) as u32;
                let u_var7: u32 = if ld(&raw mut DAT_00b6a5aa) == 0 {
                    (u_var19 == 0) as u32
                } else {
                    ldi(
                        &raw mut DAT_00b6d84e,
                        (ld(&raw mut DAT_00b6a5a6) ^ 1).wrapping_mul(7),
                    ) as u32
                };
                let go = if u_var7 == u_var19 {
                    u_var19 = 0x15b;
                    true
                } else if u_var19 != 1 {
                    u_var19 = 0x173;
                    true
                } else {
                    false
                };
                if go {
                    let p2 = FUN_004ddc10(u_var19);
                    let u_var18: u32 = 1;
                    let p3 = FUN_004ddc10(0x171);
                    _sprintf(buf, &raw const s__c_s__c_s_0052a6b0, 4i32, p3, u_var18, p2);
                } else {
                    let idx =
                        ldi(&raw mut DAT_00b6d5a6, local_28.wrapping_mul(0x16)).wrapping_mul(0x562);
                    let b = (&raw mut DAT_00a91d18).byte_offset(idx as isize).read();
                    FUN_004c9bf0(ldi(
                        &raw mut DAT_00b6d856,
                        (b as u32 as i32).wrapping_mul(7),
                    ));
                    u_var19 = 0xffff_ffff;
                    let mut p2: *mut i8 = buf;
                    let mut p3: *mut i8;
                    loop {
                        p3 = p2;
                        if u_var19 == 0 {
                            break;
                        }
                        u_var19 = u_var19.wrapping_sub(1);
                        p3 = p2.add(1);
                        let c = p2.read();
                        p2 = p3;
                        if c == 0 {
                            break;
                        }
                    }
                    u_var19 = !u_var19;
                    let mut p2: *mut i8 = p3.byte_offset(-(u_var19 as i32 as isize));
                    let mut p3: *mut i8 = local_14.as_mut_ptr();
                    let mut c = u_var19 >> 2;
                    while c != 0 {
                        p3.cast::<u32>()
                            .write_unaligned(p2.cast::<u32>().read_unaligned());
                        p2 = p2.add(4);
                        p3 = p3.add(4);
                        c -= 1;
                    }
                    let mut c = u_var19 & 3;
                    while c != 0 {
                        p3.write(p2.read());
                        p2 = p2.add(1);
                        p3 = p3.add(1);
                        c -= 1;
                    }
                    let p2 = local_14.as_mut_ptr();
                    let p3 = FUN_004ddc10(0x172);
                    let u_var18: u32 = 1;
                    let p4 = FUN_004ddc10(0x171);
                    _sprintf(
                        buf,
                        &raw const s__c_s__c_s__s__0052d4d8,
                        4i32,
                        p4,
                        u_var18,
                        p3,
                        p2,
                    );
                }
                i_var9 = i_var8.wrapping_add(0xf);
                FUN_004db4e0(0xc, buf, 0x58, i_var8, 0xffff);
            }
            i_var10 += 1;
        }
        i_var8 = i_var9;
        if in_(i_var10) {
            let u_var18 = ldi(&raw mut DAT_00b6d5b2, local_28.wrapping_mul(0x16)) as u32;
            let u_var13: u32 = 1;
            let p2 = FUN_004ddc10(0x14d);
            _sprintf(
                buf,
                &raw const s__c_s__c_d_0052c518,
                4i32,
                p2,
                u_var13,
                u_var18,
            );
            i_var8 = i_var9.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x58, i_var9, 0xffff);
        }
        i_var11 = i_var10 + 1;
        i_var9 = i_var8;
        if ld(&raw mut DAT_00b6a616) != 0 {
            if in_(i_var11) {
                _sprintf(buf, &raw const s_rank_d_0052b4e0, ld(&raw mut DAT_00b6bc32));
                FUN_004b5570(buf, 0x58, i_var8.wrapping_add(1));
                let p2 = FUN_004ddc10(ld(&raw mut DAT_00b6bc32).wrapping_add(0x7a) as u32);
                let u_var18: u32 = 6;
                let p3 = FUN_004ddc10(ld(&raw mut DAT_00b6a616).wrapping_add(0x177) as u32);
                _sprintf(buf, &raw const s__c_s__c_s_0052a6b0, 4i32, p3, u_var18, p2);
                i_var9 = i_var8.wrapping_add(0xf);
                FUN_004db4e0(0xc, buf, 0x91, i_var8, 0xffff);
            }
            i_var11 = i_var10 + 2;
        }
        DAT_0066d9f4 = 0;
        i_var10 = 0;
        loop {
            i_var8 = i_var9;
            let base = ld(&raw mut DAT_00b6d962).wrapping_mul(0x4278);
            if ldi(&raw mut DAT_00b6da12, i_var10.wrapping_add(base)) != 0 {
                if in_(i_var11) {
                    if DAT_0066d9f4 == 0 {
                        let p2 = FUN_004ddc10(0x179);
                        FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
                        DAT_0066d9f4 = 1;
                    }
                    let p2 = FUN_004ddc10(0x179);
                    let i_var16 = FUN_004dbfa0(p2, 0xc);
                    let p2: *mut i8;
                    let u_var18: u32;
                    if i_var10 == 2 {
                        if ld(&raw mut DAT_00b6d962) == 0 {
                            p2 = &raw mut s_rcitlvl_d_0052b4b4;
                            u_var18 = ld(&raw mut DAT_00b6da1a) as u32;
                        } else {
                            u_var18 = ldi(&raw mut DAT_00b6da12, base.wrapping_add(2)) as u32;
                            p2 = &raw mut s_citlvl_d_0052b4a8;
                        }
                    } else {
                        u_var18 = ldi(&raw mut DAT_00b6da12, i_var10.wrapping_add(base)) as u32;
                        p2 = &raw mut s_medlvl_d_0052b49c;
                    }
                    _sprintf(buf, p2, u_var18);
                    FUN_004b5570(buf, i_var16.wrapping_add(0x62), i_var9);
                    if i_var10 == 2 {
                        u_var19 = (ldi(&raw mut DAT_00b6da12, base.wrapping_add(2)) as u32)
                            .wrapping_add(0x293);
                    } else {
                        u_var19 = (ldi(&raw mut DAT_00b6da12, i_var10.wrapping_add(base)) as u32)
                            .wrapping_add(0x17d);
                    }
                    let p2 = FUN_004ddc10(u_var19);
                    let p3 = FUN_004ddc10(i_var10.wrapping_add(0x17a) as u32);
                    _sprintf(buf, &raw const s__s____s_0052d4d0, p3, p2);
                    i_var8 = i_var9.wrapping_add(0xf);
                    FUN_004db4e0(0xc, buf, i_var16.wrapping_add(0x8a), i_var9, 0xffff);
                }
                i_var11 += 1;
            }
            i_var10 += 1;
            i_var9 = i_var8;
            if !(i_var10 < 4) {
                break;
            }
        }
        if in_(i_var11) {
            i_var8 = i_var8.wrapping_add(0xf);
        }
        i_var10 = i_var8;
        if in_(i_var11 + 1) {
            i_var10 = i_var8.wrapping_add(0xf);
            let p2 = FUN_004ddc10(0x15e);
            FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
        }
        i_var8 = i_var10;
        if in_(i_var11 + 2) {
            let p2 = FUN_004ddc10(0x129);
            FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
            _sprintf(
                buf,
                &raw const s__d___d__0052b464,
                ld(&raw mut DAT_00b6c126),
                DAT_0066d8c8,
            );
            i_var8 = i_var10.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var10, 0xffff);
        }
        i_var10 = i_var11 + 3;
        if DAT_00b69e30 != 2 {
            i_var9 = i_var8;
            if in_(i_var10) {
                let p2 = FUN_004ddc10(0x15f);
                FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
                if ld(&raw mut DAT_00b6a5aa) == 0 {
                    _sprintf(buf, &raw const DAT_0052b46c);
                } else {
                    _sprintf(
                        buf,
                        &raw const s__d___d__0052b464,
                        DAT_0066d948,
                        DAT_0066da40,
                    );
                }
                i_var9 = i_var8.wrapping_add(0xf);
                FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
            }
            i_var8 = i_var9;
            if in_(i_var11 + 4) {
                let p2 = FUN_004ddc10(0x160);
                FUN_004db4e0(0xc, p2, 0x58, i_var9, DAT_00b6a2c0 as u32);
                _sprintf(
                    buf,
                    &raw const s__d___d__0052b464,
                    DAT_0066d958,
                    DAT_0066d9e8,
                );
                i_var8 = i_var9.wrapping_add(0xf);
                FUN_004db4e0(0xc, buf, 0x10c, i_var9, 0xffff);
            }
            i_var10 = i_var11 + 5;
        }
        i_var9 = i_var8;
        if in_(i_var10) {
            let p2 = FUN_004ddc10(0x161);
            FUN_004db4e0(0xc, p2, 0x58, i_var9, DAT_00b6a2c0 as u32);
            _sprintf(buf, &raw const DAT_0052b460, DAT_0066d908);
            i_var9 = i_var8.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
        }
        i_var8 = i_var9;
        if in_(i_var10 + 1) {
            let p2 = FUN_004ddc10(0x162);
            FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
            _sprintf(buf, &raw const DAT_0052b460, ld(&raw mut DAT_00b6d3aa));
            i_var8 = i_var9.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var9, 0xffff);
        }
        i_var9 = i_var8;
        if in_(i_var10 + 2) {
            let p2 = FUN_004ddc10(0x163);
            FUN_004db4e0(0xc, p2, 0x58, i_var9, DAT_00b6a2c0 as u32);
            u_var19 = 0;
            if ld(&raw mut DAT_00b6d3c2) != 0 {
                u_var19 = (ld(&raw mut DAT_00b6d3b6) as u32).wrapping_mul(100)
                    / (ld(&raw mut DAT_00b6d3c2) as u32);
            }
            _sprintf(buf, &raw const DAT_0052b458, u_var19);
            i_var9 = i_var8.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
        }
        i_var8 = i_var9;
        if in_(i_var10 + 3) {
            let p2 = FUN_004ddc10(0x164);
            FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
            u_var19 = 0;
            if ld(&raw mut DAT_00b6d3da) != 0 {
                u_var19 = (ld(&raw mut DAT_00b6d3ce) as u32).wrapping_mul(100)
                    / (ld(&raw mut DAT_00b6d3da) as u32);
            }
            _sprintf(buf, &raw const DAT_0052b458, u_var19);
            i_var8 = i_var9.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var9, 0xffff);
        }
        i_var9 = i_var10 + 4;
        if DAT_0066d9b0 != 0 {
            i_var11 = i_var8;
            if in_(i_var9) {
                i_var11 = i_var8.wrapping_add(0xf);
            }
            i_var8 = i_var11;
            if in_(i_var10 + 5) {
                i_var8 = i_var11.wrapping_add(0xf);
                let p2 = FUN_004ddc10(0x165);
                FUN_004db4e0(0xc, p2, 0x58, i_var11, DAT_00b6a2c0 as u32);
            }
            local_28 = 0x18;
            i_var9 = i_var10 + 6;
            loop {
                let mut p6: *mut i8 = &raw mut DAT_00b6d5ae;
                let mut p12: *mut i8 = &raw mut DAT_00b6bcc2;
                i_var10 = i_var8;
                loop {
                    i_var8 = i_var10;
                    if ld(p6) == local_28 && (ld(p12) != 0 || ldi(p12, 8) != 0) {
                        if in_(i_var9) {
                            let name = (p6 as usize as i32).wrapping_sub(0x16);
                            let u_var18: u32 = 1;
                            let p2 = FUN_004ddc10(local_28.wrapping_add(0x9a) as u32);
                            _sprintf(
                                buf,
                                &raw const s__c_s__c_s_0052a6b0,
                                6i32,
                                p2,
                                u_var18,
                                name,
                            );
                            FUN_004db4e0(0xc, buf, 0x58, i_var10, DAT_00b6a2c0 as u32);
                            _sprintf(buf, &raw const s__d__d__0052b450, ld(p12), ldi(p12, 8));
                            i_var8 = i_var10.wrapping_add(0xf);
                            FUN_004db4e0(0xc, buf, 0x10c, i_var10, 0xffff);
                        }
                        i_var9 += 1;
                    }
                    p6 = p6.byte_add(0x58);
                    p12 = p12.byte_add(4);
                    i_var10 = i_var8;
                    if !(p12 < &raw mut DAT_00b6bce2) {
                        break;
                    }
                }
                local_28 -= 1;
                if !(-1 < local_28) {
                    break;
                }
            }
        }
        if DAT_0066d9b4 != 0 {
            i_var10 = i_var8;
            if in_(i_var9) {
                i_var10 = i_var8.wrapping_add(0xf);
            }
            i_var8 = i_var10;
            if in_(i_var9 + 1) {
                i_var8 = i_var10.wrapping_add(0xf);
                let p2 = FUN_004ddc10(0x166);
                FUN_004db4e0(0xc, p2, 0x58, i_var10, DAT_00b6a2c0 as u32);
            }
            i_var9 += 2;
        }
        i_var10 = 0;
        let mut p12: *mut i8 = &raw mut DAT_00b6c13e;
        loop {
            i_var11 = 1;
            DAT_0066d9f4 = 0;
            let mut p6 = p12;
            loop {
                if ld(p6) != 0 || ldi(p6, 300) != 0 {
                    DAT_0066d9f4 = 1;
                }
                p6 = p6.byte_add(400);
                i_var11 -= 1;
                if i_var11 == 0 {
                    break;
                }
            }
            i_var11 = i_var8;
            if DAT_0066d9f4 != 0 {
                if in_(i_var9) {
                    let p2 = FUN_004ddc10(i_var10.wrapping_add(0x15) as u32);
                    FUN_004db4e0(0xc, p2, 0x58, i_var11, DAT_00b69e38 as u32);
                    _sprintf(buf, &raw const s__d___d__0052b464, ld(p12), ldi(p12, 300));
                    i_var11 = i_var8.wrapping_add(0xf);
                    FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
                }
                i_var9 += 1;
            }
            p12 = p12.byte_add(4);
            i_var10 += 1;
            i_var8 = i_var11;
            if !(p12 < &raw mut DAT_00b6c2ce) {
                break;
            }
        }
        if in_(i_var9) {
            i_var11 = i_var11.wrapping_add(0xf);
        }
        i_var10 = i_var11;
        if in_(i_var9 + 1) {
            i_var10 = i_var11.wrapping_add(0xf);
            let p2 = FUN_004ddc10(0x168);
            FUN_004db4e0(0xc, p2, 0x58, i_var11, DAT_00b6a2c0 as u32);
        }
        i_var8 = i_var10;
        if in_(i_var9 + 2) {
            let p2 = FUN_004ddc10(0x169);
            FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
            _sprintf(buf, &raw const DAT_0052b460, ld(&raw mut DAT_00b6d3e6));
            i_var8 = i_var10.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var10, 0xffff);
        }
        i_var10 = i_var9 + 3;
        if DAT_00b69e30 != 2 {
            i_var11 = i_var8;
            if in_(i_var10) {
                let p2 = FUN_004ddc10(0x16a);
                FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
                _sprintf(buf, &raw const DAT_0052b460, DAT_0066da60);
                i_var11 = i_var8.wrapping_add(0xf);
                FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
            }
            i_var8 = i_var11;
            if in_(i_var9 + 4) {
                let p2 = FUN_004ddc10(0x16b);
                FUN_004db4e0(0xc, p2, 0x58, i_var11, DAT_00b6a2c0 as u32);
                _sprintf(buf, &raw const DAT_0052b460, DAT_0066da50);
                i_var8 = i_var11.wrapping_add(0xf);
                FUN_004db4e0(0xc, buf, 0x10c, i_var11, 0xffff);
            }
            i_var10 = i_var9 + 5;
        }
        i_var9 = i_var8;
        if in_(i_var10) {
            let p2 = FUN_004ddc10(0x16c);
            FUN_004db4e0(0xc, p2, 0x58, i_var9, DAT_00b6a2c0 as u32);
            _sprintf(buf, &raw const DAT_0052b460, ld(&raw mut DAT_00b6d3fe));
            i_var9 = i_var8.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
        }
        i_var8 = i_var9;
        if in_(i_var10 + 1) {
            let p2 = FUN_004ddc10(0x16d);
            FUN_004db4e0(0xc, p2, 0x58, i_var8, DAT_00b6a2c0 as u32);
            _sprintf(buf, &raw const DAT_0052b460, ld(&raw mut DAT_00b6d40a));
            i_var8 = i_var9.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var9, 0xffff);
        }
        i_var9 = i_var8;
        if in_(i_var10 + 2) {
            let p2 = FUN_004ddc10(0x16e);
            FUN_004db4e0(0xc, p2, 0x58, i_var9, DAT_00b6a2c0 as u32);
            _sprintf(buf, &raw const DAT_0052b460, ld(&raw mut DAT_00b6d3f2));
            i_var9 = i_var8.wrapping_add(0xf);
            FUN_004db4e0(0xc, buf, 0x10c, i_var8, 0xffff);
        }
        if DAT_0066d9e4 != 0 {
            if in_(i_var10 + 3) {
                i_var9 = i_var9.wrapping_add(0xf);
            }
            i_var8 = i_var9;
            if in_(i_var10 + 4) {
                i_var8 = i_var9.wrapping_add(0xf);
                let p2 = FUN_004ddc10(0x184);
                FUN_004db4e0(0xc, p2, 0x58, i_var9, DAT_00b6a2c0 as u32);
            }
            local_28 = 0x18;
            i_var10 += 5;
            loop {
                let mut p12: *mut i8 = &raw mut DAT_00b6d5ae;
                let mut p6: *mut i8 = &raw mut DAT_00b6be82;
                i_var9 = i_var8;
                loop {
                    i_var8 = i_var9;
                    if ld(p12) == local_28 && (ld(p6) != 0 || ldi(p6, 8) != 0) {
                        if in_(i_var10) {
                            let name = (p12 as usize as i32).wrapping_sub(0x16);
                            let u_var18: u32 = 1;
                            let p2 = FUN_004ddc10(local_28.wrapping_add(0x9a) as u32);
                            _sprintf(
                                buf,
                                &raw const s__c_s__c_s_0052a6b0,
                                6i32,
                                p2,
                                u_var18,
                                name,
                            );
                            FUN_004db4e0(0xc, buf, 0x58, i_var9, DAT_00b6a2c0 as u32);
                            _sprintf(buf, &raw const s__d__d__0052b450, ld(p6), ldi(p6, 8));
                            i_var8 = i_var9.wrapping_add(0xf);
                            FUN_004db4e0(0xc, buf, 0x10c, i_var9, 0xffff);
                        }
                        i_var10 += 1;
                    }
                    p12 = p12.byte_add(0x58);
                    p6 = p6.byte_add(4);
                    i_var9 = i_var8;
                    if !(p6 < &raw mut DAT_00b6bea2) {
                        break;
                    }
                }
                local_28 -= 1;
                if !(-1 < local_28) {
                    break;
                }
            }
        }
        1
    }
}
