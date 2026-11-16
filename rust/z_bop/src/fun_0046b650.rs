// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e9683: [i8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08374: u16;
    static mut DAT_00a0a2ea: [i16; 0];
    static mut DAT_00a08ba6: i16;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_005233c8: u8;
    static mut DAT_00a08c90: i16;
    static mut DAT_00a08c82: i16;
    static mut DAT_00a08c8e: i16;
    static mut DAT_00a08c8a: i16;
    static mut DAT_00521548: i16;
    static mut DAT_0052154c: i16;
    static mut DAT_009d1300: *const c_void;
    static mut DAT_009eca20: *const c_void;
    static mut DAT_0099f928: u32;
    static mut DAT_009a20ae: u8;
    static mut DAT_00559790: i16;
    static mut DAT_00559794: i16;
    static mut DAT_00559798: u16;
    static mut _DAT_0055979c: i16;
    static mut DAT_00a004d4: u16;
    static mut DAT_00a080f8: i16;
    static mut DAT_009d6930: i16;
    static mut DAT_0051bf60: [u16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32) -> u32;
    fn FUN_004a9640(a: i8);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_00480530() -> u32;
    fn FUN_004a9630(a: i32);
    fn FUN_004a9590(a: i32);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9a50(a: u32);
    fn FUN_0046be10(a: u16, b: i16) -> u32;
    fn FUN_0046bf90(a: u32, b: u16, c: i16);
}

unsafe fn rd8(a: i32) -> u8 {
    // SAFETY: the game's own memory, read where the original reads.
    unsafe { (a as u32 as usize as *const u8).read() }
}
unsafe fn wr8(a: i32, v: u8) {
    // SAFETY: the game's own memory, written where the original writes.
    unsafe { (a as u32 as usize as *mut u8).write(v) }
}
unsafe fn rd16(a: i32) -> i16 {
    // SAFETY: the game's own memory, read where the original reads.
    unsafe { (a as u32 as usize as *const i16).read_unaligned() }
}
unsafe fn rd16u(a: i32) -> u16 {
    // SAFETY: the game's own memory, read where the original reads.
    unsafe { (a as u32 as usize as *const u16).read_unaligned() }
}
unsafe fn rd32(a: i32) -> i32 {
    // SAFETY: the game's own memory, read where the original reads.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}
unsafe fn bit(i: i32) -> u16 {
    // SAFETY: DAT_0051bf60 is the game's own table.
    unsafe {
        (&raw const DAT_0051bf60)
            .cast::<u16>()
            .offset(i as isize)
            .read()
    }
}
unsafe fn shortarr(i: usize) -> i16 {
    // SAFETY: DAT_00a0a2ea is the game's own table.
    unsafe { (&raw const DAT_00a0a2ea).cast::<i16>().add(i).read() }
}
unsafe fn call0(p: *const c_void) {
    // SAFETY: p is a function pointer held by the game.
    unsafe { core::mem::transmute::<*const c_void, unsafe extern "C" fn()>(p)() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046b650() -> u32 {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed in the order and at the offsets the original uses.
    unsafe {
        let mut aux = [0u32; 5];
        let ap = aux.as_mut_ptr().cast::<u8>();
        let uvar_16: u16 = 0;
        let mut uvar12: u32;

        let rec = (&raw mut DAT_009e9670)
            .cast::<u8>()
            .add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize);
        let mut iv3: i32 = rec.cast::<i32>().read_unaligned();
        if iv3 == -1 {
            uvar12 = DAT_00a08374 as u32;
            if shortarr(uvar12 as usize) != DAT_00a08ba6 {
                FUN_0040e0c0(
                    DAT_009a806c,
                    DAT_009a7bac,
                    DAT_009a8c14,
                    (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
                );
                FUN_004a9640(0);
                let s7 = DAT_00a08c90;
                let s5 = DAT_00a08c82;
                let s6 = DAT_00a08c82.wrapping_add(DAT_00a08c8e);
                let s10 = DAT_00a08c8a.wrapping_add(DAT_00a08c90);
                FUN_004a95c0(DAT_005233c8 as u32);
                FUN_004a9520(s5, s7, s6, s10);
                call0(DAT_009d1300);
                if DAT_00a08ba6 == -2 && DAT_00521548 == 3 {
                    DAT_00521548 = DAT_0052154c;
                    uvar12 = FUN_00480530();
                    DAT_0052154c = uvar12 as i16;
                }
                uvar12 = FUN_0040e0c0(0, 0x140, 200, 0);
            }
            return uvar12 & 0xffff0000;
        }
        let iv3a = rd32(
            DAT_009a1fe8
                .wrapping_add((iv3 as u32).wrapping_mul(0x23) as i32)
                .wrapping_add(0x1f),
        );
        iv3 = rd32(iv3a.wrapping_add(0xa9));
        if iv3 == 0 {
            return (DAT_009a1fe8 as u32) & 0xffff0000;
        }
        let mut s6: i16 = 0;
        loop {
            let b = rd8((s6 as i32).wrapping_add(0x2a1).wrapping_add(iv3)) as usize;
            ap.add(b * 2).cast::<i16>().write_unaligned(s6);
            s6 = s6.wrapping_add(1);
            if s6 >= 10 {
                break;
            }
        }
        s6 = 0;
        let mut b4: u8 = 1;
        uvar12 = 0;
        loop {
            let tmp = ap.offset(s6 as isize * 2).cast::<u16>().read_unaligned();
            let uvar13 = tmp as u32;
            let mask = rd16u(iv3.wrapping_add(0x1e3));
            if rd16(((uvar13 * 2) as i32).wrapping_add(0x2ac).wrapping_add(iv3)) == 0
                && (mask & bit(uvar13 as i32)) != 0
            {
                uvar12 = (uvar12 & 0xffff0000) | ((uvar12 as i16).wrapping_add(1) as u16 as u32);
                b4 = 0;
            }
            s6 = s6.wrapping_add(1);
            if s6 >= 10 {
                break;
            }
        }
        if b4 != 0 {
            DAT_00a08ba6 = 0;
        }
        FUN_0040e0c0(
            DAT_009a806c,
            DAT_009a7bac,
            DAT_009a8c14,
            (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
        );
        FUN_004a9640(0);
        let s5 = DAT_00a08c90;
        let s10 = DAT_00a08c82;
        let s7 = DAT_00a08c82.wrapping_add(2);
        let mut s14 = DAT_00a08c90.wrapping_add(2);
        s6 = DAT_00a08c82.wrapping_add(-2).wrapping_add(DAT_00a08c8e);
        let mut iv17: i32 = ((DAT_00a08c8a as u16 as u32)
            .wrapping_add(0xffff_fffe)
            .wrapping_add(DAT_00a08c90 as u16 as u32)) as i32;
        let uv8: u16 = DAT_009a20ae.wrapping_add(1) as u16;
        let s15 = uvar12 as i16;
        let mut local_1a: u16;
        if shortarr(DAT_00a08374 as usize) == DAT_00a08ba6 {
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9630(1);
        } else {
            FUN_004a95c0(DAT_005233c8 as u32);
            let s16 = (iv17 as i16).wrapping_add(2);
            iv17 = ((iv17 as u32 & 0xffff0000) | s16 as u16 as u32) as i32;
            FUN_004a9520(s10, s5, s6.wrapping_add(2), s16);
            call0(DAT_009d1300);
            if DAT_00a08ba6 == -2 || b4 != 0 {
                if DAT_00521548 == 3 {
                    DAT_00521548 = DAT_0052154c;
                    uvar12 = FUN_00480530();
                    DAT_0052154c = uvar12 as i16;
                }
                if b4 != 0 {
                    (&raw mut DAT_00a0a2ea)
                        .cast::<i16>()
                        .add(DAT_00a08374 as usize)
                        .write(-2);
                }
                uvar12 = FUN_0040e0c0(0, 0x140, 200, 0);
                return uvar12 & 0xffff0000;
            }
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9630(1);
            FUN_004a9590(0x46);
            call0(DAT_009d1300);
            FUN_004a9500(s7, s14);
            FUN_004a9a50(DAT_0099f928);
            DAT_00559790 = s15;
        }
        local_1a = (s14 as i32).wrapping_add(uv8 as i32) as u16;
        s14 = ((s15 as i32 as u32)
            .wrapping_mul(uv8 as u32)
            .wrapping_add(2)
            .wrapping_add(local_1a as u32)) as i16;
        if s15 != DAT_00559790 {
            FUN_004a9520(
                s10,
                local_1a as i16,
                s6.wrapping_add(2),
                ((DAT_00559790 as i32).wrapping_add(1) as u32)
                    .wrapping_mul(uv8 as u32)
                    .wrapping_add(local_1a as u32) as i16,
            );
            call0(DAT_009d1300);
            DAT_00a004d4 = 0xffff;
            DAT_00559790 = s15;
        }
        let flag = (&raw mut DAT_009e9683)
            .cast::<i8>()
            .add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .read();
        if flag == 0 && DAT_00521548 == -1 {
            DAT_00521548 = 3;
        }
        let mut lab: Option<u32> = None;
        if DAT_00521548 == 3 {
            lab = Some(0x46);
        } else if DAT_0052154c == 3 {
            lab = Some(DAT_005233c8 as u32);
        }
        if let Some(uvar13) = lab {
            FUN_004a95c0(uvar13);
            uvar12 = (uvar12 & 0xffff0000) | s6.wrapping_add(2) as u16 as u32;
            FUN_004a9520(s10, s5, s6.wrapping_add(2), s14.wrapping_add(2));
            let f = core::mem::transmute::<
                *const c_void,
                unsafe extern "C" fn(u32, u32, u32, u32, u32),
            >(DAT_009eca20);
            f(
                (iv17 as u32 & 0xffff0000) | s10 as u16 as u32,
                s5 as i32 as u32,
                uvar12,
                ((uvar_16 as u32) << 16) | s14.wrapping_add(2) as u16 as u32,
                1,
            );
        }
        FUN_004a9520(s7, local_1a as i16, s6, s14);
        FUN_004a95c0(DAT_005233c8 as u32);
        FUN_004a9590(0x43);
        DAT_00559794 = 1;
        'bc8d: {
            if shortarr(DAT_00a08374 as usize) == DAT_00a08ba6 {
                if DAT_00521548 == 3 && DAT_00a080f8 == 1 {
                    if DAT_009d6930 == 0xd {
                        uvar12 &= 0xffff0000;
                        let b1 = rd8((DAT_00a004d4 as i16 as i32)
                            .wrapping_add(0x2a1)
                            .wrapping_add(iv3));
                        loop {
                            let sv10 = uvar12 as i16;
                            let addr = (sv10 as i32).wrapping_add(0x2a1).wrapping_add(iv3);
                            let b2 = rd8(addr);
                            if b2 < b1 {
                                wr8(addr, b2.wrapping_add(1));
                            }
                            uvar12 = (uvar12 & 0xffff0000) | sv10.wrapping_add(1) as u16 as u32;
                            if sv10.wrapping_add(1) >= 10 {
                                break;
                            }
                        }
                        wr8(
                            (DAT_00a004d4 as i16 as i32)
                                .wrapping_add(0x2a1)
                                .wrapping_add(iv3),
                            0,
                        );
                    } else if DAT_009d6930 == 0xa6 {
                        let mut uvar13 = DAT_00a004d4 as u32;
                        uvar12 = 0;
                        loop {
                            let mut uv9;
                            loop {
                                uvar13 = FUN_0046be10(uvar13 as u16, -1);
                                uv9 = uvar13 as u16;
                                let mask = rd16u(iv3.wrapping_add(0x1e3));
                                if (mask & bit(uv9 as i16 as i32)) != 0 {
                                    break;
                                }
                            }
                            if rd16(
                                (uv9 as i16 as i32)
                                    .wrapping_mul(2)
                                    .wrapping_add(0x2ac)
                                    .wrapping_add(iv3),
                            ) == 0
                            {
                                DAT_00a004d4 = uv9;
                            }
                            let mask = rd16u(iv3.wrapping_add(0x1e3));
                            if (mask & bit(uv9 as i16 as i32)) != 0 {
                                break;
                            }
                        }
                    } else {
                        if DAT_009d6930 != 0xa7 {
                            break 'bc8d;
                        }
                        let mut uvar13 = DAT_00a004d4 as u32;
                        uvar12 = 0;
                        loop {
                            uvar13 = FUN_0046be10(uvar13 as u16, 1);
                            let uv9 = uvar13 as u16;
                            let mask = rd16u(iv3.wrapping_add(0x1e3));
                            if (mask & bit(uv9 as i16 as i32)) != 0
                                && rd16(
                                    (uv9 as i16 as i32)
                                        .wrapping_mul(2)
                                        .wrapping_add(0x2ac)
                                        .wrapping_add(iv3),
                                ) == 0
                            {
                                DAT_00a004d4 = uv9;
                            }
                            let mask = rd16u(iv3.wrapping_add(0x1e3));
                            if (mask & bit(DAT_00a004d4 as i16 as i32)) != 0 {
                                break;
                            }
                        }
                    }
                    _DAT_0055979c = 1;
                }
            } else {
                DAT_00a004d4 = 0xffff;
                _DAT_0055979c = 1;
            }
        }
        let mut s10b: i16 = 0;
        loop {
            let b = rd8((s10b as i32).wrapping_add(0x2a1).wrapping_add(iv3)) as usize;
            ap.add(b * 2).cast::<i16>().write_unaligned(s10b);
            s10b = s10b.wrapping_add(1);
            if s10b >= 10 {
                break;
            }
        }
        let mut s24: i16 = 0;
        loop {
            let uv9 = ap.offset(s24 as isize * 2).cast::<u16>().read_unaligned();
            uvar12 = (uvar12 & 0xffff0000) | uv9 as u32;
            let uvar13 = uv9 as u32;
            let mut uv11 = local_1a;
            let mask = rd16u(iv3.wrapping_add(0x1e3));
            if rd16(iv3.wrapping_add(0x2ac).wrapping_add((uvar13 * 2) as i32)) == 0
                && (mask & bit(uvar13 as i32)) != 0
                && (uv8 as i32 + local_1a as i32) < s14 as i32
            {
                uv11 = uv8.wrapping_add(local_1a);
                FUN_004a9520(s7, local_1a as i16, s6, uv11 as i16);
                FUN_004a9500(s7, local_1a as i16);
                if DAT_00a004d4 == 0xffff {
                    DAT_00559798 = uv9;
                    DAT_00a004d4 = uv9;
                }
                let uvar18 = if (DAT_00a004d4 as i16 as i32) == uvar13 as i32 {
                    0x33
                } else {
                    DAT_005233c8 as u32
                };
                FUN_004a95c0(uvar18);
                if DAT_00559794 != 0
                    || (DAT_00a004d4 as i16 as i32) == uvar13 as i32
                    || (DAT_00559798 as i16 as i32) == uvar13 as i32
                {
                    FUN_0046bf90(uvar12, local_1a, s7);
                }
            }
            local_1a = uv11;
            s24 = s24.wrapping_add(1);
            if s24 >= 10 {
                break;
            }
        }
        _DAT_0055979c = 0;
        DAT_00559794 = 0;
        DAT_00559798 = DAT_00a004d4;
        uvar12 = FUN_0040e0c0(0, 0x140, 200, 0);
        (uvar12 & 0xffff0000) | 1
    }
}
