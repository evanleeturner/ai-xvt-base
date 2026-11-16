// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005235f0: i32;
    static mut DAT_005233b8: i32;
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009fd3bc: [i32; 0];
    static mut DAT_009fd43c: [i32; 0];
    static mut DAT_009a8d8c: i32;
    static mut DAT_009a8d80: i32;
    static mut DAT_009a8d60: i32;
    static mut DAT_009d1264: i32;
    static mut DAT_009d1154: i32;
    static mut DAT_009d12e8: i32;
    static mut DAT_009a8e28: i32;
    static mut DAT_009a8e1c: i32;
    static mut DAT_009a8e20: i32;
    static mut DAT_00523400: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427a60(a: i16, b: i16, c: i16, d: i32, e: i32);
}

#[inline(always)]
fn fx_mul(a: i32, b: i32) -> u32 {
    let p = (a as i64).wrapping_mul(b as i64);
    let lo = (p as u32) >> 0xf;
    let hi = (((p as u64) >> 32) as u32).wrapping_shl(0x11);
    lo | hi
}

#[inline(always)]
unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: address supplied by the game's own data.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(addr: i32) -> i32 {
    // SAFETY: address supplied by the game's own data.
    unsafe { (addr as u32 as usize as *const i16).read_unaligned() as i32 }
}

#[inline(always)]
unsafe fn rd8(addr: i32) -> u8 {
    // SAFETY: address supplied by the game's own data.
    unsafe { (addr as u32 as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044f880(param_1: i32) {
    // SAFETY: all globals and pointers belong to the game; accesses mirror the original.
    unsafe {
        DAT_005235f0 = 0;
        if DAT_005233b8 != 0 {
            let idx = rd8(param_1.wrapping_add(3)) as usize * 0x1a;
            let i_var2 = (&raw mut DAT_00521f1c as *mut u8)
                .add(idx)
                .cast::<i32>()
                .read_unaligned();
            let i_var3 = rd32(param_1.wrapping_add(4));
            let i_var4 = rd32(param_1.wrapping_add(8));
            let i_var5 = rd32(param_1.wrapping_add(0xc));
            let mut local_a8: i32 = 0;
            let mut local_a4: i32 = 0;
            if 0 < DAT_00a07ce8 {
                let mut i_var17: i32 = DAT_009a1fe8;
                let mut pi_var18: *mut i32 = &raw mut DAT_009fd3bc as *mut i32;
                'outer: loop {
                    let mut pi_var19 = pi_var18;
                    if rd8(i_var17.wrapping_add(3)) != 0 && rd8(i_var17.wrapping_add(2)) == 0x0d {
                        let i_var16 = rd32(i_var17.wrapping_add(4)).wrapping_sub(i_var3);
                        let mut i_var13 = rd32(i_var17.wrapping_add(8)).wrapping_sub(i_var4);
                        let i_var14 = rd32(i_var17.wrapping_add(0xc)).wrapping_sub(i_var5);
                        let u_var15 = FUN_0041f410(i_var16, i_var13, i_var14);
                        if u_var15 < (i_var2 as u32).wrapping_add(0x4000) {
                            let mut i_var6 = rd32(param_1.wrapping_add(0x1f));
                            if i_var6 == 0 {
                                FUN_00427a60(
                                    rd16(param_1.wrapping_add(0x14)) as i16,
                                    rd16(param_1.wrapping_add(0x12)) as i16,
                                    rd16(param_1.wrapping_add(0x10)) as i16,
                                    0,
                                    0,
                                );
                                let i_var12 = DAT_009d1264;
                                let i_var11 = DAT_009d1154;
                                pi_var18.offset(-3).write(
                                    fx_mul(i_var14, DAT_009a8d8c)
                                        .wrapping_add(fx_mul(i_var16, DAT_009a8d80))
                                        .wrapping_add(fx_mul(i_var13, DAT_009a8d60))
                                        as i32,
                                );
                                let i_var10 = DAT_009a8e28;
                                i_var6 = DAT_009a8e1c;
                                pi_var18.offset(-2).write(
                                    0u32.wrapping_sub(
                                        fx_mul(i_var14, i_var11)
                                            .wrapping_add(fx_mul(i_var16, DAT_009d12e8))
                                            .wrapping_add(fx_mul(i_var13, i_var12)),
                                    ) as i32,
                                );
                                i_var13 = fx_mul(i_var14, i_var10)
                                    .wrapping_add(fx_mul(i_var16, DAT_009a8e20))
                                    .wrapping_add(fx_mul(i_var13, i_var6))
                                    as i32;
                            } else {
                                let s0 = rd16(i_var6.wrapping_add(0x99));
                                let s1 = rd16(i_var6.wrapping_add(0x9b));
                                let s2 = rd16(i_var6.wrapping_add(0x9d));
                                pi_var18.offset(-3).write(
                                    fx_mul(i_var14, s2)
                                        .wrapping_add(fx_mul(i_var16, s0))
                                        .wrapping_add(fx_mul(i_var13, s1))
                                        as i32,
                                );
                                i_var6 = rd32(param_1.wrapping_add(0x1f));
                                let s0 = rd16(i_var6.wrapping_add(0x93));
                                let s1 = rd16(i_var6.wrapping_add(0x95));
                                let s2 = rd16(i_var6.wrapping_add(0x97));
                                pi_var18.offset(-2).write(
                                    0u32.wrapping_sub(
                                        fx_mul(i_var14, s2)
                                            .wrapping_add(fx_mul(i_var16, s0))
                                            .wrapping_add(fx_mul(i_var13, s1)),
                                    ) as i32,
                                );
                                i_var6 = rd32(param_1.wrapping_add(0x1f));
                                let s0 = rd16(i_var6.wrapping_add(0x9f));
                                let s1 = rd16(i_var6.wrapping_add(0xa1));
                                let s2 = rd16(i_var6.wrapping_add(0xa3));
                                i_var13 = fx_mul(i_var14, s2)
                                    .wrapping_add(fx_mul(i_var16, s0))
                                    .wrapping_add(fx_mul(i_var13, s1))
                                    as i32;
                            }
                            pi_var18.offset(-1).write(i_var13);
                            pi_var18.write(0x10);
                            match rd8(i_var17.wrapping_add(3)) {
                                0x7f..=0x82 => {
                                    match rd8(i_var17.wrapping_add(0x19)) {
                                        2 => pi_var18.write(0xc0),
                                        3 => pi_var18.write(0x140),
                                        4 => pi_var18.write(0x1e0),
                                        5..=8 => pi_var18.write(0x140),
                                        9 => pi_var18.write(0xc0),
                                        10 => pi_var18.write(0x60),
                                        0xb => pi_var18.write(0x30),
                                        _ => {}
                                    }
                                    let t = rd32(i_var17.wrapping_add(0x1f));
                                    if t != 0 {
                                        let b_var1 = rd8(t.wrapping_add(1));
                                        if 3 < b_var1 {
                                            let m = ((b_var1 as i32 + 4) >> 2) as u32;
                                            pi_var18.write(
                                                m.wrapping_mul(pi_var18.read() as u32) as i32
                                            );
                                        }
                                    }
                                }
                                0x83 | 0x84 => match rd8(i_var17.wrapping_add(0x19)) {
                                    2 => pi_var18.write(0x30),
                                    3 => pi_var18.write(0x60),
                                    4 => pi_var18.write(0x40),
                                    5 => pi_var18.write(0x20),
                                    _ => {}
                                },
                                _ => {
                                    pi_var18.write(
                                        (DAT_00523400 as u32).wrapping_add(0xffffff00) as i32
                                    );
                                }
                            }
                            pi_var19 = pi_var18.add(4);
                            local_a8 = local_a8.wrapping_add(1);
                            pi_var18.write(((pi_var18.read() as u32) << 3) as i32);
                            if pi_var19 == (&raw mut DAT_009fd43c as *mut i32) {
                                break 'outer;
                            }
                        }
                    }
                    i_var17 = i_var17.wrapping_add(0x23);
                    local_a4 = local_a4.wrapping_add(1);
                    pi_var18 = pi_var19;
                    if !(local_a4 < DAT_00a07ce8) {
                        break;
                    }
                }
            }
            DAT_005235f0 = local_a8;
        }
    }
}
