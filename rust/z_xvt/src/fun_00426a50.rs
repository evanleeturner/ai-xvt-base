// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: u32;
    static mut DAT_0070acc8: u32;
    static mut DAT_00525534: u32;
    static mut DAT_007325e0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00426a50(
    param_1: i32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u16,
    param_8: u16,
    param_9: u32,
) {
    // SAFETY: the globals are the game's own; the pointers address the
    // caller's source and destination surfaces, accessed as the original does.
    unsafe {
        let u_var4: u16;
        if DAT_00525518 == 1 {
            u_var4 = (((param_2 as u32) << 8) | (param_2 as u32)) as u16;
        } else {
            // the C indexes a `uint` object, so the offset scales by 4
            u_var4 = (&raw mut DAT_007325e0)
                .cast::<u8>()
                .wrapping_byte_add((param_2 as u32).wrapping_mul(2).wrapping_mul(4) as usize)
                .cast::<u16>()
                .read_unaligned();
        }
        let mut p7: *mut u8 = DAT_00525534
            .wrapping_add((param_6 & 0xffff).wrapping_mul(DAT_0070acc8))
            .wrapping_add((param_5 & 0xffff).wrapping_mul(DAT_00525518))
            as usize as *mut u8;
        let u_var1: u32 = param_9 & 0xffff;
        let mut p6: *mut u8 = (param_1 as u32)
            .wrapping_add((param_3 & 0xffff).wrapping_mul(DAT_00525518))
            .wrapping_add((param_4 & 0xffff).wrapping_mul(u_var1))
            as usize as *mut u8;
        if param_2 == 0xffff {
            if param_8 != 0 {
                let mut u_var8: u32 = param_8 as u32;
                loop {
                    let u_var2: u32 = DAT_00525518.wrapping_mul(param_7 as u32);
                    let mut p9 = p6;
                    let mut p10 = p7;
                    let mut u_var5 = u_var2 >> 2;
                    while u_var5 != 0 {
                        let v = p9.cast::<u32>().read_unaligned();
                        p10.cast::<u32>().write_unaligned(v);
                        p9 = p9.wrapping_byte_add(4);
                        p10 = p10.wrapping_byte_add(4);
                        u_var5 = u_var5.wrapping_sub(1);
                    }
                    let mut r = u_var2 & 3;
                    while r != 0 {
                        p10.write(p9.read());
                        p9 = p9.wrapping_byte_add(1);
                        p10 = p10.wrapping_byte_add(1);
                        r = r.wrapping_sub(1);
                    }
                    p7 = p7.wrapping_byte_add(DAT_0070acc8 as usize);
                    p6 = p6.wrapping_byte_add(u_var1 as usize);
                    u_var8 = u_var8.wrapping_sub(1);
                    if u_var8 == 0 {
                        break;
                    }
                }
                return;
            }
        } else if param_8 != 0 {
            let mut u_var8: u32 = param_8 as u32;
            loop {
                let mut u_var2: u32 = param_7 as u32;
                if param_7 != 0 {
                    loop {
                        if DAT_00525518 == 1 {
                            if (u_var4 as u8) != p6.read() {
                                p7.write(p6.read());
                            }
                        } else if DAT_00525518 == 2 {
                            let s = p6.cast::<u16>().read_unaligned();
                            if u_var4 != s {
                                p7.cast::<u16>().write_unaligned(s);
                            }
                        }
                        p6 = p6.wrapping_byte_add(DAT_00525518 as usize);
                        p7 = p7.wrapping_byte_add(DAT_00525518 as usize);
                        u_var2 = u_var2.wrapping_sub(1);
                        if u_var2 == 0 {
                            break;
                        }
                    }
                }
                let i_var3: u32 = DAT_00525518.wrapping_mul(param_7 as u32);
                p7 = p7.wrapping_byte_add(DAT_0070acc8.wrapping_sub(i_var3) as usize);
                p6 = p6.wrapping_byte_add(u_var1.wrapping_sub(i_var3) as usize);
                u_var8 = u_var8.wrapping_sub(1);
                if u_var8 == 0 {
                    break;
                }
            }
        }
    }
}
