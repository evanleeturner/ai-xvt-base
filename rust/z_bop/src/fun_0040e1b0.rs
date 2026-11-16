// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523400: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e1b0(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: the pointers are caller-supplied addresses of pixel buffers,
    // read and written exactly as the original does; the global is the game's own.
    unsafe {
        let off = (param_3 as u32).wrapping_mul(3);
        if DAT_00523400 == 0x100 {
            let mut dst = param_2.wrapping_add(off as i32) as u32 as usize as *mut u8;
            let mut src = param_1.wrapping_add(off as i32) as u32 as usize as *const u8;
            let mut n = param_4 as u32;
            while n != 0 {
                *dst = *src;
                *dst.add(1) = *src.add(1);
                *dst.add(2) = *src.add(2);
                dst = dst.wrapping_add(3);
                src = src.wrapping_add(3);
                n = n.wrapping_sub(1);
            }
            return;
        }
        let mut pbvar14 = param_2.wrapping_add(off as i32) as u32 as usize as *mut u8;
        let mut pbvar13 = param_1.wrapping_add(off as i32) as u32 as usize as *const u8;
        let mut n = param_4 as u32;
        while n != 0 {
            let b1 = *pbvar13;
            let b2 = *pbvar13.add(1);
            let b3 = *pbvar13.add(2);

            let mut b10: u8;
            if (b1 < b2 || b1 < b3) && (b2 < b1 || b2 < b3) {
                b10 = b3;
            } else if !(b1 < b2 || b1 < b3) {
                b10 = b1;
            } else {
                b10 = b2;
            }
            let b12: u8;
            if (b2 < b1 || b3 < b1) && (b1 < b2 || b3 < b2) {
                b12 = b3;
            } else if !(b2 < b1 || b3 < b1) {
                b12 = b1;
            } else {
                b12 = b2;
            }

            let local_4: u8;
            if b10 == 0 {
                local_4 = 0;
            } else {
                let num = ((b10 as u32).wrapping_sub(b12 as u32)).wrapping_mul(0x3f) as i32;
                local_4 = (num / (b10 as i32)) as u8;
            }

            let mut local_10: u8 = 0;
            let mut local_11: u8 = 0;
            if local_4 != 0 {
                let d = (b10 as u32).wrapping_sub(b12 as u32) as i32;
                if b1 == b10 {
                    let num = ((b2 as u32).wrapping_sub(b3 as u32)).wrapping_mul(0x3f) as i32;
                    if b2 < b3 {
                        local_10 = 5;
                        local_11 = ((num / d) as u8).wrapping_add(0x3f);
                    } else {
                        local_10 = 0;
                        local_11 = (num / d) as u8;
                    }
                } else if b2 == b10 {
                    let num = ((b3 as u32).wrapping_sub(b1 as u32)).wrapping_mul(0x3f) as i32;
                    if b3 < b1 {
                        local_10 = 1;
                        local_11 = ((num / d) as u8).wrapping_add(0x3f);
                    } else {
                        local_10 = 2;
                        local_11 = (num / d) as u8;
                    }
                } else {
                    let num = ((b1 as u32).wrapping_sub(b2 as u32)).wrapping_mul(0x3f) as i32;
                    if b1 < b2 {
                        local_10 = 3;
                        local_11 = ((num / d) as u8).wrapping_add(0x3f);
                    } else {
                        local_10 = 4;
                        local_11 = (num / d) as u8;
                    }
                }
            }

            b10 = ((b10 as u32).wrapping_mul(DAT_00523400) >> 8) as u8;
            if 0x3f < b10 {
                b10 = 0x3f;
            }
            let mut out0 = b10;
            let mut local_e = b10;
            let mut local_d = b10;
            if local_4 != 0 {
                let u8_ = local_4 as u32;
                let u9 = b10 as u32;
                let b4 = ((0x3fu32.wrapping_sub(u8_).wrapping_mul(u9)) as i32 / 0x3f) as u8;
                let t5 = ((local_11 as u32).wrapping_mul(u8_) as i32 / -0x3f) as u32;
                let b5 = ((t5.wrapping_add(0x3f).wrapping_mul(u9)) as i32 / 0x3f) as u8;
                let t6 =
                    ((local_11 as u32).wrapping_sub(0x3f).wrapping_mul(u8_) as i32 / 0x3f) as u32;
                let b6 = ((t6.wrapping_add(0x3f).wrapping_mul(u9)) as i32 / 0x3f) as u8;
                out0 = b1;
                local_e = b3;
                local_d = b2;
                match local_10 {
                    0 => {
                        out0 = b10;
                        local_e = b4;
                        local_d = b6;
                    }
                    1 => {
                        out0 = b5;
                        local_e = b4;
                        local_d = b10;
                    }
                    2 => {
                        out0 = b4;
                        local_e = b6;
                        local_d = b10;
                    }
                    3 => {
                        out0 = b4;
                        local_e = b10;
                        local_d = b5;
                    }
                    4 => {
                        out0 = b6;
                        local_e = b10;
                        local_d = b4;
                    }
                    5 => {
                        out0 = b10;
                        local_e = b5;
                        local_d = b4;
                    }
                    _ => {}
                }
            }
            *pbvar14 = out0;
            *pbvar14.add(1) = local_d;
            *pbvar14.add(2) = local_e;
            pbvar14 = pbvar14.wrapping_add(3);
            pbvar13 = pbvar13.wrapping_add(3);
            n = n.wrapping_sub(1);
        }
    }
}
