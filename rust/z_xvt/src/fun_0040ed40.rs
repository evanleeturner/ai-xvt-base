// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_0052686c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040efa0(param_1: u32) -> i32;
}

#[inline(always)]
unsafe fn r32(p: *mut u8) -> i32 {
    // SAFETY: caller's buffer, read where the original reads.
    unsafe { p.cast::<i32>().read_unaligned() }
}
#[inline(always)]
unsafe fn w32(p: *mut u8, v: i32) {
    // SAFETY: caller's buffer, written where the original writes.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}
#[inline(always)]
unsafe fn r16(p: *mut u8) -> i16 {
    // SAFETY: caller's buffer, read where the original reads.
    unsafe { p.cast::<i16>().read_unaligned() }
}
#[inline(always)]
unsafe fn w16(p: *mut u8, v: i16) {
    // SAFETY: caller's buffer, written where the original writes.
    unsafe { p.cast::<i16>().write_unaligned(v) }
}
#[inline(always)]
fn ad(p: *mut u8, n: u32) -> *mut u8 {
    p.wrapping_add(n as usize)
}
#[inline(always)]
fn sb(p: *mut u8, n: u32) -> *mut u8 {
    p.wrapping_sub(n as usize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040ed40(param_1: *mut u8, param_2: u16, param_3: u32) {
    // SAFETY: globals and callee are the game's own; param_1 and the table
    // entries are the game's buffers, accessed as the original does.
    unsafe {
        let i_var4 = ad(DAT_006d4988, (param_2 as u32).wrapping_mul(0x23));
        let i_var3 = ad(DAT_006d4988, (param_3 & 0xffff).wrapping_mul(0x23));

        let mut i_var2 = r32(ad(i_var4, 0xc)).wrapping_sub(r32(ad(i_var3, 0xc)));
        let mut i_var9 = r32(ad(i_var4, 8)).wrapping_sub(r32(ad(i_var3, 8)));
        let mut i_var5 = r32(ad(i_var4, 4)).wrapping_sub(r32(ad(i_var3, 4)));

        if i_var5 < 0 {
            i_var5 = i_var5.wrapping_neg();
        }
        if i_var9 < 0 {
            i_var9 = i_var9.wrapping_neg();
        }
        if i_var2 < 0 {
            i_var2 = i_var2.wrapping_neg();
        }

        if i_var9 < i_var5 && i_var2 < i_var5 {
            i_var5 = (i_var9 >> 2).wrapping_add(i_var2 >> 2).wrapping_add(i_var5);
        } else if i_var5 < i_var9 && i_var2 < i_var9 {
            i_var5 = i_var9.wrapping_add(i_var5 >> 2).wrapping_add(i_var2 >> 2);
        } else {
            i_var5 = (i_var5 >> 2).wrapping_add(i_var9 >> 2).wrapping_add(i_var2);
        }

        let tbl = (&raw mut DAT_0052686c).cast::<u8>();
        let us = r16(ad(tbl, (*i_var3.wrapping_add(3) as u32).wrapping_mul(0x16))) as u16;
        i_var5 = i_var5.wrapping_sub(us as u32 as i32);
        let us = r16(ad(tbl, (*i_var4.wrapping_add(3) as u32).wrapping_mul(0x16))) as u16;
        i_var5 = i_var5.wrapping_sub(us as u32 as i32);

        if i_var5 < 0 {
            i_var2 = 0;
        } else {
            let x = FUN_0040efa0(param_3);
            let a = FUN_0040efa0(param_2 as u32);
            i_var2 = x.wrapping_add(a) >> 8;
            if i_var2 == 0 {
                return;
            }
            let dividend = (i_var5 >> 8).wrapping_mul(0x33db);
            let q: i32;
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) i_var2,
                inout("eax") dividend => q,
                out("edx") _,
            );
            i_var2 = q;
        }

        let s3 = param_3 as i16;
        let mut u10: u32 = 0;
        let b_var1 = *param_1;
        let mut u6: u32 = 0;
        let mut p7: *mut u8 = param_1;
        let mut p8: *mut u8 = param_1;
        let mut mode = 0;

        if b_var1 != 0 {
            p7 = ad(param_1, 1);
            p8 = ad(param_1, 0x41);
            loop {
                if r16(p8) == s3 {
                    u6 = u10.wrapping_add(1);
                    w32(ad(param_1, u10.wrapping_mul(4).wrapping_add(1)), i_var2);
                    if (*param_1 as i32) <= (u6 as i32) {
                        mode = 2;
                        break;
                    }
                    p7 = ad(param_1, u6.wrapping_mul(2).wrapping_add(0x41));
                    p8 = ad(param_1, u6.wrapping_mul(4).wrapping_add(1));
                    mode = 1;
                    break;
                }
                if i_var2 < r32(p7) {
                    break;
                }
                p7 = ad(p7, 4);
                p8 = ad(p8, 2);
                u10 = u10.wrapping_add(1);
                if !((u10 as i32) < (b_var1 as i32)) {
                    break;
                }
            }
        }

        if mode == 0 {
            u6 = b_var1 as u32;
            if u10 != u6 {
                if u6 == 0x10 {
                    let a = r32(ad(param_1, 0x3d));
                    let b = r32(ad(param_1, 0x61));
                    if a < b {
                        w32(ad(param_1, 0x61), a);
                    }
                    u6 = 0xf;
                    *param_1 = b_var1.wrapping_sub(1);
                }
                if (u10 as i32) < (u6 as i32) {
                    let mut q7 = ad(param_1, u6.wrapping_mul(4).wrapping_add(1));
                    let mut c = u6.wrapping_sub(u10) as i32;
                    let mut q8 = ad(param_1, u6.wrapping_mul(2).wrapping_add(0x41));
                    loop {
                        let t = r32(sb(q7, 4));
                        w32(q7, t);
                        q7 = sb(q7, 4);
                        c = c.wrapping_sub(1);
                        let t = r16(sb(q8, 2));
                        w16(q8, t);
                        q8 = sb(q8, 2);
                        if c == 0 {
                            break;
                        }
                    }
                }
                w32(ad(param_1, u10.wrapping_mul(4).wrapping_add(1)), i_var2);
                w16(ad(param_1, u10.wrapping_mul(2).wrapping_add(0x41)), s3);
                *param_1 = (*param_1).wrapping_add(1);
                return;
            }
            if b_var1 == 0x10 {
                let t = r32(ad(param_1, 0x61));
                if t <= i_var2 {
                    return;
                }
                w32(ad(param_1, 0x61), i_var2);
                return;
            }
            w32(ad(param_1, u10.wrapping_mul(4).wrapping_add(1)), i_var2);
            w16(ad(param_1, u10.wrapping_mul(2).wrapping_add(0x41)), s3);
            *param_1 = (*param_1).wrapping_add(1);
            return;
        }

        if mode == 1 {
            loop {
                if i_var2 <= r32(p8) {
                    break;
                }
                let t = r32(p8);
                w32(sb(p8, 4), t);
                let t = r16(p7);
                w16(sb(p7, 2), t);
                u6 = u6.wrapping_add(1);
                w32(p8, i_var2);
                w16(p7, s3);
                p7 = ad(p7, 2);
                p8 = ad(p8, 4);
                if *param_1 as u32 == u6 {
                    break;
                }
            }
            u10 = u6.wrapping_sub(1);
        }

        if 0 < (u10 as i32) {
            let mut c5 = u10.wrapping_sub(1) as i32;
            let mut q7 = ad(param_1, u10.wrapping_mul(2).wrapping_add(0x3f));
            let mut q8 = sb(ad(param_1, u10.wrapping_mul(4)), 3);
            loop {
                let t = r32(q8);
                if !(i_var2 < t) {
                    break;
                }
                w32(ad(q8, 4), t);
                let t = r16(q7);
                w16(ad(q7, 2), t);
                c5 = c5.wrapping_sub(1);
                w32(q8, i_var2);
                w16(q7, s3);
                q7 = sb(q7, 2);
                q8 = sb(q8, 4);
                if c5 < 0 {
                    return;
                }
            }
        }
    }
}
