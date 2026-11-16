// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_007596b7: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f8160(param_1: i32, param_2: i32, param_3: i32, param_4: u16);
    fn FUN_004f82f0(param_1: i32, param_2: i32, param_3: i32, param_4: u16);
}

#[inline(always)]
unsafe fn idiv(n: i32, d: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; it faults on a zero divisor
    // or overflow exactly as the original does.
    unsafe {
        core::arch::asm!(
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            inout("edx") (n >> 31) => _,
            options(nomem, nostack)
        );
    }
    q
}

#[inline(always)]
unsafe fn draw(mut p: u32, mut cnt: i32, neg: bool, local_4: u32, bv: u8, stride: i32) {
    let mut iv5: i32 = 0x800;
    let uv1: u32 = ((local_4 as i32) >> 12) as u32;
    if uv1 == 0 {
        if cnt < 1 {
            return;
        }
        loop {
            // SAFETY: writes into the game's own frame buffer.
            unsafe {
                (p as usize as *mut u8).write(bv);
            }
            iv5 = iv5.wrapping_add((local_4 & 0xfff) as i32);
            if 0xfff < iv5 {
                iv5 = iv5.wrapping_sub(0x1000);
                p = p.wrapping_add(1);
            }
            p = if neg {
                p.wrapping_sub(stride as u32)
            } else {
                p.wrapping_add(stride as u32)
            };
            cnt = cnt.wrapping_sub(1);
            if cnt == 0 {
                break;
            }
        }
        return;
    }
    if cnt < 1 {
        return;
    }
    loop {
        iv5 = iv5.wrapping_add((local_4 & 0xfff) as i32);
        let mut uv6 = uv1;
        if 0xfff < iv5 {
            uv6 = uv1.wrapping_add(1);
            iv5 = iv5.wrapping_sub(0x1000);
        }
        let mut q = p;
        let mut n = uv6;
        while n != 0 {
            // SAFETY: writes into the game's own frame buffer.
            unsafe {
                (q as usize as *mut u8).write(bv);
            }
            q = q.wrapping_add(1);
            n -= 1;
        }
        p = if neg {
            p.wrapping_add(uv6.wrapping_sub(stride as u32))
        } else {
            p.wrapping_add(stride as u32).wrapping_add(uv6)
        };
        cnt = cnt.wrapping_sub(1);
        if cnt == 0 {
            break;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f7a80(
    mut param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    mut param_4: i32,
    param_5: u16,
) {
    // SAFETY: the callees and globals are the game's own; the unaligned
    // globals are read with read_unaligned and the frame buffer pointer
    // comes from the game's own surface base.
    unsafe {
        if param_4 == param_2 {
            FUN_004f8160(param_1, param_3, param_2, param_5);
            return;
        }
        if param_3 == param_1 {
            FUN_004f82f0(param_2, param_4, param_1, param_5);
            return;
        }
        let cp = (&raw mut DAT_007596b7).cast::<u8>();
        let clip_l: i32 = cp.cast::<i32>().read_unaligned();
        let clip_r: i32 = cp.byte_add(4).cast::<i32>().read_unaligned();
        let clip_t: i32 = cp.byte_add(8).cast::<i32>().read_unaligned();
        let clip_b: i32 = cp.byte_add(12).cast::<i32>().read_unaligned();
        let stride: i32 = (&raw mut DAT_0075964b).cast::<i32>().read_unaligned();
        let bv = param_5 as u8;

        let mut local_4: u32;
        let mut local_9: i32;
        let mut i_var4: i32;

        if param_2 < param_4 {
            if param_1 < param_3 {
                local_4 = idiv(
                    param_3
                        .wrapping_sub(param_1)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    param_4.wrapping_sub(param_2).wrapping_add(1),
                ) as u32;
                local_9 = (param_1 < clip_l) as i32;
                if local_9 != 0 {
                    param_2 = param_2.wrapping_add(idiv(
                        clip_l.wrapping_sub(param_1).wrapping_mul(0x1000),
                        local_4 as i32,
                    ));
                    param_1 = clip_l;
                }
                i_var4 = param_2;
                if param_2 < clip_t {
                    param_1 = param_1.wrapping_add(
                        ((clip_t.wrapping_sub(param_2) as u32).wrapping_mul(local_4) as i32) >> 12,
                    );
                    i_var4 = clip_t;
                }
                local_9 = (param_2 < clip_t || local_9 != 0) as i32;
                if clip_r < param_3 {
                    local_9 = 1;
                    param_4 = param_4.wrapping_add(idiv(
                        clip_r.wrapping_sub(param_3).wrapping_mul(0x1000),
                        local_4 as i32,
                    ));
                    param_3 = clip_r;
                }
                if clip_b < param_4 {
                    local_9 = 1;
                    param_3 = param_3.wrapping_sub(
                        ((param_4.wrapping_sub(clip_b) as u32).wrapping_mul(local_4) as i32) >> 12,
                    );
                    param_4 = clip_b;
                }
                if local_9 != 0 {
                    if param_3 < param_1 {
                        return;
                    }
                    if param_4 < i_var4 {
                        return;
                    }
                    local_4 = idiv(
                        param_3
                            .wrapping_sub(param_1)
                            .wrapping_add(1)
                            .wrapping_mul(0x1000),
                        param_4.wrapping_sub(i_var4).wrapping_add(1),
                    ) as u32;
                }
                let p = (i_var4 as u32)
                    .wrapping_mul(stride as u32)
                    .wrapping_add(param_1 as u32)
                    .wrapping_add(DAT_00758830 as u32);
                let cnt = param_4.wrapping_sub(i_var4).wrapping_add(1);
                draw(p, cnt, false, local_4, bv, stride);
                return;
            }
            if param_2 < param_4 && param_3 < param_1 {
                local_4 = idiv(
                    param_1
                        .wrapping_sub(param_3)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    param_4.wrapping_sub(param_2).wrapping_add(1),
                ) as u32;
                local_9 = (clip_r < param_1) as i32;
                if local_9 != 0 {
                    param_2 = param_2.wrapping_add(idiv(
                        param_1.wrapping_sub(clip_r).wrapping_mul(0x1000),
                        local_4 as i32,
                    ));
                    param_1 = clip_r;
                }
                i_var4 = param_2;
                if param_2 < clip_t {
                    param_1 = param_1.wrapping_sub(
                        ((clip_t.wrapping_sub(param_2) as u32).wrapping_mul(local_4) as i32) >> 12,
                    );
                    i_var4 = clip_t;
                }
                local_9 = (param_2 < clip_t || local_9 != 0) as i32;
                if param_3 < clip_l {
                    local_9 = 1;
                    param_4 = param_4.wrapping_add(idiv(
                        param_3.wrapping_sub(clip_l).wrapping_mul(0x1000),
                        local_4 as i32,
                    ));
                    param_3 = clip_l;
                }
                if clip_b < param_4 {
                    local_9 = 1;
                    param_3 = param_3.wrapping_add(
                        ((param_4.wrapping_sub(clip_b) as u32).wrapping_mul(local_4) as i32) >> 12,
                    );
                    param_4 = clip_b;
                }
                if local_9 != 0 {
                    if param_1 < param_3 {
                        return;
                    }
                    if param_4 < i_var4 {
                        return;
                    }
                    local_4 = idiv(
                        param_1
                            .wrapping_sub(param_3)
                            .wrapping_add(1)
                            .wrapping_mul(0x1000),
                        param_4.wrapping_sub(i_var4).wrapping_add(1),
                    ) as u32;
                }
                let p = (param_4 as u32)
                    .wrapping_mul(stride as u32)
                    .wrapping_add(param_3 as u32)
                    .wrapping_add(DAT_00758830 as u32);
                let cnt = param_4.wrapping_sub(i_var4).wrapping_add(1);
                draw(p, cnt, true, local_4, bv, stride);
                return;
            }
        }
        if param_4 < param_2 && param_1 < param_3 {
            local_4 = idiv(
                param_3
                    .wrapping_sub(param_1)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000),
                param_2.wrapping_sub(param_4).wrapping_add(1),
            ) as u32;
            local_9 = (param_1 < clip_l) as i32;
            if local_9 != 0 {
                param_2 = param_2.wrapping_add(idiv(
                    param_1.wrapping_sub(clip_l).wrapping_mul(0x1000),
                    local_4 as i32,
                ));
                param_1 = clip_l;
            }
            i_var4 = param_2;
            if clip_b < param_2 {
                param_1 = param_1.wrapping_add(
                    ((param_2.wrapping_sub(clip_b) as u32).wrapping_mul(local_4) as i32) >> 12,
                );
                i_var4 = clip_b;
            }
            local_9 = (clip_b < param_2 || local_9 != 0) as i32;
            if clip_r < param_3 {
                local_9 = 1;
                param_4 = param_4.wrapping_add(idiv(
                    param_3.wrapping_sub(clip_r).wrapping_mul(0x1000),
                    local_4 as i32,
                ));
                param_3 = clip_r;
            }
            if param_4 < clip_t {
                local_9 = 1;
                param_3 = param_3.wrapping_sub(
                    ((clip_t.wrapping_sub(param_4) as u32).wrapping_mul(local_4) as i32) >> 12,
                );
                param_4 = clip_t;
            }
            if local_9 != 0 {
                if param_3 < param_1 {
                    return;
                }
                if i_var4 < param_4 {
                    return;
                }
                local_4 = idiv(
                    param_3
                        .wrapping_sub(param_1)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    i_var4.wrapping_sub(param_4).wrapping_add(1),
                ) as u32;
            }
            let p = (i_var4 as u32)
                .wrapping_mul(stride as u32)
                .wrapping_add(param_1 as u32)
                .wrapping_add(DAT_00758830 as u32);
            let cnt = i_var4.wrapping_sub(param_4).wrapping_add(1);
            draw(p, cnt, true, local_4, bv, stride);
        } else {
            local_4 = idiv(
                param_1
                    .wrapping_sub(param_3)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000),
                param_2.wrapping_sub(param_4).wrapping_add(1),
            ) as u32;
            local_9 = (clip_r < param_1) as i32;
            if local_9 != 0 {
                param_2 = param_2.wrapping_add(idiv(
                    clip_r.wrapping_sub(param_1).wrapping_mul(0x1000),
                    local_4 as i32,
                ));
                param_1 = clip_r;
            }
            i_var4 = param_2;
            if clip_b < param_2 {
                param_1 = param_1.wrapping_sub(
                    ((param_2.wrapping_sub(clip_b) as u32).wrapping_mul(local_4) as i32) >> 12,
                );
                i_var4 = clip_b;
            }
            local_9 = (clip_b < param_2 || local_9 != 0) as i32;
            if param_3 < clip_l {
                local_9 = 1;
                param_4 = param_4.wrapping_add(idiv(
                    clip_l.wrapping_sub(param_3).wrapping_mul(0x1000),
                    local_4 as i32,
                ));
                param_3 = clip_l;
            }
            if param_4 < clip_t {
                local_9 = 1;
                param_3 = param_3.wrapping_add(
                    ((clip_t.wrapping_sub(param_4) as u32).wrapping_mul(local_4) as i32) >> 12,
                );
                param_4 = clip_t;
            }
            if local_9 != 0 {
                if param_1 < param_3 {
                    return;
                }
                if i_var4 < param_4 {
                    return;
                }
                local_4 = idiv(
                    param_1
                        .wrapping_sub(param_3)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    i_var4.wrapping_sub(param_4).wrapping_add(1),
                ) as u32;
            }
            let p = (param_4 as u32)
                .wrapping_mul(stride as u32)
                .wrapping_add(param_3 as u32)
                .wrapping_add(DAT_00758830 as u32);
            let cnt = i_var4.wrapping_sub(param_4).wrapping_add(1);
            draw(p, cnt, false, local_4, bv, stride);
        }
    }
}
