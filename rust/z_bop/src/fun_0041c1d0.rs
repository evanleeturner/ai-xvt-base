// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00521f1c: [u8; 0x2000];
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_00a07bdc: i32;
    static mut DAT_00a07c54: i32;
    static mut DAT_00a07c58: i32;
    static mut DAT_009a7398: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_009d80c0: i32;
    static mut DAT_009d8c18: i32;
    static mut DAT_009d1150: i32;
    static mut DAT_00523428: i32;
    static mut DAT_0051a44f: [u8; 256];
    static mut DAT_0051bf58: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f3d0(a: u32, b: u32, c: u32) -> u32;
    fn FUN_004a5490(a: u16, b: u16) -> u32;
    fn FUN_0041c570(a: i32) -> u32;
}

fn ab(x: i32) -> i32 {
    if x < 0 { x.wrapping_neg() } else { x }
}

unsafe fn rd32(a: u32) -> i32 {
    // SAFETY: caller supplies an address inside the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041c1d0(param_1: u16, param_2: u16) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers derived from them are read where the original reads.
    unsafe {
        let mut b_var4: bool = true;
        let i_var3: i32 = (param_2 as u32).wrapping_mul(0x23) as i32;
        let tbl = (&raw mut DAT_00521f1c) as *mut u8;
        let base = DAT_009a1fe8;
        let idx_a = *base.add((3u32.wrapping_add(i_var3 as u32)) as usize) as usize * 0x1a;
        let mut tmp_a: i32 = tbl.add(idx_a).cast::<i32>().read_unaligned();
        let idx_b = *base.add((3u32.wrapping_add((param_1 as u32).wrapping_mul(0x23))) as usize)
            as usize
            * 0x1a;
        let mut tmp_b: i32 = tbl.add(idx_b).cast::<i32>().read_unaligned();
        let i_var11_init = (tmp_a as u32)
            .wrapping_add(0x20000)
            .wrapping_add(tmp_b as u32) as i32;
        let mut u_var5: u32 = (DAT_009a1fd4 as u32).wrapping_sub(DAT_00a07bdc as u32);
        DAT_009a7398 = i_var11_init;
        if (u_var5 as i32) < 0 {
            u_var5 = 0u32.wrapping_sub(u_var5);
        }
        if i_var11_init < u_var5 as i32 {
            return u_var5 & 0xffff0000;
        }
        let mut u_var10: u32 = (DAT_009a1fd8 as u32).wrapping_sub(DAT_00a07c54 as u32);
        if (u_var10 as i32) < 0 {
            u_var10 = 0u32.wrapping_sub(u_var10);
        }
        let u_var9: u16 = (u_var5 >> 16) as u16;
        if i_var11_init < u_var10 as i32 {
            return (u_var9 as u32) << 16;
        }
        let mut u_var12: u32 = (DAT_009a1fd0 as u32).wrapping_sub(DAT_00a07c58 as u32);
        if (u_var12 as i32) < 0 {
            u_var12 = 0u32.wrapping_sub(u_var12);
        }
        if i_var11_init < u_var12 as i32 {
            return (u_var9 as u32) << 16;
        }
        u_var5 = FUN_0041f3d0(u_var5, u_var10, u_var12);
        DAT_009a7398 = u_var5 as i32;
        if i_var11_init < u_var5 as i32 {
            return u_var5 & 0xffff0000;
        }
        let i_var11 = ab(DAT_009a1fd4.wrapping_sub(DAT_009ec604));
        let i_var17 = ab(DAT_009a1fd8.wrapping_sub(DAT_00a081e8));
        let i_var15 = ab(DAT_009a1fd0.wrapping_sub(DAT_00a08298));
        let i_var6 = ab(DAT_00a07bdc.wrapping_sub(DAT_009d80c0));
        let i_var7 = ab(DAT_00a07c54.wrapping_sub(DAT_009d8c18));
        let i_var8 = ab(DAT_00a07c58.wrapping_sub(DAT_009d1150));
        let i_var13: u32 = (i_var3 as u32).wrapping_add(DAT_009a1fe8 as usize as u32);
        let p13 = i_var13 as usize as *const u8;
        let mut i_var16: i32 = tbl
            .add(*p13.add(3) as usize * 0x1a)
            .cast::<i32>()
            .read_unaligned();
        if *p13.add(2) == 7 || *p13.add(2) == 6 {
            if *p13.add(3) != 0x9b {
                i_var16 >>= 1;
            }
        } else if *p13.add(2) == 0 {
            let i_var14: u32 = (param_1 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8 as usize as u32);
            let p14 = i_var14 as usize as *const u8;
            let c_var1: i8 = *p14.add(2) as i8;
            if c_var1 == 7 || c_var1 == 6 {
                let mut i_var2 = rd32(i_var13.wrapping_add(0x1f));
                i_var2 = rd32((i_var2 as u32).wrapping_add(0xa9));
                tmp_a = rd32((i_var2 as u32).wrapping_add(0x215));
                tmp_b = rd32((i_var2 as u32).wrapping_add(0x211));
                if (tmp_a as u32).wrapping_add(tmp_b as u32) as i32 != 0 {
                    i_var16 = (i_var16 as u32).wrapping_add((i_var16 >> 1) as u32) as i32;
                    if 0x447 < i_var16 {
                        i_var16 = 0x446;
                    }
                }
                if DAT_00523428 != 0 {
                    tmp_a = rd32(i_var13.wrapping_add(0x1b));
                    if tmp_a != -1 && c_var1 == 6 {
                        let local_20 = *p14.add(3) as usize;
                        let flags = (&raw mut DAT_0051a44f) as *mut u8;
                        if *flags.add(local_20) == 0 {
                            match *p13.add(3) {
                                2 | 4 => {}
                                3 | 6 | 8 => {
                                    i_var16 = (i_var16 as u32).wrapping_mul(2) as i32;
                                }
                                5 => {
                                    let t = ((i_var16 as u32)
                                        .wrapping_add(((i_var16 >> 31) as u32) & 3)
                                        as i32)
                                        >> 2;
                                    i_var16 = (i_var16 as u32)
                                        .wrapping_add((i_var16 / 2) as u32)
                                        .wrapping_add(t as u32)
                                        as i32;
                                }
                                _ => {
                                    i_var16 =
                                        (i_var16 as u32).wrapping_add((i_var16 / 2) as u32) as i32;
                                }
                            }
                            b_var4 = false;
                        }
                    }
                }
            }
        }
        u_var10 = FUN_0041f3d0(
            (i_var11 as u32)
                .wrapping_add(i_var6 as u32)
                .wrapping_add(i_var16 as u32),
            (i_var17 as u32)
                .wrapping_add(i_var7 as u32)
                .wrapping_add(i_var16 as u32),
            (i_var15 as u32)
                .wrapping_add(i_var8 as u32)
                .wrapping_add(i_var16 as u32),
        );
        if u_var5 <= u_var10 {
            let cur = DAT_009a1fe8;
            if (b_var4 && 0x446 < i_var16)
                || *cur.add((3u32.wrapping_add(i_var3 as u32)) as usize) == 0x3a
            {
                if DAT_0051bf58 == 0 {
                    if (i_var17 as u32)
                        .wrapping_add(i_var15 as u32)
                        .wrapping_add(i_var11 as u32) as i32
                        == 0
                    {
                        return 0;
                    }
                    return FUN_004a5490(param_1, param_2);
                }
                DAT_0051bf58 = 0;
                let i_var2 = rd32(i_var13.wrapping_add(0x1f));
                let tmp_s: u16 =
                    ((i_var2 as u32).wrapping_add(0x79) as usize as *const u16).read_unaligned();
                if tmp_s < 0x28 {
                    return FUN_004a5490(param_1, param_2);
                }
            }
            return FUN_0041c570(((i_var16 >> 3) as u32).wrapping_add((i_var16 >> 2) as u32) as i32);
        }
        u_var10 & 0xffff0000
    }
}
