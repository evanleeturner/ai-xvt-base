// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006db7ac: u32;
    static mut DAT_006db7b0: u32;
    static mut DAT_006db7b4: u32;
    static mut DAT_006d9980: i16;
    static mut DAT_00732e8c: i16;
    static mut DAT_00732e50: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071eae8: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f450: i32;
    static mut DAT_00732f3c: u32;
    static mut DAT_006da538: u32;
    static mut DAT_006db580: u32;
    static mut DAT_006d4870: u8;
    static mut DAT_006d4a4e: u8;
    static mut DAT_005254c4: u8;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071bb1b: u8;
    static mut DAT_0071bb1f: u8;
    static mut DAT_0071bb23: u8;
    static mut DAT_0071bb27: u8;
    static mut DAT_0070ad47: u8;
    static mut DAT_006fedd4: *const c_void;
    static mut DAT_006d9984: *const c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ca90(a: u32, b: u32) -> i32;
    fn FUN_0040cad0(a: u32, b: u32) -> i32;
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_00421c70(a: u32, b: u32);
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0040c970(a: u32, b: u32, c: u32);
    fn FUN_0048eda0(a: u32);
    fn FUN_0048ee20(a: i8);
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00483540(a: i16, b: i32, c: i32);
    fn FUN_00413ca0(a: i32, b: i32) -> u64;
    fn FUN_004428e0(a: u32) -> u32;
    fn FUN_004ac210(a: u16, b: i32);
    fn FUN_004b4060(a: *mut u8) -> u64;
    fn FUN_0048ece0(a: i16, b: i16);
    fn FUN_0048f140(a: *mut u8);
    fn FUN_0042a140(a: u32, b: u32);
    fn FUN_0040ccc0(a: u16, b: i32, c: i32);
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const i32).read_unaligned() }
}

unsafe fn r16(a: i32) -> i16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const i16).read_unaligned() }
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const u8).read() }
}

unsafe fn tab(p: *mut u8) -> i32 {
    // SAFETY: reads the game's own table index global.
    let idx = unsafe { DAT_0071ecf4 } as u32;
    (p as usize as i32).wrapping_add(idx.wrapping_mul(0x5bd) as i32)
}

unsafe fn zoff() -> i32 {
    // SAFETY: reads the game's own table memory.
    unsafe {
        let tmp = r32(tab(&raw mut DAT_0071bb23));
        0xffff0000u32.wrapping_sub(tmp as u32) as i32
    }
}

unsafe fn call_draw(a: i32, b: i32, c: i32, d: i32, e: u32) {
    // SAFETY: the pointer is the game's draw callback with this signature.
    unsafe {
        let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) = core::mem::transmute(DAT_006fedd4);
        f(a, b, c, d, e);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00434990(param_1: u32) {
    // SAFETY: all globals, callees and addresses are the game's own memory;
    // reads and writes happen in the original's order.
    unsafe {
        let mut u_var15 = DAT_006db7b4;
        let mut u_var7 = DAT_006db7b0;
        let mut u_var14 = DAT_006db7ac;
        let i_var5 = param_1.wrapping_mul(0x23) as i32;
        let base = DAT_006d4988;
        let c_var1 = r8(base.wrapping_add(2).wrapping_add(i_var5));
        if c_var1 == 13 || c_var1 == 11 || (DAT_006db7b4 as i32) <= 0 {
            return;
        }
        let mut i_var8 = FUN_0040ca90(u_var14, u_var15);
        let mut i_var9 = FUN_0040cad0(u_var7, u_var15);
        i_var8 = i_var8.wrapping_add(DAT_006d9980 as i32);
        i_var9 = i_var9.wrapping_add(DAT_00732e8c as i32);

        let us = r16(tab(&raw mut DAT_0071b74a)) as u16;
        if us as u32 == param_1 {
            let mut u_var16: u32 = 0xffff;
            if (param_1 as i32) < DAT_0071eae8 {
                let ent = r32(base.wrapping_add(0x1f).wrapping_add(i_var5));
                let tmp = r32(ent.wrapping_add(0xa7));
                let i_var10 = FUN_0042b150(tmp);
                u_var16 = r16(i_var10.wrapping_add(0x18)) as u16 as u32;
            } else {
                let mut i_var10 = r32(base.wrapping_add(0x1f).wrapping_add(i_var5));
                if i_var10 != 0 {
                    i_var10 = r32(i_var10.wrapping_add(0xa7));
                    if i_var10 != 0 {
                        u_var16 = r16(i_var10.wrapping_add(4)) as u16 as u32;
                    }
                }
            }
            if u_var16 != 0xffff {
                FUN_00421c70(
                    u_var16,
                    r8(base.wrapping_add(0x16).wrapping_add(i_var5)) as u32,
                );
                let t = r32(tab(&raw mut DAT_0071bb1b));
                DAT_00732a2c = DAT_00732a2c.wrapping_sub(t);
                let t = r32(tab(&raw mut DAT_0071bb1f));
                DAT_00732a28 = DAT_00732a28.wrapping_sub(t);
                let t = r32(tab(&raw mut DAT_0071bb23));
                DAT_00732a24 = DAT_00732a24.wrapping_sub(t);
                DAT_006db7ac = FUN_0040ca00(DAT_00732a2c, DAT_00732a28, DAT_00732a24);
                DAT_006db7b0 = FUN_0040ca30(DAT_00732a2c, DAT_00732a28, DAT_00732a24);
                DAT_006db7b4 = FUN_0040ca60(DAT_00732a2c, DAT_00732a28, DAT_00732a24);
                if (DAT_006db7b4 as i32) < 1 {
                    FUN_0040c970(u_var14, u_var7, u_var15);
                }
                let i_var10 = i_var8;
                let i_var13 = i_var9;
                let i_var11 =
                    FUN_0040cad0(DAT_006db7b0, DAT_006db7b4).wrapping_add(DAT_00732e8c as i32);
                let i_var12 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                call_draw(
                    i_var12.wrapping_add(DAT_006d9980 as i32),
                    i_var11,
                    i_var10,
                    i_var13,
                    0x36,
                );
            }
        }

        let i_var10 = r32(base.wrapping_add(i_var5).wrapping_add(0x1f));
        let u_var2 = if i_var10 == 0 {
            let idx = (r8(base.wrapping_add(i_var5).wrapping_add(0x16)) as u32).wrapping_mul(0x566);
            r8((&raw mut DAT_0070ad47 as usize as i32).wrapping_add(idx as i32))
        } else {
            r8(i_var10.wrapping_add(0x86))
        };
        let u_var16: u32 = match u_var2 {
            0 => 0x3f,
            1 | 4 => 0x37,
            2 => 0x33,
            _ => 0x3b,
        };
        FUN_0048eda0(u_var16);
        FUN_0048ee20(0);

        let mut b_var6 = false;
        if (param_1 as i32) < DAT_0071eae8 {
            b_var6 = true;
        } else {
            let i10 = r32(base.wrapping_add(0x1f).wrapping_add(i_var5));
            if i10 == 0 {
                b_var6 = true;
            } else {
                let tmp = r32(i10.wrapping_add(0xa7));
                if tmp != 0 {
                    b_var6 = true;
                }
            }
        }
        if !b_var6 {
            return;
        }

        DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, zoff());
        DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, zoff());
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, zoff());
        if (DAT_006db7b4 as i32) < 1 {
            FUN_0040c970(u_var14, u_var7, u_var15);
        }
        let mut i_var10 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
        let i_var13 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
        i_var10 = i_var10.wrapping_add(DAT_006d9980 as i32);
        let i_var13 = i_var13.wrapping_add(DAT_00732e8c as i32);
        let last =
            ((((DAT_006d9980 as u16) >> 8) as i8 as i32 as u32) << 8) | (DAT_00732e50 as u32);
        call_draw(i_var10, i_var13, i_var8, i_var9, last);

        let mut i_var11 = base.wrapping_add(i_var5);
        let pcv = r32(i_var11.wrapping_add(0x1f));
        if pcv != 0 && r8(pcv) == 0 {
            if r8(pcv.wrapping_add(0x90)) != 0 {
                let ss = r16(i_var11.wrapping_add(0x12));
                let ss2 = r16(i_var11.wrapping_add(0x10));
                FUN_00483410(ss, ss2, i_var11);
                let ss = r16(DAT_006d4988.wrapping_add(i_var5).wrapping_add(0x14));
                FUN_00483540(ss, 0, DAT_006d4988.wrapping_add(i_var5));
            }
            i_var11 = r32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let ss = r16(i_var11.wrapping_add(0x8a));
            let u18 = FUN_00413ca0(0x100, ss as i32);
            DAT_0072f44c = DAT_0072f44c.wrapping_add(u18 as i32);
            let tmp = r32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let ss = r16(tmp.wrapping_add(0x8c));
            let u18 = FUN_00413ca0(0x100, ss as i32);
            DAT_0072f450 = DAT_0072f450.wrapping_add(u18 as i32);
            i_var11 = r32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
            let us = r16(i_var11.wrapping_add(0x79)) as u16;
            if us < 0x400 {
                let ss = r16(i_var11.wrapping_add(0x8a));
                let u18 = FUN_00413ca0(((us as u32) << 5) as i32, ss as i32);
                DAT_0072f44c = DAT_0072f44c.wrapping_add(u18 as i32);
                i_var11 = r32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
                let us = r16(i_var11.wrapping_add(0x79)) as u16;
                let ss = r16(i_var11.wrapping_add(0x8c));
                let u18 = FUN_00413ca0(((us as u32) << 5) as i32, ss as i32);
                i_var11 = u18 as i32;
            } else {
                let ss = r16(i_var11.wrapping_add(0x8a));
                DAT_0072f44c = DAT_0072f44c.wrapping_add(ss as i32);
                let tmp = r32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var5));
                let ss = r16(tmp.wrapping_add(0x8c));
                i_var11 = ss as i32;
            }
            let u16_ = DAT_006db7b4;
            u_var7 = DAT_006db7b0;
            u_var14 = DAT_006db7ac;
            DAT_0072f450 = DAT_0072f450.wrapping_add(i_var11);
            DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, zoff());
            DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, zoff());
            DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, zoff());
            if (DAT_006db7b4 as i32) < 1 {
                FUN_0040c970(u_var14, u_var7, u16_);
            }
            let u14 = DAT_00732e50 as u32;
            let i11 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4).wrapping_add(DAT_00732e8c as i32);
            let i12 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
            call_draw(
                i12.wrapping_add(DAT_006d9980 as i32),
                i11,
                i_var10,
                i_var13,
                u14,
            );
        }

        FUN_0048eda0(0x40);
        let local_10 = i_var8 as i16;
        let local_c = i_var9 as i16;
        u_var14 = FUN_004428e0(param_1);
        u_var15 = DAT_00732f3c.wrapping_mul(u_var14) / u_var15;
        if u_var15 < DAT_006da538 / 0x50 {
            u_var15 = DAT_006da538 / 0x50;
        }
        if (DAT_006da538 >> 1) < u_var15 {
            u_var15 = DAT_006da538 >> 1;
        }
        let i_var8 = u_var15.wrapping_add(4) as i32;
        let tmp = r32(DAT_006d4988.wrapping_add(i_var5).wrapping_add(0x1f));
        if tmp == 0 && r8(DAT_006d4988.wrapping_add(i_var5).wrapping_add(2)) == 8 {
            DAT_006d4870 = 0;
        } else {
            FUN_004ac210(param_1 as u16, 2);
        }
        if DAT_006d4870 != 0 {
            let i9 = ((local_c as i32 as u32)
                .wrapping_sub(DAT_006d4a4e as u32)
                .wrapping_sub((i_var8 / 2) as u32))
            .wrapping_sub(1) as i32;
            let u18 = FUN_004b4060(&raw mut DAT_006d4870);
            let half = ((u18 as u16) >> 1) as i32;
            FUN_0048ece0((local_10 as i32).wrapping_sub(half) as i16, i9 as i16);
            FUN_0048f140(&raw mut DAT_006d4870);
        }
        let c_var1 = r8(DAT_006d4988.wrapping_add(2).wrapping_add(i_var5));
        if c_var1 != 6 && c_var1 != 7 {
            FUN_0048eda0(0x40);
            let u_var3 = r16(tab(&raw mut DAT_0071bb27)) as u16;
            if u_var3 != 0xffff {
                let s17 = local_c.wrapping_add((i_var8 >> 1) as i16).wrapping_add(1);
                FUN_0042a140(u_var3 as u32, param_1);
                let u18 = FUN_004b4060(&raw mut DAT_005254c4);
                FUN_0048ece0(local_10.wrapping_sub(u18 as i16), s17);
                DAT_006db580 = DAT_006db580.wrapping_mul(0xa1);
                let mut u14 = DAT_006db580 >> 0x10;
                if 9999 < u14 {
                    u14 = 9999;
                }
                FUN_0040ccc0((u14 / 100) as u16, 2, 1);
                let f: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_006d9984);
                f(0x2e);
                let q = (u14 / 100) as i32;
                let v = ((q.wrapping_neg()) as i16)
                    .wrapping_mul(100)
                    .wrapping_add(u14 as i16);
                FUN_0040ccc0(v as u16, 2, 2);
            }
        }
    }
}
