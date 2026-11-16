// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: u32;
    static mut DAT_009d80c9: u8;
    static mut DAT_00bb7372: u8;
    static mut DAT_00bb737b: u8;
    static mut DAT_009ecc34: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_00a0b3d8: [u8; 0];
    static mut DAT_00a0b420: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9300(a: u32, b: u32) -> u32;
    fn FUN_004a9380(a: u32);
    fn FUN_0042d120(p: *mut u8, a: u32, b: u32, c: u32, d: u32, e: u32);
    fn FUN_00425d10() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042f5d0() {
    // SAFETY: globals and callees are the game's own; pointers computed
    // from game tables are read where the original reads them.
    unsafe {
        if DAT_00523410 == 0 && DAT_009d80c9 != 0 && DAT_00bb7372 != 0 && DAT_00bb737b != 0 {
            let mut u_var5: u32 = 0x7f;
            if DAT_00bb737b < 10 {
                u_var5 = (DAT_00bb737b as u32).wrapping_mul(0xd);
            }
            let mut i_var1: u32 = (&raw mut DAT_009e9670)
                .cast::<u8>()
                .byte_add(DAT_009ecc34.wrapping_mul(0x5bd) as usize)
                .cast::<u32>()
                .read_unaligned();
            if i_var1 == 0xffff_ffff {
                if FUN_004a9300(0x3f, 0x100) != 0 {
                    FUN_004a9380(0x3f);
                }
                if FUN_004a9300(0x40, 0x100) != 0 {
                    FUN_004a9380(0x40);
                }
                if FUN_004a9300(0x41, 0x100) != 0 {
                    FUN_004a9380(0x41);
                }
                if FUN_004a9300(0x42, 0x100) != 0 {
                    FUN_004a9380(0x42);
                    return;
                }
            } else {
                let p: u32 = (i_var1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8) as usize
                    as *const u32)
                    .read_unaligned();
                i_var1 = (p.wrapping_add(0xa9) as usize as *const u32).read_unaligned();
                let f19: u32 = (i_var1.wrapping_add(0x19) as usize as *const u32).read_unaligned();
                if f19 == 0 {
                    if FUN_004a9300(0x3f, 0x100) != 0 {
                        FUN_004a9380(0x3f);
                    }
                    if FUN_004a9300(0x40, 0x100) != 0 {
                        FUN_004a9380(0x40);
                    }
                    if FUN_004a9300(0x41, 0x100) != 0 {
                        FUN_004a9380(0x41);
                    }
                } else {
                    if FUN_004a9300(0x3f, 0x100) == 0 {
                        FUN_0042d120(
                            (&raw mut DAT_00a0b3d8).cast::<u8>(),
                            1,
                            1,
                            0x7d,
                            u_var5,
                            0x40,
                        );
                    }
                    if FUN_004a9300(0x40, 0x100) == 0 && FUN_004a9300(0x41, 0x100) == 0 {
                        if (FUN_00425d10() as u16) < 0x1000 {
                            let u_var3 = u_var5;
                            let u_var4 = FUN_00425d10();
                            FUN_0042d120(
                                (&raw mut DAT_00a0b3d8)
                                    .cast::<u8>()
                                    .byte_add(((u_var4 & 1).wrapping_mul(0x18)) as usize),
                                1,
                                1,
                                0x7d,
                                u_var3,
                                0x40,
                            );
                        }
                    }
                }
                let f1d: u32 = (i_var1.wrapping_add(0x1d) as usize as *const u32).read_unaligned();
                if f1d == 0 {
                    if FUN_004a9300(0x42, 0x100) != 0 {
                        FUN_004a9380(0x42);
                    }
                } else if FUN_004a9300(0x42, 0x100) == 0 {
                    FUN_0042d120(
                        (&raw mut DAT_00a0b420).cast::<u8>(),
                        1,
                        1,
                        0x7d,
                        u_var5 / 2,
                        0x40,
                    );
                    return;
                }
            }
        }
    }
}
