// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da500: [i8; 0];
    static mut DAT_006da504: [i8; 0];
    static mut DAT_0073254c: i16;
    static mut DAT_00732554: i16;
    static mut DAT_006db592: u16;
    static mut DAT_00732f94: i16;
    static mut DAT_00732f92: i16;
    static mut DAT_0071b6b0: i16;
    static mut DAT_0085e7a0: [u8; 0];
    static mut DAT_00536920: u32;
    static mut DAT_00536924: i32;
    static mut DAT_00523618: [u8; 0];
    static mut DAT_006d9d88: i16;
    static mut DAT_006d9d92: i16;
    static mut DAT_0070acc6: i16;
    static mut DAT_006d9d3c: i16;
    static mut DAT_006d9d3e: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048faa0(a: *mut u32, b: *mut u32, c: *mut u32) -> u32;
    fn FUN_00436ec0() -> i32;
    fn FUN_00436ef0(a: i32, b: i32);
    fn FUN_004a53f0() -> i32;
    fn FUN_004a5470() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405cf0(param_1: i32) -> u32 {
    let mut local_1a = core::mem::MaybeUninit::<i16>::uninit();
    let mut local_18 = core::mem::MaybeUninit::<i16>::uninit();
    // SAFETY: the globals and callees are the game's own memory and code;
    // the byte tables are read at the offsets the original uses.
    unsafe {
        let b500 = (&raw mut DAT_006da500).cast::<u8>();
        let b504 = (&raw mut DAT_006da504).cast::<u8>();
        let mut u_var8: u32 = 0;
        if -1 < param_1 {
            let p = param_1 as u32;
            DAT_0073254c = (b500
                .byte_add(p.wrapping_mul(6).wrapping_add(3) as usize)
                .read() as i8) as i16;
            let i_var3 = p.wrapping_mul(6) as usize;
            DAT_00732554 = (b504.byte_add(i_var3).read() as i8) as i16;
            u_var8 = (((p.wrapping_mul(3) >> 8) << 8) | (b500.byte_add(i_var3 + 2).read() as u32))
                & 0xffff00ff;
            DAT_006db592 = u_var8 as u16;
            DAT_00732f94 = (b504.byte_add(i_var3 + 1).read() & 3) as i16;
            return u_var8;
        }
        let mut local_16: i16 = 0;
        let mut local_10: u32 = 0;
        let mut local_4: u32 = 0;
        let mut local_8: u32 = 0;
        if DAT_00732f92 != 0 {
            u_var8 = FUN_0048faa0(&raw mut local_8, &raw mut local_4, &raw mut local_10);
        }
        if DAT_0071b6b0 != 0 {
            let r = FUN_00436ec0();
            local_16 = r as i16;
            FUN_00436ef0(0, 0);
        }
        let mut u_var4: u32 = 0;
        if FUN_004a53f0() != 0 {
            u_var4 = FUN_004a5470() & 0xffff;
        }
        let mut s_var9: i16 = 0;
        let mut u_var7: u32 = 1;
        let mut i_var3: i32 = 0;
        let mut local_c: i32 = 0;
        let tab = (&raw mut DAT_0085e7a0).cast::<u8>();
        loop {
            let b_var1 = tab.add(i_var3 as usize).read();
            let mut u_var6 = u_var4;
            if b_var1 != 0 {
                if (u_var8 & u_var7) == 0 {
                    if (DAT_00536920 & u_var7) != 0 {
                        let u_var5: u32 = if b_var1 == 0xae {
                            0xae
                        } else if b_var1 < 0xaf || 0xb7 < b_var1 {
                            0
                        } else {
                            0xb6
                        };
                        if u_var5 != 0 {
                            u_var6 = u_var5;
                            if (u_var4 as i16) != 0 {
                                u_var8 |= u_var7;
                                u_var6 = u_var4;
                            }
                        }
                    }
                } else {
                    if b_var1 == 0x98 {
                        local_c = 1;
                    } else if b_var1 == 0x99 {
                        s_var9 = 1;
                    }
                    if (DAT_00536920 & u_var7) == 0 {
                        if (u_var4 as i16) == 0 {
                            u_var6 = b_var1 as u32;
                        } else {
                            u_var8 &= !u_var7;
                        }
                    }
                }
            }
            DAT_006db592 = u_var6 as u16;
            u_var7 = u_var7.wrapping_mul(2);
            i_var3 += 1;
            u_var4 = u_var6;
            if i_var3 >= 0x14 {
                break;
            }
        }
        DAT_00536920 = u_var8;
        let s_var2 = local_c as i16;
        let mut i_var3 = DAT_00536924;
        if DAT_006db592 == 0 {
            local_c = ((local_10 as i8 as i32) as u32).wrapping_add(0x80) as i32;
            i_var3 = local_c;
            if DAT_00536924 != -1 {
                let d = DAT_00536924;
                let diff = local_c.wrapping_sub(d);
                let adj = (diff as u32).wrapping_add(((diff >> 31) as u32) & 3) as i32;
                i_var3 = d.wrapping_add(adj >> 2);
                let t = i_var3.wrapping_add(8);
                local_c = ((t as u32).wrapping_add(((t >> 31) as u32) & 0xf) as i32) >> 4;
                let old = d.wrapping_add(8);
                let old_q = ((old as u32).wrapping_add(((old >> 31) as u32) & 0xf) as i32) >> 4;
                if local_c != old_q {
                    if local_c < 0 {
                        local_c = 0;
                    }
                    if 0x10 < local_c {
                        local_c = 0x10;
                    }
                    let t2 = (&raw mut DAT_00523618).cast::<u8>();
                    DAT_006db592 = t2.offset(-(local_c as isize)).read() as u16;
                }
            }
        }
        DAT_00536924 = i_var3;
        DAT_006d9d88 = 0;
        DAT_006d9d92 = 0;
        DAT_0073254c = local_8 as i16;
        DAT_00732554 = local_4 as i16;
        DAT_00732f94 = s_var2.wrapping_add(s_var9.wrapping_mul(2));
        DAT_0070acc6 = local_16;
        let p1a = local_1a.as_mut_ptr();
        let p18 = local_18.as_mut_ptr();
        DAT_006d9d3c = p1a.read_volatile();
        DAT_006d9d3e = p18.read_volatile();
        (((local_c as u32) >> 16) << 16) | (DAT_006db592 as u32)
    }
}
