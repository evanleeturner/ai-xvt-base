// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525540: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00462010(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: the global is the game's own; the pointers are caller-supplied
    // addresses read and written exactly where the original does.
    unsafe {
        let off = (param_3 as u32).wrapping_mul(3);
        let count = param_4 as u32;
        if DAT_00525540 == 0x100 {
            let mut dst = (off.wrapping_add(param_2 as u32)) as *mut u8;
            let mut src = ((param_1 as u32).wrapping_add(off)) as *const u8;
            for _ in 0..count {
                *dst = *src;
                *dst.add(1) = *src.add(1);
                *dst.add(2) = *src.add(2);
                dst = dst.add(3);
                src = src.add(3);
            }
            return;
        }
        let mut src = ((param_1 as u32).wrapping_add(off)) as *const u8;
        let mut dst = ((param_2 as u32).wrapping_add(off)) as *mut u8;
        for _ in 0..count {
            let b1 = *src;
            let b2 = *src.add(1);
            let b3 = *src.add(2);

            let mut b11: u8 = 0;
            let c1 = if b1 < b2 {
                true
            } else {
                b11 = b1;
                b1 < b3
            };
            if c1 {
                let c2 = if b2 < b1 {
                    true
                } else {
                    b11 = b2;
                    b2 < b3
                };
                if c2 {
                    b11 = b3;
                }
            }

            let mut b4: u8 = 0;
            let d1 = if b2 < b1 {
                true
            } else {
                b4 = b1;
                b3 < b1
            };
            if d1 {
                let d2 = if b1 < b2 {
                    true
                } else {
                    b4 = b2;
                    b3 < b2
                };
                if d2 {
                    b4 = b3;
                }
            }

            let local_4: u8 = if b11 == 0 {
                0
            } else {
                ((((b11 as u32).wrapping_sub(b4 as u32)).wrapping_mul(0x3f)) as i32 / (b11 as i32))
                    as u8
            };
            let mut local_d: u8 = 0;
            let mut local_e: u8 = 0;
            if local_4 != 0 {
                let diff = (b11 as u32).wrapping_sub(b4 as u32) as i32;
                if b11 == b1 {
                    let q =
                        (((b2 as u32).wrapping_sub(b3 as u32)).wrapping_mul(0x3f) as i32) / diff;
                    if b2 < b3 {
                        local_d = 5;
                        local_e = (q as u8).wrapping_add(0x3f);
                    } else {
                        local_d = 0;
                        local_e = q as u8;
                    }
                } else if b11 == b2 {
                    let q =
                        (((b3 as u32).wrapping_sub(b1 as u32)).wrapping_mul(0x3f) as i32) / diff;
                    if b3 < b1 {
                        local_d = 1;
                        local_e = (q as u8).wrapping_add(0x3f);
                    } else {
                        local_d = 2;
                        local_e = q as u8;
                    }
                } else {
                    let q =
                        (((b1 as u32).wrapping_sub(b2 as u32)).wrapping_mul(0x3f) as i32) / diff;
                    if b1 < b2 {
                        local_d = 3;
                        local_e = (q as u8).wrapping_add(0x3f);
                    } else {
                        local_d = 4;
                        local_e = q as u8;
                    }
                }
            }
            let mut v = (((b11 as u32).wrapping_mul(DAT_00525540)) >> 8) as u8;
            if 0x3f < v {
                v = 0x3f;
            }
            let mut local_b = v;
            let mut local_a = v;
            let mut local_9 = v;
            if local_4 != 0 {
                let u8_ = local_4 as u32;
                let u9 = v as u32;
                let c4 = ((0x3fu32.wrapping_sub(u8_)).wrapping_mul(u9) as i32 / 0x3f) as u8;
                let t5 = (u8_.wrapping_mul(local_e as u32) as i32 / -0x3f).wrapping_add(0x3f);
                let c5 = ((t5 as u32).wrapping_mul(u9) as i32 / 0x3f) as u8;
                let t6 = ((((local_e as u32).wrapping_sub(0x3f)).wrapping_mul(u8_)) as i32 / 0x3f)
                    .wrapping_add(0x3f);
                let c6 = ((t6 as u32).wrapping_mul(u9) as i32 / 0x3f) as u8;
                local_b = b3;
                local_a = b1;
                local_9 = b2;
                match local_d {
                    0 => {
                        local_b = c4;
                        local_a = v;
                        local_9 = c6;
                    }
                    1 => {
                        local_b = c4;
                        local_a = c5;
                        local_9 = v;
                    }
                    2 => {
                        local_b = c6;
                        local_a = c4;
                        local_9 = v;
                    }
                    3 => {
                        local_b = v;
                        local_a = c4;
                        local_9 = c5;
                    }
                    4 => {
                        local_b = v;
                        local_a = c6;
                        local_9 = c4;
                    }
                    5 => {
                        local_b = c5;
                        local_a = v;
                        local_9 = c4;
                    }
                    _ => {}
                }
            }
            *dst = local_a;
            *dst.add(1) = local_9;
            src = src.add(3);
            *dst.add(2) = local_b;
            dst = dst.add(3);
        }
    }
}
