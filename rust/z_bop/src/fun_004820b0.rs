// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009e9670: [u32; 1];
    static mut DAT_00521f2e: [u16; 1];
    static mut DAT_009a73a0: u16;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009d1308: u32;
    static mut DAT_009e9683: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00482640(a: u32, b: u32, c: u16, d: i32, e: i32) -> u64;
    fn FUN_00484f80(a: u16) -> u32;
    fn FUN_00450650(a: u16, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004820b0(param_1: i32) -> u16 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the entity table addresses are read exactly where the original does.
    unsafe {
        let mut u_var3: u16 = 0xffff;
        let mut local_4: u16 = 0xffff;
        let mut u_var4: u32 = 0xffff_ffff;
        let mut local_2: u16 = 0xffff;
        let off = param_1.wrapping_mul(0x5bd) as isize;
        let mut u_var5: u16 = DAT_00a07cec;
        if (u_var5 as i32) < (DAT_00a07ce8 as i32) {
            loop {
                let b_var1 = ((u_var5 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8) as usize
                    as *const u8)
                    .read();
                let cur = (&raw mut DAT_009e9670)
                    .cast::<u8>()
                    .byte_offset(off)
                    .cast::<u32>()
                    .read_unaligned();
                if b_var1 != 0 && cur != u_var5 as u32 {
                    let u_var2 = b_var1 as u32;
                    let w = (&raw mut DAT_00521f2e)
                        .cast::<u16>()
                        .add((u_var2 * 0x1a) as usize)
                        .read_unaligned();
                    if (w & 1) != 0 {
                        let r = FUN_00482640(
                            u_var2.wrapping_mul(0x19),
                            (u_var2.wrapping_mul(5) >> 8) | w as u32,
                            u_var5,
                            1,
                            param_1,
                        );
                        if (r as u16) == 0 {
                            if DAT_009a73a0 < u_var3 {
                                local_4 = u_var5;
                                u_var3 = DAT_009a73a0;
                            }
                        } else if DAT_009a1ff8 < u_var4 {
                            u_var4 = DAT_009a1ff8;
                            local_2 = u_var5;
                        }
                    }
                }
                u_var5 = u_var5.wrapping_add(1);
                if !((u_var5 as i32) < (DAT_00a07ce8 as i32)) {
                    break;
                }
            }
        }
        let mut u_var2: u32 = DAT_00a07ce8 & 0xffff;
        if (u_var2 as i32) < (DAT_009d1308.wrapping_add(DAT_00a07ce8) as i32) {
            loop {
                let b_var1 = (u_var2
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8) as usize
                    as *const u8)
                    .read();
                let u_var5 = u_var2 as u16;
                if b_var1 != 0 {
                    let v = b_var1 as u32;
                    let w = (&raw mut DAT_00521f2e)
                        .cast::<u16>()
                        .add((v * 0x1a) as usize)
                        .read_unaligned();
                    if (w & 1) != 0 {
                        let r = FUN_00482640(
                            v.wrapping_mul(0x19),
                            (v.wrapping_mul(5) >> 8) | w as u32,
                            u_var5,
                            1,
                            param_1,
                        );
                        if (r as u16) == 0 {
                            if DAT_009a73a0 < u_var3 {
                                local_4 = u_var5;
                                u_var3 = DAT_009a73a0;
                            }
                        } else if DAT_009a1ff8 < u_var4 {
                            u_var4 = DAT_009a1ff8;
                            local_2 = u_var5;
                        }
                    }
                }
                u_var2 = u_var5.wrapping_add(1) as u32;
                if !((u_var2 as i32) < (DAT_009d1308.wrapping_add(DAT_00a07ce8) as i32)) {
                    break;
                }
            }
        }
        let flag = (&raw mut DAT_009e9683).cast::<u8>().byte_offset(off).read();
        'done: {
            if local_2 == 0xffff {
                if flag == 0 {
                    if 0x31 < u_var3 {
                        break 'done;
                    }
                } else if 0xf9 < u_var3 {
                    break 'done;
                }
                local_2 = local_4;
            }
        }
        if flag == 0 {
            u_var4 = FUN_00484f80(local_2);
            if (u_var4 as u8) == 1 {
                local_2 = 0xffff;
                FUN_00450650(0x101, param_1);
            }
        }
        let _ = u_var4;
        local_2
    }
}
