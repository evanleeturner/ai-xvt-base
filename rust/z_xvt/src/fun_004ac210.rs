// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4870: u8;
    static mut DAT_00655a80: *mut i8;
    static mut DAT_00655190: [*mut i8; 0];
    static mut DAT_0073b268: [*mut i8; 0];
    static mut DAT_0073b1b4: [*mut i8; 0];
    static mut DAT_0052b070: [i8; 0];
    static mut DAT_0070acf0: [i8; 0];
    static mut DAT_0070ad43: [i8; 0];
    static mut DAT_0070ad47: [i8; 0];
    static mut DAT_0070ad51: [i8; 0];
    static mut DAT_0070b210: [i8; 0];
    static mut DAT_0070b216: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f040(param_1: u8);
    fn FUN_0048f000(param_1: *mut i8);
    fn FUN_0048efd0(param_1: *mut i8);
}

fn sel(c: i8, a: u8, b: u8, d: u8, e: u8, f: u8) -> u8 {
    if c == 0 {
        a
    } else if c == 1 || c == 4 {
        b
    } else if c == 2 {
        d
    } else if c == 5 {
        e
    } else {
        f
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ac210(param_1: u16, param_2: u8) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read at the offsets the original uses.
    unsafe {
        DAT_006d4870 = 0;
        if 0x7fff < param_1 {
            if (param_2 & 1) == 0 {
                DAT_006d4870 = 0;
                return;
            }
            FUN_0048f040(0xfe);
            FUN_0048f040(0x43);
            let idx = param_1.wrapping_add(0x8000) as usize;
            FUN_0048f000((&raw mut DAT_00655190).cast::<*mut i8>().add(idx).read());
            return;
        }
        let i_var6: u32 = (param_1 as u32).wrapping_mul(0x23);
        let i_var8: *mut u8 = DAT_006d4988.byte_add(i_var6 as usize);
        let i_var11: i32 = i_var8.byte_add(0x1f).cast::<i32>().read_unaligned();
        if i_var11 == 0 {
            let u_var7: u16 = i_var8.byte_add(3).read() as u16;
            if u_var7 == 0 {
                FUN_0048efd0(DAT_00655a80);
                return;
            }
            FUN_0048f040(0xfe);
            let off = (DAT_006d4988.byte_add(0x16).byte_add(i_var6 as usize).read() as u32)
                .wrapping_mul(0x566);
            let c_var2: i8 = (&raw mut DAT_0070ad47)
                .cast::<i8>()
                .byte_add(off as usize)
                .read();
            FUN_0048f040(sel(c_var2, 0x51, 0x49, 0x45, 0x56, 0x4d));
            if (param_2 & 1) != 0 && 0x45 < u_var7 && u_var7 < 0x56 {
                FUN_0048f000(
                    (&raw mut DAT_0073b268)
                        .cast::<*mut i8>()
                        .add(u_var7 as usize)
                        .read(),
                );
            }
            let mut skip = false;
            let u_var10: u8;
            if (param_2 & 4) == 0 {
                if (param_2 & 3) != 3 {
                    skip = true;
                    u_var10 = 0;
                } else {
                    FUN_0048f040(0x3a);
                    u_var10 = 0x20;
                }
            } else {
                u_var10 = 0x3a;
            }
            if !skip {
                FUN_0048f040(u_var10);
            }
            if (param_2 & 2) == 0 {
                return;
            }
            FUN_0048f040(0xfe);
            FUN_0048f040(sel(c_var2, 0x52, 0x4a, 0x46, 0x56, 0x4e));
            let off2 = (DAT_006d4988.byte_add(0x16).byte_add(i_var6 as usize).read() as u32)
                .wrapping_mul(0x566);
            FUN_0048f000((&raw mut DAT_0070acf0).cast::<i8>().byte_add(off2 as usize));
            return;
        }
        let b_var1: u8 = i_var8.byte_add(3).read();
        if b_var1 == 0 {
            FUN_0048efd0(DAT_00655a80);
            return;
        }
        FUN_0048f040(0xfe);
        let p_tmp: *mut u8 = DAT_006d4988
            .byte_add(0x1f)
            .byte_add(i_var6 as usize)
            .cast::<*mut u8>()
            .read_unaligned();
        let c_var2: i8 = p_tmp.byte_add(0x86).cast::<i8>().read();
        FUN_0048f040(sel(c_var2, 0x51, 0x49, 0x45, 0x55, 0x4d));
        let pc_var3: *mut i8 = DAT_006d4988
            .byte_add(0x1f)
            .byte_add(i_var6 as usize)
            .cast::<*mut i8>()
            .read_unaligned();
        if pc_var3.read() != 0 {
            if (param_2 & 1) == 0 {
                return;
            }
            if 0x8e < b_var1 && b_var1 < 0x9c {
                FUN_0048f000(
                    (&raw mut DAT_0073b1b4)
                        .cast::<*mut i8>()
                        .add(b_var1 as usize)
                        .read(),
                );
                return;
            }
            if b_var1 < 0x46 {
                return;
            }
            if 0x54 < b_var1 {
                return;
            }
            FUN_0048f000(
                (&raw mut DAT_0073b268)
                    .cast::<*mut i8>()
                    .add(b_var1 as usize)
                    .read(),
            );
            return;
        }
        let puvar4: *mut u32 = pc_var3.byte_add(0xa7).cast::<*mut u32>().read_unaligned();
        if (param_2 & 1) != 0 {
            let o = ((puvar4.add(1).read() as u8) as u32).wrapping_mul(0xef);
            FUN_0048f000((&raw mut DAT_0052b070).cast::<i8>().byte_add(o as usize));
        }
        let mut skip = false;
        let u_var10: u8;
        if (param_2 & 4) == 0 {
            if (param_2 & 3) != 3 {
                skip = true;
                u_var10 = 0;
            } else {
                FUN_0048f040(0x3a);
                u_var10 = 0x20;
            }
        } else {
            u_var10 = 0x3a;
        }
        if !skip {
            FUN_0048f040(u_var10);
        }
        if (param_2 & 2) != 0 {
            FUN_0048f040(0xfe);
            let p_tmp2: *mut u8 = DAT_006d4988
                .byte_add(0x1f)
                .byte_add(i_var6 as usize)
                .cast::<*mut u8>()
                .read_unaligned();
            let c2: i8 = p_tmp2.byte_add(0x86).cast::<i8>().read();
            FUN_0048f040(sel(c2, 0x52, 0x4a, 0x46, 0x56, 0x4e));
            let mut u_var9: u32 =
                DAT_006d4988.byte_add(0x16).byte_add(i_var6 as usize).read() as u32;
            FUN_0048f000(
                (&raw mut DAT_0070acf0)
                    .cast::<i8>()
                    .byte_add(u_var9.wrapping_mul(0x566) as usize),
            );
            let i6 = u_var9.wrapping_mul(0x566) as usize;
            if (&raw mut DAT_0070b210).cast::<i8>().byte_add(i6).read() == 1
                || ((&raw mut DAT_0070b216).cast::<i8>().byte_add(i6).read() == 0
                    && (&raw mut DAT_0070ad43).cast::<i8>().byte_add(i6).read() == 1
                    && (&raw mut DAT_0070ad51).cast::<i8>().byte_add(i6).read() == 0)
            {
                u_var9 = 0;
            } else {
                u_var9 = puvar4.read();
            }
            if (u_var9 as u16) != 0 {
                FUN_0048f040(0x20);
                if 999 < (u_var9 as u16) {
                    u_var9 = 999;
                }
                if (u_var9 as u16) < 100 {
                    if (u_var9 as u16) < 10 {
                        FUN_0048f040((u_var9 as u8).wrapping_add(b'0'));
                        return;
                    }
                    FUN_0048f040((((u_var9 & 0xffff) / 10) as u8).wrapping_add(b'0'));
                    FUN_0048f040((((u_var9 & 0xffff) % 10) as u8).wrapping_add(b'0'));
                    return;
                }
                let u_var5 = (u_var9 & 0xffff) % 100;
                FUN_0048f040((((u_var9 & 0xffff) / 100) as u8).wrapping_add(b'0'));
                FUN_0048f040(((u_var5 / 10) as u8).wrapping_add(b'0'));
                FUN_0048f040(((u_var5 % 10) as u8).wrapping_add(b'0'));
                return;
            }
        }
    }
}
