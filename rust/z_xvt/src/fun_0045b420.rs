// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525730: i32;
    static mut DAT_005254f8: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0072f46c: i32;
    static mut DAT_0072f4ec: i32;
    static mut DAT_006db700: i32;
    static mut DAT_006db6e0: i32;
    static mut DAT_006db70c: i32;
    static mut DAT_00703be0: i32;
    static mut DAT_00703adc: i32;
    static mut DAT_00703c68: i32;
    static mut DAT_006db7a8: i32;
    static mut DAT_006db79c: i32;
    static mut DAT_006db7a0: i32;
    static mut DAT_00525540: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00412810(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00413d00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004832a0(a: i16, b: i16, c: i16, d: i32, e: i32);
}

#[inline(always)]
fn ptr(addr: i32) -> *const u8 {
    addr as u32 as usize as *const u8
}

#[inline(always)]
unsafe fn rd_i32(addr: i32) -> i32 {
    // SAFETY: caller guarantees addr points at readable game memory.
    unsafe { ptr(addr).cast::<i32>().read_unaligned() }
}

#[inline(always)]
unsafe fn rd_i16(addr: i32) -> i16 {
    // SAFETY: caller guarantees addr points at readable game memory.
    unsafe { ptr(addr).cast::<i16>().read_unaligned() }
}

#[inline(always)]
unsafe fn rd_u8(addr: i32) -> u8 {
    // SAFETY: caller guarantees addr points at readable game memory.
    unsafe { ptr(addr).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045b420(param_1: i32) {
    // SAFETY: all globals, callees and pointers are the game's own memory;
    // reads and writes happen at the same places and order as the original.
    unsafe {
        DAT_00525730 = 0;
        if DAT_005254f8 != 0 {
            let mut local_10: i32 = 0;
            let mut local_14: i32 = 0;
            let iv2 = rd_i32(param_1.wrapping_add(4));
            let iv3 = rd_i32(param_1.wrapping_add(8));
            let iv4 = rd_i32(param_1.wrapping_add(0xc));
            if 0 < DAT_00732a38 {
                let mut iv12 = DAT_006d4988;
                let mut p13: *mut i32 = &raw mut DAT_0072f46c;
                loop {
                    let mut p14 = p13;
                    if rd_u8(iv12.wrapping_add(3)) != 0 && rd_u8(iv12.wrapping_add(2)) == 0x0d {
                        let t0 = rd_i32(iv12.wrapping_add(4));
                        let t1 = rd_i32(iv12.wrapping_add(8));
                        let t2 = rd_i32(iv12.wrapping_add(0xc));
                        let iv10 = t0.wrapping_sub(iv2);
                        let mut iv5 = t1.wrapping_sub(iv3);
                        let iv11 = t2.wrapping_sub(iv4);
                        let u6 = FUN_00412810(iv10, iv5, iv11);
                        if u6 < 0x4000 {
                            let mut iv7 = rd_i32(param_1.wrapping_add(0x1f));
                            let iv9: i32;
                            let iv8: i32;
                            if iv7 == 0 {
                                FUN_004832a0(
                                    rd_i16(param_1.wrapping_add(0x14)),
                                    rd_i16(param_1.wrapping_add(0x12)),
                                    rd_i16(param_1.wrapping_add(0x10)),
                                    0,
                                    0,
                                );
                                let r = FUN_00413d00(
                                    iv10,
                                    iv5,
                                    iv11,
                                    DAT_006db700,
                                    DAT_006db6e0,
                                    DAT_006db70c,
                                );
                                let a8 = DAT_00703be0;
                                let a7 = DAT_00703adc;
                                p13.offset(-3).write(r);
                                let r2 = FUN_00413d00(iv10, iv5, iv11, DAT_00703c68, a8, a7);
                                iv8 = DAT_006db7a8;
                                iv7 = DAT_006db79c;
                                p13.offset(-2).write(0i32.wrapping_sub(r2));
                                iv9 = DAT_006db7a0;
                            } else {
                                let s0 = rd_i16(iv7.wrapping_add(0x97)) as i32;
                                let s1 = rd_i16(iv7.wrapping_add(0x99)) as i32;
                                let s2 = rd_i16(iv7.wrapping_add(0x9b)) as i32;
                                let r = FUN_00413d00(iv10, iv5, iv11, s0, s1, s2);
                                p13.offset(-3).write(r);
                                iv7 = rd_i32(param_1.wrapping_add(0x1f));
                                let s0 = rd_i16(iv7.wrapping_add(0x91)) as i32;
                                let s1 = rd_i16(iv7.wrapping_add(0x93)) as i32;
                                let s2 = rd_i16(iv7.wrapping_add(0x95)) as i32;
                                let r = FUN_00413d00(iv10, iv5, iv11, s0, s1, s2);
                                p13.offset(-2).write(0i32.wrapping_sub(r));
                                let b = rd_i32(param_1.wrapping_add(0x1f));
                                let s0 = rd_i16(b.wrapping_add(0xa1));
                                let s1 = rd_i16(b.wrapping_add(0x9f));
                                let s2 = rd_i16(b.wrapping_add(0x9d));
                                iv8 = s0 as i32;
                                iv7 = s1 as i32;
                                iv9 = s2 as i32;
                            }
                            iv5 = FUN_00413d00(iv10, iv5, iv11, iv9, iv7, iv8);
                            p13.offset(-1).write(iv5);
                            p13.write(0x10);
                            match rd_u8(iv12.wrapping_add(3)) {
                                0x7f..=0x82 => {
                                    match rd_u8(iv12.wrapping_add(0x19)) {
                                        2 => p13.write(0xc0),
                                        3 => p13.write(0x140),
                                        4 => p13.write(0x1e0),
                                        5..=8 => p13.write(0x140),
                                        9 => p13.write(0xc0),
                                        10 => p13.write(0x60),
                                        0xb => p13.write(0x30),
                                        _ => {}
                                    }
                                    let v7 = rd_i32(iv12.wrapping_add(0x1f));
                                    if v7 != 0 {
                                        let b1 = rd_u8(v7.wrapping_add(1));
                                        if 3 < b1 {
                                            let m = ((b1 as i32 + 4) >> 2) as u32;
                                            let cur = p13.read() as u32;
                                            p13.write(m.wrapping_mul(cur) as i32);
                                        }
                                    }
                                }
                                0x83 | 0x84 => match rd_u8(iv12.wrapping_add(0x19)) {
                                    2 => p13.write(0x30),
                                    3 => p13.write(0x60),
                                    4 => p13.write(0x40),
                                    5 => p13.write(0x20),
                                    _ => {}
                                },
                                _ => {
                                    p13.write(DAT_00525540.wrapping_sub(0x100));
                                }
                            }
                            p14 = p13.add(4);
                            local_10 = local_10.wrapping_add(1);
                            p13.write((p13.read() as u32).wrapping_shl(3) as i32);
                            if p14 == (&raw mut DAT_0072f4ec) {
                                break;
                            }
                        }
                    }
                    iv12 = iv12.wrapping_add(0x23);
                    local_14 = local_14.wrapping_add(1);
                    p13 = p14;
                    if !(local_14 < DAT_00732a38) {
                        break;
                    }
                }
            }
            DAT_00525730 = local_10;
        }
    }
}
