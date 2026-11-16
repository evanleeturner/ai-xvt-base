// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d9d7a: u8;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d499c: u32;
    static mut DAT_00732e54: *mut u8;
    static mut DAT_006d9d7b: [u8; 4];
    static mut DAT_006d9d7f: [u8; 4];
    static mut DAT_006d9d83: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(a: u16, b: u16, c: u16) -> u16;
    fn FUN_0042b150(a: *mut u8) -> *mut u8;
    fn FUN_00429a20(a: u32) -> u32;
    fn FUN_00429c40(a: u32) -> u16;
    fn FUN_0044c560(a: u16, b: u32) -> u16;
    fn FUN_00412810(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0042a1a0(a: u32, b: u32);
    fn FUN_00444040(a: u32, b: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044c2b0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let mut local_a: u16 = 0xffff;
    let mut local_4: u32 = 0xffff_ffff;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read from the game's tables exactly where the original does.
    unsafe {
        let mut u_var8: u16 = DAT_00732a3c;
        if (u_var8 as i32) < DAT_006d4980 {
            loop {
                let s_var7 = u_var8 as i16;
                let p = DAT_006d4988.byte_add(u_var8 as usize * 0x23);
                if p.byte_add(3).read() != 0 {
                    let q = p.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                    let i_var4 = q.byte_add(0xa7).cast::<*mut u8>().read_unaligned();
                    let mut b_any = false;
                    let mut k: u16 = 0;
                    loop {
                        if i_var4.byte_add(k as usize + 0x1e6).read() != 0 {
                            b_any = true;
                        }
                        k = k.wrapping_add(1);
                        if k >= 10 {
                            break;
                        }
                    }
                    if b_any {
                        let mut u_var9 = FUN_0049f010(u_var8, param_1, param_2);
                        let u_var3 = FUN_0049f010(u_var8, param_4, param_5);
                        if param_3 == 1 {
                            u_var9 |= u_var3;
                        } else {
                            u_var9 &= u_var3;
                        }
                        if u_var9 != 0 && (DAT_00732a3c as i32) < DAT_006d4980 {
                            let mut u_var10: u16 = DAT_00732a3c;
                            loop {
                                'cont: {
                                    let p = DAT_006d4988.byte_add(u_var10 as usize * 0x23);
                                    if p.byte_add(3).read() == 0 {
                                        break 'cont;
                                    }
                                    let q = p.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                                    let q = q.byte_add(0xa7).cast::<*mut u8>().read_unaligned();
                                    let i_var5 = FUN_0042b150(q);
                                    let c_var1 = i_var5.byte_add(0x34).read();
                                    let cond1 = (c_var1 == 0xb || c_var1 == 0xc || c_var1 == 0x17)
                                        && i_var5.byte_add(0x18).cast::<i16>().read_unaligned()
                                            == s_var7;
                                    let cond2 =
                                        i_var4.byte_add(0x6e).cast::<u16>().read_unaligned()
                                            == u_var10;
                                    if !((cond1 || cond2)
                                        && DAT_00732e54
                                            .byte_add(0x342)
                                            .cast::<i16>()
                                            .read_unaligned()
                                            != s_var7)
                                    {
                                        break 'cont;
                                    }
                                    if FUN_00429a20(u_var10 as u32) == 0 {
                                        break 'cont;
                                    }
                                    if (DAT_006d9d7a & 4) != 0
                                        && FUN_00429c40(u_var10 as u32) as i16 == 0
                                    {
                                        break 'cont;
                                    }
                                    if (DAT_006d9d7a & 1) != 0
                                        && FUN_0044c560(u_var10, 0xff) as i16 == 0
                                    {
                                        break 'cont;
                                    }
                                    if (DAT_006d9d7a & 0x20) == 0 {
                                        FUN_0042a1a0(DAT_006d9d50 as u32, u_var10 as u32);
                                    } else {
                                        let p = DAT_006d4988.byte_add(u_var10 as usize * 0x23);
                                        let a = p.byte_add(4).cast::<u32>().read_unaligned();
                                        let b = p.byte_add(8).cast::<u32>().read_unaligned();
                                        let c = p.byte_add(0xc).cast::<u32>().read_unaligned();
                                        let d = (&raw const DAT_006d9d7b)
                                            .cast::<u32>()
                                            .read_unaligned();
                                        let e = (&raw const DAT_006d9d7f)
                                            .cast::<u32>()
                                            .read_unaligned();
                                        let f = (&raw const DAT_006d9d83)
                                            .cast::<u32>()
                                            .read_unaligned();
                                        DAT_006d499c = FUN_00412810(
                                            a.wrapping_sub(d) as i32,
                                            b.wrapping_sub(e) as i32,
                                            c.wrapping_sub(f) as i32,
                                        );
                                    }
                                    let take = if (DAT_006d9d7a & 0x10) == 0 {
                                        true
                                    } else if (DAT_006d499c as i32) < 0x10001 {
                                        let r = DAT_006d4988
                                            .byte_add(DAT_006d9d50 as usize * 0x23 + 0x1f)
                                            .cast::<*mut u8>()
                                            .read_unaligned();
                                        FUN_00444040(u_var10 as u32, r.byte_add(0x87).read() as u32)
                                            != 0
                                    } else {
                                        false
                                    };
                                    if take && DAT_006d499c < local_4 {
                                        local_4 = DAT_006d499c;
                                        local_a = u_var10;
                                    }
                                }
                                u_var10 = u_var10.wrapping_add(1);
                                if !((u_var10 as i32) < DAT_006d4980) {
                                    break;
                                }
                            }
                        }
                    }
                }
                u_var8 = (s_var7 as u16).wrapping_add(1);
                if !((u_var8 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
    }
    local_a
}
