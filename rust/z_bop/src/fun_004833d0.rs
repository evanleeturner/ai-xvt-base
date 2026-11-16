// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e9683: [u8; 0];
    static mut DAT_009e968a: [u8; 0];
    static mut DAT_009e9696: [u8; 0];
    static mut DAT_009e9699: [u8; 0];
    static mut DAT_009e9a67: [u8; 0];
    static mut DAT_009e9a77: [u8; 0];
    static mut DAT_009e9a8a: [u8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a00870: [u32; 0];
    static mut DAT_00a00874: [u32; 0];
    static mut _DAT_00a07be0: u16;
    static mut _DAT_00a07be2: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042e4d0(param_1: u32, param_2: u32, param_3: i32) -> u32;
    fn FUN_004adc40(param_1: u32) -> u32;
    fn FUN_004ae2a0(param_1: u32, param_2: u32) -> u32;
    fn FUN_00483800(param_1: u32, param_2: i32) -> u32;
    fn FUN_00451f50(param_1: u32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_00450650(param_1: i32, param_2: i32);
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004833d0(param_1: u32, param_2: i32) {
    // SAFETY: all globals and callees are the game's own; addresses computed
    // as the original does.
    unsafe {
        if (param_1 as i16) == -1 {
            return;
        }
        let mut uvar7: u32 = param_1 & 0xffff;
        let rec = uvar7.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
        if rd8(rec.wrapping_add(3)) == 0 || rd8(rec.wrapping_add(2)) == 13 {
            return;
        }
        let ivar3: u32 = (param_2 as u32).wrapping_mul(0x5bd);
        let a670 = (&raw mut DAT_009e9670).cast::<u8>() as u32;
        let a683 = (&raw mut DAT_009e9683).cast::<u8>() as u32;
        let a68a = (&raw mut DAT_009e968a).cast::<u8>() as u32;
        let a696 = (&raw mut DAT_009e9696).cast::<u8>() as u32;
        let a699 = (&raw mut DAT_009e9699).cast::<u8>() as u32;
        let a967 = (&raw mut DAT_009e9a67).cast::<u8>() as u32;
        let a977 = (&raw mut DAT_009e9a77).cast::<u8>() as u32;
        let a98a = (&raw mut DAT_009e9a8a).cast::<u8>() as u32;
        let t870 = (&raw mut DAT_00a00870).cast::<u32>();
        let t874 = (&raw mut DAT_00a00874).cast::<u32>();

        let uvar4 = rd32(a670.wrapping_add(ivar3));
        if uvar4 == uvar7 {
            return;
        }
        let mut bvar2 = true;
        if uvar4 != 0xffff_ffff {
            let pa = rd32(
                uvar4
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8),
            );
            let pb = rd32(pa.wrapping_add(0xa9));
            if rd8(pb.wrapping_add(0x1e5)) & 4 == 0 {
                bvar2 = false;
            }
        }
        if !bvar2 {
            _DAT_00a07be0 = 0x60;
            _DAT_00a07be2 = 0x57;
            FUN_00450650(0x56, param_2);
            return;
        }
        let s16 = (a68a.wrapping_add(ivar3) as usize as *const u16).read_unaligned();
        if (s16 as i16) == (param_1 as i16) {
            return;
        }
        FUN_0042e4d0(0x31, 0xffff_ffff, param_2);
        wr16(a68a.wrapping_add(ivar3), param_1 as u16);
        let ivar5 = DAT_009a1fe0;
        wr16(a699.wrapping_add(ivar3), 0);
        if (uvar7 as i32) < ivar5 {
            if rd8(a683.wrapping_add(ivar3)) == 0 {
                uvar7 = FUN_00483800(param_1, param_2);
                wr16(a699.wrapping_add(ivar3), uvar7 as u16);
            } else {
                let s = (a68a.wrapping_add(ivar3) as usize as *const u16).read_unaligned();
                let b1 = rd8((s as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8));
                let local_4: i32 = if b1 < 0x49 {
                    t870.wrapping_add((b1 as usize) * 0x65).read_unaligned() as i32
                } else {
                    FUN_004adc40(b1 as u32) as i32
                };
                uvar7 = 0;
                if 0 < local_4 {
                    loop {
                        let s = (a68a.wrapping_add(ivar3) as usize as *const u16).read_unaligned();
                        let b1 = rd8((s as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(3)
                            .wrapping_add(DAT_009a1fe8));
                        let uvar4 = b1 as u32;
                        let iv5: i32;
                        if b1 < 0x49 {
                            let mut uvar6 = uvar7;
                            let cnt = t870.wrapping_add((uvar4 as usize) * 0x65).read_unaligned();
                            if (cnt as i32) <= (uvar7 as i32) {
                                uvar6 = cnt.wrapping_sub(1);
                            }
                            iv5 = t874
                                .wrapping_add((uvar4 as usize) * 0x65 + uvar6 as usize)
                                .read_unaligned() as i32;
                        } else {
                            iv5 = FUN_004ae2a0(uvar4, uvar7) as i32;
                        }
                        if iv5 == 1 || iv5 == 3 {
                            wr16(a699.wrapping_add(ivar3), uvar7 as u16);
                            break;
                        }
                        uvar7 = (uvar7 as u16).wrapping_add(1) as u32;
                        if (uvar7 as i32) >= local_4 {
                            break;
                        }
                    }
                }
            }
        }
        let v = (a98a.wrapping_add(ivar3) as usize as *const u16).read_unaligned();
        if (v as i16) != 0 {
            let s = (a68a.wrapping_add(ivar3) as usize as *const u16).read_unaligned();
            wr16(a967.wrapping_add(ivar3), s);
        }
        (a696.wrapping_add(ivar3) as usize as *mut u8).write(0);
        let u32v = rd32(a670.wrapping_add(ivar3));
        if u32v != 0xffff_ffff {
            let pa = rd32(
                u32v.wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8),
            );
            let pb = rd32(pa.wrapping_add(0xa9));
            wr16(pb.wrapping_add(0x23b), 0);
        }
        if rd8(a683.wrapping_add(ivar3)) != 0 {
            FUN_00451f50(param_1, param_2, 1, 0);
            return;
        }
        let u32v = rd32(a670.wrapping_add(ivar3));
        let pa = rd32(
            u32v.wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8),
        );
        let pb = rd32(pa.wrapping_add(0xa9));
        if rd8(pb.wrapping_add(0x1e1)) & 1 != 0 {
            let x = rd8(a977.wrapping_add(ivar3));
            if x == 0x13
                || x == 0
                || rd8(a977.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd))) == 0x14
            {
                FUN_00451f50(param_1, param_2, 1, 0);
                return;
            }
        }
    }
}
