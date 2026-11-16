// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527e90: i32;
    static mut DAT_00a0813f: i8;
    static mut DAT_00999454: i32;
    static mut DAT_00999450: i32;
    static mut DAT_0099944a: [u8; 4];
    static mut DAT_005233ac: i32;
    static mut DAT_005233d8: f32;
    static mut DAT_00527378: [i32; 0];
    static mut DAT_005270b8: [f32; 0];
    static mut DAT_00612280: i32;
    static mut DAT_00612284: u32;
    static mut DAT_0061228c: u32;
    static mut DAT_00612294: f32;
    static mut _DAT_0061229c: f32;
    static mut _DAT_006122b0: f32;
    static mut _DAT_006122b4: f32;
    static mut DAT_006122c8: u32;
    static mut DAT_006122d4: u32;
    static mut DAT_00612adc: u32;
    static mut _DAT_00612b58: i32;
    static mut _DAT_00612b5c: f32;
    static mut _DAT_00612b60: f32;
    static mut DAT_00612b64: i32;
    static mut DAT_00612b68: i32;
    static mut DAT_00612b6c: i32;
    static mut DAT_00612b70: i32;
    static mut DAT_00612b74: i32;
    static mut DAT_00612b7c: u8;
    static mut DAT_009a7b60: i32;
    static mut DAT_009a8e3c: i32;
    static mut DAT_009d8c08: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004206a0();
    fn FUN_0040b2c0();
    fn FUN_004aba50();
    fn __ftol() -> i64;
    fn FUN_004879d0(a: i32, b: i32, c: f32);
    fn FUN_004abe50();
}

#[inline(always)]
unsafe fn rd_i32(addr: i32) -> i32 {
    // SAFETY: the address is a game-owned structure field.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_u32(addr: i32) -> u32 {
    // SAFETY: the address is a game-owned structure field.
    unsafe { (addr as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_f32(addr: i32) -> f32 {
    // SAFETY: the address is a game-owned structure field.
    unsafe { (addr as usize as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr_i32(addr: i32, v: i32) {
    // SAFETY: the address is a game-owned structure field.
    unsafe { (addr as usize as *mut i32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004865e0() {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed in the order the original does.
    unsafe {
        let mut local_1c = [0u8; 8];
        FUN_004206a0();
        if DAT_00527e90 != 0 {
            FUN_0040b2c0();
            return;
        }
        if DAT_00a0813f == 0 {
            FUN_004aba50();
        }
        let mut local_20 = DAT_00999454;
        _DAT_00612b58 = 0;
        if DAT_00999454 < DAT_00999450 {
            let mut local_24 = (DAT_00999454 as u32).wrapping_mul(0x70) as i32;
            loop {
                let ibase = (&raw const DAT_0099944a).cast::<i32>().read_unaligned();
                DAT_00612280 = ibase.wrapping_add(local_24);
                DAT_006122c8 = rd_u32(DAT_00612280.wrapping_add(0x40));
                wr_i32(
                    DAT_00612280.wrapping_add(0x38),
                    local_1c.as_mut_ptr() as usize as i32,
                );
                let mut i_var8: i32 = 0;
                let pi7 = DAT_00612280.wrapping_add(4);
                let mut i_var2 = rd_i32(rd_i32(pi7).wrapping_add(0xa8));
                let mut u_var6 = rd_u32(i_var2.wrapping_add(0x14));
                let mut u_var4 = rd_u32(i_var2.wrapping_add(0x10));
                if DAT_005233ac != 0
                    && u_var6
                        .wrapping_mul(u_var4)
                        .wrapping_sub(rd_u32(i_var2.wrapping_add(8)))
                        == 0
                {
                    i_var2 = __ftol() as i32;
                    while i_var2 > 0x100 && u_var4 != 8 && u_var6 != 8 {
                        i_var2 >>= 2;
                        let i_var3 = u_var6.wrapping_mul(u_var4) as i32;
                        u_var4 = ((u_var4 as i32) >> 1) as u32;
                        i_var8 = i_var8.wrapping_add(i_var3);
                        u_var6 = ((u_var6 as i32) >> 1) as u32;
                    }
                }
                _DAT_00612b5c = (u_var4 as i32) as f32;
                _DAT_00612b60 = (u_var6 as i32) as f32;
                let tbl = (&raw mut DAT_00527378).cast::<i32>();
                DAT_00612b64 = tbl
                    .offset((((u_var4 & 0xffff_fff3) as i32) >> 2) as isize)
                    .read();
                DAT_00612b74 = u_var6.wrapping_mul(u_var4).wrapping_sub(1) as i32;
                DAT_00612b68 = tbl
                    .offset((((u_var6 & 0xffff_fff3) as i32) >> 2) as isize)
                    .read();
                DAT_00612b6c = rd_i32(rd_i32(pi7).wrapping_add(0xb0));
                DAT_00612b70 = rd_i32(rd_i32(pi7).wrapping_add(0xac)).wrapping_add(i_var8);
                _DAT_00612b58 = rd_i32(pi7);
                let mut local_28: f32 = (DAT_006122c8 as f32)
                    * rd_f32(DAT_00612280.wrapping_add(0x2c))
                    + rd_f32(DAT_00612280.wrapping_add(0x30));
                DAT_00612294 = ((DAT_009a7b60 as u32).wrapping_add(DAT_006122c8) as i32 as f32)
                    * DAT_009d8c08
                    + (DAT_009a8e3c as f32) * DAT_005233d8;
                if DAT_006122c8 < rd_u32(DAT_00612280.wrapping_add(0x44)) {
                    let mut i_off: i32 = 0;
                    loop {
                        let p7 =
                            rd_i32(rd_i32(DAT_00612280.wrapping_add(0x68)).wrapping_add(i_off));
                        'skip: {
                            if p7 != 0 {
                                let mut u6 = DAT_006122c8 & DAT_00612284;
                                if u6 == 0 {
                                    _DAT_0061229c = 0.0;
                                    _DAT_006122b4 = 0.0;
                                    u6 = DAT_0061228c;
                                } else {
                                    _DAT_006122b4 = u6 as f32;
                                    _DAT_0061229c = _DAT_006122b4
                                        * (&raw const DAT_005270b8)
                                            .cast::<f32>()
                                            .add(DAT_0061228c as usize)
                                            .read();
                                    u6 = DAT_0061228c.wrapping_sub(u6);
                                }
                                _DAT_006122b0 = u6 as f32;
                                DAT_006122d4 = (DAT_006122c8 >> (DAT_00612b7c & 0x1f) as u32)
                                    .wrapping_add(DAT_00612adc);
                                let _local_10 = rd_i32(p7.wrapping_add(12));
                                let mut p5 = rd_i32(p7);
                                let mut u_6 = rd_u32(p7.wrapping_add(4));
                                let mut i8v = rd_i32(p7.wrapping_add(8));
                                let local_14 = (u_6 as i32) as f32;
                                wr_i32(
                                    DAT_00612280.wrapping_add(0x34),
                                    rd_i32(p7.wrapping_add(16)),
                                );
                                let f_var1: f32;
                                if p5 == 0 {
                                    f_var1 = rd_f32(DAT_00612280.wrapping_add(0x28)) * local_14;
                                } else {
                                    loop {
                                        let i_var3 = rd_i32(p5.wrapping_add(4));
                                        if i8v <= i_var3 {
                                            break;
                                        }
                                        if (u_6 as i32) < i_var3 {
                                            FUN_004879d0(
                                                u_6 as i32,
                                                i_var3,
                                                ((u_6 as i32) as f32)
                                                    * rd_f32(DAT_00612280.wrapping_add(0x28))
                                                    + local_28,
                                            );
                                            u_6 = rd_u32(p5.wrapping_add(8));
                                            i8v = rd_i32(p7.wrapping_add(8));
                                            if i8v <= u_6 as i32 {
                                                break;
                                            }
                                        } else {
                                            let u4 = rd_u32(p5.wrapping_add(8));
                                            if (u_6 as i32) < (u4 as i32) {
                                                u_6 = u4;
                                                if i8v <= u4 as i32 {
                                                    break 'skip;
                                                }
                                            }
                                        }
                                        p5 = rd_i32(p5);
                                        if p5 == 0 {
                                            break;
                                        }
                                    }
                                    if i8v <= u_6 as i32 {
                                        break 'skip;
                                    }
                                    f_var1 = ((u_6 as i32) as f32)
                                        * rd_f32(DAT_00612280.wrapping_add(0x28));
                                }
                                FUN_004879d0(u_6 as i32, i8v, f_var1 + local_28);
                            }
                        }
                        local_28 = rd_f32(DAT_00612280.wrapping_add(0x2c)) + local_28;
                        DAT_00612294 = DAT_00612294 + DAT_009d8c08;
                        DAT_006122c8 = DAT_006122c8.wrapping_add(1);
                        i_off = i_off.wrapping_add(4);
                        if !(DAT_006122c8 < rd_u32(DAT_00612280.wrapping_add(0x44))) {
                            break;
                        }
                    }
                }
                local_24 = (local_24 as u32).wrapping_add(0x70) as i32;
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < DAT_00999450) {
                    break;
                }
            }
        }
        if DAT_00a0813f == 0 {
            FUN_004abe50();
        }
    }
}
