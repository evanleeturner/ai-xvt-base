// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_00521f1c: [u8; 1];
    static mut DAT_0051c641: [u8; 1];
    static mut DAT_0051c645: [u8; 1];
    static mut DAT_0051c649: [u8; 1];
    static mut DAT_00a00870: [i32; 1];
    static mut DAT_00a00874: [i32; 1];
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00427bd0(a: i32, b: i32, c: i32);
    fn FUN_00427d30(a: i32, b: i32, c: i32);
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004ae2a0(a: u32, b: i32) -> i32;
    fn FUN_004ae5a0(a: u32, b: i32) -> i32;
    fn FUN_004ae6e0(a: u32, b: i32) -> i32;
    fn FUN_004ae640(a: u32, b: i32) -> i32;
    fn FUN_004037b0(a: i32, b: i32, c: i32, d: i32);
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: address supplied by the game's own data.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}
unsafe fn r16(a: i32) -> i16 {
    // SAFETY: address supplied by the game's own data.
    unsafe { (a as u32 as usize as *const i16).read_unaligned() }
}
unsafe fn r8(a: i32) -> u8 {
    // SAFETY: address supplied by the game's own data.
    unsafe { (a as u32 as usize as *const u8).read() }
}

fn fix(v: i64) -> u32 {
    ((v as u32) >> 15) | ((((v as u64) >> 32) as u32) << 17)
}

unsafe fn dot(obj: i32, o: i32, a: i32, b: i32, c: i32) -> i32 {
    // SAFETY: obj is a game object pointer.
    unsafe {
        let s0 = r16(obj.wrapping_add(o)) as i32 as i64;
        let l3 = s0.wrapping_mul(a as i64);
        let s1 = r16(obj.wrapping_add(o + 2)) as i32 as i64;
        let l4 = s1.wrapping_mul(b as i64);
        let s2 = r16(obj.wrapping_add(o + 4)) as i32 as i64;
        let l5 = s2.wrapping_mul(c as i64);
        fix(l5).wrapping_add(fix(l3)).wrapping_add(fix(l4)) as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a8cc0(param_1: i32, param_2: i32) {
    // SAFETY: all memory accessed is the game's own, as in the original.
    unsafe {
        let iv14 = (param_2 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32) as i32;
        let iv2 = r32(((&raw mut DAT_00521f1c).cast::<u8>() as usize as i32)
            .wrapping_add((r8(iv14.wrapping_add(3)) as i32).wrapping_mul(0x1a)));
        let iv6 = (DAT_009a1fe8 as u32).wrapping_add((param_1 as u32).wrapping_mul(0x23)) as i32;
        let iv13 = r32(iv6.wrapping_add(4)).wrapping_sub(r32(iv14.wrapping_add(4)));
        let mut iv10 = r32(iv6.wrapping_add(8)).wrapping_sub(r32(iv14.wrapping_add(8)));
        let iv6 = r32(iv6.wrapping_add(0xc)).wrapping_sub(r32(iv14.wrapping_add(0xc)));
        let iv7 = FUN_0041f410(iv13, iv10, iv6);
        if (iv2 as u32).wrapping_mul(2) as i32 > iv7 - 0 && false {
            return;
        }
        if ((iv2 as u32).wrapping_mul(2) as i32) < iv7 {
            return;
        }
        let iv8 = r32(iv14.wrapping_add(0x1f));
        if iv8 == 0 {
            return;
        }
        let obj2 = r32(iv8.wrapping_add(0xa9));
        if r16(obj2.wrapping_add(0x1e5)) == 0 {
            return;
        }
        if r8(iv14.wrapping_add(3)) == b'E' {
            if 0x1000 < iv7 {
                return;
            }
        } else {
            if r8(iv8.wrapping_add(0x92)) != 0 {
                let s0 = r16(iv14.wrapping_add(0x12));
                let s1 = r16(iv14.wrapping_add(0x10));
                FUN_00427bd0(s0 as i32, s1 as i32, iv14);
                let s0 = r16(iv14.wrapping_add(0x14));
                FUN_00427d30(s0 as i32, 0, iv14);
            }
            let obj = r32(iv14.wrapping_add(0x1f));
            let iv7 = dot(obj, 0x99, iv13, iv10, iv6);
            let iv8 = dot(obj, 0x93, iv13, iv10, iv6);
            iv10 = dot(obj, 0x9f, iv13, iv10, iv6);

            let iv6 = r32(obj.wrapping_add(0xa9));
            if iv6 == 0 {
                return;
            }
            let iv11 = (r8(iv6.wrapping_add(4)) as u32).wrapping_mul(0x101) as i32;
            let iv6 = r32((&raw mut DAT_0051c641).cast::<u8>() as usize as i32 + iv11);
            let iv13 = r32((&raw mut DAT_0051c649).cast::<u8>() as usize as i32 + iv11);
            let mut iv11 = r32((&raw mut DAT_0051c645).cast::<u8>() as usize as i32 + iv11);
            let c1 = r8(iv14.wrapping_add(3));
            if (c1 == b'5' || c1 == b'3' || c1 == b'4') && 0 < iv10 {
                return;
            }
            if c1 == b'6' {
                iv10 = iv10.wrapping_sub(iv11);
                if iv10 < 0 {
                    iv10 = iv10.wrapping_neg();
                }
                if iv11 < 0 {
                    iv11 = iv11.wrapping_neg();
                }
                if iv11.wrapping_sub(iv11 >> 2) < iv10 {
                    return;
                }
                let r = FUN_0041f410(iv7.wrapping_sub(iv6), iv8.wrapping_sub(iv13), iv10);
                if (iv2 >> 3) < r {
                    return;
                }
            } else if c1 == b'[' {
                let r = FUN_0041f410(
                    iv7.wrapping_sub(iv6),
                    iv8.wrapping_sub(iv13),
                    iv10.wrapping_sub(iv11),
                );
                if iv2 / 6 < r {
                    let u12 = r8(iv14.wrapping_add(3)) as u32;
                    let p870 = (&raw mut DAT_00a00870).cast::<i32>();
                    let p874 = (&raw mut DAT_00a00874).cast::<i32>();
                    let local_58 = if u12 < 0x49 {
                        p870.add((u12 * 0x65) as usize).read()
                    } else {
                        FUN_004adc40(u12)
                    };
                    let mut iv6 = 0i32;
                    if local_58 < 1 {
                        return;
                    }
                    loop {
                        let mut iv13;
                        if u12 < 0x49 {
                            if iv6 < 0 {
                                iv13 = 0;
                            } else {
                                iv13 = iv6;
                                let n = p870.add((u12 * 0x65) as usize).read();
                                if n <= iv6 {
                                    iv13 = n.wrapping_sub(1);
                                }
                                iv13 = p874
                                    .add((iv13 as u32).wrapping_add(u12 * 0x65) as usize)
                                    .read();
                            }
                        } else {
                            iv13 = FUN_004ae2a0(u12, iv6);
                        }
                        if iv13 == 0x10 {
                            let a = FUN_004ae5a0(u12, iv6);
                            let b = FUN_004ae6e0(u12, iv6);
                            let c = FUN_004ae640(u12, iv6);
                            FUN_004037b0(iv14, a, b, c.wrapping_neg());
                            let d = FUN_0041f410(
                                iv7.wrapping_sub(DAT_009d8a58),
                                iv8.wrapping_sub(DAT_009d8a54),
                                iv10.wrapping_sub(DAT_009d8b60),
                            );
                            if d < iv2 / 6 {
                                break;
                            }
                        }
                        iv6 = iv6.wrapping_add(1);
                        if local_58 <= iv6 {
                            return;
                        }
                    }
                }
            } else {
                let r = FUN_0041f410(
                    iv7.wrapping_sub(iv6),
                    iv8.wrapping_sub(iv13),
                    iv10.wrapping_sub(iv11),
                );
                if iv2 / 6 < r {
                    return;
                }
            }
        }
        let o = r32((DAT_009a1fe8 as u32)
            .wrapping_add(0x1f)
            .wrapping_add((param_1 as u32).wrapping_mul(0x23)) as i32);
        if o != 0 {
            let o = r32(o.wrapping_add(0xa9));
            if o != 0 {
                ((o.wrapping_add(0x1d)) as u32 as usize as *mut i32).write_unaligned(0x28000);
                ((o.wrapping_add(0x248)) as u32 as usize as *mut i16).write_unaligned(0);
            }
        }
    }
}
