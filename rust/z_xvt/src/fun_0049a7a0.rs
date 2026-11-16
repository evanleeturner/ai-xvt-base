// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e48: i32;
    static mut DAT_00525550: i32;
    static mut DAT_00525554: i32;
    static mut DAT_0071b71c: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0071e6be: u16;
    static mut DAT_0070a1d2: i16;
    static mut DAT_0071e690: i32;
    static mut DAT_0071ecf4: u32;
    static mut DAT_0073290c: i32;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_006db730: [i32; 1];
    static mut DAT_006da500: [u8; 1];
    static mut DAT_0071f720: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00444a30();
    fn FUN_004253a0();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439a30(param_1: u32);
}

#[inline(always)]
unsafe fn ld32(p: *const u8) -> i32 {
    // SAFETY: the caller passes an address inside the game's own records.
    unsafe { p.cast::<i32>().read_unaligned() }
}
#[inline(always)]
unsafe fn st32(p: *mut u8, v: i32) {
    // SAFETY: the caller passes an address inside the game's own records.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}
#[inline(always)]
unsafe fn ld16(p: *const u8) -> i16 {
    // SAFETY: the caller passes an address inside the game's own records.
    unsafe { p.cast::<i16>().read_unaligned() }
}
#[inline(always)]
unsafe fn st16(p: *mut u8, v: i16) {
    // SAFETY: the caller passes an address inside the game's own records.
    unsafe { p.cast::<i16>().write_unaligned(v) }
}
#[inline(always)]
fn ad(i: i32) -> *mut u8 {
    i as u32 as usize as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049a7a0(param_1: i32) {
    // SAFETY: all globals, records and callees are the game's own memory;
    // pointers are walked and written exactly as the original does.
    unsafe {
        let i_stack_20: i32 = if DAT_00732e48 == 1 { 1 } else { DAT_00525550 };
        let mut pvar11: *mut u8 = (&raw mut DAT_0071b730).cast::<u8>();
        let mut pvar10: *mut i32 = (&raw mut DAT_006db730).cast::<i32>();
        let mut u_stack_14: u32 = 0;
        let mut pstack_1c: *mut u8 = (&raw mut DAT_006da500).cast::<u8>();
        let mut pstack_18: *mut u8 = (&raw mut DAT_0071f720).cast::<u8>();
        loop {
            if *pvar11.add(0x10) != 0 {
                let ivar1 = *pvar10;
                let mut i_stack_10: i32 = 0;
                let mut pvar9: *mut u8 = pstack_18;
                if 0 < ivar1 {
                    'inner: loop {
                        'body: {
                            let ivar5 = DAT_0071b71c;
                            let ivar8 = ld32(pvar9);
                            let mut ivar10 = ld32(pvar9.add(8));
                            let ivar13 = ld32(pvar11.add(0x58c));
                            if (i_stack_20 == 0 || DAT_00732e48 == 1)
                                && ivar10 <= ivar13
                                && ivar8 == 0
                            {
                                if *pvar10 != 0 && pstack_18 <= pvar9 {
                                    let n = (*pvar10 as u32).wrapping_sub(1) as i32;
                                    let mut ivar6: i32 = 0;
                                    *pvar10 = n;
                                    let mut pvar8: *mut u8 = pstack_18;
                                    if 0 < n {
                                        loop {
                                            if pvar9 <= pvar8 {
                                                st32(pvar8, ld32(pvar8.add(0x12)));
                                                st32(pvar8.add(4), ld32(pvar8.add(0x16)));
                                                st32(pvar8.add(8), ld32(pvar8.add(0x1a)));
                                                st32(pvar8.add(0xc), ld32(pvar8.add(0x1e)));
                                                st16(pvar8.add(0x10), ld16(pvar8.add(0x22)));
                                            }
                                            ivar6 = ivar6.wrapping_add(1);
                                            pvar8 = pvar8.wrapping_add(0x12);
                                            if !(ivar6 < *pvar10) {
                                                break;
                                            }
                                        }
                                    }
                                }
                                pvar9 = pvar9.wrapping_sub(0x12);
                            } else {
                                let mut ivar6 = ivar10;
                                DAT_0071b71c = ivar5;
                                let ivar9 = ld32(pvar9.add(4));
                                if !(ivar6 <= param_1 && (i_stack_20 != 0 || ivar9 == 0)) {
                                    break 'body;
                                }
                                let ivar2 = ld32(pvar11.add(0x58c));
                                if !(ivar2 < ivar6) {
                                    break 'body;
                                }
                                let mut ivar14 = ld32(pvar11);
                                if ivar6 <= DAT_0071b71c && ivar14 != -1 {
                                    let mut psvar12 = ad((DAT_006d4988 as u32)
                                        .wrapping_add((ivar14 as u32).wrapping_mul(0x23))
                                        as i32);
                                    ivar6 = ld32(psvar12.add(0x1f));
                                    DAT_0071b71c = ivar5;
                                    let mut ivar15: i32 = 0;
                                    if ivar6 != 0 {
                                        ivar15 = ld32(ad(ivar6).add(0xa7));
                                    }
                                    if ivar6 == 0 || ivar15 == 0 {
                                        break 'body;
                                    }
                                    let svar16 = ld16(pvar11.add(0x5aa));
                                    let svar17 = ld16(psvar12);
                                    if svar16 == svar17 && *pvar11.add(0x5ac) == *pvar11.add(0x11) {
                                        let ivar15 = ld32(ad(ivar6).add(2));
                                        if ivar2 < ivar15 {
                                            st32(ad(ivar6).add(2), ivar2);
                                            ivar14 = ld32(pvar11);
                                            psvar12 = ad((DAT_006d4988 as u32)
                                                .wrapping_add((ivar14 as u32).wrapping_mul(0x23))
                                                as i32);
                                            st32(psvar12.add(4), ld32(pvar11.add(0x590)));
                                            st32(psvar12.add(8), ld32(pvar11.add(0x594)));
                                            st32(psvar12.add(0xc), ld32(pvar11.add(0x598)));
                                            st16(psvar12.add(0x14), ld32(pvar11.add(0x59c)) as i16);
                                            st16(psvar12.add(0x12), ld16(pvar11.add(0x59e)));
                                            st16(psvar12.add(0x10), ld32(pvar11.add(0x5a0)) as i16);
                                            let pb19 = ad(ld32(psvar12.add(0x1f)));
                                            st16(pb19.add(0x7f), ld16(pvar11.add(0x5a2)));
                                            st16(pb19.add(0x77), ld32(pvar11.add(0x5a8)) as i16);
                                            st16(pb19.add(0x79), ld32(pvar11.add(0x5a4)) as i16);
                                            st16(pb19.add(0x7b), ld16(pvar11.add(0x5a6)));
                                            let pb20 = ad(ld32(pb19.add(0xa7)));
                                            st16(pb20.add(0xd), ld16(pvar11.add(0x59e)));
                                            *pb19.add(0x90) = 1;
                                            *pb19.add(0x89) = 1;
                                        }
                                        ivar14 = ld32(pvar11);
                                        psvar12 = ad((DAT_006d4988 as u32)
                                            .wrapping_add((ivar14 as u32).wrapping_mul(0x23))
                                            as i32);
                                        let ivar21 = ld32(psvar12.add(0x1f));
                                        let ivar6b = ld32(ad(ivar21).add(2));
                                        if ivar6b < DAT_0071b71c {
                                            DAT_0071b71c = ivar6b;
                                        }
                                    } else {
                                        if i_stack_20 != 0 {
                                            break 'body;
                                        }
                                        let ivar7 = (DAT_0071b71c as u32).wrapping_add(4) as i32;
                                        st32(pvar9.add(8), ivar7);
                                        st32(pvar11.add(0x58c), DAT_0071b71c);
                                    }
                                }
                                let u_var4 = DAT_0071e6be;
                                let u_var3 = DAT_0070a1d2 as u16;
                                let ivar6 = ld32(pvar11);
                                if ivar6 != -1 {
                                    let mut psvar12 = ad((DAT_006d4988 as u32)
                                        .wrapping_add((ivar6 as u32).wrapping_mul(0x23))
                                        as i32);
                                    let ivar21 = ld32(psvar12.add(0x1f));
                                    if ivar21 != 0 {
                                        ivar10 = ld32(pvar9.add(8));
                                        DAT_0071e6be = (ivar10 as i16)
                                            .wrapping_sub(DAT_0071b71c as i16)
                                            as u16;
                                        if DAT_0071e6be == 0 {
                                            DAT_0070a1d2 = 0xec;
                                        } else {
                                            DAT_0070a1d2 = (0xecu32 / DAT_0071e6be as u32) as i16;
                                        }
                                        if DAT_0070a1d2 == 0 {
                                            DAT_0070a1d2 = 1;
                                        }
                                        DAT_0071e690 = ivar6;
                                        FUN_00444a30();
                                        FUN_004253a0();
                                        psvar12 = ad((DAT_0071e690 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(DAT_006d4988 as u32)
                                            as i32);
                                        let ivar21 = ld32(psvar12.add(0x1f));
                                        ivar10 = ld32(pvar9.add(8));
                                        st32(ad(ivar21).add(2), ivar10);
                                        psvar12 = ad((DAT_0071e690 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(DAT_006d4988 as u32)
                                            as i32);
                                        st32(pvar11.add(0x590), ld32(psvar12.add(4)));
                                        st32(pvar11.add(0x594), ld32(psvar12.add(8)));
                                        st32(pvar11.add(0x598), ld32(psvar12.add(0xc)));
                                        st16(pvar11.add(0x59c), ld16(psvar12.add(0x14)));
                                        st16(pvar11.add(0x59e), ld16(psvar12.add(0x12)));
                                        st16(pvar11.add(0x5a0), ld16(psvar12.add(0x10)));
                                        let pb19 = ad(ld32(psvar12.add(0x1f)));
                                        st16(pvar11.add(0x5a2), ld16(pb19.add(0x7f)));
                                        st16(pvar11.add(0x5a8), ld16(pb19.add(0x77)));
                                        st16(pvar11.add(0x5a4), ld16(pb19.add(0x79)));
                                        st16(pvar11.add(0x5a6), ld16(pb19.add(0x7b)));
                                        st16(pvar11.add(0x5aa), ld16(psvar12));
                                        *pvar11.add(0x5ac) = *pvar11.add(0x11);
                                    }
                                    DAT_0071e690 = -1;
                                }
                                ivar10 = ld32(pvar9.add(8));
                                let ivar7 = ld32(pvar11.add(0x58c));
                                DAT_0071e6be = (ivar10 as i16).wrapping_sub(ivar7 as i16) as u16;
                                if DAT_0071e6be < 4 {
                                    DAT_0071e6be = 4;
                                }
                                if DAT_0071e6be == 0 {
                                    DAT_0070a1d2 = 0xec;
                                } else {
                                    DAT_0070a1d2 = (0xecu32 / DAT_0071e6be as u32) as i16;
                                }
                                if DAT_0070a1d2 == 0 {
                                    DAT_0070a1d2 = 1;
                                }
                                ivar10 = ld32(pvar9.add(8));
                                st32(pvar11.add(0x58c), ivar10);
                                let ivar11 = ld32(pvar9.add(0xc));
                                st32(pstack_1c, ivar11);
                                let ivar12 = ld32(pvar9.add(0x10));
                                st16(pstack_1c.add(4), ivar12 as i16);
                                ivar10 = ld32(pvar9.add(8));
                                if i_stack_20 != 0 && ivar10 <= ivar5 {
                                    *pstack_1c.add(2) = 0;
                                    *pstack_1c.add(5) = 0;
                                }
                                ivar10 = ld32(pvar9.add(8));
                                if 1 < DAT_00732e48 && u_stack_14 == DAT_0071ecf4 {
                                    DAT_00525554 = 2;
                                    if DAT_0073290c < ivar10 {
                                        DAT_00525554 = 1;
                                        ivar10 = ld32(pvar9.add(8));
                                        DAT_0073290c = ivar10;
                                    }
                                }
                                FUN_00439a30(u_stack_14);
                                DAT_00525554 = 0;
                                DAT_0070a1d2 = u_var3 as i16;
                                DAT_0071b71c = ivar5;
                                DAT_0071e6be = u_var4;
                                if *pvar11.add(0x10) == 0 {
                                    break 'inner;
                                }
                            }
                        }
                        pvar9 = pvar9.wrapping_add(0x12);
                        i_stack_10 = i_stack_10.wrapping_add(1);
                        if !(i_stack_10 < ivar1) {
                            break;
                        }
                    }
                }
            }
            pvar11 = pvar11.wrapping_add(0x5bd);
            pstack_1c = pstack_1c.wrapping_add(6);
            pstack_18 = pstack_18.wrapping_add(0x7e9 * 4);
            pvar10 = pvar10.wrapping_add(1);
            u_stack_14 = u_stack_14.wrapping_add(1);
            if (pvar10 as usize) > 0x6db74f {
                return;
            }
        }
    }
}
