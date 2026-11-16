// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_009a8d8c: i32;
    static mut DAT_009a8d80: i32;
    static mut DAT_009a8d60: i32;
    static mut DAT_009d1154: i32;
    static mut DAT_009d12e8: i32;
    static mut DAT_009d1264: i32;
    static mut DAT_009a8e28: i32;
    static mut DAT_009a8e20: i32;
    static mut DAT_009a8e1c: i32;
    static mut _DAT_00622c50: f32;
    static mut _DAT_00622c54: f32;
    static mut _DAT_00622c58: f32;
    static mut _DAT_00622c60: f32;
    static mut _DAT_00622c64: f32;
    static mut _DAT_00622c68: f32;
    static mut DAT_00622c6c: i32;
    static mut _DAT_00622c98: f32;
    static mut DAT_00622c70: f32;
    static mut DAT_00622c74: f32;
    static mut DAT_00622c78: f32;
    static mut DAT_00622c80: f32;
    static mut DAT_00622c84: f32;
    static mut DAT_00622c88: f32;
    static mut DAT_00622c8c: i32;
    static mut DAT_00527e88: i32;
    static mut DAT_00622c90: f32;
    static mut DAT_00622c94: i32;
    static mut DAT_009a7ed0: [u16; 0];
    static mut DAT_00a00870: [i32; 0];
    static mut DAT_00a00874: [i32; 0];
    static mut DAT_00a0093c: [i32; 0];
    static mut DAT_009d6940: u32;
    static mut _DAT_00518200: f32;
    static mut _DAT_00518204: f32;
    static mut _DAT_005181fc: f32;
    static mut DAT_009d77fc: i32;
    static mut DAT_009d6828: i32;
    static mut DAT_009cd260: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
    fn FUN_004ac720(a: u32) -> *mut i32;
    fn FUN_00471ff0(a: *mut i32);
    fn FUN_004ae2a0(a: u32, b: i32) -> i32;
    fn FUN_004ae200(a: u32, b: i32) -> i32;
    #[link_name = "_ftol"]
    fn __ftol() -> i64;
    fn FUN_004a6080(a: i32, b: *mut u32);
    fn FUN_004ac740();
}

fn mulshr(a: i32, b: i32) -> u32 {
    let p = (a as i64).wrapping_mul(b as i64);
    ((p as u32) >> 15) | ((((p as u64) >> 32) as u32) << 17)
}

fn sum3(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    mulshr(a, b)
        .wrapping_add(mulshr(c, d))
        .wrapping_add(mulshr(e, f)) as i32
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: address supplied by the caller's game data.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn r16(a: i32) -> i16 {
    // SAFETY: address supplied by the caller's game data.
    unsafe { (a as u32 as usize as *const i16).read_unaligned() }
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: address supplied by the caller's game data.
    unsafe { (a as u32 as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a5490(param_1: u16, param_2: u16) -> i32 {
    // SAFETY: all globals and callees are the game's own; pointers are
    // read at the offsets the original uses.
    unsafe {
        let i_var5 = (param_2 as u32).wrapping_mul(0x23) as i32;
        let i_var13 = DAT_009a1fe8.wrapping_add(i_var5);
        let pb = i_var13;
        let mut tmp_p = r32(pb.wrapping_add(0x1f));
        if tmp_p != 0 {
            DAT_00a08104 = r32(tmp_p.wrapping_add(0xa9));
        }
        let tmp_a = r32(pb.wrapping_add(4));
        let tmp_b = r32(pb.wrapping_add(8));
        let tmp_c = r32(pb.wrapping_add(0xc));
        let mut i_var6 = DAT_009a1fd4.wrapping_sub(tmp_a);
        let i_var7 = DAT_009ec604.wrapping_sub(tmp_a);
        let u_stack_180 = i_var7 as u16;
        let i_var14 = DAT_009a1fd8.wrapping_sub(tmp_b);
        let i_var10 = DAT_00a081e8.wrapping_sub(tmp_b);
        let i_var12 = DAT_009a1fd0.wrapping_sub(tmp_c);
        let i_var11 = DAT_00a08298.wrapping_sub(tmp_c);

        let mut local_15c: i32;
        let mut local_164: i32;
        let local_16c: i32;
        let local_160: i32;
        let mut local_168: i32;

        tmp_p = r32(pb.wrapping_add(0x1f));
        if tmp_p == 0 {
            let s0 = r16(pb.wrapping_add(0x12));
            let s1 = r16(pb.wrapping_add(0x10));
            let s2 = r16(pb.wrapping_add(0x14));
            FUN_00427bd0(s0, s1, 0);
            FUN_00427d30(s2, 0, 0);
            local_15c = sum3(
                DAT_009a8d8c,
                i_var12,
                DAT_009a8d80,
                i_var6,
                DAT_009a8d60,
                i_var14,
            );
            local_164 = sum3(
                DAT_009d1154,
                i_var12,
                DAT_009d12e8,
                i_var6,
                DAT_009d1264,
                i_var14,
            );
            local_16c = sum3(
                DAT_009a8e28,
                i_var12,
                DAT_009a8e20,
                i_var6,
                DAT_009a8e1c,
                i_var14,
            );
            local_160 = sum3(
                DAT_009a8d8c,
                i_var11,
                DAT_009a8d80,
                i_var7,
                DAT_009a8d60,
                i_var10,
            );
            local_168 = sum3(
                DAT_009d1154,
                i_var11,
                DAT_009d12e8,
                i_var7,
                DAT_009d1264,
                i_var10,
            );
            i_var6 = sum3(
                DAT_009a8e28,
                i_var11,
                DAT_009a8e20,
                i_var7,
                DAT_009a8e1c,
                i_var10,
            );
        } else {
            if r8(tmp_p.wrapping_add(0x92)) != 0 {
                let s0 = r16(pb.wrapping_add(0x12));
                let s1 = r16(pb.wrapping_add(0x10));
                let s2 = r16(pb.wrapping_add(0x14));
                FUN_00427bd0(s0, s1, i_var13);
                FUN_00427d30(s2, 0, i_var13);
            }
            let s0 = r16(tmp_p.wrapping_add(0x99)) as i32;
            let s1 = r16(tmp_p.wrapping_add(0x9b)) as i32;
            let s2 = r16(tmp_p.wrapping_add(0x9d)) as i32;
            let s3 = r16(tmp_p.wrapping_add(0x93)) as i32;
            let s4 = r16(tmp_p.wrapping_add(0x95)) as i32;
            let s5 = r16(tmp_p.wrapping_add(0x97)) as i32;
            let s6 = r16(tmp_p.wrapping_add(0x9f)) as i32;
            let s7 = r16(tmp_p.wrapping_add(0xa1)) as i32;
            let s8 = r16(tmp_p.wrapping_add(0xa3)) as i32;
            local_15c = sum3(s2, i_var12, s0, i_var6, s1, i_var14);
            local_164 = sum3(s5, i_var12, s3, i_var6, s4, i_var14);
            local_16c = sum3(s8, i_var12, s6, i_var6, s7, i_var14);
            local_160 = sum3(s2, i_var11, s0, i_var7, s1, i_var10);
            local_168 = sum3(s5, i_var11, s3, i_var7, s4, i_var10);
            i_var6 = sum3(s8, i_var11, s6, i_var7, s7, i_var10);
        }
        local_164 = 0u32.wrapping_sub(local_164 as u32) as i32;
        local_168 = 0u32.wrapping_sub(local_168 as u32) as i32;
        _DAT_00622c50 = local_160 as f32;
        _DAT_00622c54 = local_168 as f32;
        _DAT_00622c58 = i_var6 as f32;
        _DAT_00622c60 = local_15c as f32;
        _DAT_00622c64 = local_164 as f32;
        _DAT_00622c68 = local_16c as f32;
        DAT_00622c6c = 0;
        _DAT_00622c98 = 2.0f32;
        DAT_00622c70 = _DAT_00622c50;
        DAT_00622c74 = _DAT_00622c54;
        DAT_00622c78 = _DAT_00622c58;
        DAT_00622c80 = _DAT_00622c60;
        DAT_00622c84 = _DAT_00622c64;
        DAT_00622c88 = _DAT_00622c68;

        let idx = ((u_stack_180 as u32) << 16)
            | (*(&raw mut DAT_009a7ed0)
                .cast::<u16>()
                .add(r8(pb.wrapping_add(3)) as usize)) as u32;
        let pi_var8 = FUN_004ac720(idx);
        if pi_var8.is_null() {
            return 0;
        }
        if pi_var8.read() as u32 as usize != pi_var8 as usize {
            FUN_00471ff0(pi_var8);
        }
        DAT_00622c8c = 0;
        DAT_00527e88 = 0;
        let mut local_158: u32 = 0;
        let mut local_154: i32 = 0;
        let mut tmp_cnt: u32 = pi_var8.byte_add(6).cast::<u32>().read_unaligned();
        if tmp_cnt != 0 {
            loop {
                'body: {
                    let mut i_var7 = DAT_00622c8c;
                    DAT_00622c90 = 0.0f32;
                    let ta = pi_var8.byte_add(10).cast::<i32>().read_unaligned();
                    let pu_var2 = r32(ta.wrapping_add(local_154)) as u32 as usize as *mut u32;
                    if pu_var2.add(1).read() != 0x14 {
                        let mut i_var10 = DAT_00622c8c.wrapping_add(1);
                        if param_1 == param_2 {
                            let b_var1 = r8(DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var5));
                            let u_var9 = b_var1 as u32;
                            if b_var1 < 0x49 {
                                if DAT_00622c8c < 0 {
                                    i_var7 = 0;
                                    DAT_00622c8c = i_var10;
                                } else {
                                    let lim = (&raw mut DAT_00a00870)
                                        .cast::<i32>()
                                        .add(u_var9.wrapping_mul(0x65) as usize)
                                        .read();
                                    if lim <= DAT_00622c8c {
                                        DAT_00622c8c = (lim as u32).wrapping_sub(1) as i32;
                                    }
                                    i_var7 = (&raw mut DAT_00a00874)
                                        .cast::<i32>()
                                        .add(
                                            u_var9
                                                .wrapping_mul(0x65)
                                                .wrapping_add(DAT_00622c8c as u32)
                                                as usize,
                                        )
                                        .read();
                                    DAT_00622c8c = i_var10;
                                }
                            } else {
                                DAT_00622c8c = i_var10;
                                i_var7 = FUN_004ae2a0(u_var9, i_var7);
                            }
                            if i_var7 == 4 || i_var7 == 5 || i_var7 == 0x15 {
                                break 'body;
                            }
                            i_var10 = DAT_00622c8c;
                            if r8(DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var5)) as i8 == 0x36
                                && DAT_00622c8c.wrapping_sub(DAT_00622c94) == 1
                            {
                                break 'body;
                            }
                        }
                        DAT_00622c8c = i_var10;
                        let tp = r32(pb.wrapping_add(0x1f));
                        if tp != 0 {
                            i_var7 = r32(tp.wrapping_add(0xa9));
                            if i_var7 != 0 {
                                i_var7 = DAT_00622c8c.wrapping_add(i_var7);
                                if r8(i_var7.wrapping_add(0x33b)) == 0 {
                                    break 'body;
                                }
                                let b_var1 = r8(i_var7.wrapping_add(0x309));
                                if b_var1 != 0 && ((DAT_009d6940 >> 8) & 0xff) == 0 {
                                    let tc = r32((param_1 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(0x1b)
                                        .wrapping_add(DAT_009a1fe8 as u32)
                                        as i32);
                                    if tc != -1 {
                                        break 'body;
                                    }
                                }
                                DAT_00622c90 = (b_var1 as f32) * _DAT_00518200;
                            }
                        }
                        i_var7 = DAT_00622c8c.wrapping_sub(1);
                        let u_var9 = r8(pb.wrapping_add(3)) as u32;
                        if (u_var9 as u8) < 0x49 {
                            if i_var7 < 0 {
                                i_var7 = 0;
                            } else {
                                let lim = (&raw mut DAT_00a00870)
                                    .cast::<i32>()
                                    .add(u_var9.wrapping_mul(0x65) as usize)
                                    .read();
                                if lim <= i_var7 {
                                    i_var7 = (lim as u32).wrapping_sub(1) as i32;
                                }
                                i_var7 = (&raw mut DAT_00a0093c)
                                    .cast::<i32>()
                                    .add((i_var7 as u32).wrapping_add(u_var9.wrapping_mul(0x65))
                                        as usize)
                                    .read();
                            }
                        } else {
                            i_var7 = FUN_004ae200(u_var9, i_var7);
                        }
                        if i_var7 != 0 {
                            let mut l = __ftol() as i32;
                            if l <= local_15c || l <= local_160 {
                                l = __ftol() as i32;
                                if l <= local_164 || l <= local_168 {
                                    l = __ftol() as i32;
                                    if l <= local_16c || l <= i_var6 {
                                        l = __ftol() as i32;
                                        if local_15c <= l || local_160 <= l {
                                            l = __ftol() as i32;
                                            if local_164 <= l || local_168 <= l {
                                                l = __ftol() as i32;
                                                if local_16c <= l || i_var6 <= l {
                                                    FUN_004a6080(pi_var8 as usize as i32, pu_var2);
                                                    _DAT_00622c50 = DAT_00622c70;
                                                    _DAT_00622c54 = DAT_00622c74;
                                                    _DAT_00622c58 = DAT_00622c78;
                                                    _DAT_00622c60 = DAT_00622c80;
                                                    _DAT_00622c64 = DAT_00622c84;
                                                    _DAT_00622c68 = DAT_00622c88;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                local_154 = local_154.wrapping_add(4);
                local_158 = local_158.wrapping_add(1);
                tmp_cnt = pi_var8.byte_add(6).cast::<u32>().read_unaligned();
                if !(local_158 < tmp_cnt) {
                    break;
                }
            }
        }
        FUN_004ac740();
        if DAT_00622c6c != 0 {
            _DAT_00622c98 -= _DAT_00518204;
            if _DAT_00622c98 < _DAT_005181fc {
                _DAT_00622c98 = 0.0f32;
            }
            DAT_009d77fc = __ftol() as i32;
            DAT_009d6828 = __ftol() as i32;
            DAT_009cd260 = __ftol() as i32;
        }
        DAT_00622c6c
    }
}
