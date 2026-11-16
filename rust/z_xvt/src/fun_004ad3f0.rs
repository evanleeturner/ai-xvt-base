// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00655184: u16;
    static mut DAT_006da9ec: u32;
    static mut DAT_006da538: u32;
    static mut DAT_006db594: u32;
    static mut DAT_00525518: u32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: u32;
    static mut DAT_006d4a4e: u8;
    static mut DAT_0071f2e2: u8;
    static mut DAT_0071e524: u8;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b757: [u8; 0];
    static mut DAT_0071b758: [u8; 0];
    static mut DAT_00656cae: [u16; 0];
    static mut DAT_00656ca6: [u16; 0];
    static mut DAT_00655b68: [u8; 0];
    static mut DAT_00655b64: [u8; 0];
    static mut DAT_00655b66: [u8; 0];
    static mut DAT_00655b34: [u8; 0];
    static mut DAT_00655b36: [u8; 0];
    static mut DAT_00655b38: [u8; 0];
    static mut DAT_00655b3a: [u8; 0];
    static mut DAT_0071f2f0: [u32; 0];
    static mut DAT_00703c80: *const c_void;
    static mut DAT_0070a184: *const c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461f20(a: u32, b: u32, c: u32, d: u32);
    fn FUN_0048ee20(a: i8);
    fn FUN_0048ed00(a: i16, b: i16, c: i16, d: i16);
    fn FUN_0048ece0(a: i16, b: i16);
    fn FUN_0048eda0(a: i32);
    fn FUN_0048ed70(a: u32);
    fn FUN_004a7aa0(a: u32) -> u32;
    fn FUN_0040ccc0(a: u32, b: u32, c: u32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address in the game's own memory.
    unsafe { (a as *const u8).read() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address in the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address in the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ad3f0(param_1: u32, param_2: u16, param_3: u32) {
    // SAFETY: all globals, callees and computed addresses belong to the game;
    // they are read and written exactly where the original does.
    unsafe {
        let b730 = (&raw mut DAT_0071b730).cast::<u8>();
        let b757 = (&raw mut DAT_0071b757).cast::<u8>();
        let b758 = (&raw mut DAT_0071b758).cast::<u8>();
        let a6cae = (&raw mut DAT_00656cae).cast::<u16>();
        let a6ca6 = (&raw mut DAT_00656ca6).cast::<u16>();
        let c68 = (&raw mut DAT_00655b68).cast::<u8>();
        let c64 = (&raw mut DAT_00655b64).cast::<u8>();
        let c66 = (&raw mut DAT_00655b66).cast::<u8>();
        let c34 = (&raw mut DAT_00655b34).cast::<u8>();
        let c36 = (&raw mut DAT_00655b36).cast::<u8>();
        let c38 = (&raw mut DAT_00655b38).cast::<u8>();
        let c3a = (&raw mut DAT_00655b3a).cast::<u8>();
        let f2f0 = (&raw mut DAT_0071f2f0).cast::<u32>();

        if DAT_00655184 != 0 {
            FUN_00461f20(
                DAT_006da9ec,
                DAT_006da538,
                DAT_006db594,
                DAT_00525518.wrapping_mul(DAT_006da538),
            );
            let i_var4 = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
            let t0 = b730.add(i_var4 as usize).cast::<u32>().read_unaligned();
            let i_var3 = t0.wrapping_mul(0x23);
            let p1 = i_var3.wrapping_add(0x1f).wrapping_add(DAT_006d4988);
            let t1 = r32(p1);
            let mut i_var3 = r32(t1.wrapping_add(0xa7));
            let u_var10: u16;
            if r8(i_var3.wrapping_add(0x228)) == 0 {
                u_var10 = 0;
            } else {
                u_var10 = r8(i_var3
                    .wrapping_add(0x346)
                    .wrapping_add((param_1 & 0xffff).wrapping_mul(4)))
                    as u16;
            }
            let idx = (DAT_00655184 as u32).wrapping_add(param_2 as u32);
            a6cae.add(idx as usize).write_unaligned(u_var10);
            if u_var10 != 0 {
                let t1 = r32(p1);
                i_var3 = r32(t1.wrapping_add(0xa7));
            }
            let c_var8: i8;
            if u_var10 == 0 || (r8(i_var3.wrapping_add(0x1df)) & 8) == 0 {
                c_var8 = 0;
            } else if *b758.add(i_var4 as usize) == 0 {
                c_var8 = 1;
            } else if (param_3 & 0xffff) == *b757.add(i_var4 as usize) as u32 {
                let b_var2 = r8((*b757.add(i_var4 as usize) as u32)
                    .wrapping_add(0x22b)
                    .wrapping_add(i_var3));
                if (b_var2 & 0x7f) == 3 {
                    c_var8 = 2;
                } else {
                    c_var8 = (((param_2 & 1) == (b_var2 >> 7) as u16) as i8) + 1;
                }
            } else {
                c_var8 = 1;
            }
            if u_var10 == 0 {
                FUN_0048ee20(2);
                let s_var9 = ((DAT_0071f2e2 as u32)
                    .wrapping_add(1)
                    .wrapping_mul(param_2 as u32)) as i16;
                FUN_0048ed00(
                    s_var9.wrapping_add(2),
                    2,
                    s_var9.wrapping_add(DAT_0071f2e2 as i16).wrapping_add(3),
                    (DAT_006d4a4e as u32).wrapping_add(3) as i16,
                );
                FUN_0048eda0(0);
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
                f();
            } else {
                FUN_0048ee20(2);
                FUN_0048ed00(0, 0, DAT_006da538 as i16, DAT_006db594 as i16);
                FUN_0048ece0(
                    ((DAT_0071f2e2 as u32)
                        .wrapping_add(1)
                        .wrapping_mul(param_2 as u32)
                        .wrapping_add(2)) as i16,
                    2,
                );
                FUN_0048eda0(0);
                let u_var11: u32 = if c_var8 == 2 { 0x52 } else { 0x4a };
                FUN_0048ed70(u_var11);
                DAT_0071e524 = 0;
                let o = (((DAT_00655184 as u32).wrapping_add(param_2 as u32)).wrapping_mul(0xc))
                    as usize;
                if c68.add(o).cast::<i16>().read_unaligned() != 0 {
                    FUN_0048ee20(0);
                    let t0 = b730
                        .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                        .cast::<u32>()
                        .read_unaligned();
                    let u_var11 = FUN_004a7aa0(
                        ((DAT_006d4988 & 0xffffff00)
                            | r8(t0
                                .wrapping_mul(0x23)
                                .wrapping_add(3)
                                .wrapping_add(DAT_006d4988)) as u32)
                            & 0xffff00ff,
                    );
                    let u_var6 = FUN_004a7aa0(0xc);
                    if (u_var11 as i16) == (u_var6 as i16) {
                        FUN_0040ccc0(u_var10 as u32, 2, 1);
                    } else {
                        FUN_0040ccc0(u_var10 as u32, 1, 1);
                    }
                }
            }
            FUN_00461f20(0, 0x140, 200, 0);
            return;
        }

        let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let t0 = b730.add(off).cast::<u32>().read_unaligned();
        let t1 = r32(t0
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988));
        let mut i_var3 = r32(t1.wrapping_add(0xa7));
        let mut u_var10: u16;
        if r8(i_var3.wrapping_add(0x228)) == 0 {
            u_var10 = 0;
        } else {
            u_var10 = r8(i_var3
                .wrapping_add(0x346)
                .wrapping_add((param_1 & 0xffff).wrapping_mul(4))) as u16;
        }
        let u_var11 = param_2 as u32;
        if a6cae.add(u_var11 as usize).read_unaligned() != u_var10 {
            a6cae.add(u_var11 as usize).write_unaligned(u_var10);
            FUN_0048ee20(2);
            FUN_0048ed00(0, 0, DAT_006da538 as i16, DAT_006db594 as i16);
            let o = (((DAT_00655184 as u32).wrapping_add(u_var11)).wrapping_mul(0xc)) as usize;
            FUN_0048ece0(
                c64.add(o).cast::<i16>().read_unaligned(),
                c66.add(o).cast::<i16>().read_unaligned(),
            );
            FUN_0048eda0(0);
            FUN_0048ed70(0x4a);
            DAT_0071e524 = 0;
            let t0 = b730
                .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                .cast::<u32>()
                .read_unaligned();
            let u_var6 = FUN_004a7aa0(
                ((DAT_006d4988 & 0xffffff00)
                    | r8(t0
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_006d4988)) as u32)
                    & 0xffff00ff,
            );
            let u_var7 = FUN_004a7aa0(0xc);
            let u_var6: u32 = if (u_var6 as i16) == (u_var7 as i16) {
                2
            } else {
                1
            };
            FUN_0040ccc0(u_var10 as u32, u_var6, 1);
        }
        'lab: {
            if u_var10 != 0 {
                let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
                let t0 = b730.add(off).cast::<u32>().read_unaligned();
                let t1 = r32(t0
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_006d4988));
                i_var3 = r32(t1.wrapping_add(0xa7));
                if (r8(i_var3.wrapping_add(0x1df)) & 8) != 0 {
                    if *b758.add(off) == 0 {
                        u_var10 = 1;
                    } else if (param_3 & 0xffff) == *b757.add(off) as u32 {
                        let b_var2 = r8((*b757.add(off) as u32)
                            .wrapping_add(0x22b)
                            .wrapping_add(i_var3));
                        if (b_var2 & 0x7f) == 3 {
                            u_var10 = 2;
                        } else {
                            u_var10 = ((param_2 & 1) == (b_var2 >> 7) as u16) as u16 + 1;
                        }
                    } else {
                        u_var10 = 1;
                    }
                    break 'lab;
                }
            }
            u_var10 = 0;
        }
        let i_var3 = b730
            .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        let b_var5: bool;
        if (i_var3 as i32) == -1 {
            b_var5 = false;
        } else {
            let c = r8(i_var3
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988)) as i8;
            if c != 1 && c != 2 && c != 3 && c != 0xe && c != 4 {
                b_var5 = false;
            } else {
                b_var5 = true;
            }
        }
        if b_var5 && u_var10 == 2 {
            u_var10 = 4;
        }
        if a6ca6.add(u_var11 as usize).read_unaligned() != u_var10 {
            a6ca6.add(u_var11 as usize).write_unaligned(u_var10);
            let o = u_var11.wrapping_mul(0xc) as usize;
            let f: unsafe extern "C" fn(u32, i16, i16, i16, i32) =
                core::mem::transmute(DAT_0070a184);
            let k = (c38.add(o).cast::<u16>().read_unaligned() as u32).wrapping_add(u_var10 as u32);
            f(
                f2f0.add(k as usize).read_unaligned(),
                c34.add(o).cast::<i16>().read_unaligned(),
                c36.add(o).cast::<i16>().read_unaligned(),
                c3a.add(o).cast::<i16>().read_unaligned(),
                0,
            );
        }
        let _ = r16;
    }
}
