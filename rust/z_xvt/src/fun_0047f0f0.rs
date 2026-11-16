// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0071f714: i16;
    static mut DAT_006a89f8: i32;
    static mut DAT_00526872: [u8; 0];
    static mut DAT_006a89f4: u16;
    static mut DAT_006d498c: u16;
    static mut DAT_005281f8: [u8; 0];
    static mut DAT_006db7b4: u32;
    static mut DAT_006db7ac: u32;
    static mut DAT_006db7b0: u32;
    static mut DAT_0070a134: i32;
    static mut DAT_0070a00c: i32;
    static mut DAT_0070a130: u32;
    static mut DAT_0070a010: u32;
    static mut DAT_0070a12c: u32;
    static mut DAT_0070a138: u32;
    static mut DAT_006d4984: i16;
    static mut DAT_006da9e2: i16;
    static mut DAT_0071eaf0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00486750(param_1: i32, param_2: u32);
    fn FUN_00419100(param_1: u32, param_2: u32) -> u32;
    fn FUN_0040ca90(param_1: u32, param_2: u32) -> u32;
    fn FUN_0040cad0(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047f0f0(param_1: i32) {
    // SAFETY: all globals, callees and the pointed-to records are the game's
    // own memory, accessed at the same offsets and in the same order as the
    // original.
    unsafe {
        let mut iv10: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let mut iv8: i32 = (DAT_006d4988 as u32).wrapping_add(iv10 as u32) as i32;
        DAT_0071f714 = param_1 as i16;
        let mut local_2: u16 = ((iv8.wrapping_add(3)) as u32 as usize as *const u8).read() as u16;
        let src = (&raw mut DAT_00526872)
            .cast::<u8>()
            .byte_add((local_2 as usize).wrapping_mul(0x16));
        DAT_006a89f8 = src.cast::<i32>().read_unaligned();
        let mut u11: u16;
        if local_2 == 0x59 {
            let b = (iv8.wrapping_add(0x19) as u32 as usize as *const u8).read();
            u11 = (b >> 1) as u16;
        } else {
            if DAT_006a89f8 == 0 {
                return;
            }
            DAT_006a89f4 = (iv8.wrapping_add(0x19) as u32 as usize as *const u8).read() as u16;
            let addr = (DAT_006a89f8 as u32).wrapping_add((DAT_006a89f4 as u32).wrapping_mul(2));
            u11 = (addr as usize as *const u16).read_unaligned();
        }
        if u11 < 0xff00 {
            if u11 < 0x8000 {
                if local_2 == 0x59 {
                    let t = (iv8.wrapping_add(0x1f) as u32 as usize as *const i32).read_unaligned();
                    local_2 = (t.wrapping_add(0x85) as u32 as usize as *const u8).read() as u16;
                }
                DAT_006d498c = u11;
                FUN_00486750(iv8, u11 as u32);
                if local_2 == 0x59 {
                    let a = (DAT_006d4988 as u32)
                        .wrapping_add(0x1a)
                        .wrapping_add(iv10 as u32);
                    DAT_006a89f4 = (a as usize as *const u8).read() as u16;
                    u11 = (&raw mut DAT_005281f8)
                        .cast::<u8>()
                        .byte_add((DAT_006a89f4 as usize) * 2)
                        .cast::<u16>()
                        .read_unaligned();
                }
            }
            if u11 < 0xff00 && 0x7fff < u11 && (DAT_006db7b4 as i32) >= 0 {
                iv8 = DAT_0070a134;
                if DAT_0070a134 < 0 {
                    iv8 = DAT_0070a134.wrapping_neg();
                }
                let mut iv9 = DAT_0070a00c;
                if DAT_0070a00c < 0 {
                    iv9 = DAT_0070a00c.wrapping_neg();
                }
                let mut u5 = DAT_0070a130;
                let mut u6 = DAT_0070a010;
                if iv8 < iv9 {
                    u5 = DAT_0070a12c;
                    u6 = DAT_0070a138;
                }
                let s2: i16;
                if (u5 as i32) < 0 {
                    let u4 = FUN_00419100(u6, 0u32.wrapping_sub(u5));
                    s2 = u4 as i16;
                } else {
                    let u4 = FUN_00419100(u6, u5);
                    s2 = 0u32.wrapping_sub((u4 as u16) as u32) as i16;
                }
                u5 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                let m5 = (u5 & 0xffff0000) as i32;
                if m5 < 1 && -0x10001 < m5 {
                    u6 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
                    let m6 = (u6 & 0xffff0000) as i32;
                    if m6 < 1 && -0x10001 < m6 {
                        let s7: i16 = DAT_006d4984.wrapping_sub(u6 as i16);
                        let a = (DAT_006d4988 as u32)
                            .wrapping_add(0x1f)
                            .wrapping_add(iv10 as u32);
                        let t = (a as usize as *const i32).read_unaligned();
                        let b1 = (t.wrapping_add(1) as u32 as usize as *const u8).read();
                        let mut u3: u16;
                        if b1 == 0 {
                            u3 = 0x100;
                        } else {
                            u3 = (b1 as u16).wrapping_mul(0x40);
                            if 0xff < u3 {
                                u3 = u3.wrapping_add(0x100);
                            }
                        }
                        if DAT_006da9e2 < 0x20 {
                            iv10 = DAT_006da9e2 as i32;
                            let e = (&raw mut DAT_0071eaf0)
                                .cast::<u8>()
                                .byte_add((iv10 as u32).wrapping_mul(16) as usize);
                            e.cast::<i16>().write_unaligned(DAT_0071f714);
                            e.byte_add(2).cast::<u16>().write_unaligned(u11);
                            e.byte_add(4).cast::<u16>().write_unaligned(u3);
                            let u6b = DAT_006db7b4;
                            e.byte_add(6).cast::<i16>().write_unaligned(u5 as i16);
                            e.byte_add(8).cast::<i16>().write_unaligned(s7);
                            DAT_006da9e2 = DAT_006da9e2.wrapping_add(1);
                            e.byte_add(10).cast::<u32>().write_unaligned(u6b);
                            e.byte_add(14).cast::<i16>().write_unaligned(s2);
                        }
                    }
                }
            }
        }
    }
}
