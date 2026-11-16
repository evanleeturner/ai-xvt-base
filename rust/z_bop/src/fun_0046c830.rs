// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994c4: *mut i32;
    static mut DAT_00999500: u32;
    static mut DAT_0052702c: u32;
    static mut DAT_00527024: i32;
    static mut DAT_00527030: u32;
    static mut DAT_00527020: u32;
    static mut DAT_0052701c: u32;
    static mut DAT_00999548: i32;
    static mut DAT_009997d0: u32;
    static mut DAT_009999dc: u32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_0052705c: [i8; 1];

    static mut DAT_005599a8: [u32; 1];
    static mut DAT_005599ac: [u32; 1];
    static mut DAT_005599b0: [u32; 1];
    static mut DAT_005599b4: [u8; 1];
    static mut DAT_005599b5: [u8; 1];
    static mut DAT_005599b6: [u8; 1];
    static mut DAT_005599b7: [u8; 1];
    static mut DAT_005599b8: [u32; 1];
    static mut DAT_005599bc: [u32; 1];

    static mut DAT_00999e14: [u32; 1];
    static mut DAT_00999bf0: [u32; 1];
    static mut DAT_00999bf8: [u32; 1];
    static mut DAT_00999c00: [u32; 1];
    static mut DAT_00999c04: [u32; 1];

    static mut DAT_005ee1bc: [u32; 1];
    static mut DAT_005ee1cc: [u32; 1];
    static mut DAT_005ee1c6: [u8; 1];
    static mut DAT_005ee1c8: [u8; 1];

    static mut DAT_005dd9bc: [u32; 1];
    static mut DAT_005dd9b8: [u32; 1];
    static mut DAT_005dd9c5: [u8; 1];
    static mut DAT_005dd9c6: [u8; 1];
    static mut DAT_005dd9c8: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046fad0() -> u32;
    fn FUN_0046c200();
    fn FUN_00470060();
    fn FUN_0046c220(param_1: *mut i8);
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_0046fee0(param_1: i32) -> u32;
    fn FUN_0046dd80(param_1: i32, param_2: i8, param_3: u8, param_4: *mut u32, param_5: u32);
    fn FUN_0046fc00(param_1: i32, param_2: i32, param_3: u32, param_4: u32) -> i32;
}

macro_rules! g32 {
    ($n:ident, $i:expr) => {
        (&raw mut $n).cast::<u32>().add((($i) as u32) as usize)
    };
}

macro_rules! g8 {
    ($n:ident, $i:expr) => {
        (&raw mut $n).cast::<u8>().add((($i) as u32) as usize)
    };
}

unsafe fn cp(dst: *mut u8, src: *const u8, n: u32) {
    // SAFETY: the original copies n bytes from the stack buffer into a
    // queue slot; the regions never overlap.
    unsafe {
        core::ptr::copy_nonoverlapping(src, dst, n as usize);
    }
}

type RecvFn =
    unsafe extern "stdcall" fn(*mut i32, *mut i32, *mut i32, i32, *mut u16, *mut u32) -> i32;

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_labels
)]
pub unsafe extern "stdcall" fn FUN_0046c830() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are into the game's tables and this function's own buffers.
    unsafe {
        let mut i_stack_428: i32 = 0;
        let mut u_stack_424: u32 = 0;
        let mut u_stack_420: u32;
        let mut u_stack_41c: u32;
        let mut u_stack_418: u32 = 0;
        let mut i_stack_414: i32;
        let mut u_stack_410: u32;
        let mut au_stack_40c: [u32; 2] = [0; 2];
        let mut i_stack_404: i32 = 0;
        let mut u_stack_400: [u16; 0x200] = [0; 0x200];
        let mut u_var4: u32 = 0;
        let mut u_var6: u32 = 0;
        let mut u_var9: u32 = 0;

        if DAT_009994c4.is_null() {
            return 0;
        }
        DAT_00999500 = 0;
        let buf: *mut u16 = u_stack_400.as_mut_ptr();
        let bufb: *mut u8 = buf.cast::<u8>();

        'l1: loop {
            let mut p11: *mut u8;
            'l2: loop {
                'l3: loop {
                    'l4: loop {
                        'l5: loop {
                            'l6: loop {
                                if 0x3fe < DAT_0052702c {
                                    FUN_0046c200();
                                    FUN_00470060();
                                    return 0;
                                }
                                u_stack_418 = 0x400;
                                let vt = (*DAT_009994c4) as u32;
                                let fp = ((vt.wrapping_add(100)) as *const u32).read_unaligned();
                                let f = core::mem::transmute::<*const core::ffi::c_void, RecvFn>(
                                    fp as usize as *const core::ffi::c_void,
                                );
                                let r = f(
                                    DAT_009994c4,
                                    &raw mut i_stack_428,
                                    &raw mut i_stack_404,
                                    1,
                                    buf,
                                    &raw mut u_stack_418,
                                );
                                if r != 0 {
                                    return 0;
                                }
                                if i_stack_428 != 0 {
                                    break 'l6;
                                }
                                FUN_0046c220((&raw mut DAT_0052705c).cast::<i8>());
                                let i_var3 = DAT_00527024;
                                if 0x200 < u_stack_418 {
                                    u_stack_418 = 0x200;
                                }
                                let i_var5 = (DAT_00527024 as u32).wrapping_mul(0x210);
                                let i_var7 = (DAT_00527024 as u32).wrapping_mul(0x84);
                                *g32!(DAT_005599a8, (DAT_00527024 as u32).wrapping_mul(0x84)) =
                                    i_stack_428 as u32;
                                *g32!(DAT_005599ac, (i_var3 as u32).wrapping_mul(0x84)) =
                                    u_stack_418;
                                *g8!(DAT_005599b7, i_var5) = 0;
                                cp(g32!(DAT_005599b8, i_var7).cast::<u8>(), bufb, u_stack_418);
                                DAT_00527024 = (DAT_00527024 as u32).wrapping_add(1) as i32;
                                DAT_0052702c = DAT_0052702c.wrapping_add(1);
                                if 0x3ff < DAT_00527024 {
                                    DAT_00527024 = 0;
                                }
                            }
                            if DAT_00527030 == 0 {
                                break 'l5;
                            }
                            DAT_00527030 = DAT_00527030.wrapping_sub(1);
                        }
                        if i_stack_404 == DAT_00999548 {
                            break 'l4;
                        }
                    }
                    let w0 = u_stack_400[0];
                    p11 = bufb.add(2);
                    u_var9 = (w0 & 0x7f) as u32;
                    u_var6 = ((w0 & 0x80) != 0) as u32;
                    u_stack_410 = ((w0 & 0x8000) == 0) as u32;
                    u_stack_420 = ((w0 & 0x7f00) >> 8) as u32;
                    if u_var9 < 0x3c || 0x3f < u_var9 {
                        i_stack_414 = 1;
                    } else {
                        i_stack_414 = 0;
                    }
                    if i_stack_414 != 0 {
                        u_stack_424 = FUN_0046fad0();
                        if u_stack_424 == 0 {
                            p11 = bufb.add(4);
                            let w = bufb.add(2).cast::<u16>().read_unaligned();
                            u_stack_424 = w as u32;
                        }
                    }
                    if u_var9 != 0x35 {
                        break 'l3;
                    }
                    au_stack_40c[0] = 0x34;
                    FUN_0046d350(i_stack_428, au_stack_40c.as_mut_ptr(), 4);
                }
                if u_var9 == 0x36 {
                    continue 'l2;
                }
                break 'l2;
            }

            if u_var9 == 0x33 {
                u_var6 = p11.cast::<u32>().read_unaligned();
                u_var9 = p11.add(4).cast::<u32>().read_unaligned();
                u_var4 = FUN_0046fee0(i_stack_428);
                if u_var4 < DAT_0099f6e0 && u_var4 < 0x28 {
                    let p = g32!(DAT_00999e14, u_var4.wrapping_mul(0x8e));
                    *p = (*p).wrapping_add(1);
                }
                let mut u_var8: u32 = 0;
                u_var4 = DAT_00527020;
                loop {
                    if *g32!(DAT_005ee1bc, u_var4.wrapping_mul(0x84)) != 0
                        && (*g32!(DAT_005ee1cc, u_var4.wrapping_mul(0x84)) & 0x7fffffff) == u_var6
                    {
                        FUN_0046dd80(
                            i_stack_428,
                            0,
                            *g8!(DAT_005ee1c6, u_var4.wrapping_mul(0x210)),
                            g8!(DAT_005ee1c8, u_var4.wrapping_mul(0x210)).cast::<u32>(),
                            *g32!(DAT_005ee1bc, u_var4.wrapping_mul(0x84)),
                        );
                        break;
                    }
                    u_var8 = u_var8.wrapping_add(1);
                    u_var4 = u_var4.wrapping_add(1);
                    if 0xff < u_var4 {
                        u_var4 = 0;
                    }
                    if u_var8 >= 0x100 {
                        break;
                    }
                }
                if 0xff < u_var8 {
                    au_stack_40c[0] = 0x39;
                    FUN_0046dd80(i_stack_428, 0, u_var9 as u8, au_stack_40c.as_mut_ptr(), 4);
                }
                continue 'l1;
            }
            if u_var9 == 0x38 {
                u_var6 = p11.cast::<u32>().read_unaligned();
                u_var9 = p11.add(4).cast::<u32>().read_unaligned();
                u_var4 = FUN_0046fee0(i_stack_428);
                if u_var4 < DAT_0099f6e0 && u_var4 < 0x28 {
                    let p = g32!(DAT_00999e14, u_var4.wrapping_mul(0x8e));
                    *p = (*p).wrapping_add(1);
                }
                let mut u_var8: u32 = 0;
                u_var4 = DAT_0052701c;
                loop {
                    let i_var3 = u_var4.wrapping_mul(0x210);
                    if *g32!(DAT_005dd9bc, u_var4.wrapping_mul(0x84)) != 0 {
                        let mut check = false;
                        let mut b_var1: u8 = 0;
                        if u_var9 == 0 || u_var9 == 2 {
                            if (*g8!(DAT_005dd9c6, i_var3) as u32) == u_var6 {
                                b_var1 = *g8!(DAT_005dd9c5, i_var3);
                                check = true;
                            }
                        } else if (*g32!(DAT_005dd9b8, u_var4.wrapping_mul(0x84)) as i32)
                            == i_stack_428
                            && (*g8!(DAT_005dd9c6, i_var3) as u32) == u_var6
                        {
                            b_var1 = *g8!(DAT_005dd9c5, i_var3);
                            check = true;
                        }
                        if check && (b_var1 as u32) == u_var9 {
                            FUN_0046dd80(
                                i_stack_428,
                                u_var9 as i8,
                                u_var6 as u8,
                                g8!(DAT_005dd9c8, u_var4.wrapping_mul(0x210)).cast::<u32>(),
                                *g32!(DAT_005dd9bc, u_var4.wrapping_mul(0x84)),
                            );
                            break;
                        }
                    }
                    u_var8 = u_var8.wrapping_add(1);
                    u_var4 = u_var4.wrapping_add(1);
                    if 0x7f < u_var4 {
                        u_var4 = 0;
                    }
                    if u_var8 >= 0x80 {
                        break;
                    }
                }
                if 0x7f < u_var8 {
                    au_stack_40c[0] = 0x39;
                    FUN_0046dd80(
                        i_stack_428,
                        u_var9 as i8,
                        u_var6 as u8,
                        au_stack_40c.as_mut_ptr(),
                        4,
                    );
                }
            } else if u_var9 == 0x37 {
                u_var6 = p11.cast::<u32>().read_unaligned();
                u_var9 = p11.add(4).cast::<u32>().read_unaligned();
                u_var4 = p11.add(8).cast::<u32>().read_unaligned();
                if u_var6 == DAT_009997d0 {
                    u_var6 = 0x80;
                }
                if u_var9 == DAT_009999dc {
                    u_var9 = 0x80;
                }
                let mut u_var8 = FUN_0046fee0(i_stack_428);
                if DAT_0099f6e0 <= u_var8
                    || *g32!(DAT_00999c04, u_var8.wrapping_mul(0x8e)) == u_var4
                {
                    u_var4 = 0x80;
                }
                u_stack_41c = 0x80;
                u_var8 = DAT_0052701c;
                loop {
                    if u_var6 == 0x80 && u_var9 == 0x80 && u_var4 == 0x80 {
                        break;
                    }
                    let i_var3 = u_var8.wrapping_mul(0x210);
                    let u_var2 = *g32!(DAT_005dd9bc, u_var8.wrapping_mul(0x84));
                    let mut u_var14 = u_var4;
                    if u_var2 != 0 {
                        let c5 = *g8!(DAT_005dd9c5, i_var3);
                        if c5 == 0 {
                            if (*g8!(DAT_005dd9c6, i_var3) as u32) == u_var6 {
                                FUN_0046dd80(
                                    i_stack_428,
                                    0,
                                    u_var6 as u8,
                                    g8!(DAT_005dd9c8, i_var3).cast::<u32>(),
                                    u_var2,
                                );
                                u_var6 = 0x80;
                            }
                        } else if c5 == 2 {
                            if (*g8!(DAT_005dd9c6, i_var3) as u32) == u_var9 {
                                FUN_0046dd80(
                                    i_stack_428,
                                    2,
                                    u_var9 as u8,
                                    g8!(DAT_005dd9c8, i_var3).cast::<u32>(),
                                    u_var2,
                                );
                                u_var9 = 0x80;
                            }
                        } else if (*g32!(DAT_005dd9b8, u_var8.wrapping_mul(0x84)) as i32)
                            == i_stack_428
                            && (*g8!(DAT_005dd9c6, i_var3) as u32) == u_var4
                        {
                            u_var14 = 0x80;
                            FUN_0046dd80(
                                i_stack_428,
                                1,
                                u_var4 as u8,
                                g8!(DAT_005dd9c8, i_var3).cast::<u32>(),
                                u_var2,
                            );
                        }
                    }
                    u_var8 = u_var8.wrapping_add(1);
                    if 0x7f < u_var8 {
                        u_var8 = 0;
                    }
                    u_stack_41c = u_stack_41c.wrapping_sub(1);
                    u_var4 = u_var14;
                    if u_stack_41c == 0 {
                        break;
                    }
                }
            } else {
                FUN_0046fee0(i_stack_428);
                if u_stack_410 == 0 || u_var6 == 0 {
                    u_var4 = FUN_0046fee0(i_stack_428);
                    if i_stack_414 != 0 {
                        let mut i_var3: i32 = 0x7f;
                        if u_stack_420 != 0 {
                            i_var3 = u_stack_420.wrapping_sub(1) as i32;
                        }
                        i_var3 = FUN_0046fc00(i_stack_428, i_var3, u_stack_410, u_var6);
                        if i_var3 == 0 {
                            if u_var4 < DAT_0099f6e0 && u_var4 < 0x28 && u_var6 == 0 {
                                let p = g32!(DAT_00999e14, u_var4.wrapping_mul(0x8e));
                                *p = (*p).wrapping_add(1);
                            }
                            u_var4 = *p11.add(u_stack_424 as usize) as u32;
                            let pb12 = p11.add(u_stack_424 as usize).add(1);
                            if u_var4 != 0x39 {
                                let diff = (pb12 as usize as u32)
                                    .wrapping_sub(bufb as usize as u32)
                                    & 0xffff;
                                let mut u_var8 = u_stack_418.wrapping_sub(diff);
                                if 0x1fc < u_var8 {
                                    u_var8 = 0x1fc;
                                }
                                *g32!(DAT_005599b8, (DAT_00527024 as u32).wrapping_mul(0x84)) =
                                    u_var4;
                                let pb15 =
                                    g32!(DAT_005599bc, (DAT_00527024 as u32).wrapping_mul(0x84))
                                        .cast::<u8>();
                                cp(pb15, pb12, u_var8);
                                let i_var3b = DAT_00527024;
                                let i_var7 = (DAT_00527024 as u32).wrapping_mul(0x210);
                                *g32!(DAT_005599a8, (DAT_00527024 as u32).wrapping_mul(0x84)) =
                                    i_stack_428 as u32;
                                *g32!(DAT_005599ac, (i_var3b as u32).wrapping_mul(0x84)) =
                                    u_var8.wrapping_add(4);
                                *g32!(DAT_005599b0, (i_var3b as u32).wrapping_mul(0x84)) = 0;
                                *g8!(DAT_005599b4, i_var7) = 0;
                                *g8!(DAT_005599b7, i_var7) = 0;
                                if u_stack_410 == 0 {
                                    *g8!(DAT_005599b5, i_var7) = 2;
                                    if u_var6 == 0 {
                                        *g8!(DAT_005599b5, i_var7) = 1;
                                    }
                                } else {
                                    *g8!(DAT_005599b5, i_var7) = 0;
                                }
                                if u_stack_420 == 0 {
                                    *g8!(DAT_005599b6, i_var7) = 0x7f;
                                } else {
                                    *g8!(DAT_005599b6, i_var7) = u_stack_420.wrapping_sub(1) as u8;
                                }
                                DAT_00527024 = (DAT_00527024 as u32).wrapping_add(1) as i32;
                                DAT_0052702c = DAT_0052702c.wrapping_add(1);
                                if 0x3ff < DAT_00527024 {
                                    DAT_00527024 = 0;
                                }
                            }
                        }
                    }
                    u_stack_41c =
                        FUN_0046fc00(i_stack_428, u_stack_420 as i32, u_stack_410, u_var6) as u32;
                    if u_var6 == 0 || u_stack_41c == 0 {
                        let mut p13: *mut u8 = bufb.add(2);
                        if i_stack_414 == 0 {
                            u_stack_424 = u_stack_418.wrapping_sub(2);
                        } else {
                            u_stack_424 = FUN_0046fad0();
                            if u_stack_424 == 0 {
                                p13 = bufb.add(4);
                                let w = bufb.add(2).cast::<u16>().read_unaligned();
                                u_stack_424 = w as u32;
                            }
                        }
                        if 0x1fc < u_stack_424 {
                            u_stack_424 = 0x1fc;
                        }
                        *g32!(DAT_005599b8, (DAT_00527024 as u32).wrapping_mul(0x84)) = u_var9;
                        let p16 = g32!(DAT_005599bc, (DAT_00527024 as u32).wrapping_mul(0x84))
                            .cast::<u8>();
                        cp(p16, p13, u_stack_424);
                        let i_var3 = DAT_00527024;
                        let i_var7 = (DAT_00527024 as u32).wrapping_mul(0x210);
                        *g32!(DAT_005599a8, (DAT_00527024 as u32).wrapping_mul(0x84)) =
                            i_stack_428 as u32;
                        *g32!(DAT_005599ac, (i_var3 as u32).wrapping_mul(0x84)) =
                            u_stack_424.wrapping_add(4);
                        *g32!(DAT_005599b0, (i_var3 as u32).wrapping_mul(0x84)) = 0;
                        *g8!(DAT_005599b4, i_var7) = 0;
                        *g8!(DAT_005599b7, i_var7) = 1;
                        if u_stack_41c == 0 {
                            *g8!(DAT_005599b7, i_var7) = 0;
                        }
                        if u_stack_410 == 0 {
                            *g8!(DAT_005599b5, i_var7) = 2;
                            if u_var6 == 0 {
                                *g8!(DAT_005599b5, i_var7) = 1;
                            }
                        } else {
                            *g8!(DAT_005599b5, i_var7) = 0;
                        }
                        DAT_0052702c = DAT_0052702c.wrapping_add(1);
                        *g8!(DAT_005599b6, i_var7) = u_stack_420 as u8;
                        DAT_00527024 = (DAT_00527024 as u32).wrapping_add(1) as i32;
                        if 0x3ff < DAT_00527024 {
                            DAT_00527024 = 0;
                        }
                    }
                } else {
                    let b_var17 = *bufb.add(2) != 0;
                    if b_var17 {
                        u_stack_41c = (*bufb.add(2) == 2) as u32;
                    } else {
                        u_stack_41c = 0;
                    }
                    let mut p13: *mut u8 = bufb.add(3);
                    if i_stack_414 == 0 {
                        u_stack_424 = u_stack_418.wrapping_sub(2);
                    } else {
                        u_stack_424 = FUN_0046fad0();
                        if u_stack_424 == 0 {
                            p13 = bufb.add(5);
                            let w = bufb.add(3).cast::<u16>().read_unaligned();
                            u_stack_424 = w as u32;
                        }
                    }
                    if 0x1fc < u_stack_424 {
                        u_stack_424 = 0x1fc;
                    }
                    *g32!(DAT_005599b8, (DAT_00527024 as u32).wrapping_mul(0x84)) = u_var9;
                    let p16 =
                        g32!(DAT_005599bc, (DAT_00527024 as u32).wrapping_mul(0x84)).cast::<u8>();
                    cp(p16, p13, u_stack_424);
                    let i_var3 = DAT_00527024;
                    let i_var7 = (DAT_00527024 as u32).wrapping_mul(0x210);
                    *g32!(DAT_005599a8, (DAT_00527024 as u32).wrapping_mul(0x84)) =
                        i_stack_428 as u32;
                    *g32!(DAT_005599ac, (i_var3 as u32).wrapping_mul(0x84)) =
                        u_stack_424.wrapping_add(4);
                    *g32!(DAT_005599b0, (i_var3 as u32).wrapping_mul(0x84)) = 0;
                    *g8!(DAT_005599b4, i_var7) = 0;
                    *g8!(DAT_005599b7, i_var7) = 1;
                    u_var6 = FUN_0046fee0(i_stack_428);
                    let base: *mut u32 = if b_var17 {
                        if u_stack_41c == 0 {
                            *g8!(DAT_005599b5, (DAT_00527024 as u32).wrapping_mul(0x210)) = 1;
                            (&raw mut DAT_00999bf0).cast::<u32>()
                        } else {
                            *g8!(DAT_005599b5, (DAT_00527024 as u32).wrapping_mul(0x210)) = 2;
                            (&raw mut DAT_00999c00).cast::<u32>()
                        }
                    } else {
                        *g8!(DAT_005599b5, (DAT_00527024 as u32).wrapping_mul(0x210)) = 0;
                        (&raw mut DAT_00999bf8).cast::<u32>()
                    };
                    if u_var6 < DAT_0099f6e0 && u_var6 < 0x28 {
                        let p = base.add(u_var6.wrapping_mul(0x8e) as usize);
                        let mut v = (*p).wrapping_add(1);
                        if 0x7f < v {
                            v = 0;
                        }
                        u_var6 = v;
                        if v == u_stack_420 {
                            *p = v;
                        }
                    }
                    DAT_0052702c = DAT_0052702c.wrapping_add(1);
                    *g8!(DAT_005599b6, (DAT_00527024 as u32).wrapping_mul(0x210)) =
                        u_stack_420 as u8;
                    DAT_00527024 = (DAT_00527024 as u32).wrapping_add(1) as i32;
                    if 0x3ff < DAT_00527024 {
                        DAT_00527024 = 0;
                    }
                }
            }
        }
    }
}
