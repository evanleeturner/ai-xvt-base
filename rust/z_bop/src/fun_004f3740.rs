// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5cb0: [i32; 64];
    static mut DAT_00aa5e10: [i32; 8];
    static mut DAT_00aa5df0: [i32; 8];
    static mut DAT_00aa5e38: i32;
    static mut DAT_00aa5e40: [i32; 8];
    static mut DAT_00aa5e60: i32;
    static mut DAT_00aa47ba: [i8; 1];
    static mut DAT_00aa4f4e: [i8; 1];
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a606: [u8; 4];
    static mut DAT_00aa6120: i32;
    static mut DAT_00aa6124: i32;
    static mut DAT_00aa612c: i32;
    static mut DAT_00aa6130: i32;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00528118: [i8; 1];
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b6a270: [u8; 1];
    static mut DAT_00bb73a7: i8;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa6af8: i32;
    static mut DAT_00aa6afc: i32;
    static mut DAT_00aa6150: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004db7d0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004dc2e0(a: i32) -> i8;
    fn FUN_004dc2a0(a: i32) -> u8;
    fn FUN_004dc2c0(a: i32) -> u8;
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
    fn FUN_004de5a0(a: *const i8, b: i32, c: i32, d: i32);
    fn FUN_004dc1d0();
    fn FUN_004d1400() -> i32;
    fn FUN_004d4cf0(a: *mut core::ffi::c_void);
    fn FUN_004d4d20(a: *mut core::ffi::c_void);
    fn FUN_004d11e0() -> i32;
    fn FUN_004cfee0() -> i32;
    fn FUN_004dc1e0() -> u8;
    fn FUN_004dc200() -> u8;
    fn FUN_004d11d0() -> i32;
    fn FUN_004dc1c0(a: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "C" fn FUN_004f3740(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; pointers are
    // derived from the game's arrays and walked exactly as the original does.
    unsafe {
        let cb0 = (&raw mut DAT_00aa5cb0).cast::<i32>();
        let e10 = (&raw mut DAT_00aa5e10).cast::<i32>();
        let df0 = (&raw mut DAT_00aa5df0).cast::<i32>();
        let e40 = (&raw mut DAT_00aa5e40).cast::<i32>();
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();
        let base = (&raw mut DAT_00aa6150).cast::<u8>();
        let af0 = &raw mut DAT_00aa6af0;

        let mut local_24: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        let mut local_30: i32 = 0;
        let mut local_34: i32 = 0;
        let mut local_14: i32 = 0;

        FUN_004d6460(local_24.as_mut_ptr(), 0x5a, 0xcf, 0x100, 0xdd);
        let mut local_38: i32 = 0;
        if 0 < DAT_00aa5e68 {
            let mut local_2c = cb0;
            let mut local_3c: i32 = 0;
            let mut local_28 = e10;
            let mut local_40 = (&raw mut DAT_00aa47ba).cast::<i8>();
            loop {
                if (DAT_00aa5e68 == 8 || DAT_00aa5e68 == 7)
                    && local_40 == (&raw mut DAT_00aa4f4e).cast::<i8>()
                {
                    FUN_004d6460(local_24.as_mut_ptr(), 0x106, 0xcf, 0x1ac, 0xdd);
                }
                let tmp_a: i32 = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();
                if tmp_a == 3 {
                    let tmp_b: i32 = (&raw mut DAT_00b6a606).cast::<i32>().read_unaligned();
                    if DAT_00aa6120 == 0 || DAT_00aa6124 == 0 || tmp_b != 1 {
                        _sprintf(
                            buf,
                            (&raw mut DAT_00528118).cast::<i8>() as *const i8,
                            local_40,
                        );
                    } else if local_40 == (&raw mut DAT_00aa47ba).cast::<i8>() {
                        let u_var12 = DAT_00aa6130;
                        let pc = FUN_004ddc10(0x308);
                        _sprintf(
                            buf,
                            c"%s - %c%s: %d".as_ptr(),
                            (&raw mut DAT_00aa47ba).cast::<i8>(),
                            4i32,
                            pc,
                            u_var12,
                        );
                    } else {
                        let u_var12 = DAT_00aa612c;
                        let pc = FUN_004ddc10(0x308);
                        _sprintf(buf, c"%s - %c%s: %d".as_ptr(), local_40, 4i32, pc, u_var12);
                    }
                } else {
                    let i_var6 = local_38.wrapping_add(1);
                    let p13 = local_40;
                    let pc = FUN_004ddc10(0xd9);
                    _sprintf(buf, c"%s %u: %s".as_ptr(), pc, i_var6, p13);
                }
                FUN_004db7d0(0xc, buf, local_24.as_mut_ptr(), 0, 1, 0xffff);
                FUN_004d64a0(local_24.as_mut_ptr(), 0, 0xf);
                let mut local_44: i32 = 0;
                if 0 < *local_28 {
                    loop {
                        'body: {
                            let mut b_var1 = FUN_004dc2e0(2);
                            if b_var1 != 0 && (FUN_004dc2a0(2) != 0 || FUN_004dc2c0(2) != 0) {
                                FUN_004de090(&raw mut local_30, &raw mut local_34);
                                let mut i_var4 =
                                    FUN_004d6b90(local_24.as_mut_ptr(), local_30, local_34);
                                let mut i_var10 = DAT_00b69e30;
                                let mut i_var6 = DAT_00aa5e38;
                                if i_var4 != 0 {
                                    i_var4 =
                                        cb0.offset(local_3c.wrapping_add(local_44) as isize).read();
                                    let mut i_var9: i32 = 0;
                                    let mut need = i_var4 == 0;
                                    if !need {
                                        let mut p5 = df0;
                                        loop {
                                            if *p5 == i_var4 {
                                                *p5 = 0;
                                            }
                                            p5 = p5.add(1);
                                            if !(p5 < e10) {
                                                break;
                                            }
                                        }
                                        if i_var10 != 2 {
                                            DAT_00aa6af0 = 0x4f;
                                            DAT_00aa6af8 = 10;
                                            DAT_00aa6af4 = i_var4;
                                            FUN_004cef70(0, af0, 0xc);
                                            i_var10 = DAT_00b69e30;
                                            i_var6 = DAT_00aa5e38;
                                        }
                                        i_var9 = local_44;
                                        if i_var4 == 0 {
                                            need = true;
                                        }
                                    }
                                    if need {
                                        i_var9 = 0;
                                        let mut p5 = local_2c;
                                        if 0 < local_44 {
                                            loop {
                                                if *p5 == 0 {
                                                    break;
                                                }
                                                i_var9 = i_var9.wrapping_add(1);
                                                p5 = p5.add(1);
                                                if !(i_var9 < local_44) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    i_var4 = 0;
                                    cb0.offset(local_3c.wrapping_add(i_var9) as isize)
                                        .write(i_var6);
                                    let mut p5 = df0;
                                    loop {
                                        if *p5 == i_var6 {
                                            break;
                                        }
                                        p5 = p5.add(1);
                                        i_var4 = i_var4.wrapping_add(1);
                                        if !(p5 < e10) {
                                            break;
                                        }
                                    }
                                    if i_var4 == 8 {
                                        i_var4 = 0;
                                        let mut p5 = df0;
                                        loop {
                                            if *p5 == 0 {
                                                df0.offset(i_var4 as isize).write(i_var6);
                                                break;
                                            }
                                            p5 = p5.add(1);
                                            i_var4 = i_var4.wrapping_add(1);
                                            if !(p5 < e10) {
                                                break;
                                            }
                                        }
                                    }
                                    if DAT_00bb73a7 != 0 {
                                        FUN_004de5a0(c"slotsound".as_ptr(), 1, 0, 0xff);
                                        i_var10 = DAT_00b69e30;
                                        i_var6 = DAT_00aa5e38;
                                    }
                                    if i_var10 != 2 {
                                        DAT_00aa6af8 = local_38;
                                        DAT_00aa6af0 = 0x4f;
                                        DAT_00aa6af4 = i_var6;
                                        DAT_00aa6afc = i_var9;
                                        FUN_004cef70(0, af0, 0x10);
                                        if DAT_00b69e30 != 2 {
                                            DAT_00aa6af4 = DAT_00aa5e38;
                                            DAT_00aa6af0 = 0x67;
                                            FUN_004cef70(0, af0, 8);
                                        }
                                    }
                                    DAT_00aa5e38 = 0;
                                    FUN_004dc1d0();
                                }
                            }
                            if cb0.offset(local_3c.wrapping_add(local_44) as isize).read() != 0 {
                                let mut i_var6: i32 = 1;
                                if DAT_00b69e30 != 2 {
                                    i_var6 = FUN_004d1400();
                                }
                                let mut i_var10: i32 = 0;
                                if 0 < i_var6 {
                                    let mut pb = base.add(0xe);
                                    loop {
                                        let t: i32 = pb.cast::<i32>().read_unaligned();
                                        if t == cb0
                                            .offset(local_3c.wrapping_add(local_44) as isize)
                                            .read()
                                        {
                                            break;
                                        }
                                        pb = pb.add(0x2a);
                                        i_var10 = i_var10.wrapping_add(1);
                                        if !(i_var10 < i_var6) {
                                            break;
                                        }
                                    }
                                }
                                if i_var10 < i_var6 {
                                    i_var6 = (base as usize as i32)
                                        .wrapping_add(i_var10.wrapping_mul(0x2a));
                                    i_var10 = i_var10.wrapping_mul(0x2a);
                                    let u_var12: i32 = 1;
                                    let t: i32 = base
                                        .offset(0x12 + i_var10 as isize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    let pc = FUN_004ddc10(t.wrapping_add(0x9a));
                                    _sprintf(buf, c"%c%s %c%s".as_ptr(), 6i32, pc, u_var12, i_var6);
                                    FUN_004d4cf0(local_10.as_mut_ptr().cast());
                                    FUN_004d4d20(local_24.as_mut_ptr().cast());
                                    i_var6 = FUN_004d11e0();
                                    let t: i32 = base
                                        .offset(0xe + i_var10 as isize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    let mut u_var14: u32 = 0;
                                    let mut use_tbl = false;
                                    if i_var6 == t {
                                        use_tbl = true;
                                    } else {
                                        u_var14 = DAT_00b6a2c0;
                                        if DAT_00b69e30 == 2 {
                                            use_tbl = true;
                                        }
                                    }
                                    if use_tbl {
                                        let idx =
                                            ((param_1 % 0x18) as u32 & 0xffff_fffe).wrapping_mul(2);
                                        u_var14 = (&raw mut DAT_00b6a270)
                                            .cast::<u8>()
                                            .byte_add(idx as usize)
                                            .cast::<u32>()
                                            .read_unaligned();
                                    }
                                    FUN_004db7d0(0xc, buf, local_24.as_mut_ptr(), 0, 1, u_var14);
                                    FUN_004d4d20(local_10.as_mut_ptr().cast());
                                    b_var1 = FUN_004dc2e0(2);
                                    if b_var1 == 0 {
                                        i_var6 = FUN_004cfee0();
                                        if (i_var6 != 0 || DAT_00b69e30 == 2)
                                            && (FUN_004dc1e0() != 0 || FUN_004dc200() != 0)
                                        {
                                            FUN_004de090(&raw mut local_30, &raw mut local_34);
                                            i_var6 = FUN_004d6b90(
                                                local_24.as_mut_ptr(),
                                                local_30,
                                                local_34,
                                            );
                                            if i_var6 != 0 {
                                                if DAT_00b69e30 == 2 {
                                                    DAT_00aa5e38 = base
                                                        .offset(0xe + i_var10 as isize)
                                                        .cast::<i32>()
                                                        .read_unaligned();
                                                    FUN_004dc1c0(2);
                                                    i_var6 = 0;
                                                    let mut p5 = df0;
                                                    loop {
                                                        if *p5 != DAT_00aa5e38 {
                                                            p5 = p5.add(1);
                                                            i_var6 = i_var6.wrapping_add(1);
                                                            if 0xaa5e0f < (p5 as usize) {
                                                                break 'body;
                                                            }
                                                            continue;
                                                        }
                                                        df0.offset(i_var6 as isize).write(0);
                                                        i_var6 = local_44.wrapping_add(1);
                                                        if i_var6 < 8 {
                                                            i_var10 = 8 - i_var6;
                                                            let mut p7 = cb0.wrapping_offset(
                                                                (local_3c.wrapping_add(i_var6)
                                                                    as isize)
                                                                    - 1,
                                                            );
                                                            loop {
                                                                i_var10 = i_var10.wrapping_sub(1);
                                                                p7.write(p7.add(1).read());
                                                                p7 = p7.add(1);
                                                                if i_var10 == 0 {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        local_2c.add(7).write(0);
                                                        break 'body;
                                                    }
                                                }
                                                i_var6 = 0;
                                                if 0 < DAT_00aa5e60 {
                                                    let mut p5 = e40;
                                                    loop {
                                                        let t: i32 = base
                                                            .offset(0xe + i_var10 as isize)
                                                            .cast::<i32>()
                                                            .read_unaligned();
                                                        if *p5 == t {
                                                            break;
                                                        }
                                                        p5 = p5.add(1);
                                                        i_var6 = i_var6.wrapping_add(1);
                                                        if !(i_var6 < DAT_00aa5e60) {
                                                            break;
                                                        }
                                                    }
                                                }
                                                if i_var6 == DAT_00aa5e60 {
                                                    DAT_00aa6af4 = base
                                                        .offset(0xe + i_var10 as isize)
                                                        .cast::<i32>()
                                                        .read_unaligned();
                                                    DAT_00aa5e60 = DAT_00aa5e60.wrapping_add(1);
                                                    e40.offset(i_var6 as isize).write(DAT_00aa6af4);
                                                    DAT_00aa6af0 = 0x68;
                                                    DAT_00aa6af8 = FUN_004d11d0();
                                                    FUN_004cef70(0, af0, 0xc);
                                                    DAT_00aa5e38 = base
                                                        .offset(0xe + i_var10 as isize)
                                                        .cast::<i32>()
                                                        .read_unaligned();
                                                    FUN_004dc1c0(2);
                                                    DAT_00aa6af0 = 0x4f;
                                                    DAT_00aa6af4 = DAT_00aa5e38;
                                                    DAT_00aa6af8 = 10;
                                                    FUN_004cef70(0, af0, 0xc);
                                                    i_var10 = DAT_00aa5e68;
                                                    i_var6 = DAT_00aa5e38;
                                                    let mut i_var4: i32 = 0;
                                                    let mut p5 = df0;
                                                    loop {
                                                        if *p5 == DAT_00aa5e38 {
                                                            df0.offset(i_var4 as isize).write(0);
                                                            break;
                                                        }
                                                        p5 = p5.add(1);
                                                        i_var4 = i_var4.wrapping_add(1);
                                                        if !(p5 < e10) {
                                                            break;
                                                        }
                                                    }
                                                    if 0 < i_var10 {
                                                        let mut p5 = cb0;
                                                        i_var4 = 0;
                                                        let mut p11 = e10;
                                                        local_14 = i_var10;
                                                        loop {
                                                            i_var10 = 0;
                                                            let mut p8 = p5;
                                                            if 0 < *p11 {
                                                                loop {
                                                                    if *p8 != i_var6 {
                                                                        i_var10 =
                                                                            i_var10.wrapping_add(1);
                                                                        p8 = p8.add(1);
                                                                        if *p11 <= i_var10 {
                                                                            break;
                                                                        }
                                                                        continue;
                                                                    }
                                                                    i_var10 =
                                                                        i_var10.wrapping_add(1);
                                                                    if i_var10 < 8 {
                                                                        let mut i_var9 =
                                                                            8 - i_var10;
                                                                        let mut p7 = cb0
                                                                            .wrapping_offset(
                                                                                (i_var4
                                                                                    .wrapping_add(
                                                                                        i_var10,
                                                                                    )
                                                                                    as isize)
                                                                                    - 1,
                                                                            );
                                                                        loop {
                                                                            i_var9 = i_var9
                                                                                .wrapping_sub(1);
                                                                            p7.write(
                                                                                p7.add(1).read(),
                                                                            );
                                                                            p7 = p7.add(1);
                                                                            if i_var9 == 0 {
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                    p5.add(7).write(0);
                                                                    break;
                                                                }
                                                            }
                                                            p5 = p5.add(8);
                                                            i_var4 = i_var4.wrapping_add(8);
                                                            p11 = p11.add(1);
                                                            local_14 = local_14.wrapping_sub(1);
                                                            if local_14 == 0 {
                                                                break;
                                                            }
                                                        }
                                                        local_14 = 0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FUN_004d64a0(local_24.as_mut_ptr(), 0, 0xf);
                        local_44 = local_44.wrapping_add(1);
                        if !(local_44 < *local_28) {
                            break;
                        }
                    }
                }
                local_2c = local_2c.add(8);
                local_3c = local_3c.wrapping_add(8);
                local_28 = local_28.add(1);
                local_40 = local_40.wrapping_add(0x1e5);
                local_38 = local_38.wrapping_add(1);
                if !(local_38 < DAT_00aa5e68) {
                    break;
                }
            }
        }
    }
}
