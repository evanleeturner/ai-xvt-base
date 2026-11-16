// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4974: i32;
    static mut DAT_0071e6c4: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_00732f38: i32;
    static mut DAT_0073294c: i32;
    static mut DAT_0070a180: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_00732fe8: i32;
    static mut DAT_007329b0: i32;
    static mut DAT_0070acd8: i32;
    static mut DAT_007329b4: i32;
    static mut DAT_00703ad8: i32;
    static mut DAT_006ffbe0: i32;
    static mut DAT_007091a8: i32;
    static mut DAT_0070a17c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413ca0(param_3: i32, param_4: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

macro_rules! dv {
    ($l:lifetime, $n:expr, $d:expr) => {{
        let n: i32 = $n;
        let d: i32 = $d;
        if d == 0 || (n == i32::MIN && d == -1) {
            break $l;
        }
        n / d
    }};
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_0040fb70(param_1: i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        'fault: {
            let i_var1 = DAT_006d4974.wrapping_sub(DAT_0071e6c4);
            let i_var2 = DAT_006d4978.wrapping_sub(DAT_00732f38);
            let i_var12 = DAT_0073294c.wrapping_sub(DAT_0070a180);
            let i_var3 = DAT_006d4970.wrapping_sub(DAT_00732fe8);
            let mut i_var14 = DAT_0070a180.wrapping_sub(DAT_0071e6c4);
            let i_var4 = DAT_007329b0.wrapping_sub(DAT_0070acd8);
            let i_var5 = DAT_007329b4.wrapping_sub(DAT_00703ad8);
            let i_var6: i32;
            let i_var10: i32;
            let local_24: u32;
            let mut u_var8: u16;
            if param_1 < i_var14 {
                i_var6 = param_1.wrapping_sub(i_var14);
                local_24 = i_var12.wrapping_sub(i_var1) as u32;
                u_var8 = ((i_var6 as u32) >> 16) as u16;
                if -1 < (local_24 as i32) {
                    return (u_var8 as u32) << 16;
                }
                if i_var6 < (local_24 as i32) {
                    return (u_var8 as u32) << 16;
                }
                i_var10 = 0i32.wrapping_sub(param_1.wrapping_add(i_var14));
            } else if 0i32.wrapping_sub(i_var14) == param_1 || 0i32.wrapping_sub(param_1) < i_var14
            {
                i_var10 = 0i32.wrapping_sub(param_1.wrapping_add(i_var14));
                local_24 = 0;
                i_var6 = param_1.wrapping_sub(i_var14);
            } else {
                i_var10 = 0i32.wrapping_sub(param_1.wrapping_add(i_var14));
                local_24 = i_var12.wrapping_sub(i_var1) as u32;
                if (local_24 as i32) < 0 {
                    return local_24 & 0xffff0000;
                }
                if (local_24 as i32) <= i_var10 {
                    return local_24 & 0xffff0000;
                }
                i_var6 = param_1.wrapping_sub(i_var14);
            }
            i_var14 = DAT_0070acd8.wrapping_sub(DAT_00732f38);
            u_var8 = ((i_var6 as u32) >> 16) as u16;
            let local_18: i32;
            let mut local_20: i32;
            let i_var15: i32;
            if param_1 < i_var14 {
                local_18 = param_1.wrapping_sub(i_var14);
                i_var15 = i_var4.wrapping_sub(i_var2);
                if -1 < i_var15 {
                    return (u_var8 as u32) << 16;
                }
                if local_18 < i_var15 {
                    return (u_var8 as u32) << 16;
                }
                local_20 = 0i32.wrapping_sub(i_var14.wrapping_add(param_1));
            } else if 0i32.wrapping_sub(i_var14) == param_1 || 0i32.wrapping_sub(param_1) < i_var14
            {
                local_18 = param_1.wrapping_sub(i_var14);
                local_20 = 0i32.wrapping_sub(param_1.wrapping_add(i_var14));
                i_var15 = 0;
            } else {
                local_20 = 0i32.wrapping_sub(param_1.wrapping_add(i_var14));
                i_var15 = i_var4.wrapping_sub(i_var2);
                if i_var15 < 0 {
                    return (u_var8 as u32) << 16;
                }
                if i_var15 <= local_20 {
                    return (u_var8 as u32) << 16;
                }
                local_18 = param_1.wrapping_sub(i_var14);
            }
            i_var14 = DAT_00703ad8.wrapping_sub(DAT_00732fe8);
            let local_28: i32;
            let mut local_1c: i32;
            let i_var9: i32;
            if param_1 < i_var14 {
                local_28 = param_1.wrapping_sub(i_var14);
                i_var9 = i_var5.wrapping_sub(i_var3);
                if -1 < i_var9 {
                    return (u_var8 as u32) << 16;
                }
                if local_28 < i_var9 {
                    return (u_var8 as u32) << 16;
                }
                local_1c = 0i32.wrapping_sub(i_var14.wrapping_add(param_1));
            } else if 0i32.wrapping_sub(i_var14) == param_1 || 0i32.wrapping_sub(param_1) < i_var14
            {
                local_28 = param_1.wrapping_sub(i_var14);
                local_1c = 0i32.wrapping_sub(i_var14.wrapping_add(param_1));
                i_var9 = 0;
            } else {
                local_1c = 0i32.wrapping_sub(param_1.wrapping_add(i_var14));
                i_var9 = i_var5.wrapping_sub(i_var3);
                if i_var9 < 0 {
                    return (u_var8 as u32) << 16;
                }
                if i_var9 <= local_1c {
                    return (u_var8 as u32) << 16;
                }
                local_28 = param_1.wrapping_sub(i_var14);
            }
            local_20 = local_20.wrapping_shl(8);
            local_1c = local_1c.wrapping_shl(8);
            let mut u_var7: u32 = 0;
            let mut u_var11: u32;
            let mut u_var13: u32;
            if local_24 == 0 {
                let i_var14b = DAT_0073294c.wrapping_sub(DAT_006d4974);
                if param_1 < i_var14b {
                    let n = i_var6.wrapping_shl(8);
                    let d = i_var12.wrapping_sub(i_var1);
                    u_var11 = dv!('fault, n, d) as u32;
                    u_var13 = 0;
                } else if 0i32.wrapping_sub(i_var14b) == param_1
                    || 0i32.wrapping_sub(param_1) < i_var14b
                {
                    u_var11 = 0xff;
                    u_var13 = 0;
                } else {
                    let n = i_var10.wrapping_shl(8);
                    let d = i_var12.wrapping_sub(i_var1);
                    u_var11 = dv!('fault, n, d) as u32;
                    u_var13 = 0;
                }
            } else {
                let d = local_24 as i32;
                u_var13 = dv!('fault, i_var6.wrapping_shl(8), d) as u32;
                u_var7 = dv!('fault, i_var10.wrapping_shl(8), d) as u32;
                u_var11 = u_var7;
                if -1 < (local_24 as i32) {
                    u_var11 = u_var13;
                    u_var13 = u_var7;
                }
            }
            'lab: {
                if i_var15 == 0 {
                    let i_var12b = DAT_007329b0.wrapping_sub(DAT_006d4978);
                    if param_1 < i_var12b {
                        let d = i_var4.wrapping_sub(i_var2);
                        u_var7 = dv!('fault, local_18.wrapping_shl(8), d) as u32;
                    } else if 0i32.wrapping_sub(i_var12b) == param_1
                        || 0i32.wrapping_sub(param_1) < i_var12b
                    {
                        u_var7 = 0xff;
                    } else {
                        let d = i_var4.wrapping_sub(i_var2);
                        u_var7 = dv!('fault, local_20, d) as u32;
                    }
                    u_var8 = (u_var7 >> 16) as u16;
                    if (u_var11 as i32) < 0 {
                        return (u_var8 as u32) << 16;
                    }
                    if (u_var13 as i32) < 0 {
                        u_var13 = 0;
                    }
                    if (u_var7 as i32) < (u_var13 as i32) {
                        return (u_var8 as u32) << 16;
                    }
                } else {
                    let d = i_var15;
                    u_var7 = dv!('fault, local_18.wrapping_shl(8), d) as u32;
                    u_var8 = (u_var7 >> 16) as u16;
                    if -1 < i_var15 {
                        if (u_var7 as i32) < (u_var13 as i32) {
                            return (u_var8 as u32) << 16;
                        }
                        if (u_var7 as i32) < (u_var11 as i32) {
                            u_var11 = u_var7;
                        }
                        u_var7 = dv!('fault, local_20, d) as u32;
                        if (u_var11 as i32) < (u_var7 as i32) {
                            return u_var7 & 0xffff0000;
                        }
                        if (u_var13 as i32) < (u_var7 as i32) {
                            u_var13 = u_var7;
                        }
                        break 'lab;
                    }
                    if (u_var11 as i32) < (u_var7 as i32) {
                        return (u_var8 as u32) << 16;
                    }
                    if (u_var13 as i32) < (u_var7 as i32) {
                        u_var13 = u_var7;
                    }
                    u_var7 = dv!('fault, local_20, d) as u32;
                    if (u_var7 as i32) < (u_var13 as i32) {
                        return u_var7 & 0xffff0000;
                    }
                }
                if (u_var7 as i32) < (u_var11 as i32) {
                    u_var11 = u_var7;
                }
            }
            if i_var9 == 0 {
                let i_var4b = DAT_007329b4.wrapping_sub(DAT_006d4970);
                if param_1 < i_var4b {
                    let d = i_var5.wrapping_sub(i_var3);
                    u_var7 = dv!('fault, local_28.wrapping_shl(8), d) as u32;
                } else if 0i32.wrapping_sub(i_var4b) == param_1
                    || 0i32.wrapping_sub(param_1) < i_var4b
                {
                    u_var7 = 0xff;
                } else {
                    let d = i_var5.wrapping_sub(i_var3);
                    u_var7 = dv!('fault, local_1c, d) as u32;
                }
                u_var8 = (u_var7 >> 16) as u16;
                if (u_var11 as i32) < 0 {
                    return (u_var8 as u32) << 16;
                }
                if (u_var13 as i32) < 0 {
                    u_var13 = 0;
                }
                if (u_var7 as i32) < (u_var13 as i32) {
                    return (u_var8 as u32) << 16;
                }
            } else {
                let d = i_var9;
                u_var7 = dv!('fault, local_28.wrapping_shl(8), d) as u32;
                u_var8 = (u_var7 >> 16) as u16;
                if i_var9 < 0 {
                    if (u_var11 as i32) < (u_var7 as i32) {
                        return (u_var8 as u32) << 16;
                    }
                    if (u_var13 as i32) < (u_var7 as i32) {
                        u_var13 = u_var7;
                    }
                    u_var7 = dv!('fault, local_1c, d) as u32;
                    if (u_var7 as i32) < (u_var13 as i32) {
                        return u_var7 & 0xffff0000;
                    }
                } else {
                    if (u_var7 as i32) < (u_var13 as i32) {
                        return (u_var8 as u32) << 16;
                    }
                    if (u_var7 as i32) < (u_var11 as i32) {
                        u_var11 = u_var7;
                    }
                    u_var7 = dv!('fault, local_1c, d) as u32;
                    if (u_var11 as i32) < (u_var7 as i32) {
                        return u_var7 & 0xffff0000;
                    }
                    if (u_var13 as i32) < (u_var7 as i32) {
                        u_var13 = u_var7;
                    }
                }
            }
            if (u_var13 as i32) < 0x100 {
                u_var13 = (u_var13.wrapping_shl(7) as u16) as u32;
                DAT_006ffbe0 = i_var3;
                DAT_007091a8 = i_var2;
                DAT_0070a17c = i_var1;
                let mut u_var16 = FUN_00413ca0(u_var13 as i32, i_var1);
                DAT_0070a17c = u_var16;
                u_var16 = FUN_00413ca0(u_var13 as i32, DAT_007091a8);
                DAT_007091a8 = u_var16;
                u_var16 = FUN_00413ca0(u_var13 as i32, DAT_006ffbe0);
                DAT_006ffbe0 = u_var16;
                return (((((u_var16 as u32) >> 16) as u16) as u32) << 16) | 0xffff;
            }
            return u_var7 & 0xffff0000;
        }
        RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        0
    }
}
