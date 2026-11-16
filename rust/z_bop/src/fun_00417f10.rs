// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a080f8: i32;
    static mut DAT_00523410: i32;
    static mut DAT_00523414: i32;
    static mut DAT_00a0085c: i32;
    static mut DAT_009e9658: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009ec5fe: i16;
    static mut DAT_009d8112: i16;
    static mut DAT_009ec5d0: i32;
    static mut DAT_009ecc34: *mut u8;
    static mut DAT_009a7b70: [u8; 0];
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009ed670: [u8; 0];
    static mut DAT_009a8db0: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004186e0(a: i32, b: *mut i32);
    fn FUN_0047a5e0(a: *mut u8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ace80();
    fn FUN_00445570();
}

unsafe fn ld32(p: *const u8) -> i32 {
    // SAFETY: caller supplies a valid game pointer.
    unsafe { p.cast::<i32>().read_unaligned() }
}
unsafe fn ld16(p: *const u8) -> i16 {
    // SAFETY: caller supplies a valid game pointer.
    unsafe { p.cast::<i16>().read_unaligned() }
}
unsafe fn st32(p: *mut u8, v: i32) {
    // SAFETY: caller supplies a valid game pointer.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}
unsafe fn st16(p: *mut u8, v: i16) {
    // SAFETY: caller supplies a valid game pointer.
    unsafe { p.cast::<i16>().write_unaligned(v) }
}
unsafe fn rec(i: i32) -> *mut u8 {
    // SAFETY: reads the game's record table pointer.
    unsafe { DAT_009a1fe8.wrapping_add((i as u32).wrapping_mul(0x23) as usize) }
}
unsafe fn inner(r: *mut u8) -> *mut u8 {
    // SAFETY: r points at a record.
    unsafe { ld32(r.wrapping_add(0x1f)) as u32 as usize as *mut u8 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00417f10(param_1: i32) {
    // SAFETY: all globals and records are the game's own memory, accessed
    // where and in the order the original does.
    unsafe {
        let i_stack_20: i32 = if DAT_00a080f8 == 1 { 1 } else { DAT_00523410 };
        let mut p9: *mut u8 = (&raw mut DAT_009a7b70).cast::<u8>();
        let mut p11: *mut u8 = (&raw mut DAT_009e9670).cast::<u8>();
        let mut idx: u32 = 0;
        let mut p10: *mut u8 = (&raw mut DAT_009ed670).cast::<u8>();
        let mut p18: *mut i32 = (&raw mut DAT_009a8db0).cast::<i32>();
        loop {
            if *p11.wrapping_add(0x10) != 0 {
                let mut i_stack_14: i32 = 0;
                let i_var1: i32 = p18.read();
                let mut p8: *mut u8 = p10;
                if 0 < i_var1 {
                    'inner: loop {
                        'cont: {
                            let i_var5 = DAT_009e9658;
                            if (i_stack_20 == 0 || DAT_00a080f8 == 1)
                                && ld32(p8.wrapping_add(8)) <= ld32(p11.wrapping_add(0x58c))
                                && ld32(p8) == 0
                            {
                                FUN_004186e0(idx as i32, p8.cast::<i32>());
                                p8 = p8.wrapping_sub(0x12);
                            } else {
                                let mut i_var2 = ld32(p8.wrapping_add(8));
                                DAT_009e9658 = i_var5;
                                if i_var2 <= param_1
                                    && (i_stack_20 != 0 || ld32(p8.wrapping_add(4)) == 0)
                                    && ld32(p11.wrapping_add(0x58c)) < i_var2
                                {
                                    let i_var3 = ld32(p11.wrapping_add(0x58c));
                                    if i_var2 <= DAT_009e9658 && ld32(p11) != -1 {
                                        let psvar10 = rec(ld32(p11));
                                        i_var2 = ld32(psvar10.wrapping_add(0x1f));
                                        DAT_009e9658 = i_var5;
                                        if i_var2 == 0
                                            || ld32(
                                                (i_var2 as u32 as usize as *const u8)
                                                    .wrapping_add(0xa9),
                                            ) == 0
                                        {
                                            break 'cont;
                                        }
                                        if ld16(p11.wrapping_add(0x5aa)) == ld16(psvar10)
                                            && *p11.wrapping_add(0x5ac) == *p11.wrapping_add(0x11)
                                        {
                                            if i_var3
                                                < ld32(
                                                    (i_var2 as u32 as usize as *const u8)
                                                        .wrapping_add(2),
                                                )
                                            {
                                                st32(
                                                    (i_var2 as u32 as usize as *mut u8)
                                                        .wrapping_add(2),
                                                    i_var3,
                                                );
                                                st32(
                                                    rec(ld32(p11)).wrapping_add(4),
                                                    ld32(p11.wrapping_add(0x590)),
                                                );
                                                st32(
                                                    rec(ld32(p11)).wrapping_add(8),
                                                    ld32(p11.wrapping_add(0x594)),
                                                );
                                                st32(
                                                    rec(ld32(p11)).wrapping_add(0xc),
                                                    ld32(p11.wrapping_add(0x598)),
                                                );
                                                st16(
                                                    rec(ld32(p11)).wrapping_add(0x14),
                                                    ld16(p11.wrapping_add(0x59c)),
                                                );
                                                st16(
                                                    rec(ld32(p11)).wrapping_add(0x12),
                                                    ld16(p11.wrapping_add(0x59e)),
                                                );
                                                st16(
                                                    rec(ld32(p11)).wrapping_add(0x10),
                                                    ld16(p11.wrapping_add(0x5a0)),
                                                );
                                                st16(
                                                    inner(rec(ld32(p11))).wrapping_add(0x81),
                                                    ld16(p11.wrapping_add(0x5a2)),
                                                );
                                                st16(
                                                    inner(rec(ld32(p11))).wrapping_add(0x77),
                                                    ld16(p11.wrapping_add(0x5a8)),
                                                );
                                                st16(
                                                    inner(rec(ld32(p11))).wrapping_add(0x79),
                                                    ld16(p11.wrapping_add(0x5a4)),
                                                );
                                                st16(
                                                    inner(rec(ld32(p11))).wrapping_add(0x7b),
                                                    ld16(p11.wrapping_add(0x5a6)),
                                                );
                                                let q =
                                                    ld32(inner(rec(ld32(p11))).wrapping_add(0xa9))
                                                        as u32
                                                        as usize
                                                        as *mut u8;
                                                st16(
                                                    q.wrapping_add(0xd),
                                                    ld16(p11.wrapping_add(0x59e)),
                                                );
                                                *inner(rec(ld32(p11))).wrapping_add(0x92) = 1;
                                                *inner(rec(ld32(p11))).wrapping_add(0x8b) = 1;
                                            }
                                            let v = ld32(inner(rec(ld32(p11))).wrapping_add(2));
                                            if v < DAT_009e9658 {
                                                DAT_009e9658 = v;
                                            }
                                        } else {
                                            if i_stack_20 != 0 {
                                                break 'cont;
                                            }
                                            st32(p8.wrapping_add(8), DAT_009e9658.wrapping_add(4));
                                            st32(p11.wrapping_add(0x58c), DAT_009e9658);
                                        }
                                    }
                                    let u_var6 = DAT_009ec5fe;
                                    let u_var4 = DAT_009d8112;
                                    i_var2 = ld32(p11);
                                    if i_var2 != -1 {
                                        if ld32(rec(i_var2).wrapping_add(0x1f)) != 0 {
                                            DAT_009ec5fe = (ld32(p8.wrapping_add(8)) as i16)
                                                .wrapping_sub(DAT_009e9658 as i16);
                                            if DAT_009ec5fe == 0 {
                                                DAT_009d8112 = 0xec;
                                            } else {
                                                DAT_009d8112 =
                                                    (0xecu64 / (DAT_009ec5fe as u16 as u64)) as i16;
                                            }
                                            if DAT_009d8112 == 0 {
                                                DAT_009d8112 = 1;
                                            }
                                            DAT_009ec5d0 = i_var2;
                                            FUN_004ace80();
                                            FUN_00445570();
                                            st32(
                                                inner(rec(DAT_009ec5d0)).wrapping_add(2),
                                                ld32(p8.wrapping_add(8)),
                                            );
                                            let p7 = rec(DAT_009ec5d0);
                                            st32(p11.wrapping_add(0x590), ld32(p7.wrapping_add(4)));
                                            st32(p11.wrapping_add(0x594), ld32(p7.wrapping_add(8)));
                                            st32(
                                                p11.wrapping_add(0x598),
                                                ld32(p7.wrapping_add(0xc)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x59c),
                                                ld16(p7.wrapping_add(0x14)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x59e),
                                                ld16(p7.wrapping_add(0x12)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x5a0),
                                                ld16(p7.wrapping_add(0x10)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x5a2),
                                                ld16(inner(p7).wrapping_add(0x81)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x5a8),
                                                ld16(inner(p7).wrapping_add(0x77)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x5a4),
                                                ld16(inner(p7).wrapping_add(0x79)),
                                            );
                                            st16(
                                                p11.wrapping_add(0x5a6),
                                                ld16(inner(p7).wrapping_add(0x7b)),
                                            );
                                            st16(p11.wrapping_add(0x5aa), ld16(p7));
                                            *p11.wrapping_add(0x5ac) = *p11.wrapping_add(0x11);
                                        }
                                        DAT_009ec5d0 = -1;
                                    }
                                    DAT_009ec5fe = (ld32(p8.wrapping_add(8)) as i16)
                                        .wrapping_sub(ld32(p11.wrapping_add(0x58c)) as i16);
                                    if DAT_009ec5fe < 4 {
                                        DAT_009ec5fe = 4;
                                    }
                                    if DAT_009ec5fe == 0 {
                                        DAT_009d8112 = 0xec;
                                    } else {
                                        DAT_009d8112 =
                                            (0xecu64 / (DAT_009ec5fe as u16 as u64)) as i16;
                                    }
                                    if DAT_009d8112 == 0 {
                                        DAT_009d8112 = 1;
                                    }
                                    st32(p11.wrapping_add(0x58c), ld32(p8.wrapping_add(8)));
                                    st32(p9, ld32(p8.wrapping_add(0xc)));
                                    st16(p9.wrapping_add(4), ld32(p8.wrapping_add(0x10)) as i16);
                                    if i_stack_20 != 0 && ld32(p8.wrapping_add(8)) <= i_var5 {
                                        *p9.wrapping_add(2) = 0;
                                        *p9.wrapping_add(5) = 0;
                                    }
                                    if 1 < DAT_00a080f8 && idx == (DAT_009ecc34 as usize as u32) {
                                        DAT_00523414 = 2;
                                        if DAT_00a0085c < ld32(p8.wrapping_add(8)) {
                                            DAT_00523414 = 1;
                                            DAT_00a0085c = ld32(p8.wrapping_add(8));
                                        }
                                    }
                                    FUN_0047a5e0(idx as usize as *mut u8);
                                    DAT_00523414 = 0;
                                    DAT_009d8112 = u_var4;
                                    DAT_009e9658 = i_var5;
                                    DAT_009ec5fe = u_var6;
                                    if *p11.wrapping_add(0x10) == 0 {
                                        break 'inner;
                                    }
                                }
                            }
                        }
                        p8 = p8.wrapping_add(0x12);
                        i_stack_14 = i_stack_14.wrapping_add(1);
                        if !(i_stack_14 < i_var1) {
                            break;
                        }
                    }
                }
            }
            p9 = p9.wrapping_add(6);
            p11 = p11.wrapping_add(0x5bd);
            p10 = p10.wrapping_add(0x7e9 * 4);
            p18 = p18.wrapping_add(1);
            idx = idx.wrapping_add(1);
            if (p18 as usize) > 0x9a8dcf {
                return;
            }
        }
    }
}
