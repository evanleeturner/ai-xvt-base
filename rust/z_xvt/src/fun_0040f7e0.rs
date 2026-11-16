// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d38: u32;
    static mut DAT_006d4974: i32;
    static mut DAT_0073294c: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_007329b0: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_007329b4: i32;
    static mut DAT_0071e6c4: i32;
    static mut DAT_00732f38: i32;
    static mut DAT_00732fe8: i32;
    static mut DAT_0070a180: i32;
    static mut DAT_0070acd8: i32;
    static mut DAT_00703ad8: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00525568: i32;
    static mut DAT_00523b68: i32;
    static mut DAT_0052686c: [u8; 0];
    static mut DAT_00525bf7: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00419190(param_1: u16, param_2: u16) -> u32;
    fn FUN_0040fb70(param_1: i32) -> u32;
}

unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the caller supplies an address inside the game's own memory.
    unsafe { (a as usize as *const T).read_unaligned() }
}

fn absu(v: u32) -> u32 {
    if (v as i32) < 0 {
        0u32.wrapping_sub(v)
    } else {
        v
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040f7e0(param_1: u16, param_2: u16) -> u32 {
    let in_eax: u32 = 0;
    // SAFETY: globals and callees are the game's own; pointers are the
    // game's object addresses, read where the original reads.
    unsafe {
        DAT_006d9d38 = 0x20000;
        let mut b_var4: i32 = 1;
        let mut u_var10: u32 = absu((DAT_006d4974 as u32).wrapping_sub(DAT_0073294c as u32));
        if 0x20000 < (u_var10 as i32) {
            return in_eax & 0xffff0000;
        }
        let mut u_var12: u32 = absu((DAT_006d4978 as u32).wrapping_sub(DAT_007329b0 as u32));
        if (u_var12 as i32) >= 0x20001 {
            return (DAT_007329b0 as u32) & 0xffff0000;
        }
        let mut u_var5: u32 = absu((DAT_006d4970 as u32).wrapping_sub(DAT_007329b4 as u32));
        if 0x20000 < (u_var5 as i32) {
            return u_var5 & 0xffff0000;
        }
        if u_var12 < u_var10 && u_var5 < u_var10 {
            u_var5 >>= 2;
            DAT_006d9d38 = (u_var12 >> 2).wrapping_add(u_var5).wrapping_add(u_var10);
        } else if u_var10 < u_var12 && u_var5 < u_var12 {
            u_var5 >>= 2;
            DAT_006d9d38 = (u_var10 >> 2).wrapping_add(u_var5).wrapping_add(u_var12);
        } else {
            DAT_006d9d38 = (u_var10 >> 2)
                .wrapping_add(u_var12 >> 2)
                .wrapping_add(u_var5);
        }
        if (DAT_006d9d38 as i32) >= 0x20001 {
            return u_var5 & 0xffff0000;
        }
        let i_var11 = absu((DAT_006d4974 as u32).wrapping_sub(DAT_0071e6c4 as u32));
        let i_var14 = absu((DAT_006d4978 as u32).wrapping_sub(DAT_00732f38 as u32));
        let i_var15 = absu((DAT_006d4970 as u32).wrapping_sub(DAT_00732fe8 as u32));
        let i_var6 = absu((DAT_0073294c as u32).wrapping_sub(DAT_0070a180 as u32));
        let i_var7 = absu((DAT_007329b0 as u32).wrapping_sub(DAT_0070acd8 as u32));
        let i_var8 = absu((DAT_007329b4 as u32).wrapping_sub(DAT_00703ad8 as u32));
        let i_var13: u32 = (param_2 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988 as u32);
        let b_var1: u8 = rd::<u8>(i_var13.wrapping_add(3));
        u_var10 = rd::<u16>(
            (&raw mut DAT_0052686c)
                .cast::<u8>()
                .add(0)
                .byte_add((b_var1 as usize).wrapping_mul(0x16)) as usize as u32,
        ) as u32;
        let c_var2: i8 = rd::<i8>(i_var13.wrapping_add(2));
        if c_var2 == 7 || c_var2 == 6 {
            if b_var1 != 0x9b {
                u_var10 = ((u_var10 as i32) >> 1) as u32;
            }
        } else if c_var2 == 0 && {
            let i_var9 = (param_1 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_006d4988 as u32);
            let c = rd::<i8>(i_var9.wrapping_add(2));
            c == 7 || c == 6
        } {
            let i_var9 = (param_1 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_006d4988 as u32);
            let mut tmp_i: i32 = rd::<i32>(i_var13.wrapping_add(0x1f));
            tmp_i = rd::<i32>((tmp_i as u32).wrapping_add(0xa7));
            let i_var3 = tmp_i;
            let tmp_s0: i16 = rd::<i16>((i_var3 as u32).wrapping_add(0x20b));
            let tmp_s1: i16 = rd::<i16>((i_var3 as u32).wrapping_add(0x20d));
            if (tmp_s0 as i32).wrapping_add(tmp_s1 as i32) != 0 {
                u_var10 = u_var10.wrapping_add(((u_var10 as i32) >> 1) as u32);
                if 0x447 < u_var10 {
                    u_var10 = 0x446;
                }
            }
            tmp_i = rd::<i32>(i_var13.wrapping_add(0x1b));
            if DAT_00525568 != 0 && tmp_i != -1 && rd::<i8>(i_var9.wrapping_add(2)) == 6 && {
                let local_18 = rd::<u8>(i_var9.wrapping_add(3)) as usize;
                *(&raw mut DAT_00525bf7).cast::<i8>().add(local_18) == 0
            } {
                match b_var1 as u32 {
                    2 | 4 => {}
                    3 | 6 | 8 => {
                        u_var10 = u_var10.wrapping_mul(2);
                    }
                    5 => {
                        u_var10 = u_var10
                            .wrapping_add(u_var10 / 2)
                            .wrapping_add(((u_var10 as i32) >> 2) as u32);
                    }
                    _ => {
                        u_var10 = u_var10.wrapping_add(u_var10 / 2);
                    }
                }
                b_var4 = 0;
            }
        }
        u_var12 = i_var11.wrapping_add(i_var6).wrapping_add(u_var10);
        u_var5 = i_var14.wrapping_add(i_var7).wrapping_add(u_var10);
        let u_var16: u32 = i_var15.wrapping_add(i_var8).wrapping_add(u_var10);
        if u_var5 < u_var12 && u_var16 < u_var12 {
            u_var12 = (u_var5 >> 2)
                .wrapping_add(u_var16 >> 2)
                .wrapping_add(u_var12);
        } else if u_var12 < u_var5 && u_var16 < u_var5 {
            u_var12 = (u_var12 >> 2)
                .wrapping_add(u_var16 >> 2)
                .wrapping_add(u_var5);
        } else {
            u_var12 = (u_var12 >> 2)
                .wrapping_add(u_var5 >> 2)
                .wrapping_add(u_var16);
        }
        if u_var12 < DAT_006d9d38 {
            return u_var12 & 0xffff0000;
        }
        if (b_var4 != 0 && 0x446 < u_var10) || b_var1 == 0x3a {
            if DAT_00523b68 == 0 {
                return FUN_00419190(param_1, param_2);
            }
            DAT_00523b68 = 0;
            let tmp_i: i32 = rd::<i32>(i_var13.wrapping_add(0x1f));
            let tmp_us: u16 = rd::<u16>((tmp_i as u32).wrapping_add(0x79));
            if tmp_us < 0x28 {
                return FUN_00419190(param_1, param_2);
            }
        }
        FUN_0040fb70(((u_var10 as i32) >> 3).wrapping_add((u_var10 as i32) >> 2))
    }
}
