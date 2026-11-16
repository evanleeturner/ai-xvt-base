// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523400: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac4b0() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449410(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let mut param_4 = param_4;
    // SAFETY: the globals and callee are the game's own; the pointers are
    // caller-supplied buffers read and written where the original does.
    unsafe {
        if DAT_00523400 == 0x100 {
            if param_3 < (param_4 as u32).wrapping_add(param_3 as u32) as i32 {
                let mut puvar9 =
                    (param_2 as u32).wrapping_add((param_3 as u32).wrapping_mul(2)) as *mut u16;
                let mut pbvar8 =
                    (param_1 as u32).wrapping_add((param_3 as u32).wrapping_mul(3)) as *const u8;
                loop {
                    let u_var1 = FUN_004ac4b0();
                    let r = pbvar8.read();
                    let g = pbvar8.add(1).read();
                    let b = pbvar8.add(2).read();
                    let v: u16 = if u_var1 == 0 {
                        ((g as u16) << 5)
                            | (((((r >> 1) << 3) as u8) as u16) << 8)
                            | ((b >> 1) as u16)
                    } else {
                        ((((g & 0xfe) as u32) << 4)
                            | ((b >> 1) as u32)
                            | (((r & 0x3e) as u32) << 9)) as u16
                    };
                    puvar9.write_unaligned(v);
                    puvar9 = puvar9.add(1);
                    pbvar8 = pbvar8.add(3);
                    param_4 = (param_4 as u32).wrapping_sub(1) as i32;
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
                param_4 = (param_4 as u32).wrapping_sub(1) as i32;
                let r = pbvar8.read();
                let mut u_var6: u16 = r as u16;
                let mut g = pbvar8.add(1).read();
                let mut b = pbvar8.add(2).read();

                let mut mx: u8 = 0;
                if (if r < g {
                    true
                } else {
                    mx = r;
                    r < b
                }) && (if g < r {
                    true
                } else {
                    mx = g;
                    g < b
                }) {
                    mx = b;
                }
                let mut mn: u8 = 0;
                if (if g < r {
                    true
                } else {
                    mn = r;
                    b < r
                }) && (if r < g {
                    true
                } else {
                    mn = g;
                    b < g
                }) {
                    mn = b;
                }

                let sat: u8 = if mx == 0 {
                    0
                } else {
                    (((mx as u32).wrapping_sub(mn as u32).wrapping_mul(0x3f)) as i32
                        / (mx as u32 as i32)) as u8
                };
                let mut sector: u8 = 0;
                let mut hue: u8 = 0;
                if sat != 0 {
                    let d = (mx as u32).wrapping_sub(mn as u32) as i32;
                    if mx == r {
                        if g < b {
                            sector = 5;
                            hue = ((((g as u32).wrapping_sub(b as u32).wrapping_mul(0x3f)) as i32
                                / d) as u8)
                                .wrapping_add(0x3f);
                        } else {
                            sector = 0;
                            hue = (((g as u32).wrapping_sub(b as u32).wrapping_mul(0x3f)) as i32
                                / d) as u8;
                        }
                    } else if mx == g {
                        if b < r {
                            sector = 1;
                            hue = ((((b as u32).wrapping_sub(r as u32).wrapping_mul(0x3f)) as i32
                                / d) as u8)
                                .wrapping_add(0x3f);
                        } else {
                            sector = 2;
                            hue = (((b as u32).wrapping_sub(r as u32).wrapping_mul(0x3f)) as i32
                                / d) as u8;
                        }
                    } else if r < g {
                        sector = 3;
                        hue = ((((r as u32).wrapping_sub(g as u32).wrapping_mul(0x3f)) as i32 / d)
                            as u8)
                            .wrapping_add(0x3f);
                    } else {
                        sector = 4;
                        hue = (((r as u32).wrapping_sub(g as u32).wrapping_mul(0x3f)) as i32 / d)
                            as u8;
                    }
                }

                let mut v7: u8 = (((mx as u32).wrapping_mul(DAT_00523400)) >> 8) as u8;
                if 0x3f < v7 {
                    v7 = 0x3f;
                }
                if sat == 0 {
                    u_var6 = v7 as u16;
                    g = v7;
                    b = v7;
                } else {
                    let s = sat as u32;
                    let v = v7 as u32;
                    let h = hue as u32;
                    let b2 = ((0x3fu32.wrapping_sub(s).wrapping_mul(v)) as i32 / 0x3f) as u8;
                    let t = (s.wrapping_mul(h)) as i32 / -0x3f;
                    let b3 = (((t as u32).wrapping_add(0x3f).wrapping_mul(v)) as i32 / 0x3f) as u8;
                    let u = (h.wrapping_sub(0x3f).wrapping_mul(s)) as i32 / 0x3f;
                    let i5 = ((u as u32).wrapping_add(0x3f).wrapping_mul(v)) as i32 / 0x3f;
                    let b4 = i5 as u8;
                    match sector {
                        0 => {
                            u_var6 = v7 as u16;
                            g = b4;
                            b = b2;
                        }
                        1 => {
                            u_var6 = b3 as u16;
                            g = v7;
                            b = b2;
                        }
                        2 => {
                            u_var6 = b2 as u16;
                            g = v7;
                            b = b4;
                        }
                        3 => {
                            u_var6 = b2 as u16;
                            g = b3;
                            b = v7;
                        }
                        4 => {
                            u_var6 = (i5 as u16) & 0xff;
                            g = b2;
                            b = v7;
                        }
                        5 => {
                            u_var6 = v7 as u16;
                            g = b2;
                            b = b3;
                        }
                        _ => {}
                    }
                }

                let u_var1 = FUN_004ac4b0();
                if u_var1 == 0 {
                    let r8 = u_var6 as u8;
                    u_var6 = ((g as u16) << 5)
                        | (((((r8 >> 1) << 3) as u8) as u16) << 8)
                        | ((b >> 1) as u16);
                } else {
                    u_var6 = ((((g & 0xfe) as u32) << 4)
                        | (((u_var6 & 0x3e) as u32) << 9)
                        | ((b >> 1) as u32)) as u16;
                }
                puvar9.write_unaligned(u_var6);
                puvar9 = puvar9.add(1);
                pbvar8 = pbvar8.add(3);
                if param_4 == 0 {
                    break;
                }
            }
        }
    }
}
