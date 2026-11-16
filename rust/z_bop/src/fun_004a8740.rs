// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_00a00870: [i32; 0];
    static mut DAT_00a0093c: [i32; 0];
    static mut DAT_009e969d: [u8; 0];
    static mut DAT_009e969f: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004ae200(a: u32, b: i32) -> *mut i32;
    fn __ftol() -> i64;
    fn FUN_0041dc30(a: u32, b: u16, c: u32, d: u32, e: u32, f: u16, g: u32, h: u16, i: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

unsafe fn r32(p: usize) -> i32 {
    // SAFETY: the address is a game object field read where the original reads it.
    unsafe { (p as *const i32).read_unaligned() }
}

unsafe fn r16(p: usize) -> i16 {
    // SAFETY: the address is a game object field read where the original reads it.
    unsafe { (p as *const i16).read_unaligned() }
}

unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
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

unsafe fn raise() {
    // SAFETY: the OS call is made with the original's arguments.
    unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) }
}

fn fix15(l: i64) -> u32 {
    ((l as u64) >> 15) as u32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a8740(param_1: u32, param_2: i32) {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed in the original's order and at its offsets.
    unsafe {
        let u_var9: u32 = (param_2 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32);
        let p: usize = u_var9 as usize;
        let tbl = (&raw mut DAT_00521f1c).cast::<u8>();
        let i_var3: i32 = tbl
            .add((*((p + 3) as *const u8) as usize).wrapping_mul(0x1a))
            .cast::<i32>()
            .read_unaligned();
        let i_var12_base: usize =
            (DAT_009a1fe8 as u32).wrapping_add(param_1.wrapping_mul(0x23)) as usize;
        let i_var15_0 = (r32(i_var12_base + 4) as u32).wrapping_sub(r32(p + 4) as u32) as i32;
        let i_var17_0 = (r32(i_var12_base + 8) as u32).wrapping_sub(r32(p + 8) as u32) as i32;
        let i_var13_0 = (r32(i_var12_base + 0xc) as u32).wrapping_sub(r32(p + 0xc) as u32) as i32;
        let mut i_var15 = i_var15_0;
        let mut i_var17 = i_var17_0;
        let mut i_var13 = i_var13_0;
        let mut i_var12 = FUN_0041f410(i_var15, i_var17, i_var13);
        let mut i_obj = r32(p + 0x1f);
        if i_var12 <= (i_var3 as u32).wrapping_mul(3) as i32 && i_obj != 0 {
            let mut q = i_obj as u32 as usize;
            if *((q + 0x92) as *const u8) != 0 {
                let a = r16(p + 0x12);
                let b = r16(p + 0x10);
                FUN_00427bd0(a, b, u_var9 as i32);
                let c = r16(p + 0x14);
                FUN_00427d30(c, 0, u_var9 as i32);
            }
            i_obj = r32(p + 0x1f);
            q = i_obj as u32 as usize;
            let l_var4 = (r16(q + 0x99) as i32 as i64).wrapping_mul(i_var15 as i64);
            let l_var5 = (r16(q + 0x9b) as i32 as i64).wrapping_mul(i_var17 as i64);
            let l_var6 = (r16(q + 0x9d) as i32 as i64).wrapping_mul(i_var13 as i64);
            let mut l_var19 = (r16(q + 0x93) as i32 as i64).wrapping_mul(i_var15 as i64);
            let mut l_var7 = (r16(q + 0x95) as i32 as i64).wrapping_mul(i_var17 as i64);
            let mut l_var8 = (r16(q + 0x97) as i32 as i64).wrapping_mul(i_var13 as i64);
            i_var12 = fix15(l_var8)
                .wrapping_add(fix15(l_var19))
                .wrapping_add(fix15(l_var7)) as i32;
            let i_var10: i32 = 0u32.wrapping_sub(i_var12 as u32) as i32;
            if i_var12 == i_var3 || (0u32.wrapping_sub(i_var3 as u32) as i32) < i_var10 {
                l_var19 = (r16(q + 0x9f) as i32 as i64).wrapping_mul(i_var15 as i64);
                l_var7 = (r16(q + 0xa1) as i32 as i64).wrapping_mul(i_var17 as i64);
                l_var8 = (r16(q + 0xa3) as i32 as i64).wrapping_mul(i_var13 as i64);
                i_var12 = fix15(l_var8)
                    .wrapping_add(fix15(l_var19))
                    .wrapping_add(fix15(l_var7)) as i32;
                let b_var1: u8 = *((p + 3) as *const u8);
                let n00870 = (&raw mut DAT_00a00870).cast::<i32>();
                let n0093c = (&raw mut DAT_00a0093c).cast::<i32>();
                let local_5c: i32 = if b_var1 < 0x49 {
                    n00870.add((b_var1 as usize).wrapping_mul(0x65)).read()
                } else {
                    FUN_004adc40(b_var1 as u32)
                };
                let mut local_6c: i32 = 0;
                if 0 < local_5c {
                    loop {
                        let b3: u8 = *((p + 3) as *const u8);
                        let u_idx: u32 = b3 as u32;
                        let pvar18: *mut i32;
                        if b3 < 0x49 {
                            if local_6c < 0 {
                                pvar18 = core::ptr::null_mut();
                            } else {
                                i_var13 = local_6c;
                                let cnt = n00870.add((u_idx as usize).wrapping_mul(0x65)).read();
                                if cnt <= local_6c {
                                    i_var13 = cnt.wrapping_sub(1);
                                }
                                let idx = (i_var13 as u32).wrapping_add(u_idx.wrapping_mul(0x65))
                                    as usize;
                                pvar18 = n0093c.add(idx).read() as u32 as usize as *mut i32;
                            }
                        } else {
                            pvar18 = FUN_004ae200(u_idx, local_6c);
                        }
                        if !pvar18.is_null() && pvar18.read_unaligned() == 6 {
                            i_var13 = __ftol() as i32;
                            let c_var2: i8 = *((p + 3) as *const u8) as i8;
                            if c_var2 == 0x36 {
                                i_var13 >>= 6;
                            }
                            i_var15 = __ftol() as i32;
                            i_var17 = __ftol() as i32;
                            let f12 = i_var12 as f32;
                            let f13 = pvar18.add(0xd).read_unaligned() as f32;
                            let p7 = pvar18.add(7).read_unaligned() as u32;
                            if (c_var2 != 0x36 || f12 <= f13)
                                && (c_var2 != 0x31 || p7 < 0x80000001 || f12 <= f13)
                            {
                                if i_var13 < i_var15 {
                                    i_var13 = i_var15;
                                }
                                if i_var13 < i_var17 {
                                    i_var13 = i_var17;
                                }
                                let mut i_var16: i32 = (i_var13 as u32).wrapping_mul(8) as i32;
                                if ((i_var3 as u32).wrapping_mul(2) as i32) < i_var16 {
                                    i_var16 = (i_var3 as u32).wrapping_mul(2) as i32;
                                }
                                let mut local_68: i32;
                                if c_var2 == 0x36 {
                                    let f = __ftol() as i32;
                                    local_68 =
                                        ((i_var16 >> 8) as u32).wrapping_sub(f as u32) as i32;
                                } else {
                                    let f = __ftol() as i32;
                                    local_68 = 0u32.wrapping_sub(f as u32) as i32;
                                }
                                local_68 = (i_var10 as u32).wrapping_add(local_68 as u32) as i32;
                                if 0 < local_68 && local_68 <= i_var16 {
                                    let f = __ftol() as i32;
                                    i_var13 = fix15(l_var6)
                                        .wrapping_add(fix15(l_var4))
                                        .wrapping_add(fix15(l_var5))
                                        .wrapping_sub(f as u32)
                                        as i32;
                                    let f = __ftol() as i32;
                                    let mut i_var14: i32 =
                                        (i_var12 as u32).wrapping_sub(f as u32) as i32;
                                    i_var15 =
                                        (i_var15 as u32).wrapping_add((i_var15 >> 1) as u32) as i32;
                                    if i_var16 == 0 {
                                        raise();
                                    }
                                    let m = (i_var15 as u32).wrapping_mul(local_68 as u32) as i32;
                                    i_var15 = (i_var15 as u32).wrapping_add(idiv(m, i_var16) as u32)
                                        as i32;
                                    i_var17 =
                                        (i_var17 as u32).wrapping_add((i_var17 >> 1) as u32) as i32;
                                    let m = (i_var17 as u32).wrapping_mul(local_68 as u32) as i32;
                                    i_var17 = (i_var17 as u32).wrapping_add(idiv(m, i_var16) as u32)
                                        as i32;
                                    if i_var13 < 0 {
                                        i_var13 = 0u32.wrapping_sub(i_var13 as u32) as i32;
                                    }
                                    if i_var14 < 0 {
                                        i_var14 = 0u32.wrapping_sub(i_var14 as u32) as i32;
                                    }
                                    if i_var13 <= i_var15 && i_var14 <= i_var17 {
                                        let d1 =
                                            (i_var17 as u32).wrapping_add(i_var15 as u32) as i32;
                                        if d1 == 0 {
                                            raise();
                                        }
                                        let mut n1 = (i_var17 as u32)
                                            .wrapping_sub(i_var14 as u32)
                                            .wrapping_sub(i_var13 as u32)
                                            .wrapping_add(i_var15 as u32)
                                            .wrapping_mul(100)
                                            as i32;
                                        n1 = idiv(n1, d1);
                                        let mut n2 = (i_var16 as u32)
                                            .wrapping_sub(local_68 as u32)
                                            .wrapping_mul(100)
                                            as i32;
                                        n2 = idiv(n2, i_var16);
                                        let mut d2 = (n1 as u32).wrapping_mul(n2 as u32) as i32;
                                        d2 /= 100;
                                        let m =
                                            (d2 as u32).wrapping_mul((i_var16 >> 8) as u32) as i32;
                                        let mut u_var11: u32 = (m / 100) as u32;
                                        if c_var2 == 0x36 {
                                            u_var11 = (((u_var11 as i32) >> 4)
                                                .wrapping_add((u_var11 as i32) >> 5))
                                                as u32;
                                        }
                                        if 0x40 < u_var11 as i32 {
                                            u_var11 = 0x40;
                                        }
                                        let base15: usize = (DAT_009a1fe8 as u32)
                                            .wrapping_add(param_1.wrapping_mul(0x23))
                                            as usize;
                                        i_var13 = r32(base15 + 0x1b);
                                        if i_var13 != -1 {
                                            i_var13 = (i_var13 as u32).wrapping_mul(0x5bd) as i32;
                                            let pa = (&raw mut DAT_009e969f)
                                                .cast::<u8>()
                                                .byte_offset(i_var13 as isize);
                                            let us = pa.cast::<u16>().read_unaligned();
                                            if (us as i32) < u_var11 as i32 {
                                                let pb = (&raw mut DAT_009e969d)
                                                    .cast::<u8>()
                                                    .byte_offset(i_var13 as isize);
                                                pb.cast::<u16>().write_unaligned(param_2 as u16);
                                                pa.cast::<u16>().write_unaligned(u_var11 as u16);
                                            }
                                        }
                                        i_var13 = r32(base15 + 0x1f);
                                        if i_var13 != 0 {
                                            i_var13 = r32((i_var13 as u32 as usize) + 0xa9);
                                            if i_var13 != 0 {
                                                let e1 = r32((i_var13 as u32 as usize) + 0x215);
                                                let e2 = r32((i_var13 as u32 as usize) + 0x211);
                                                if (e1 as u32).wrapping_add(e2 as u32) as i32 == 0 {
                                                    u_var11 = ((u_var11 as i32) >> 3) as u32;
                                                }
                                            }
                                        }
                                        if (u_var11 as i32) < 1 {
                                            u_var11 = 1;
                                        }
                                        FUN_0041dc30(
                                            param_1, 0xffff, 0xfffe, u_var11, 0, 0, 0, 0, u_var9,
                                        );
                                    }
                                }
                            }
                        }
                        local_6c = local_6c.wrapping_add(1);
                        if !(local_6c < local_5c) {
                            break;
                        }
                    }
                }
            }
        }
    }
}
