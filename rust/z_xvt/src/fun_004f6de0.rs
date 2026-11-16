// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00759657: i32;
    static mut DAT_007596b7: i32;
    static mut DAT_007596bb: i32;
    static mut DAT_007596bf: i32;
    static mut DAT_007596c3: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f7a80(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u16);
    fn FUN_004f8160(param_1: i32, param_2: i32, param_3: i32, param_4: u16);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; faults on a zero divisor as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[inline(always)]
unsafe fn fill_row(p: u32, count: u32, mode: i32, b: u8, w: u16) {
    let mut q = p as *mut u8;
    // SAFETY: writes the caller's framebuffer where the original does.
    unsafe {
        if mode == 0 {
            let mut c = count >> 2;
            while c != 0 {
                q.cast::<u32>()
                    .write_unaligned(u32::from(b).wrapping_mul(0x01010101));
                q = q.byte_add(4);
                c -= 1;
            }
            let mut c = count & 3;
            while c != 0 {
                q.write(b);
                q = q.byte_add(1);
                c -= 1;
            }
        } else if mode == 1 && (count as i32) > 0 {
            let mut c = count;
            while c != 0 {
                q.cast::<u16>().write_unaligned(w);
                q = q.byte_add(2);
                c -= 1;
            }
        }
    }
}

#[inline(always)]
unsafe fn fill_rows(p: u32, rows: i32, pitch: i32, count: u32, mode: i32, b: u8, w: u16) {
    let mut p = p;
    let mut n = rows;
    // SAFETY: framebuffer writes as in the original.
    unsafe {
        loop {
            fill_row(p, count, mode, b, w);
            p = p.wrapping_add(pitch as u32);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f6de0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u16,
    param_6: i32,
) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut param_4 = param_4;
    // SAFETY: globals and callees are the game's own; framebuffer pointers are
    // derived from the game's surface base exactly as the original does.
    unsafe {
        let mut local_10: i32 = 0x800;
        if param_6 == 1 {
            FUN_004f7a80(param_1, param_2, param_3, param_4, param_5);
            return;
        }
        let local_48: i32 = (&raw const DAT_00759657).read_unaligned();
        let local_44: i32 = (&raw const DAT_0075964b).read_unaligned();
        let local_4c: i32 = (&raw const DAT_007596b7).read_unaligned();
        let local_50: i32 = (&raw const DAT_007596bb).read_unaligned();
        let local_54: i32 = (&raw const DAT_007596bf).read_unaligned();
        let local_58: i32 = (&raw const DAT_007596c3).read_unaligned();

        let i_var10 = local_48 >> 4;
        let i_var5 = param_6 >> 1;
        let i_var3 = param_6.wrapping_sub(1) >> 1;
        let sh = i_var10 as u8 as u32;
        let u_var2 = param_5 as u8;
        let base = DAT_00758830 as u32;
        let mut local_c: u32;
        let mut local_5: bool;
        let b_var1: bool;

        if param_4 == param_2 {
            if local_54 <= param_2 && param_2 <= local_58 {
                if param_3 < param_1 {
                    if param_3 < local_4c {
                        param_3 = local_4c;
                    }
                    if local_50 < param_1 {
                        param_1 = local_50;
                    }
                    param_2 = param_2.wrapping_sub(i_var5);
                    param_4 = param_4.wrapping_add(i_var3);
                    if local_58 < param_4 {
                        param_4 = local_58;
                    }
                    if param_2 < local_54 {
                        param_2 = local_54;
                    }
                    let p = base
                        .wrapping_add((param_3 as u32).wrapping_shl(sh))
                        .wrapping_add((param_2 as u32).wrapping_mul(local_44 as u32));
                    let u_var8 = (param_1.wrapping_sub(param_3) as u32).wrapping_add(1);
                    if param_2 <= param_4 {
                        let rows = param_4.wrapping_sub(param_2).wrapping_add(1);
                        fill_rows(p, rows, local_44, u_var8, i_var10, u_var2, param_5);
                        return;
                    }
                } else {
                    if local_50 < param_3 {
                        param_3 = local_50;
                    }
                    if param_1 < local_4c {
                        param_1 = local_4c;
                    }
                    param_2 = param_2.wrapping_sub(i_var5);
                    param_4 = param_4.wrapping_add(i_var3);
                    if local_58 < param_4 {
                        param_4 = local_58;
                    }
                    if param_2 < local_54 {
                        param_2 = local_54;
                    }
                    let p = base
                        .wrapping_add((param_1 as u32).wrapping_shl(sh))
                        .wrapping_add((param_2 as u32).wrapping_mul(local_44 as u32));
                    let u_var8 = (param_3.wrapping_sub(param_1) as u32).wrapping_add(1);
                    if param_2 <= param_4 {
                        let rows = param_4.wrapping_sub(param_2).wrapping_add(1);
                        fill_rows(p, rows, local_44, u_var8, i_var10, u_var2, param_5);
                        return;
                    }
                }
            }
        } else if param_2 < param_4 && param_1 <= param_3 {
            local_c = idiv(
                (param_3.wrapping_sub(param_1) as u32)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000) as i32,
                param_4.wrapping_sub(param_2).wrapping_add(1),
            ) as u32;
            b_var1 = param_1 < local_4c;
            if b_var1 {
                let i_var4 = local_4c.wrapping_sub(param_1);
                param_1 = local_4c;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_2 = param_2.wrapping_add(idiv(
                    (i_var4 as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
            }
            local_5 = param_2 < local_54;
            if local_5 {
                param_1 = param_1.wrapping_add(
                    (((local_54.wrapping_sub(param_2)) as u32).wrapping_mul(local_c) as i32) >> 0xc,
                );
                param_2 = local_54;
            }
            local_5 = local_5 || b_var1;
            if local_50 < param_3 {
                local_5 = true;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_4 = param_4.wrapping_add(idiv(
                    (local_50.wrapping_sub(param_3) as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
                param_3 = local_50;
            }
            if local_58 < param_4 {
                let i_var4 = param_4.wrapping_sub(local_58);
                param_4 = local_58;
                local_5 = true;
                param_3 =
                    param_3.wrapping_sub(((i_var4 as u32).wrapping_mul(local_c) as i32) >> 0xc);
            }
            if local_5 {
                if param_3 < param_1 {
                    return;
                }
                if param_4 < param_2 {
                    return;
                }
                local_c = idiv(
                    (param_3.wrapping_sub(param_1) as u32)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000) as i32,
                    param_4.wrapping_sub(param_2).wrapping_add(1),
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 0xc) as u32;
            if u_var8 == 0 {
                if param_2 <= param_4 {
                    loop {
                        FUN_004f8160(
                            param_1.wrapping_sub(i_var5),
                            i_var3.wrapping_add(param_1),
                            param_2,
                            param_5,
                        );
                        local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                        if 0xfff < local_10 {
                            local_10 = local_10.wrapping_sub(0x1000);
                            param_1 = param_1.wrapping_add(1);
                        }
                        param_2 = param_2.wrapping_add(1);
                        if !(param_2 <= param_4) {
                            break;
                        }
                    }
                    return;
                }
            } else if param_2 <= param_4 {
                loop {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        u_var6 = u_var8.wrapping_add(1);
                        local_10 = local_10.wrapping_sub(0x1000);
                    }
                    let mut local_14 = param_2.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_2);
                    if local_58 < i_var3.wrapping_add(param_2) {
                        i_var4 = local_58;
                    }
                    if local_14 < local_54 {
                        local_14 = local_54;
                    }
                    let p = (local_14 as u32)
                        .wrapping_mul(local_44 as u32)
                        .wrapping_add((param_1 as u32).wrapping_shl(sh))
                        .wrapping_add(base);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, local_44, u_var6, i_var10, u_var2, param_5);
                    }
                    param_1 = (param_1 as u32).wrapping_add(u_var6) as i32;
                    param_2 = param_2.wrapping_add(1);
                    if !(param_2 <= param_4) {
                        break;
                    }
                }
                return;
            }
        } else if param_2 < param_4 && param_3 <= param_1 {
            local_c = idiv(
                (param_1.wrapping_sub(param_3) as u32)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000) as i32,
                param_4.wrapping_sub(param_2).wrapping_add(1),
            ) as u32;
            b_var1 = local_50 < param_1;
            if b_var1 {
                let i_var4 = param_1.wrapping_sub(local_50);
                param_1 = local_50;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_2 = param_2.wrapping_add(idiv(
                    (i_var4 as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
            }
            local_5 = param_2 < local_54;
            if local_5 {
                param_1 = param_1.wrapping_sub(
                    (((local_54.wrapping_sub(param_2)) as u32).wrapping_mul(local_c) as i32) >> 0xc,
                );
                param_2 = local_54;
            }
            local_5 = local_5 || b_var1;
            if param_3 < local_4c {
                let i_var4 = param_3.wrapping_sub(local_4c);
                param_3 = local_4c;
                local_5 = true;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_4 = param_4.wrapping_add(idiv(
                    (i_var4 as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
            }
            if local_58 < param_4 {
                let i_var4 = param_4.wrapping_sub(local_58);
                param_4 = local_58;
                local_5 = true;
                param_3 =
                    param_3.wrapping_add(((i_var4 as u32).wrapping_mul(local_c) as i32) >> 0xc);
            }
            if local_5 {
                if param_1 < param_3 {
                    return;
                }
                if param_4 < param_2 {
                    return;
                }
                local_c = idiv(
                    (param_1.wrapping_sub(param_3) as u32)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000) as i32,
                    param_4.wrapping_sub(param_2).wrapping_add(1),
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 0xc) as u32;
            if u_var8 == 0 {
                if param_2 <= param_4 {
                    loop {
                        FUN_004f8160(
                            param_1.wrapping_sub(i_var5),
                            i_var3.wrapping_add(param_1),
                            param_2,
                            param_5,
                        );
                        local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                        if 0xfff < local_10 {
                            local_10 = local_10.wrapping_sub(0x1000);
                            param_1 = param_1.wrapping_sub(1);
                        }
                        param_2 = param_2.wrapping_add(1);
                        if !(param_2 <= param_4) {
                            break;
                        }
                    }
                    return;
                }
            } else if param_2 <= param_4 {
                loop {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        u_var6 = u_var8.wrapping_add(1);
                        local_10 = local_10.wrapping_sub(0x1000);
                    }
                    let mut local_14 = param_4.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_4);
                    if local_58 < i_var3.wrapping_add(param_4) {
                        i_var4 = local_58;
                    }
                    if local_14 < local_54 {
                        local_14 = local_54;
                    }
                    let p = (param_3 as u32)
                        .wrapping_shl(sh)
                        .wrapping_add((local_14 as u32).wrapping_mul(local_44 as u32))
                        .wrapping_add(base);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, local_44, u_var6, i_var10, u_var2, param_5);
                    }
                    param_3 = (param_3 as u32).wrapping_add(u_var6) as i32;
                    param_4 = param_4.wrapping_sub(1);
                    if !(param_2 <= param_4) {
                        break;
                    }
                }
                return;
            }
        } else if param_4 < param_2 && param_1 <= param_3 {
            local_c = idiv(
                (param_3.wrapping_sub(param_1) as u32)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000) as i32,
                param_2.wrapping_sub(param_4).wrapping_add(1),
            ) as u32;
            b_var1 = param_1 < local_4c;
            if b_var1 {
                let i_var4 = param_1.wrapping_sub(local_4c);
                param_1 = local_4c;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_2 = param_2.wrapping_add(idiv(
                    (i_var4 as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
            }
            local_5 = local_58 < param_2;
            if local_5 {
                let i_var4 = param_2.wrapping_sub(local_58);
                param_2 = local_58;
                param_1 =
                    param_1.wrapping_add(((i_var4 as u32).wrapping_mul(local_c) as i32) >> 0xc);
            }
            local_5 = local_5 || b_var1;
            if local_50 < param_3 {
                let i_var4 = param_3.wrapping_sub(local_50);
                param_3 = local_50;
                local_5 = true;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_4 = param_4.wrapping_add(idiv(
                    (i_var4 as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
            }
            if param_4 < local_54 {
                local_5 = true;
                param_3 = param_3.wrapping_sub(
                    (((local_54.wrapping_sub(param_4)) as u32).wrapping_mul(local_c) as i32) >> 0xc,
                );
                param_4 = local_54;
            }
            if local_5 {
                if param_3 < param_1 {
                    return;
                }
                if param_2 < param_4 {
                    return;
                }
                local_c = idiv(
                    (param_3.wrapping_sub(param_1) as u32)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000) as i32,
                    param_2.wrapping_sub(param_4).wrapping_add(1),
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 0xc) as u32;
            if u_var8 == 0 {
                if param_4 <= param_2 {
                    loop {
                        FUN_004f8160(
                            param_1.wrapping_sub(i_var5),
                            i_var3.wrapping_add(param_1),
                            param_2,
                            param_5,
                        );
                        local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                        if 0xfff < local_10 {
                            local_10 = local_10.wrapping_sub(0x1000);
                            param_1 = param_1.wrapping_add(1);
                        }
                        param_2 = param_2.wrapping_sub(1);
                        if !(param_4 <= param_2) {
                            break;
                        }
                    }
                    return;
                }
            } else if param_4 <= param_2 {
                loop {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        u_var6 = u_var8.wrapping_add(1);
                        local_10 = local_10.wrapping_sub(0x1000);
                    }
                    let mut local_14 = param_2.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_2);
                    if local_58 < i_var3.wrapping_add(param_2) {
                        i_var4 = local_58;
                    }
                    if local_14 < local_54 {
                        local_14 = local_54;
                    }
                    let p = (local_14 as u32)
                        .wrapping_mul(local_44 as u32)
                        .wrapping_add((param_1 as u32).wrapping_shl(sh))
                        .wrapping_add(base);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, local_44, u_var6, i_var10, u_var2, param_5);
                    }
                    param_1 = (param_1 as u32).wrapping_add(u_var6) as i32;
                    param_2 = param_2.wrapping_sub(1);
                    if !(param_4 <= param_2) {
                        break;
                    }
                }
                return;
            }
        } else {
            local_c = idiv(
                (param_1.wrapping_sub(param_3) as u32)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000) as i32,
                param_2.wrapping_sub(param_4).wrapping_add(1),
            ) as u32;
            b_var1 = local_50 < param_1;
            if b_var1 {
                let i_var4 = local_50.wrapping_sub(param_1);
                param_1 = local_50;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_2 = param_2.wrapping_add(idiv(
                    (i_var4 as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
            }
            local_5 = local_58 < param_2;
            if local_5 {
                let i_var4 = param_2.wrapping_sub(local_58);
                param_2 = local_58;
                param_1 =
                    param_1.wrapping_sub(((i_var4 as u32).wrapping_mul(local_c) as i32) >> 0xc);
            }
            local_5 = local_5 || b_var1;
            if param_3 < local_4c {
                local_5 = true;
                if local_c as i32 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_4 = param_4.wrapping_add(idiv(
                    (local_4c.wrapping_sub(param_3) as u32).wrapping_mul(0x1000) as i32,
                    local_c as i32,
                ));
                param_3 = local_4c;
            }
            if param_4 < local_54 {
                local_5 = true;
                param_3 = param_3.wrapping_add(
                    (((local_54.wrapping_sub(param_4)) as u32).wrapping_mul(local_c) as i32) >> 0xc,
                );
                param_4 = local_54;
            }
            if local_5 {
                if param_1 < param_3 {
                    return;
                }
                if param_2 < param_4 {
                    return;
                }
                local_c = idiv(
                    (param_1.wrapping_sub(param_3) as u32)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000) as i32,
                    param_2.wrapping_sub(param_4).wrapping_add(1),
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 0xc) as u32;
            if u_var8 == 0 {
                while param_4 <= param_2 {
                    FUN_004f8160(
                        param_1.wrapping_sub(i_var5),
                        i_var3.wrapping_add(param_1),
                        param_2,
                        param_5,
                    );
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    if 0xfff < local_10 {
                        local_10 = local_10.wrapping_sub(0x1000);
                        param_1 = param_1.wrapping_sub(1);
                    }
                    param_2 = param_2.wrapping_sub(1);
                }
            } else if param_4 <= param_2 {
                loop {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        u_var6 = u_var8.wrapping_add(1);
                        local_10 = local_10.wrapping_sub(0x1000);
                    }
                    let mut local_14 = param_4.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_4);
                    if local_58 < i_var3.wrapping_add(param_4) {
                        i_var4 = local_58;
                    }
                    if local_14 < local_54 {
                        local_14 = local_54;
                    }
                    let p = (param_3 as u32)
                        .wrapping_shl(sh)
                        .wrapping_add((local_14 as u32).wrapping_mul(local_44 as u32))
                        .wrapping_add(base);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, local_44, u_var6, i_var10, u_var2, param_5);
                    }
                    param_3 = (param_3 as u32).wrapping_add(u_var6) as i32;
                    param_4 = param_4.wrapping_add(1);
                    if !(param_4 <= param_2) {
                        break;
                    }
                }
                return;
            }
        }
    }
}
