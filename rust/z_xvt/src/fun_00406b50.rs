// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e42: u16;
    static mut DAT_00525738: i32;
    static mut DAT_00525518: i32;
    static mut DAT_00523738: [u16; 1];
    static mut DAT_00523620: [u16; 1];
    static mut DAT_00523850: [u16; 1];
    static mut DAT_0071ed00: u16;
    static mut DAT_0070acec: u16;
    static mut DAT_00732a20: u16;
    static mut DAT_0071f2e0: u16;
    static mut DAT_0071b6b2: i16;
    static mut DAT_006db6d0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004183f0(a: u16) -> u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[inline(always)]
unsafe fn div32(n: u32, d: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own unsigned divide; faults on a zero divisor as the original does.
    unsafe {
        asm!("div {d}", d = in(reg) d, inout("eax") n => q, inout("edx") 0u32 => _);
    }
    q
}

#[inline(always)]
unsafe fn rd(p: *mut u16, i: usize) -> u16 {
    // SAFETY: caller's buffer.
    unsafe { p.add(i).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut u16, i: usize, v: u16) {
    // SAFETY: caller's buffer.
    unsafe { p.add(i).write_unaligned(v) }
}

#[inline(always)]
unsafe fn tab(t: *mut u16, i: usize) -> u16 {
    // SAFETY: the game's own table.
    unsafe { t.add(i).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406b50(param_1: u16, param_2: *mut u16) {
    let mut param_1 = param_1;
    // SAFETY: the globals, tables and callees are the game's own; param_2 points at the
    // caller's buffer, read and written where the original does.
    unsafe {
        let mut u_var1: u16;
        let mut u_var2: u16;
        let mut u_var3: u32;
        let mut i_var4: i32;
        let mut u_var5: u16;
        let mut u_var7: u16;
        let mut s_var8: i16;
        let mut u_var9: u16;
        let u_var10: u16;
        let local_8: i16;

        wr(param_2, 0, param_1);
        u_var1 = 0;
        wr(param_2, 2, param_1 & 0x8000);
        wr(param_2, 4, param_1.wrapping_add(0x4000) & 0x8000);
        u_var9 = 0;
        'l: {
            if param_1 < 0x8000 {
                u_var2 = 0;
                if param_1 < 0x4000 {
                    break 'l;
                }
            } else {
                param_1 &= 0x7fff;
                u_var1 = 1;
                u_var2 = 1;
                if 0x3fff < param_1 {
                    break 'l;
                }
            }
            u_var1 = u_var2;
            u_var9 = 2;
        }
        wr(param_2, 8, u_var1);
        wr(param_2, 9, u_var9);
        u_var9 = param_1;
        if 0x3fff < param_1 {
            u_var9 = 0x8000u16.wrapping_sub(param_1);
        }
        u_var1 = FUN_004183f0(u_var9);
        wr(param_2, 1, u_var1);
        u_var1 = FUN_004183f0(u_var9.wrapping_add(0x4000));
        wr(param_2, 3, u_var1);
        if u_var9 < DAT_00732e42 {
            local_8 = 0;
            wr(param_2, 0xc, 0);
            if DAT_00525738 == 1 {
                u_var1 = tab(
                    (&raw mut DAT_00523738).cast::<u16>(),
                    (u_var9 >> 6) as usize,
                );
            } else {
                u_var1 = tab(
                    (&raw mut DAT_00523620).cast::<u16>(),
                    (u_var9 >> 6) as usize,
                );
            }
        } else {
            local_8 = 4;
            wr(param_2, 0xc, 4);
            u_var9 = 0x4000u16.wrapping_sub(u_var9);
            if DAT_00525738 == 1 {
                u_var1 = tab(
                    (&raw mut DAT_00523738).cast::<u16>(),
                    (u_var9 >> 6) as usize,
                );
            } else {
                u_var1 = tab(
                    (&raw mut DAT_00523850).cast::<u16>(),
                    (u_var9 >> 6) as usize,
                );
            }
        }
        u_var2 = FUN_004183f0(u_var9.wrapping_add(0x4000));
        wr(param_2, 5, u_var2);
        if u_var2 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q = div32(0x80000000, u_var2 as u32);
        u_var9 = q as u16;
        if local_8 != 0 && DAT_00525738 == 0 {
            u_var9 = ((DAT_0071ed00 as u32)
                .wrapping_mul(q & 0xffff)
                .wrapping_add(0x8000)
                >> 0x10) as u16;
        }
        wr(param_2, 6, u_var9);
        u_var9 = 0;
        wr(param_2, 0x18, 0);
        wr(param_2, 0x19, 0);
        if rd(param_2, 0xc) == 0 {
            u_var5 = 0;
            wr(param_2, 7, DAT_0070acec);
            u_var2 = 0x8000;
            u_var7 = 2;
            s_var8 = DAT_0070acec.wrapping_sub(2) as i16;
            if DAT_0070acec != 1 {
                loop {
                    u_var9 = u_var9.wrapping_add(1);
                    if u_var2.wrapping_add(u_var1) < u_var2 {
                        u_var5 = u_var5.wrapping_add(1);
                    }
                    wr(param_2, u_var7 as usize + 0x18, u_var9);
                    wr(param_2, u_var7 as usize + 0x19, u_var5);
                    let b11 = s_var8 != 0;
                    u_var2 = u_var2.wrapping_add(u_var1);
                    u_var7 = u_var7.wrapping_add(2);
                    s_var8 = s_var8.wrapping_sub(1);
                    if !b11 {
                        break;
                    }
                }
            }
        } else {
            u_var7 = 0;
            u_var10 = 0;
            let mut u_var10 = u_var10;
            wr(param_2, 7, DAT_00732a20);
            u_var2 = 0x8000;
            u_var5 = 2;
            u_var9 = DAT_00732a20;
            loop {
                u_var9 = u_var9.wrapping_sub(1);
                if u_var9 == 0 {
                    break;
                }
                u_var10 = u_var10.wrapping_add(1);
                if u_var2.wrapping_add(u_var1) < u_var2 {
                    u_var7 = u_var7.wrapping_add(1);
                }
                wr(param_2, u_var5 as usize + 0x18, u_var7);
                wr(param_2, u_var5 as usize + 0x19, u_var10);
                u_var2 = u_var2.wrapping_add(u_var1);
                u_var5 = u_var5.wrapping_add(2);
            }
        }
        u_var9 = rd(param_2, 7);
        u_var3 = (rd(param_2, 0xc) == 0) as u32;
        u_var2 = 0;
        while u_var9 != 0 {
            u_var5 = u_var3 as u16;
            u_var7 = 1;
            i_var4 = u_var3.wrapping_add(0x18) as i32;
            loop {
                u_var9 = u_var9.wrapping_sub(1);
                if u_var9 == 0 {
                    break;
                }
                u_var5 = u_var3 as u16;
                if rd(param_2, u_var3.wrapping_add(0x1a) as usize) != rd(param_2, i_var4 as usize) {
                    break;
                }
                u_var7 = u_var7.wrapping_add(1);
                u_var5 = u_var5.wrapping_add(2);
                u_var3 = u_var5 as u32;
            }
            u_var3 = u_var5.wrapping_add(2) as u32;
            wr(param_2, u_var2 as usize + 0xc9a, u_var7);
            u_var2 = u_var2.wrapping_add(1);
        }
        u_var9 = param_1.wrapping_add(0x4000) & 0x7fff;
        wr(param_2, 0xc99, u_var2);
        if 0x3fff < u_var9 {
            u_var9 = 0x8000u16.wrapping_sub(u_var9);
        }
        if u_var9 < DAT_00732e42 {
            s_var8 = 0;
            wr(param_2, 0xd, 0);
            if DAT_00525738 == 1 {
                u_var9 = tab(
                    (&raw mut DAT_00523738).cast::<u16>(),
                    (u_var9 >> 6) as usize,
                );
            } else {
                u_var9 = tab(
                    (&raw mut DAT_00523620).cast::<u16>(),
                    (u_var9 >> 6) as usize,
                );
            }
        } else {
            s_var8 = 4;
            wr(param_2, 0xd, 4);
            let idx = (0x4000u16.wrapping_sub(u_var9) >> 6) as usize;
            if DAT_00525738 == 1 {
                u_var9 = tab((&raw mut DAT_00523738).cast::<u16>(), idx);
            } else {
                u_var9 = tab((&raw mut DAT_00523850).cast::<u16>(), idx);
            }
        }
        wr(param_2, 0xf, 0);
        wr(param_2, 0xe, u_var9);
        if s_var8 == local_8 && DAT_00525738 == 0 {
            u_var9 = rd(param_2, 0);
            if (u_var9 < 0xe000 && 0x9fff < u_var9) || (u_var9 < 0x6000 && 0x1fff < u_var9) {
                if rd(param_2, 0xe) == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                u_var3 = div32(0x1000000, rd(param_2, 0xe) as u32) & 0xffff;
                wr(param_2, 0xe, (u_var3 << 8) as u16);
                wr(param_2, 0xf, (u_var3 >> 8) as u16);
                wr(param_2, 0xc98, u_var1 >> 8);
            } else {
                wr(param_2, 0xe, 0x100);
                wr(param_2, 0xf, 0x100);
                wr(
                    param_2,
                    0xc98,
                    ((((u_var1 as u32).wrapping_mul(0xd800)) >> 0x18) as u8) as u16,
                );
            }
        } else {
            wr(
                param_2,
                0xc98,
                (((rd(param_2, 0xe) as u32)
                    .wrapping_mul(u_var1 as u32)
                    .wrapping_add(0x800000)
                    >> 0x18) as u8) as u16,
            );
        }
        wr(
            param_2,
            0xb,
            rd(param_2, 0xc) | rd(param_2, 8) | rd(param_2, 9),
        );
        wr(
            param_2,
            10,
            (rd(param_2, 9) >> 1).wrapping_add(rd(param_2, 8)),
        );
        u_var9 = rd(param_2, 7);
        wr(param_2, 0x10, rd(param_2, 0x18));
        wr(param_2, 0x14, rd(param_2, 0x19));
        u_var7 = DAT_0071f2e0.wrapping_sub(rd(param_2, 0x19));
        wr(param_2, 0x12, u_var7);
        u_var1 = rd(param_2, (u_var9 as usize) * 2 + 0x16);
        wr(param_2, 0x11, u_var1);
        u_var2 = rd(param_2, (u_var9 as usize) * 2 + 0x17);
        wr(param_2, 0x15, u_var2);
        u_var2 = DAT_0071f2e0.wrapping_sub(u_var2);
        u_var1 = u_var1.wrapping_sub(rd(param_2, 0x18));
        wr(param_2, 0x13, u_var2);
        wr(param_2, 0x16, u_var1);
        if (u_var1 as i16) < 0 {
            wr(param_2, 0x16, 0u16.wrapping_sub(u_var1));
        }
        u_var2 = u_var2.wrapping_sub(u_var7);
        wr(param_2, 0x17, u_var2);
        if (u_var2 as i16) < 0 {
            wr(param_2, 0x17, 0u16.wrapping_sub(u_var2));
        }
        let doubled = DAT_00525518 == 2;
        u_var1 = 0;
        if u_var9 != 0 {
            loop {
                u_var3 = u_var1 as u32;
                u_var9 = rd(param_2, (u_var3 as usize) * 2 + 0x18);
                if rd(param_2, 9) != 0 {
                    u_var9 = 0u16.wrapping_sub(u_var9);
                }
                u_var2 = rd(param_2, (u_var3 as usize) * 2 + 0x19);
                if rd(param_2, 8) != 0 {
                    u_var2 = 0u16.wrapping_sub(u_var2);
                }
                let a = (u_var9 as i16) as i32;
                let a = if doubled { a.wrapping_mul(2) } else { a };
                let b = ((u_var2 as i16) as i32).wrapping_mul(DAT_006db6d0);
                if DAT_0071b6b2 < 1 {
                    i_var4 = a.wrapping_sub(b);
                } else {
                    i_var4 = b.wrapping_add(a);
                }
                u_var1 = u_var1.wrapping_add(1);
                param_2
                    .add((u_var3 as usize) * 2 + 0x12dc)
                    .cast::<i32>()
                    .write_unaligned(i_var4);
                if !(u_var1 < rd(param_2, 7)) {
                    break;
                }
            }
        }
    }
}
