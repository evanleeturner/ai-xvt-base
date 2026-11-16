// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d8e25: [i8; 0];
    static mut DAT_009d8e29: [i8; 0];
    static mut DAT_009d8c80: [i8; 0];
    static mut DAT_009a2196: [i8; 0];
    static mut DAT_009a210a: [i8; 0];
    static mut DAT_009a8080: [i8; 0];
    static mut DAT_009a8081: [i8; 0];
    static mut DAT_009a8082: [i8; 0];
    static mut DAT_009e967c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432fa0(a: u16, b: u16, c: u16) -> i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044e170(param_1: u32) -> u32 {
    // SAFETY: all globals and the pointers derived from them are the game's
    // own memory, read at the offsets the original uses.
    unsafe {
        let uvar14: u32 = param_1 & 0xffff;
        if DAT_009e9640 <= uvar14 as i32 && (uvar14 as i32) < DAT_00a00854 {
            return 0;
        }
        let base = DAT_009a1fe8 as u32 as usize as *const u8;
        let local_18: i32;
        if (uvar14 as i32) < DAT_009a1fe0 {
            let ient: i32 = base
                .byte_add(0x1fusize.wrapping_add(uvar14.wrapping_mul(0x23) as usize))
                .cast::<i32>()
                .read_unaligned();
            local_18 = (ient as u32 as usize as *const u8)
                .byte_add(0xa9)
                .cast::<i32>()
                .read_unaligned();
        } else {
            local_18 = 0;
        }
        let mut b7: u8 = 0;
        let mut b11: u8 = 0;
        let mut b10: u8 = 0;
        let mut b12: u8 = 0;
        let mut b8: u8 = 0;
        let mut b9: u8 = 0;
        let mut uvar16: u32 = 0;
        let bvar3: u8 = base
            .byte_add(0x16usize.wrapping_add(uvar14.wrapping_mul(0x23) as usize))
            .read();
        let mut b6: u8 = 0;
        let ivar5: i32 = (bvar3 as u32).wrapping_mul(0x566) as i32;
        let mut pcvar15: *const i8 =
            (&raw mut DAT_009d8e25 as *const i8).byte_offset(ivar5 as isize);
        let pb: *const u8 = (&raw mut DAT_009e967c as *const u8)
            .byte_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize);
        let mut wval: u16 = pb.cast::<u16>().read_unaligned();
        let mut pcvar17: *const i8 = (&raw mut DAT_009d8e29 as *const i8)
            .byte_add((wval as u32).wrapping_add(ivar5 as u32) as usize);
        loop {
            if pcvar17.read() != 0 && pcvar15.read() == 0 {
                let ivar1: i32 = (bvar3 as u32).wrapping_mul(0x126) as i32;
                wval = pb.cast::<u16>().read_unaligned();
                let idx = (ivar1 as u32)
                    .wrapping_add(uvar16)
                    .wrapping_add((wval as u32).wrapping_mul(8));
                if (&raw mut DAT_009a2196 as *const i8)
                    .byte_add(idx as usize)
                    .read()
                    == 4
                {
                    if pcvar15.byte_add(2).read() == 6 {
                        if (&raw mut DAT_009a210a as *const i8)
                            .byte_offset(ivar1 as isize)
                            .read()
                            == 0
                        {
                            b7 = 1;
                        }
                        b6 = 1;
                    }
                    let c4 = pcvar15.byte_add(1).read();
                    if c4 == 2 {
                        b11 = 1;
                    } else if c4 == 3 {
                        b12 = 1;
                    } else if c4 == 8 {
                        b10 = 1;
                    } else if c4 == 5 {
                        b7 = 1;
                    } else if c4 == 4 {
                        b8 = 1;
                    } else if c4 == 6 {
                        b9 = 1;
                    }
                }
            }
            pcvar15 = pcvar15.byte_add(0x4e);
            pcvar17 = pcvar17.byte_add(0x4e);
            uvar16 = uvar16.wrapping_add(1);
            if uvar16 >= 8 {
                break;
            }
        }
        uvar16 = 0;
        loop {
            let mut uvar18: u32 = 0;
            loop {
                wval = pb.cast::<u16>().read_unaligned();
                let ivar1: u32 = uvar18.wrapping_add((wval as u32).wrapping_mul(0x126));
                let c4 = (&raw mut DAT_009a8080 as *const i8)
                    .byte_add(uvar16.wrapping_add(ivar1) as usize)
                    .read();
                let idx = ivar1.wrapping_add(uvar16) as usize;
                if c4 != 10 {
                    let s = FUN_00432fa0(
                        bvar3 as u16,
                        (&raw mut DAT_009a8081 as *const u8).byte_add(idx).read() as u16,
                        (&raw mut DAT_009a8082 as *const u8).byte_add(idx).read() as u16,
                    );
                    if s != 0 {
                        if c4 == 2 {
                            b11 = 1;
                        } else if c4 == 3 {
                            b12 = 1;
                        } else if c4 == 8 {
                            b10 = 1;
                        } else if c4 == 5 {
                            b7 = 1;
                        } else if c4 == 4 {
                            b8 = 1;
                        } else if c4 == 6 {
                            b9 = 1;
                        }
                    }
                }
                uvar18 = uvar18.wrapping_add(4);
                if uvar18 >= 8 {
                    break;
                }
            }
            uvar16 = uvar16.wrapping_add(0xb);
            if uvar16 >= 0x16 {
                break;
            }
        }
        if (uvar14 as i32) < DAT_009a1fe0 {
            if b7 != 0 {
                wval = pb.cast::<u16>().read_unaligned();
                let p = (wval as u32)
                    .wrapping_add(0x1f6)
                    .wrapping_add(local_18 as u32) as usize as *const i8;
                if p.read() != 0 {
                    b7 = 0;
                }
            }
            if b8 != 0 || b9 != 0 {
                let ient: i32 = base
                    .byte_add(0x1fusize.wrapping_add(uvar14.wrapping_mul(0x23) as usize))
                    .cast::<i32>()
                    .read_unaligned();
                let stmp: i16 = (ient as u32 as usize as *const u8)
                    .byte_add(0x79)
                    .cast::<i16>()
                    .read_unaligned();
                if stmp != 0 {
                    b10 = 1;
                }
                if b6 != 0 {
                    let a = (&raw mut DAT_009d8c80 as *const i8)
                        .byte_offset(ivar5 as isize)
                        .read();
                    let b = ((local_18 as u32).wrapping_add(0xb0) as usize as *const i8).read();
                    if a != b {
                        b8 = 0;
                        b9 = 0;
                    }
                }
            }
            if b10 != 0 && b6 != 0 {
                let a = (&raw mut DAT_009d8c80 as *const i8)
                    .byte_offset(ivar5 as isize)
                    .read();
                let b = ((local_18 as u32).wrapping_add(0xb0) as usize as *const i8).read();
                if a != b {
                    b10 = 0;
                }
            }
            if b11 != 0 && b6 != 0 {
                let a = (&raw mut DAT_009d8c80 as *const i8)
                    .byte_offset(ivar5 as isize)
                    .read();
                let b = ((local_18 as u32).wrapping_add(0xb0) as usize as *const i8).read();
                if a != b {
                    b11 = 0;
                }
            }
        }
        if b7 != 0 {
            return 0xf;
        }
        if b10 != 0 {
            if b8 != 0 {
                return 0x13;
            }
            let n = (b9 == 0) as u32;
            return (0u32.wrapping_sub(n) & 0xffff_fffd).wrapping_add(0x14);
        }
        if b8 != 0 {
            return 0x13;
        }
        if b9 != 0 {
            return 0x14;
        }
        if b11 != 0 {
            return 0x10;
        }
        ((b12 == 0) as u32).wrapping_sub(1) & 0xffff_0012
    }
}
