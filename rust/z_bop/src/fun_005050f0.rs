// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aa7b13: [u8; 4];
    static mut DAT_00aa7b82: [u8; 4];
    static mut DAT_00aa7b86: [u8; 4];
    static mut DAT_00aa7b8a: [u8; 4];
    static mut DAT_00aa7b8e: [u8; 4];
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00505cd0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u16);
    fn FUN_005063b0(param_1: i32, param_2: i32, param_3: i32, param_4: u16);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

fn mulshr(a: i32, c: u32) -> i32 {
    (a.wrapping_mul(c as i32)) >> 12
}

fn addr(base: i32, stride: i32, y: i32, x: i32, sh: u8) -> *mut u8 {
    (base as u32)
        .wrapping_add((stride as u32).wrapping_mul(y as u32))
        .wrapping_add((x as u32).wrapping_shl(sh as u32 & 0x1f)) as usize as *mut u8
}

unsafe fn row_fill(p: *mut u8, count: u32, mode: i32, b: u8, w: u16) {
    // SAFETY: the destination is the game's frame buffer, written where the original writes.
    unsafe {
        if mode == 0 {
            let mut q = p;
            let mut n = count >> 2;
            while n != 0 {
                q.cast::<u32>()
                    .write_unaligned((b as u32).wrapping_mul(0x01010101));
                q = q.wrapping_add(4);
                n -= 1;
            }
            let mut n = count & 3;
            while n != 0 {
                q.write(b);
                q = q.wrapping_add(1);
                n -= 1;
            }
        } else if mode == 1 && 0 < (count as i32) {
            let mut q = p;
            let mut n = count;
            while n != 0 {
                q.cast::<u16>().write_unaligned(w);
                q = q.wrapping_add(2);
                n -= 1;
            }
        }
    }
}

unsafe fn fill_rows(
    mut p: *mut u8,
    mut rows: i32,
    stride: i32,
    count: u32,
    mode: i32,
    b: u8,
    w: u16,
) {
    // SAFETY: forwarded to row_fill under the same frame buffer contract.
    unsafe {
        loop {
            row_fill(p, count, mode, b, w);
            p = p.wrapping_offset(stride as isize);
            rows = rows.wrapping_sub(1);
            if rows == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_005050f0(
    mut param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    mut param_4: i32,
    param_5: u16,
    param_6: i32,
) {
    macro_rules! chk {
        ($a:expr, $b:expr) => {{
            let a: i32 = $a;
            let b: i32 = $b;
            if b == 0 || (a == i32::MIN && b == -1) {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                return;
            }
            a / b
        }};
    }
    // SAFETY: the globals and callees are the game's own; pointers written are
    // the game's frame buffer addresses computed as the original does.
    unsafe {
        let mut local_10: i32 = 0x800;
        if param_6 == 1 {
            FUN_00505cd0(param_1, param_2, param_3, param_4, param_5);
            return;
        }
        let v_b1f = (&raw const DAT_00aa7b1f).cast::<i32>().read_unaligned();
        let v_b13 = (&raw const DAT_00aa7b13).cast::<i32>().read_unaligned();
        let v_b82 = (&raw const DAT_00aa7b82).cast::<i32>().read_unaligned();
        let v_b86 = (&raw const DAT_00aa7b86).cast::<i32>().read_unaligned();
        let v_b8a = (&raw const DAT_00aa7b8a).cast::<i32>().read_unaligned();
        let v_b8e = (&raw const DAT_00aa7b8e).cast::<i32>().read_unaligned();
        let i_var10 = v_b1f >> 4;
        let i_var5 = param_6 >> 1;
        let i_var3 = param_6.wrapping_sub(1) >> 1;
        let b_var9 = i_var10 as u8;
        let u_var2 = param_5 as u8;
        let base = DAT_00aa6cfc;

        if param_4 == param_2 {
            if v_b8a <= param_2 && param_2 <= v_b8e {
                if param_3 < param_1 {
                    if param_3 < v_b82 {
                        param_3 = v_b82;
                    }
                    if v_b86 < param_1 {
                        param_1 = v_b86;
                    }
                    param_2 = param_2.wrapping_sub(i_var5);
                    param_4 = param_4.wrapping_add(i_var3);
                    if v_b8e < param_4 {
                        param_4 = v_b8e;
                    }
                    if param_2 < v_b8a {
                        param_2 = v_b8a;
                    }
                    let p = addr(base, v_b13, param_2, param_3, b_var9);
                    let u_var8 = (param_1 as u32)
                        .wrapping_sub(param_3 as u32)
                        .wrapping_add(1);
                    if param_2 <= param_4 {
                        let rows = param_4.wrapping_sub(param_2).wrapping_add(1);
                        fill_rows(p, rows, v_b13, u_var8, i_var10, u_var2, param_5);
                        return;
                    }
                } else {
                    if v_b86 < param_3 {
                        param_3 = v_b86;
                    }
                    if param_1 < v_b82 {
                        param_1 = v_b82;
                    }
                    param_2 = param_2.wrapping_sub(i_var5);
                    param_4 = param_4.wrapping_add(i_var3);
                    if v_b8e < param_4 {
                        param_4 = v_b8e;
                    }
                    if param_2 < v_b8a {
                        param_2 = v_b8a;
                    }
                    let p = addr(base, v_b13, param_2, param_1, b_var9);
                    let u_var8 = (param_3 as u32)
                        .wrapping_sub(param_1 as u32)
                        .wrapping_add(1);
                    if param_2 <= param_4 {
                        let rows = param_4.wrapping_sub(param_2).wrapping_add(1);
                        fill_rows(p, rows, v_b13, u_var8, i_var10, u_var2, param_5);
                        return;
                    }
                }
            }
        } else if param_2 < param_4 && param_1 <= param_3 {
            let mut local_c = chk!(
                param_3
                    .wrapping_sub(param_1)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000),
                param_4.wrapping_sub(param_2).wrapping_add(1)
            ) as u32;
            let b_var1 = param_1 < v_b82;
            if b_var1 {
                param_2 = param_2.wrapping_add(chk!(
                    v_b82.wrapping_sub(param_1).wrapping_mul(0x1000),
                    local_c as i32
                ));
                param_1 = v_b82;
            }
            let mut local_5 = param_2 < v_b8a;
            if local_5 {
                param_1 = param_1.wrapping_add(mulshr(v_b8a.wrapping_sub(param_2), local_c));
                param_2 = v_b8a;
            }
            local_5 = local_5 || b_var1;
            if v_b86 < param_3 {
                local_5 = true;
                param_4 = param_4.wrapping_add(chk!(
                    v_b86.wrapping_sub(param_3).wrapping_mul(0x1000),
                    local_c as i32
                ));
                param_3 = v_b86;
            }
            if v_b8e < param_4 {
                let i_var4 = param_4.wrapping_sub(v_b8e);
                param_4 = v_b8e;
                local_5 = true;
                param_3 = param_3.wrapping_sub(mulshr(i_var4, local_c));
            }
            if local_5 {
                if param_3 < param_1 {
                    return;
                }
                if param_4 < param_2 {
                    return;
                }
                local_c = chk!(
                    param_3
                        .wrapping_sub(param_1)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    param_4.wrapping_sub(param_2).wrapping_add(1)
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 12) as u32;
            if u_var8 == 0 {
                while param_2 <= param_4 {
                    FUN_005063b0(
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
                }
                return;
            } else {
                while param_2 <= param_4 {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        local_10 = local_10.wrapping_sub(0x1000);
                        u_var6 = u_var8.wrapping_add(1);
                    }
                    let mut local_14 = param_2.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_2);
                    if v_b8e < i_var4 {
                        i_var4 = v_b8e;
                    }
                    if local_14 < v_b8a {
                        local_14 = v_b8a;
                    }
                    let p = addr(base, v_b13, local_14, param_1, b_var9);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, v_b13, u_var6, i_var10, u_var2, param_5);
                    }
                    param_1 = param_1.wrapping_add(u_var6 as i32);
                    param_2 = param_2.wrapping_add(1);
                }
                return;
            }
        } else if param_2 < param_4 && param_3 <= param_1 {
            let mut local_c = chk!(
                param_1
                    .wrapping_sub(param_3)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000),
                param_4.wrapping_sub(param_2).wrapping_add(1)
            ) as u32;
            let b_var1 = v_b86 < param_1;
            if b_var1 {
                let i_var4 = param_1.wrapping_sub(v_b86);
                param_1 = v_b86;
                param_2 = param_2.wrapping_add(chk!(i_var4.wrapping_mul(0x1000), local_c as i32));
            }
            let mut local_5 = param_2 < v_b8a;
            if local_5 {
                param_1 = param_1.wrapping_sub(mulshr(v_b8a.wrapping_sub(param_2), local_c));
                param_2 = v_b8a;
            }
            local_5 = local_5 || b_var1;
            if param_3 < v_b82 {
                let i_var4 = param_3.wrapping_sub(v_b82);
                param_3 = v_b82;
                local_5 = true;
                param_4 = param_4.wrapping_add(chk!(i_var4.wrapping_mul(0x1000), local_c as i32));
            }
            if v_b8e < param_4 {
                let i_var4 = param_4.wrapping_sub(v_b8e);
                param_4 = v_b8e;
                local_5 = true;
                param_3 = param_3.wrapping_add(mulshr(i_var4, local_c));
            }
            if local_5 {
                if param_1 < param_3 {
                    return;
                }
                if param_4 < param_2 {
                    return;
                }
                local_c = chk!(
                    param_1
                        .wrapping_sub(param_3)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    param_4.wrapping_sub(param_2).wrapping_add(1)
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 12) as u32;
            if u_var8 == 0 {
                while param_2 <= param_4 {
                    FUN_005063b0(
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
                }
                return;
            } else {
                while param_2 <= param_4 {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        local_10 = local_10.wrapping_sub(0x1000);
                        u_var6 = u_var8.wrapping_add(1);
                    }
                    let mut local_14 = param_4.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_4);
                    if v_b8e < i_var4 {
                        i_var4 = v_b8e;
                    }
                    if local_14 < v_b8a {
                        local_14 = v_b8a;
                    }
                    let p = addr(base, v_b13, local_14, param_3, b_var9);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, v_b13, u_var6, i_var10, u_var2, param_5);
                    }
                    param_3 = param_3.wrapping_add(u_var6 as i32);
                    param_4 = param_4.wrapping_sub(1);
                }
                return;
            }
        } else if param_4 < param_2 && param_1 <= param_3 {
            let mut local_c = chk!(
                param_3
                    .wrapping_sub(param_1)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000),
                param_2.wrapping_sub(param_4).wrapping_add(1)
            ) as u32;
            let b_var1 = param_1 < v_b82;
            if b_var1 {
                let i_var4 = param_1.wrapping_sub(v_b82);
                param_1 = v_b82;
                param_2 = param_2.wrapping_add(chk!(i_var4.wrapping_mul(0x1000), local_c as i32));
            }
            let mut local_5 = v_b8e < param_2;
            if local_5 {
                let i_var4 = param_2.wrapping_sub(v_b8e);
                param_2 = v_b8e;
                param_1 = param_1.wrapping_add(mulshr(i_var4, local_c));
            }
            local_5 = local_5 || b_var1;
            if v_b86 < param_3 {
                let i_var4 = param_3.wrapping_sub(v_b86);
                param_3 = v_b86;
                local_5 = true;
                param_4 = param_4.wrapping_add(chk!(i_var4.wrapping_mul(0x1000), local_c as i32));
            }
            if param_4 < v_b8a {
                local_5 = true;
                param_3 = param_3.wrapping_sub(mulshr(v_b8a.wrapping_sub(param_4), local_c));
                param_4 = v_b8a;
            }
            if local_5 {
                if param_3 < param_1 {
                    return;
                }
                if param_2 < param_4 {
                    return;
                }
                local_c = chk!(
                    param_3
                        .wrapping_sub(param_1)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    param_2.wrapping_sub(param_4).wrapping_add(1)
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 12) as u32;
            if u_var8 == 0 {
                while param_4 <= param_2 {
                    FUN_005063b0(
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
                }
                return;
            } else {
                while param_4 <= param_2 {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        local_10 = local_10.wrapping_sub(0x1000);
                        u_var6 = u_var8.wrapping_add(1);
                    }
                    let mut local_14 = param_2.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_2);
                    if v_b8e < i_var4 {
                        i_var4 = v_b8e;
                    }
                    if local_14 < v_b8a {
                        local_14 = v_b8a;
                    }
                    let p = addr(base, v_b13, local_14, param_1, b_var9);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, v_b13, u_var6, i_var10, u_var2, param_5);
                    }
                    param_1 = param_1.wrapping_add(u_var6 as i32);
                    param_2 = param_2.wrapping_sub(1);
                }
                return;
            }
        } else {
            let mut local_c = chk!(
                param_1
                    .wrapping_sub(param_3)
                    .wrapping_add(1)
                    .wrapping_mul(0x1000),
                param_2.wrapping_sub(param_4).wrapping_add(1)
            ) as u32;
            let b_var1 = v_b86 < param_1;
            if b_var1 {
                param_2 = param_2.wrapping_add(chk!(
                    v_b86.wrapping_sub(param_1).wrapping_mul(0x1000),
                    local_c as i32
                ));
                param_1 = v_b86;
            }
            let mut local_5 = v_b8e < param_2;
            if local_5 {
                let i_var4 = param_2.wrapping_sub(v_b8e);
                param_2 = v_b8e;
                param_1 = param_1.wrapping_sub(mulshr(i_var4, local_c));
            }
            local_5 = local_5 || b_var1;
            if param_3 < v_b82 {
                local_5 = true;
                param_4 = param_4.wrapping_add(chk!(
                    v_b82.wrapping_sub(param_3).wrapping_mul(0x1000),
                    local_c as i32
                ));
                param_3 = v_b82;
            }
            if param_4 < v_b8a {
                local_5 = true;
                param_3 = param_3.wrapping_add(mulshr(v_b8a.wrapping_sub(param_4), local_c));
                param_4 = v_b8a;
            }
            if local_5 {
                if param_1 < param_3 {
                    return;
                }
                if param_2 < param_4 {
                    return;
                }
                local_c = chk!(
                    param_1
                        .wrapping_sub(param_3)
                        .wrapping_add(1)
                        .wrapping_mul(0x1000),
                    param_2.wrapping_sub(param_4).wrapping_add(1)
                ) as u32;
            }
            let u_var8 = ((local_c as i32) >> 12) as u32;
            if u_var8 == 0 {
                while param_4 <= param_2 {
                    FUN_005063b0(
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
            } else {
                while param_4 <= param_2 {
                    local_10 = local_10.wrapping_add((local_c & 0xfff) as i32);
                    let mut u_var6 = u_var8;
                    if 0xfff < local_10 {
                        local_10 = local_10.wrapping_sub(0x1000);
                        u_var6 = u_var8.wrapping_add(1);
                    }
                    let mut local_14 = param_4.wrapping_sub(i_var5);
                    let mut i_var4 = i_var3.wrapping_add(param_4);
                    if v_b8e < i_var4 {
                        i_var4 = v_b8e;
                    }
                    if local_14 < v_b8a {
                        local_14 = v_b8a;
                    }
                    let p = addr(base, v_b13, local_14, param_3, b_var9);
                    if local_14 <= i_var4 {
                        let rows = i_var4.wrapping_sub(local_14).wrapping_add(1);
                        fill_rows(p, rows, v_b13, u_var6, i_var10, u_var2, param_5);
                    }
                    param_3 = param_3.wrapping_add(u_var6 as i32);
                    param_4 = param_4.wrapping_add(1);
                }
                return;
            }
        }
    }
}
