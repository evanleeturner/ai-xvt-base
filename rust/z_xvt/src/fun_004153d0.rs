// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525540: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7360() -> i32;
    fn FUN_00413e80(param_1: i32, param_2: i32) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004153d0(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let mut param_4 = param_4;
    let mut rgb: [u8; 3] = [0; 3];
    // SAFETY: the globals and callees are the game's own; the pointers are
    // the caller's buffers, read and written where the original does.
    unsafe {
        if DAT_00525540 == 0x100 {
            if param_3 < (param_4 as u32).wrapping_add(param_3 as u32) as i32 {
                let mut puvar9 =
                    (param_2 as u32).wrapping_add((param_3 as u32).wrapping_mul(2)) as *mut u16;
                let mut pbvar8 =
                    (param_1 as u32).wrapping_add((param_3 as u32).wrapping_mul(3)) as *const u8;
                loop {
                    let i_var1 = FUN_004a7360();
                    if i_var1 == 0 {
                        rgb[0] = pbvar8.read();
                        rgb[1] = pbvar8.add(1).read();
                        rgb[2] = pbvar8.add(2).read();
                        let u = FUN_00413e80(0, rgb.as_mut_ptr() as usize as i32);
                        puvar9.write_unaligned(u);
                    } else {
                        let p0 = pbvar8.read() as u32;
                        let p1 = pbvar8.add(1).read() as u32;
                        let p2 = pbvar8.add(2).read() as u32;
                        let v = ((p1 & 0xfe) << 4) | (p2 >> 1) | ((p0 & 0x3e) << 9);
                        puvar9.write_unaligned(v as u16);
                    }
                    puvar9 = puvar9.add(1);
                    pbvar8 = pbvar8.add(3);
                    param_4 = param_4.wrapping_sub(1);
                    if param_4 == 0 {
                        break;
                    }
                }
                return;
            }
        } else if param_4 != 0 {
            let mut puvar9 =
                (param_2 as u32).wrapping_add((param_3 as u32).wrapping_mul(2)) as *mut u16;
            let mut pbvar8 =
                (param_1 as u32).wrapping_add((param_3 as u32).wrapping_mul(3)) as *const u8;
            loop {
                param_4 = param_4.wrapping_sub(1);
                let mut bvar7: u8 = pbvar8.read();
                let mut uvar5: u16 = bvar7 as u16;
                let mut b11: u8 = pbvar8.add(1).read();
                let mut b12: u8 = pbvar8.add(2).read();
                let r = bvar7;
                let mx = r.max(b11).max(b12);
                let mn = r.min(b11).min(b12);
                let mut stack_4: u32 = if mx == 0 {
                    0
                } else {
                    ((((mx as u32).wrapping_sub(mn as u32)).wrapping_mul(0x3f) as i32)
                        / (mx as u32 as i32)) as u8 as u32
                };
                let mut b15: u8 = 0;
                let mut u14: u8 = 0;
                let diff = (mx as u32).wrapping_sub(mn as u32) as i32;
                if stack_4 as u8 != 0 {
                    if r == mx {
                        let n = ((b11 as u32).wrapping_sub(b12 as u32)).wrapping_mul(0x3f) as i32;
                        if b11 < b12 {
                            u14 = 5;
                            b15 = ((n / diff) as u8).wrapping_add(0x3f);
                        } else {
                            u14 = 0;
                            b15 = (n / diff) as u8;
                        }
                    } else if b11 == mx {
                        let n = ((b12 as u32).wrapping_sub(r as u32)).wrapping_mul(0x3f) as i32;
                        if b12 < r {
                            u14 = 1;
                            b15 = ((n / diff) as u8).wrapping_add(0x3f);
                        } else {
                            u14 = 2;
                            b15 = (n / diff) as u8;
                        }
                    } else {
                        let n = ((r as u32).wrapping_sub(b11 as u32)).wrapping_mul(0x3f) as i32;
                        if r < b11 {
                            u14 = 3;
                            b15 = ((n / diff) as u8).wrapping_add(0x3f);
                        } else {
                            u14 = 4;
                            b15 = (n / diff) as u8;
                        }
                    }
                }
                bvar7 = ((mx as u32).wrapping_mul(DAT_00525540) >> 8) as u8;
                if 0x3f < bvar7 {
                    bvar7 = 0x3f;
                }
                if stack_4 as u8 == 0 {
                    uvar5 = bvar7 as u16;
                    b12 = bvar7;
                    b11 = bvar7;
                } else {
                    let s = stack_4 & 0xff;
                    let c = bvar7 as u32;
                    let bvar2 = ((0x3fu32.wrapping_sub(s).wrapping_mul(c) as i32) / 0x3f) as u8;
                    stack_4 = b15 as u32;
                    let t3 = ((s.wrapping_mul(stack_4) as i32) / -0x3f).wrapping_add(0x3f) as u32;
                    let bvar3 = ((t3.wrapping_mul(c) as i32) / 0x3f) as u8;
                    let t6 = ((stack_4.wrapping_sub(0x3f).wrapping_mul(s) as i32) / 0x3f)
                        .wrapping_add(0x3f) as u32;
                    let ivar6 = (t6.wrapping_mul(c) as i32) / 0x3f;
                    let bvar4 = ivar6 as u8;
                    match u14 {
                        0 => {
                            uvar5 = bvar7 as u16;
                            b12 = bvar2;
                            b11 = bvar4;
                        }
                        1 => {
                            uvar5 = bvar3 as u16;
                            b12 = bvar2;
                            b11 = bvar7;
                        }
                        2 => {
                            uvar5 = bvar2 as u16;
                            b12 = bvar4;
                            b11 = bvar7;
                        }
                        3 => {
                            uvar5 = bvar2 as u16;
                            b12 = bvar7;
                            b11 = bvar3;
                        }
                        4 => {
                            uvar5 = (ivar6 as u16) & 0xff;
                            b12 = bvar7;
                            b11 = bvar2;
                        }
                        5 => {
                            uvar5 = bvar7 as u16;
                            b12 = bvar3;
                            b11 = bvar2;
                        }
                        _ => {}
                    }
                }
                let i_var1 = FUN_004a7360();
                if i_var1 == 0 {
                    rgb[0] = uvar5 as u8;
                    rgb[1] = b11;
                    rgb[2] = b12;
                    let u = FUN_00413e80(0, rgb.as_mut_ptr() as usize as i32);
                    puvar9.write_unaligned(u);
                } else {
                    let v = (((uvar5 as u32) & 0x3e) << 9)
                        | (((b11 as u32) & 0xfe) << 4)
                        | ((b12 >> 1) as u32);
                    puvar9.write_unaligned(v as u16);
                }
                puvar9 = puvar9.add(1);
                pbvar8 = pbvar8.add(3);
                if param_4 == 0 {
                    break;
                }
            }
        }
    }
}
