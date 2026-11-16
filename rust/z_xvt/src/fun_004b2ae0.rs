// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006559b0: [i16; 8];
    static mut DAT_00525518: u32;
    static mut DAT_00525508: u8;
    static mut DAT_00655810: u16;
    static mut DAT_00655414: u16;
    static mut DAT_00655416: u16;
    static mut DAT_00655418: u16;
    static mut DAT_00655184: u16;
    static mut DAT_00655186: u16;
    static mut DAT_00655188: u16;
    static mut DAT_00655176: u16;
    static mut DAT_0065517a: u16;
    static mut DAT_0065517c: u16;
    static mut DAT_0065517e: u16;
    static mut DAT_006551c8: u16;
    static mut DAT_006551cc: u16;
    static mut DAT_00655a90: u16;
    static mut DAT_00655a92: u16;
    static mut DAT_00655a94: u16;
    static mut DAT_00655a9a: u16;
    static mut DAT_00655a9c: u16;
    static mut DAT_00655a9e: u16;
    static mut DAT_00655aa0: u16;
    static mut DAT_00655aa2: u16;
    static mut DAT_006da538: u16;
    static mut DAT_006da9ec: u32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b743: u8;
    static mut DAT_0071b730: u8;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4a4e: u8;
    static mut DAT_0071f2e2: u8;
    static mut DAT_0052b0b8: u8;
    static mut DAT_0052b0b9: u8;
    static mut DAT_00655fda: u8;
    static mut DAT_00655fd8: u8;
    static mut DAT_00655fc2: u8;
    static mut DAT_00655fc0: u8;
    static mut DAT_00655fce: u8;
    static mut DAT_00655fcc: u8;
    static mut DAT_00655faa: u8;
    static mut DAT_00655fa8: u8;
    static mut DAT_00655fb6: u8;
    static mut DAT_00655fb4: u8;
    static mut DAT_00655b68: u8;
    static mut DAT_00655b64: u8;
    static mut DAT_00655b66: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00426a50(a: u32, b: u16, c: u32, d: u32, e: u32, f: u32, g: u16, h: u16, i: u32);
    fn FUN_0048ee20(a: i8);
}

fn c22(a: u16, b: u16) -> u32 {
    ((a as u32) << 16) | (b as u32)
}

unsafe fn rd(base: *mut u8, off: u32) -> u16 {
    // SAFETY: caller passes the game's own table address and an offset into it.
    unsafe { base.byte_add(off as usize).cast::<u16>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b2ae0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read where the original reads them.
    unsafe {
        let mut u_var12: u16 = 0;
        loop {
            let s_var3: i16 = (&raw mut DAT_006559b0)
                .cast::<i16>()
                .add(u_var12 as usize)
                .read();
            let s_var5: i16 = DAT_00525518 as i16;
            let u_var9: u16 = (DAT_00525518 >> 16) as u16;
            let d13: u16 = DAT_00655810;
            let d14: u16 = DAT_00655414;
            let flag: u8 = (&raw mut DAT_0071b743)
                .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                .read();

            // (base for uVar8, base for uVar10, uVar11 low, uVar7 low, uVar13, uVar14)
            let mut sel: Option<(*mut u8, *mut u8, u16, u16, u16, u16)> = None;
            let alt = (
                &raw mut DAT_00655fb6,
                &raw mut DAT_00655fb4,
                DAT_00655186,
                DAT_006551cc,
                DAT_00655188,
                DAT_006551c8,
            );
            match u_var12 as u32 {
                0 => {
                    if s_var3 != 0 {
                        if flag != 0 {
                            sel = Some(alt);
                        } else {
                            sel = Some((
                                &raw mut DAT_00655fda,
                                &raw mut DAT_00655fd8,
                                DAT_0065517a,
                                DAT_0065517e,
                                DAT_00655176,
                                DAT_0065517c,
                            ));
                        }
                    }
                }
                1 => {
                    if s_var3 != 0 {
                        if flag != 0 {
                            sel = Some(alt);
                        } else {
                            sel = Some((
                                &raw mut DAT_00655fc2,
                                &raw mut DAT_00655fc0,
                                DAT_00655aa2,
                                DAT_00655a94,
                                DAT_00655a9c,
                                DAT_00655a90,
                            ));
                        }
                    }
                }
                3 => {
                    if s_var3 != 0 && flag == 0 {
                        sel = Some((
                            &raw mut DAT_00655fce,
                            &raw mut DAT_00655fcc,
                            DAT_00655aa0,
                            DAT_00655a92,
                            DAT_00655a9e,
                            DAT_00655a9a,
                        ));
                    }
                }
                4 => {
                    if s_var3 != 0 {
                        if flag != 0 {
                            sel = Some(alt);
                        } else {
                            sel = Some((
                                &raw mut DAT_00655faa,
                                &raw mut DAT_00655fa8,
                                DAT_00655418,
                                DAT_00655416,
                                d13,
                                d14,
                            ));
                        }
                    }
                }
                5 => {
                    if s_var3 != 0 {
                        sel = Some((
                            &raw mut DAT_00655faa,
                            &raw mut DAT_00655fa8,
                            DAT_00655418,
                            DAT_00655416,
                            d13,
                            d14,
                        ));
                    }
                }
                6 => {
                    if s_var3 != 0 && flag != 0 {
                        sel = Some(alt);
                    }
                }
                _ => {}
            }
            if let Some((b8, b10, v11, v7, a13, a14)) = sel {
                let u_var6 = c22(u_var9, s_var5.wrapping_mul(DAT_006da538 as i16) as u16);
                let i_var4 = (DAT_00655184 as u32).wrapping_mul(0xc);
                let u_var8 = c22(u_var9, rd(b8, i_var4));
                let u_var10 = c22((i_var4 >> 16) as u16, rd(b10, i_var4));
                let u_var11 = c22(((DAT_00655184 as u32).wrapping_mul(3) >> 16) as u16, v11);
                let u_var7 = c22(u_var9, v7);
                FUN_00426a50(
                    DAT_006da9ec,
                    DAT_00525508 as u16,
                    u_var7,
                    u_var11,
                    u_var10,
                    u_var8,
                    a13,
                    a14,
                    u_var6,
                );
            }
            u_var12 = u_var12.wrapping_add(1);
            if 7 < u_var12 {
                if DAT_00655184 == 0x78 {
                    let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
                    let i_a = (&raw mut DAT_0071b730)
                        .byte_add(idx)
                        .cast::<i32>()
                        .read_unaligned();
                    let addr_b = (i_a as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1f)
                        .wrapping_add(DAT_006d4988 as u32);
                    let i_b = (addr_b as usize as *const i32).read_unaligned();
                    let addr_c = (i_b as u32).wrapping_add(0xa7);
                    let i_c = (addr_c as usize as *const i32).read_unaligned();
                    let p = i_c as u32 as usize as *const u8;
                    if (p.byte_add(0x1db).read() & 8) != 0 {
                        let b = p.byte_add(4).read();
                        FUN_0048ee20(0);
                        u_var12 = 0;
                        let off = (b as usize).wrapping_mul(0xef);
                        let bvar1 = (&raw mut DAT_0052b0b9).byte_add(off).read();
                        let bvar2 = (&raw mut DAT_0052b0b8).byte_add(off).read();
                        let lim = (bvar1 as u16).wrapping_add(bvar2 as u16);
                        if lim != 0 {
                            loop {
                                let i_var4 = (u_var12 as u32)
                                    .wrapping_add(DAT_00655184 as u32)
                                    .wrapping_mul(0xc);
                                let w = rd(&raw mut DAT_00655b68, i_var4) as i16;
                                if w != 0 {
                                    let u_var6 = c22(
                                        (DAT_00525518 >> 16) as u16,
                                        (DAT_00525518 as i16).wrapping_mul(DAT_006da538 as i16)
                                            as u16,
                                    );
                                    let u_var8 =
                                        (((u_var6 >> 8) << 8) | (DAT_006d4a4e as u32)) & 0xffff00ff;
                                    let u_var9 = (u_var8 >> 16) as u16;
                                    let m = (DAT_0071f2e2 as u32).wrapping_add(1);
                                    FUN_00426a50(
                                        DAT_006da9ec,
                                        0,
                                        c22(
                                            u_var9,
                                            (u_var12 as u32).wrapping_mul(m).wrapping_add(2) as u16,
                                        ),
                                        2,
                                        c22(
                                            (i_var4 >> 16) as u16,
                                            rd(&raw mut DAT_00655b64, i_var4),
                                        ),
                                        c22(u_var9, rd(&raw mut DAT_00655b66, i_var4)),
                                        (w as i32).wrapping_mul(m as i32) as u16,
                                        u_var8 as u16,
                                        u_var6,
                                    );
                                }
                                u_var12 = u_var12.wrapping_add(1);
                                if u_var12 >= lim {
                                    break;
                                }
                            }
                        }
                    }
                }
                return;
            }
        }
    }
}
