// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b6: u32;
    static mut DAT_00a08104: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_00a08294: u16;
    static mut DAT_009d1320: [i8; 0];
    static mut DAT_0051a310: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403540(a: u32, b: u32) -> u32;
    fn FUN_00484f80(a: u32) -> u32;
    fn FUN_00404450(a: u32) -> u32;
}

#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn g_ptr() -> u32 {
    // SAFETY: the global is the game's own, packed and possibly unaligned.
    unsafe { (&raw const DAT_009a73b6).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00467380() -> u32 {
    // SAFETY: all globals, callees and the addresses derived from them are
    // the game's own memory, accessed as the original does.
    unsafe {
        'reset: {
            let mut u_var6: u32 = g_ptr();
            let mut pbase: u32 = u_var6;
            let u_var2: u16 = r16(pbase.wrapping_add(0x18));
            let t32: u32 = DAT_00a08104;
            if r16(t32.wrapping_add(0x36e)) == u_var2 {
                break 'reset;
            }
            let mut i_var5: u32 = 'calc: {
                if u_var2 == 0xffff {
                    break 'calc 0;
                }
                let i_var3: u32 = (u_var2 as u32).wrapping_mul(0x23);
                let ptbl: u32 = DAT_009a1fe8;
                let pent: u32 = ptbl.wrapping_add(i_var3);
                let mut iv5: u32 = r32(pent.wrapping_add(0x1f));
                let b11: bool;
                if iv5 == 0 {
                    b11 = r8(pent.wrapping_add(3)) == 0;
                } else {
                    if r8(pent.wrapping_add(3)) == 0 {
                        break 'calc 0;
                    }
                    iv5 = r32(iv5.wrapping_add(0xa9));
                    if iv5 == 0 {
                        break 'calc 1;
                    }
                    let pobj: u32 = iv5;
                    if r8(pobj.wrapping_add(0x5e)) == 0x15 && r8(pobj.wrapping_add(0x5f)) != 0 {
                        break 'calc 0;
                    }
                    let c_var1 = r8(pobj.wrapping_add(7));
                    if c_var1 == 1 || c_var1 == 3 || c_var1 == 4 {
                        break 'calc 0;
                    }
                    if r8(pobj.wrapping_add(0x23d)) == 3 && r8(pobj.wrapping_add(0x241)) != 0 {
                        FUN_00403540(u_var2 as u32, DAT_009a73b0 as u32);
                        let c = r8((DAT_009a73b0 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(2)
                            .wrapping_add(DAT_009a1fe8));
                        if c == 0 || c == 1 || c == 2 {
                            if 0x8000 < DAT_009a1ff8 {
                                break 'calc 0;
                            }
                        } else if 0x10000 < DAT_009a1ff8 {
                            break 'calc 0;
                        }
                    }
                    let t = r32(DAT_009a1fe8.wrapping_add(i_var3).wrapping_add(0x1b));
                    if t != 0xffff_ffff {
                        break 'calc 1;
                    }
                    let a = r32(DAT_009a1fe8.wrapping_add(i_var3).wrapping_add(0x1f));
                    let b = r32(a.wrapping_add(0xa9));
                    let c = r32(b.wrapping_add(0xcb));
                    let d = r32(pobj.wrapping_add(0xc3));
                    b11 = c < d;
                }
                1u32.wrapping_sub(b11 as u32)
            };

            u_var6 = g_ptr();
            if i_var5 == 0 {
                break 'reset;
            }
            let u_var4: u32 = u_var2 as u32;
            let pent: u32 = u_var4.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
            pbase = u_var6;
            let tmp_a = r16(pbase.wrapping_add(0x1a));
            let tmp_b = r16(pent);
            let mut b11: bool = tmp_a == tmp_b;
            if !b11 {
                break 'reset;
            }
            i_var5 = 0xf;
            let mut p9: *const i8 = (&raw const DAT_009d1320)
                .cast::<i8>()
                .byte_add((r8(pbase.wrapping_add(0xb)) as usize).wrapping_mul(0x55));
            let mut p10: *const i8 = (&raw const DAT_0051a310).cast::<i8>();
            loop {
                if i_var5 == 0 {
                    break;
                }
                i_var5 = i_var5.wrapping_sub(1);
                b11 = *p9 == *p10;
                p9 = p9.add(1);
                p10 = p10.add(1);
                if !b11 {
                    break;
                }
            }
            if b11 && (u_var4 as i32) < DAT_009a1fe0 {
                let a = r32(pent.wrapping_add(0x1f));
                let b = r32(a.wrapping_add(0xa9));
                let v = r16(b.wrapping_add(0x1e5));
                if v == 0 {
                    w16(pbase.wrapping_add(0x2b), 0xffff);
                    return (u_var6 & 0xffff_0000) | 1;
                }
            }
            u_var6 = r32(pent.wrapping_add(0x1b));
            if u_var6 != 0xffff_ffff {
                u_var6 = DAT_009a1fe0 as u32;
                if (u_var4 as i32) < DAT_009a1fe0 {
                    u_var6 = FUN_00484f80(u_var2 as u32);
                    if u_var6 as u8 == 1 {
                        u_var6 = FUN_00403540(DAT_009a73b0 as u32, u_var4);
                        if 0x4000 < DAT_009a1ff8 {
                            break 'reset;
                        }
                    }
                }
            }
            if DAT_00a08294 == 0xe {
                let e2: u32 = DAT_009a1fe8.wrapping_add(u_var4.wrapping_mul(0x23));
                let mut s_var7: i16 = r16(e2.wrapping_add(0x17)) as i16;
                let i5 = r32(e2.wrapping_add(0x1f));
                u_var6 = 0;
                if i5 != 0 {
                    u_var6 = r32(i5.wrapping_add(0xa9));
                    if u_var6 != 0 {
                        s_var7 = r16(u_var6.wrapping_add(0x1e5)) as i16;
                    }
                }
                if s_var7 == 0 {
                    u_var6 = g_ptr();
                    let v2 = r16(u_var6.wrapping_add(0x2b));
                    if v2 != u_var2 {
                        u_var6 = FUN_00404450(u_var2 as u32);
                        if (u_var6 as i16) == 0 {
                            break 'reset;
                        }
                    }
                }
            }
            return u_var6 & 0xffff_0000;
        }
        let u_var6 = g_ptr();
        w16(u_var6.wrapping_add(0x18), 0xffff);
        w16(u_var6.wrapping_add(0x1a), 0);
        (u_var6.wrapping_add(0x1e) as *mut u8).write(0);
        let u_var6 = g_ptr();
        w16(u_var6.wrapping_add(0x2b), 0xffff);
        (u_var6 & 0xffff_0000) | 1
    }
}
