// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1ed0: u8;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a08380: [*mut i8; 0];
    static mut DAT_00a08c70: *mut i8;
    static mut DAT_009d8c87: [i8; 0];
    static mut DAT_009d8c30: [i8; 0];
    static mut DAT_00a60698: [*mut i8; 0];
    static mut DAT_00a605e4: [*mut i8; 0];
    static mut DAT_0051c560: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9860(param_1: i8);
    fn FUN_004a9820(param_1: *mut i8);
    fn FUN_004a97f0(param_1: *mut i8);
    fn FUN_0043c740(param_1: u32, param_2: *mut u32) -> u32;
}

fn pick(c: i8, a: u8, b: u8, d: u8, e: u8, other: u8) -> u8 {
    if c == 0 {
        a
    } else if c == 1 || c == 4 {
        b
    } else if c == 2 {
        d
    } else if c == 5 {
        e
    } else {
        other
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043c290(param_1: u16, param_2: u8) {
    // SAFETY: all globals and callees are the game's own; pointers are
    // dereferenced exactly where the original does.
    unsafe {
        DAT_009a1ed0 = 0;
        if 0x7fff < param_1 {
            if (param_2 & 1) == 0 {
                DAT_009a1ed0 = 0;
                return;
            }
            FUN_004a9860(0xfeu8 as i8);
            FUN_004a9860(0x43);
            let idx = param_1.wrapping_add(0x8000) as usize;
            FUN_004a9820((&raw mut DAT_00a08380).cast::<*mut i8>().add(idx).read());
            return;
        }
        let i_var5: u32 = (param_1 as u32).wrapping_mul(0x23);
        let base = DAT_009a1fe8;
        let i_var6 = base.add(i_var5 as usize);
        let tmp: u32 = i_var6.add(0x1f).cast::<u32>().read_unaligned();
        if tmp == 0 {
            let u_var8: u16 = i_var6.add(3).read() as u16;
            if u_var8 == 0 {
                FUN_004a97f0(DAT_00a08c70);
                return;
            }
            FUN_004a9860(0xfeu8 as i8);
            let slot = base.add(0x16 + i_var5 as usize).read() as u32;
            let c_var2: i8 = (&raw mut DAT_009d8c87)
                .cast::<i8>()
                .add(slot.wrapping_mul(0x566) as usize)
                .read();
            FUN_004a9860(pick(c_var2, 0x51, 0x49, 0x45, 0x56, 0x4d) as i8);
            if (param_2 & 1) != 0 && 0x45 < u_var8 && u_var8 < 0x56 {
                FUN_004a9820(
                    (&raw mut DAT_00a60698)
                        .cast::<*mut i8>()
                        .add(u_var8 as usize)
                        .read(),
                );
            }
            if (param_2 & 4) == 0 {
                if (param_2 & 3) == 3 {
                    FUN_004a9860(0x3a);
                    FUN_004a9860(0x20);
                }
            } else {
                FUN_004a9860(0x3a);
            }
            if (param_2 & 2) == 0 {
                return;
            }
            FUN_004a9860(0xfeu8 as i8);
            FUN_004a9860(pick(c_var2, 0x52, 0x4a, 0x46, 0x56, 0x4e) as i8);
            let slot = base.add(0x16 + i_var5 as usize).read() as u32;
            FUN_004a9820(
                (&raw mut DAT_009d8c30)
                    .cast::<i8>()
                    .add(slot.wrapping_mul(0x566) as usize),
            );
            return;
        }
        let b_var1: u8 = i_var6.add(3).read();
        if b_var1 == 0 {
            FUN_004a97f0(DAT_00a08c70);
            return;
        }
        FUN_004a9860(0xfeu8 as i8);
        let mut pc: *mut i8 = base
            .add(0x1f + i_var5 as usize)
            .cast::<*mut i8>()
            .read_unaligned();
        let c_var2: i8 = pc.byte_add(0x88).read();
        FUN_004a9860(pick(c_var2, 0x51, 0x49, 0x45, 0x55, 0x4d) as i8);
        pc = base
            .add(0x1f + i_var5 as usize)
            .cast::<*mut i8>()
            .read_unaligned();
        if pc.read() != 0 {
            if (param_2 & 1) == 0 {
                return;
            }
            if 0x8e < b_var1 && b_var1 < 0x9c {
                FUN_004a9820(
                    (&raw mut DAT_00a605e4)
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
            FUN_004a9820(
                (&raw mut DAT_00a60698)
                    .cast::<*mut i8>()
                    .add(b_var1 as usize)
                    .read(),
            );
            return;
        }
        let puvar4: *mut u32 = pc.byte_add(0xa9).cast::<*mut u32>().read_unaligned();
        if (param_2 & 1) != 0 {
            let v = puvar4.add(1).cast::<u8>().read() as u32;
            FUN_004a9820(
                (&raw mut DAT_0051c560)
                    .cast::<i8>()
                    .add(v.wrapping_mul(0x101) as usize),
            );
        }
        if (param_2 & 4) == 0 {
            if (param_2 & 3) == 3 {
                FUN_004a9860(0x3a);
                FUN_004a9860(0x20);
            }
        } else {
            FUN_004a9860(0x3a);
        }
        if (param_2 & 2) != 0 {
            FUN_004a9860(0xfeu8 as i8);
            pc = base
                .add(0x1f + i_var5 as usize)
                .cast::<*mut i8>()
                .read_unaligned();
            let c_var2: i8 = pc.byte_add(0x88).read();
            FUN_004a9860(pick(c_var2, 0x52, 0x4a, 0x46, 0x56, 0x4e) as i8);
            let u_var9: u32 = base.add(0x16 + i_var5 as usize).read() as u32;
            FUN_004a9820(
                (&raw mut DAT_009d8c30)
                    .cast::<i8>()
                    .add(u_var9.wrapping_mul(0x566) as usize),
            );
            let u_var7 = FUN_0043c740(u_var9, puvar4);
            let mut u_var8 = u_var7 as u16;
            if u_var8 != 0 {
                FUN_004a9860(0x20);
                if 999 < u_var8 {
                    u_var8 = 999;
                }
                if u_var8 < 100 {
                    if u_var8 < 10 {
                        FUN_004a9860((u_var8 as i8).wrapping_add(b'0' as i8));
                        return;
                    }
                    FUN_004a9860(((u_var8 / 10) as i8).wrapping_add(b'0' as i8));
                    FUN_004a9860(((u_var8 % 10) as i8).wrapping_add(b'0' as i8));
                    return;
                }
                FUN_004a9860(((u_var8 / 100) as i8).wrapping_add(b'0' as i8));
                FUN_004a9860((((u_var8 % 100) / 10) as i8).wrapping_add(b'0' as i8));
                FUN_004a9860((((u_var8 % 100) % 10) as i8).wrapping_add(b'0' as i8));
                return;
            }
        }
    }
}
