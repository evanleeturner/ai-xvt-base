// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73da: u8;
    static mut DAT_009a73db: [u8; 4];
    static mut DAT_009a73df: [u8; 4];
    static mut DAT_009a73e3: [u8; 4];
    static mut DAT_00a08104: i32;
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433390(a: u32, b: u16, c: u16) -> u8;
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_00403070(a: u32) -> u8;
    fn FUN_0045dba0(a: u16, b: i32) -> u8;
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> u32;
}

#[inline(always)]
unsafe fn ld(a: u32) -> u32 {
    // SAFETY: the address is a game-memory pointer, read where the original does.
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rw(a: u32) -> u16 {
    // SAFETY: the address is a game-memory pointer, read where the original does.
    unsafe { (a as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rb(a: u32) -> u8 {
    // SAFETY: the address is a game-memory pointer, read where the original does.
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn g8() -> u32 {
    // SAFETY: the global is the game's own.
    unsafe { DAT_009a1fe8 as u32 }
}

#[inline(always)]
unsafe fn tab1(i: u32) -> u8 {
    // SAFETY: the table is the game's own memory.
    unsafe {
        (&raw const DAT_009d8c88)
            .cast::<u8>()
            .add(i as usize)
            .read()
    }
}

#[inline(always)]
unsafe fn tab2(i: u32) -> u8 {
    // SAFETY: the table is the game's own memory.
    unsafe {
        (&raw const DAT_009fd458)
            .cast::<u8>()
            .add(i as usize)
            .read()
    }
}

#[inline(always)]
unsafe fn chk(iv2: u32) -> bool {
    // SAFETY: game memory and globals, read as the original does.
    unsafe {
        let t1 = ld((DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(g8()));
        let y = rb(t1.wrapping_add(0x89)) as u32;
        let x =
            tab1((rb(g8().wrapping_add(iv2).wrapping_add(0x16)) as u32).wrapping_mul(0x566)) as u32;
        if x == y {
            false
        } else {
            tab2(x.wrapping_add(y.wrapping_mul(0x1e5))) == 0
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045d6e0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    // SAFETY: all globals, tables and pointers are the game's own memory and
    // are accessed in the order the original does.
    unsafe {
        let mut unaff_edi: u32 = 0;
        let mut local_e: u16 = DAT_00a07cec;
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_10: u16 = 0xffff;
        if (local_e as i32) < DAT_009a1fe0 {
            loop {
                let mut i5: u32 = (local_e as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8 as u32);
                if rb(i5.wrapping_add(3)) != 0 {
                    let mut u10: u32 = 0;
                    let t0 = ld(i5.wrapping_add(0x1f));
                    i5 = ld(t0.wrapping_add(0xa9));
                    let mut flag = false;
                    let ustack_c = i5 as u16;
                    loop {
                        if rb(u10.wrapping_add(0x1ec).wrapping_add(i5)) != 0 {
                            flag = true;
                        }
                        let u9 = (u10 as u16).wrapping_add(1);
                        u10 = u9 as u32;
                        if !(u9 < 10) {
                            break;
                        }
                    }
                    if flag {
                        let packed = ((ustack_c as u32) << 16) | local_e as u32;
                        let r1 = FUN_00433390(packed, param_1, param_2) as u16;
                        let r2 = FUN_00433390(packed, param_4, param_5) as u16;
                        let res = if param_3 == 1 { r1 | r2 } else { r1 & r2 };
                        if res != 0 {
                            unaff_edi = (unaff_edi & 0xffff_0000) | DAT_00a07cec as u32;
                            if (DAT_00a07cec as i32) < DAT_009a1fe0 {
                                loop {
                                    let mut uvar10: u32 = unaff_edi & 0xffff;
                                    let iv2: u32 = uvar10.wrapping_mul(0x23);
                                    let uvar9: u16 = unaff_edi as u16;
                                    'cont: {
                                        if rb(g8().wrapping_add(iv2).wrapping_add(3)) == 0 {
                                            break 'cont;
                                        }
                                        let t0 = ld(g8().wrapping_add(iv2).wrapping_add(0x1f));
                                        let i6 = ld(t0.wrapping_add(0xa9));
                                        let c1 = rb(i6.wrapping_add(0x5e));
                                        let cond = (((c1 == 0x0b || c1 == 0x0c || c1 == 0x17)
                                            && rw(i6.wrapping_add(0x42)) == local_e)
                                            || rw(i5.wrapping_add(0x6e)) == uvar9)
                                            && rw((DAT_00a08104 as u32).wrapping_add(0x36e))
                                                != local_e;
                                        if !cond {
                                            break 'cont;
                                        }
                                        let r6: i32 = 'calc: {
                                            if uvar9 == 0xffff {
                                                break 'calc 0;
                                            }
                                            let p = uvar10.wrapping_mul(0x23).wrapping_add(g8());
                                            let t0 = ld(p.wrapping_add(0x1f));
                                            if t0 == 0 {
                                                let b = (rb(p.wrapping_add(3)) == 0) as i32;
                                                break 'calc 1 - b;
                                            }
                                            if rb(p.wrapping_add(3)) == 0 {
                                                break 'calc 0;
                                            }
                                            let t0 = ld(p.wrapping_add(0x1f));
                                            let q = ld(t0.wrapping_add(0xa9));
                                            if q == 0 {
                                                break 'calc 1;
                                            }
                                            if rb(q.wrapping_add(0x5e)) == 0x15
                                                && rb(q.wrapping_add(0x5f)) != 0
                                            {
                                                break 'calc 0;
                                            }
                                            let c = rb(q.wrapping_add(7));
                                            if c == 1 || c == 3 || c == 4 {
                                                break 'calc 0;
                                            }
                                            if rb(q.wrapping_add(0x23d)) == 3
                                                && rb(q.wrapping_add(0x241)) != 0
                                            {
                                                FUN_00403540(uvar10, DAT_009a73b0 as u32);
                                                let c2 = rb((DAT_009a73b0 as u32)
                                                    .wrapping_mul(0x23)
                                                    .wrapping_add(2)
                                                    .wrapping_add(g8()));
                                                if c2 == 0 || c2 == 1 || c2 == 2 {
                                                    if (DAT_009a1ff8 as i32) >= 0x8001 {
                                                        break 'calc 0;
                                                    }
                                                } else if (DAT_009a1ff8 as i32) >= 0x10001 {
                                                    break 'calc 0;
                                                }
                                            }
                                            let i7 = g8().wrapping_add(uvar10.wrapping_mul(0x23));
                                            if ld(i7.wrapping_add(0x1b)) as i32 == -1 {
                                                let t1 = ld(i7.wrapping_add(0x1f));
                                                let t2 = ld(t1.wrapping_add(0xa9));
                                                let u0 = ld(t2.wrapping_add(0xcb));
                                                let u1 = ld(q.wrapping_add(0xc3));
                                                break 'calc 1 - (u0 < u1) as i32;
                                            }
                                            1
                                        };
                                        if r6 == 0 {
                                            break 'cont;
                                        }
                                        if ld(g8().wrapping_add(iv2).wrapping_add(0x1b)) as i32
                                            != -1
                                        {
                                            if !chk(iv2) {
                                                break 'cont;
                                            }
                                        }
                                        if (DAT_009a73da & 4) != 0 {
                                            let b = FUN_00403070(unaff_edi);
                                            if b == 0 {
                                                break 'cont;
                                            }
                                        }
                                        if (DAT_009a73da & 1) != 0 {
                                            let b = FUN_0045dba0(uvar9, 0xff);
                                            if b == 0 {
                                                break 'cont;
                                            }
                                        }
                                        if (DAT_009a73da & 0x20) == 0 {
                                            FUN_00403540(DAT_009a73b0 as u32, uvar10);
                                        } else {
                                            let i6 = g8().wrapping_add(iv2);
                                            let t0 = ld(i6.wrapping_add(4));
                                            let d0 = (&raw const DAT_009a73db)
                                                .cast::<u32>()
                                                .read_unaligned();
                                            let t1 = ld(i6.wrapping_add(8));
                                            let d1 = (&raw const DAT_009a73df)
                                                .cast::<u32>()
                                                .read_unaligned();
                                            let t2 = ld(i6.wrapping_add(0xc));
                                            let d2 = (&raw const DAT_009a73e3)
                                                .cast::<u32>()
                                                .read_unaligned();
                                            DAT_009a1ff8 = FUN_0041f410(
                                                t0.wrapping_sub(d0) as i32,
                                                t1.wrapping_sub(d1) as i32,
                                                t2.wrapping_sub(d2) as i32,
                                            );
                                        }
                                        let update;
                                        if (DAT_009a73da & 0x10) == 0 {
                                            update = true;
                                        } else if (DAT_009a1ff8 as i32) < 0x10001 {
                                            uvar10 = 0;
                                            update = chk(iv2);
                                            let _ = uvar10;
                                        } else {
                                            update = false;
                                        }
                                        if update && DAT_009a1ff8 < local_4 {
                                            local_4 = DAT_009a1ff8;
                                            local_10 = uvar9;
                                        }
                                    }
                                    let nx = uvar9.wrapping_add(1);
                                    unaff_edi = (unaff_edi & 0xffff_0000) | nx as u32;
                                    if !((nx as i32) < DAT_009a1fe0) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                local_e = local_e.wrapping_add(1);
                if !((local_e as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        local_10
    }
}
