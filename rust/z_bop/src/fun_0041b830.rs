// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00521f1c: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0041ba60(a: u32) -> i32;
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, n: u32, args: *const u32);
}

#[inline(always)]
unsafe fn rd32(p: *const u8) -> i32 {
    // SAFETY: caller supplies a valid address.
    unsafe { p.cast::<i32>().read_unaligned() }
}
#[inline(always)]
unsafe fn rd16(p: *const u8) -> i16 {
    // SAFETY: caller supplies a valid address.
    unsafe { p.cast::<i16>().read_unaligned() }
}
#[inline(always)]
unsafe fn wr32(p: *mut u8, v: i32) {
    // SAFETY: caller supplies a valid address.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr16(p: *mut u8, v: i16) {
    // SAFETY: caller supplies a valid address.
    unsafe { p.cast::<i16>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041b830(param_1: *mut u8, param_2: u32, param_3: u32) {
    // SAFETY: all pointers are the game's own memory, accessed where the original does.
    unsafe {
        let iv7_off = (param_2 & 0xffff).wrapping_mul(0x23) as usize;
        let iv2_off = (param_3 & 0xffff).wrapping_mul(0x23) as usize;
        let base = DAT_009a1fe8;

        let t0 =
            rd32(base.wrapping_add(iv7_off + 4)).wrapping_sub(rd32(base.wrapping_add(iv2_off + 4)));
        let t2 =
            rd32(base.wrapping_add(iv7_off + 8)).wrapping_sub(rd32(base.wrapping_add(iv2_off + 8)));
        let t4 = rd32(base.wrapping_add(iv7_off + 0xc))
            .wrapping_sub(rd32(base.wrapping_add(iv2_off + 0xc)));

        let iv3 = FUN_0041f410(t0, t2, t4);

        let tab = (&raw mut DAT_00521f1c).cast::<u8>();
        let b7 = *base.wrapping_add(3 + iv7_off) as usize;
        let b2 = *base.wrapping_add(3 + iv2_off) as usize;
        let t1 = rd32(tab.wrapping_add(b7 * 0x1a));
        let t3 = rd32(tab.wrapping_add(b2 * 0x1a));
        let iv7 = iv3.wrapping_sub(t1).wrapping_sub(t3);

        let iv2: i32;
        if iv7 < 0 {
            iv2 = 0;
        } else {
            let a = FUN_0041ba60(param_3);
            let b = FUN_0041ba60(param_2);
            let d = a.wrapping_add(b) >> 8;
            if d == 0 {
                return;
            }
            let n = ((iv7 >> 8) as u32).wrapping_mul(0x33db) as i32;
            if n == i32::MIN && d == -1 {
                RaiseException(0xC000_0094, 0, 0, core::ptr::null());
            }
            iv2 = n.wrapping_div(d);
        }

        let mut u8_ = 0u32;
        let b_var1 = *param_1;
        let mut mode = 0u32;
        let mut u5: u32 = 0;
        let mut p6: *mut u8;
        let mut p4: *mut u8;

        if b_var1 != 0 {
            p6 = param_1.wrapping_add(1);
            p4 = param_1.wrapping_add(0x41);
            loop {
                if rd16(p4) == param_3 as i16 {
                    u5 = u8_.wrapping_add(1);
                    wr32(
                        param_1.wrapping_add((u8_ as usize).wrapping_mul(4) + 1),
                        iv2,
                    );
                    if (*param_1 as i32) <= (u5 as i32) {
                        mode = 2;
                    } else {
                        p6 = param_1.wrapping_add((u5 as usize).wrapping_mul(2) + 0x41);
                        p4 = param_1.wrapping_add((u5 as usize).wrapping_mul(4) + 1);
                        mode = 1;
                    }
                    break;
                }
                let ti = rd32(p6);
                if iv2 < ti {
                    break;
                }
                p6 = p6.wrapping_add(4);
                p4 = p4.wrapping_add(2);
                u8_ = u8_.wrapping_add(1);
                if !((u8_ as i32) < (b_var1 as i32)) {
                    break;
                }
            }
        } else {
            p6 = param_1;
            p4 = param_1;
        }

        if mode == 0 {
            let mut u5 = b_var1 as u32;
            if u8_ != u5 {
                if u5 == 0x10 {
                    let a = rd32(param_1.wrapping_add(0x3d));
                    let b = rd32(param_1.wrapping_add(0x61));
                    if a < b {
                        wr32(param_1.wrapping_add(0x61), a);
                    }
                    u5 = 0xf;
                    *param_1 = b_var1.wrapping_sub(1);
                }
                if (u8_ as i32) < (u5 as i32) {
                    let mut q6 = param_1.wrapping_add((u5 as usize).wrapping_mul(4) + 1);
                    let mut cnt = u5.wrapping_sub(u8_) as i32;
                    let mut q4 = param_1.wrapping_add((u5 as usize).wrapping_mul(2) + 0x41);
                    loop {
                        let v = rd32(q6.wrapping_sub(4));
                        wr32(q6, v);
                        q6 = q6.wrapping_sub(4);
                        cnt = cnt.wrapping_sub(1);
                        let s = rd16(q4.wrapping_sub(2));
                        wr16(q4, s);
                        q4 = q4.wrapping_sub(2);
                        if cnt == 0 {
                            break;
                        }
                    }
                }
                wr32(
                    param_1.wrapping_add((u8_ as usize).wrapping_mul(4) + 1),
                    iv2,
                );
                wr16(
                    param_1.wrapping_add((u8_ as usize).wrapping_mul(2) + 0x41),
                    param_3 as i16,
                );
                *param_1 = (*param_1).wrapping_add(1);
                return;
            }
            if b_var1 == 0x10 {
                let ti = rd32(param_1.wrapping_add(0x61));
                if ti <= iv2 {
                    return;
                }
                wr32(param_1.wrapping_add(0x61), iv2);
                return;
            }
            wr32(
                param_1.wrapping_add((u8_ as usize).wrapping_mul(4) + 1),
                iv2,
            );
            wr16(
                param_1.wrapping_add((u8_ as usize).wrapping_mul(2) + 0x41),
                param_3 as i16,
            );
            *param_1 = (*param_1).wrapping_add(1);
            return;
        }

        if mode == 1 {
            loop {
                let ti = rd32(p4);
                if iv2 <= ti {
                    break;
                }
                wr32(p4.wrapping_sub(4), ti);
                let s = rd16(p6);
                wr16(p6.wrapping_sub(2), s);
                u5 = u5.wrapping_add(1);
                wr32(p4, iv2);
                wr16(p6, param_3 as i16);
                p6 = p6.wrapping_add(2);
                p4 = p4.wrapping_add(4);
                if (*param_1 as u32) == u5 {
                    break;
                }
            }
            u8_ = u5.wrapping_sub(1);
        }

        if 0 < (u8_ as i32) {
            let mut iv7 = u8_.wrapping_sub(1) as i32;
            let mut p6 = param_1.wrapping_add((u8_ as usize).wrapping_mul(2) + 0x3f);
            let mut p4 = param_1
                .wrapping_add((u8_ as usize).wrapping_mul(4))
                .wrapping_sub(3);
            loop {
                let ti = rd32(p4);
                if !(iv2 < ti) {
                    break;
                }
                wr32(p4.wrapping_add(4), ti);
                let s = rd16(p6);
                wr16(p6.wrapping_add(2), s);
                iv7 = iv7.wrapping_sub(1);
                wr32(p4, iv2);
                wr16(p6, param_3 as i16);
                p6 = p6.wrapping_sub(2);
                p4 = p4.wrapping_sub(4);
                if iv7 < 0 {
                    return;
                }
            }
        }
    }
}
