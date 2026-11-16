// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e967c: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
    static mut DAT_009a8c00: u32;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(param_1: u32) -> u32;
    fn FUN_00485000(param_1: i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485660(param_1: i32, param_2: u32) -> u32 {
    let mut local_14: u32 = 0xffff_ffff;
    let mut local_c: u32 = 0xffff;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read at the offsets and order the original uses.
    unsafe {
        let mut u_var4: u16 = DAT_00a07cec;
        if (u_var4 as i16 as i32) < DAT_009a1fe0 {
            loop {
                let u_var5: u32 = u_var4 as i16 as i32 as u32;
                let i_var2: *mut u8 = DAT_009a1fe8.byte_add(u_var5.wrapping_mul(0x23) as usize);
                if i_var2.byte_add(3).read() != 0 {
                    let tmp32 = (&raw mut DAT_009e9670)
                        .cast::<u8>()
                        .byte_add((param_1 as u32).wrapping_mul(0x5bd) as usize)
                        .cast::<u32>()
                        .read_unaligned();
                    if tmp32 != u_var5 && param_2 != u_var5 {
                        let u_var3 = (&raw mut DAT_009e967c)
                            .cast::<u8>()
                            .byte_add((param_1 as u32).wrapping_mul(0x5bd) as usize)
                            .cast::<u16>()
                            .read_unaligned() as u32;
                        let p_ent = i_var2.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                        if p_ent.byte_add(0x89).read() as u32 != u_var3 {
                            let idx = DAT_009a1fe8
                                .byte_add(
                                    (u_var4 as u32).wrapping_mul(0x23).wrapping_add(0x16) as usize
                                )
                                .read() as u32;
                            let b = (&raw mut DAT_009d8c88)
                                .cast::<u8>()
                                .byte_add(idx.wrapping_mul(0x566) as usize)
                                .read() as u32;
                            let b6 = if b == u_var3 {
                                false
                            } else {
                                (&raw mut DAT_009fd458)
                                    .cast::<u8>()
                                    .byte_add(b.wrapping_add(u_var3.wrapping_mul(0x1e5)) as usize)
                                    .read()
                                    == 0
                            };
                            if b6 {
                                let c = i_var2.byte_add(2).read();
                                if c != 0x0d && c != 4 && c != 3 && c != 5 {
                                    let r = FUN_00484f80(u_var4 as u32);
                                    if (r as u8) == 0 {
                                        let p_ent2 = DAT_009a1fe8
                                            .byte_add(0x1f)
                                            .byte_add(u_var5.wrapping_mul(0x23) as usize)
                                            .cast::<*mut u8>()
                                            .read_unaligned();
                                        let i2 = p_ent2
                                            .byte_add(0xa9)
                                            .cast::<*mut u8>()
                                            .read_unaligned();
                                        let t16 = i2.byte_add(0x1e5).cast::<u16>().read_unaligned();
                                        if t16 != 0 {
                                            let c1 = i2.byte_add(7).read();
                                            if c1 == 0 || c1 == 6 {
                                                FUN_00485000(param_1, u_var5);
                                                if DAT_009a8c00 < local_14 {
                                                    local_14 = DAT_009a8c00;
                                                    local_c = u_var5;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i16 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        let mut u_var4: u16 = DAT_00a07ce8 as u16;
        if (u_var4 as i16 as i32) < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32 {
            loop {
                let u_var5: u32 = u_var4 as i16 as i32 as u32;
                let i_var2: *mut u8 = DAT_009a1fe8.byte_add(u_var5.wrapping_mul(0x23) as usize);
                if i_var2.byte_add(3).read() != 0
                    && i_var2.byte_add(2).read() == 8
                    && param_2 != u_var5
                {
                    let t16 = i_var2.byte_add(0x17).cast::<u16>().read_unaligned();
                    if t16 != 0 {
                        let u_var3 = (&raw mut DAT_009e967c)
                            .cast::<u8>()
                            .byte_add((param_1 as u32).wrapping_mul(0x5bd) as usize)
                            .cast::<u16>()
                            .read_unaligned() as u32;
                        let idx =
                            DAT_009a1fe8
                                .byte_add(
                                    (u_var4 as u32).wrapping_mul(0x23).wrapping_add(0x16) as usize
                                )
                                .read() as u32;
                        let b = (&raw mut DAT_009d8c88)
                            .cast::<u8>()
                            .byte_add(idx.wrapping_mul(0x566) as usize)
                            .read() as u32;
                        let b6 = if u_var3 == b {
                            false
                        } else {
                            (&raw mut DAT_009fd458)
                                .cast::<u8>()
                                .byte_add(b.wrapping_add(u_var3.wrapping_mul(0x1e5)) as usize)
                                .read()
                                == 0
                        };
                        if b6 {
                            FUN_00485000(param_1, u_var5);
                            if DAT_009a8c00 < local_14 {
                                local_14 = DAT_009a8c00;
                                local_c = u_var5;
                            }
                        }
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i16 as i32)
                    < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32)
                {
                    break;
                }
            }
        }
    }
    local_c
}
