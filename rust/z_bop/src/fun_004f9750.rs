// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5e10: [i32; 1];
    static mut DAT_00aa5cb0: [u32; 1];
    static mut DAT_00aa5ac0: [i32; 1];
    static mut DAT_00aa5e40: [u32; 1];
    static mut DAT_00aa5e38: u32;
    static mut DAT_00aa5e60: i32;
    static mut DAT_00aa5aac: i32;
    static mut DAT_00b6a5a6: [u8; 4];
    static mut DAT_00aa6150: [u8; 1];
    static mut DAT_00aa615e: [u8; 1];
    static mut DAT_00aa6162: [u8; 1];
    static mut DAT_00aa62ae: [u8; 1];
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: u32;
    static mut DAT_00aa6af8: u32;
    static mut DAT_00aa6afc: u32;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b69e30: i32;
    static mut DAT_00b69e38: u32;
    static mut DAT_00b6a2a0: u32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b69ccc: u32;
    static mut DAT_00b6a270: [u8; 1];
    static mut DAT_00bb73a7: i8;
    static mut DAT_00a91d75: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d11e0() -> i32;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004d4cf0(param_1: *mut u32);
    fn FUN_004d4d20(param_1: *mut core::ffi::c_void);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004dc2e0(param_1: i32) -> u8;
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dc1e0() -> u8;
    fn FUN_004dc200() -> u8;
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dc1c0(param_1: i32);
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dc2a0(param_1: i32) -> u8;
    fn FUN_004de5a0(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004dc2c0(param_1: i32) -> u8;
    fn FUN_004dc1d0();
}

macro_rules! sel {
    () => {
        (&raw const DAT_00b6a5a6).cast::<i32>().read_unaligned()
    };
}
macro_rules! at {
    ($g:ident, $t:ty, $i:expr) => {
        *(&raw mut $g).cast::<$t>().offset(($i) as isize)
    };
}
macro_rules! rdat {
    ($g:ident, $off:expr) => {
        (&raw const $g)
            .cast::<u8>()
            .byte_add(($off) as usize)
            .cast::<i32>()
            .read_unaligned()
    };
}
macro_rules! e10 {
    () => {
        at!(DAT_00aa5e10, i32, sel!())
    };
}
macro_rules! s8 {
    () => {
        sel!().wrapping_mul(8)
    };
}
macro_rules! buf {
    () => {
        (&raw mut DAT_00b69d20).cast::<i8>()
    };
}
macro_rules! rect {
    ($g:ident, $i:expr) => {
        (&raw const $g).cast::<u8>().byte_add(($i) as usize)
    };
}
macro_rules! xcur {
    ($p:expr) => {
        ((($p) % 0x18) as u32 & 0xfffffffe).wrapping_mul(2)
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_004f9750(param_1: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed from those statics and local arrays exactly as
    // the original indexes them.
    unsafe {
        let mut local_34: [i32; 4] = [0; 4];
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [u32; 4] = [0; 4];
        let mut local_38: i32 = 0;
        let mut local_3c: i32 = 0;
        let mut local_40: i32;
        let local_24: u32;
        let mut u18: u32;
        let mut i5: i32 = -0x44;
        if e10!() < 5 {
            i5 = 0;
        }
        let mut i13: i32 = i5.wrapping_add(0x15d);
        let mut i16: i32 = i5.wrapping_add(0x14f);
        FUN_004d6460(local_34.as_mut_ptr(), 0x58, i16, 0xe5, i13);
        let pc = FUN_004ddc10(0x273);
        FUN_004db7d0(0xc, pc, local_34.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004d6460(local_34.as_mut_ptr(), 0xe8, i16, 0x150, i13);
        let pc = FUN_004ddc10(0x274);
        FUN_004db7d0(0xc, pc, local_34.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004d6460(local_34.as_mut_ptr(), 0x153, i16, 0x1bb, i13);
        let pc = FUN_004ddc10(0x2ad);
        FUN_004db7d0(0xc, pc, local_34.as_mut_ptr(), 0, 1, 0xffff);
        i13 = FUN_004d11e0();
        local_24 = (i13 == at!(DAT_00aa5cb0, u32, s8!()) as i32) as u32;
        if 1 < e10!() {
            FUN_004d6460(local_34.as_mut_ptr(), 0x58, 0x1a6, 0x1bb, 0x1b4);
            let u14: i32;
            if local_24 == 0 && DAT_00b69e30 != 2 {
                u14 = 0x276;
                u18 = DAT_00b69e38;
            } else {
                u14 = 0x275;
                u18 = DAT_00b6a2a0;
            }
            let pc = FUN_004ddc10(u14);
            FUN_004db640(0xc, pc, local_34.as_mut_ptr(), u18);
        }
        local_40 = 0;
        i16 = 0;
        FUN_004d6460(
            local_34.as_mut_ptr(),
            0x153,
            i5.wrapping_add(0x160),
            0x1bb,
            i5.wrapping_add(0x170),
        );
        i13 = DAT_00aa5e60;
        if 0 < e10!() {
            'outer: loop {
                let mut i15: i32 = 0;
                let mut p = (&raw const DAT_00aa615e).cast::<u8>();
                let limit = (&raw const DAT_00aa62ae).cast::<u8>();
                let mut i12: i32 = i16.wrapping_add(s8!());
                loop {
                    if p.cast::<i32>().read_unaligned() == at!(DAT_00aa5cb0, u32, i12) as i32 {
                        break;
                    }
                    p = p.add(0x2a);
                    i15 = i15.wrapping_add(1);
                    if !(p < limit) {
                        break;
                    }
                }
                if i15 != 8 && at!(DAT_00aa5ac0, i32, i12) == -1 {
                    let i1 = i15.wrapping_mul(0x2a);
                    u18 = rdat!(DAT_00aa615e, i1) as u32;
                    if DAT_00aa5e38 != u18 && at!(DAT_00aa5cb0, u32, i12) != 0 {
                        i12 = 0;
                        if 0 < i13 {
                            let mut pu = (&raw mut DAT_00aa5e40).cast::<u32>();
                            loop {
                                if *pu == u18 {
                                    break;
                                }
                                pu = pu.add(1);
                                i12 = i12.wrapping_add(1);
                                if !(i12 < i13) {
                                    break;
                                }
                            }
                        }
                        FUN_004d4cf0(local_10.as_mut_ptr());
                        FUN_004d4d20(local_34.as_mut_ptr().cast());
                        let name = rect!(DAT_00aa6150, i1).cast::<i8>();
                        if i12 == DAT_00aa5e60 {
                            let pc = FUN_004ddc10(rdat!(DAT_00aa6162, i1).wrapping_add(0x9a));
                            sprintf(buf!(), b"%c%s %c%s\0".as_ptr().cast(), 6u32, pc, 1u32, name);
                            i13 = FUN_004d11e0();
                            if i13 == rdat!(DAT_00aa615e, i1) || DAT_00b69e30 == 2 {
                                u18 = (&raw const DAT_00b6a270)
                                    .cast::<u8>()
                                    .byte_add(xcur!(param_1) as usize)
                                    .cast::<u32>()
                                    .read_unaligned();
                            } else {
                                u18 = DAT_00b6a2c0;
                            }
                        } else {
                            let pc = FUN_004ddc10(rdat!(DAT_00aa6162, i1).wrapping_add(0x9a));
                            sprintf(buf!(), b"%s %s\0".as_ptr().cast(), pc, name);
                            u18 = DAT_00b69ccc;
                        }
                        FUN_004db7d0(0xc, buf!(), local_34.as_mut_ptr(), 0, 1, u18);
                        FUN_004d4d20(local_10.as_mut_ptr().cast());
                        local_40 = local_40.wrapping_add(1);
                        i13 = FUN_004d11e0();
                        if (i13 == at!(DAT_00aa5cb0, u32, s8!()) as i32 || DAT_00b69e30 == 2)
                            && FUN_004dc2e0(3) == 0
                        {
                            FUN_004de090(&raw mut local_38, &raw mut local_3c);
                            if (FUN_004dc1e0() != 0 || FUN_004dc200() != 0) && {
                                i13 = FUN_004d6b90(local_34.as_mut_ptr(), local_38, local_3c);
                                i13 != 0
                            } {
                                if DAT_00b69e30 == 2 {
                                    DAT_00aa5e38 = rdat!(DAT_00aa615e, i1) as u32;
                                    FUN_004dc1c0(3);
                                } else {
                                    i13 = 0;
                                    if 0 < DAT_00aa5e60 {
                                        let mut pu = (&raw mut DAT_00aa5e40).cast::<u32>();
                                        loop {
                                            if *pu as i32 == rdat!(DAT_00aa615e, i1) {
                                                break;
                                            }
                                            pu = pu.add(1);
                                            i13 = i13.wrapping_add(1);
                                            if !(i13 < DAT_00aa5e60) {
                                                break;
                                            }
                                        }
                                    }
                                    if i13 == DAT_00aa5e60 {
                                        DAT_00aa6af4 = rdat!(DAT_00aa615e, i1) as u32;
                                        DAT_00aa5e60 = DAT_00aa5e60.wrapping_add(1);
                                        at!(DAT_00aa5e40, u32, i13) = DAT_00aa6af4;
                                        DAT_00aa6af0 = 0x6b;
                                        DAT_00aa6af8 = FUN_004d11e0() as u32;
                                        FUN_004cef70(0, &raw mut DAT_00aa6af0, 0xc);
                                        DAT_00aa5e38 = rdat!(DAT_00aa615e, i1) as u32;
                                        i13 = 0;
                                        FUN_004dc1c0(3);
                                        u18 = DAT_00aa5e38;
                                        if 0 < e10!() {
                                            loop {
                                                i12 = i13.wrapping_add(s8!());
                                                if at!(DAT_00aa5cb0, u32, i12) == u18
                                                    && at!(DAT_00aa5ac0, i32, i12) != -1
                                                {
                                                    DAT_00aa6af0 = 0x50;
                                                    DAT_00aa6af4 = sel!() as u32;
                                                    DAT_00aa6afc = 0xffffffff;
                                                    DAT_00aa6af8 = i13 as u32;
                                                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x14);
                                                    u18 = DAT_00aa5e38;
                                                }
                                                i13 = i13.wrapping_add(1);
                                                if !(i13 < e10!()) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FUN_004d64a0(local_34.as_mut_ptr(), 0, 0x11);
                        i13 = DAT_00aa5e60;
                        if 3 < local_40 {
                            break 'outer;
                        }
                    }
                }
                i16 = i16.wrapping_add(1);
                if !(i16 < e10!()) {
                    break;
                }
            }
        }
        local_40 = 0;
        FUN_004de090(&raw mut local_38, &raw mut local_3c);
        FUN_004d6460(
            local_34.as_mut_ptr(),
            0x58,
            i5.wrapping_add(0x160),
            0xe5,
            i5.wrapping_add(0x170),
        );
        i13 = 0;
        FUN_004d6460(
            local_20.as_mut_ptr(),
            0xe8,
            i5.wrapping_add(0x160),
            0x14c,
            i5.wrapping_add(0x170),
        );
        if 0 < DAT_00aa5aac {
            let mut local_44: *mut i8 = (&raw mut DAT_00a91d75).cast::<i8>();
            loop {
                if *local_44.offset(-0x51) != 0 && (*local_44.offset(-0x5d) as u8 as i32) == sel!()
                {
                    let b9: u32 = match *local_44.offset(-0x5e) {
                        0 => 2,
                        1 | 4 => 3,
                        2 => 5,
                        3 => 4,
                        5 => 6,
                        _ => 1,
                    };
                    let pc6 = local_44.offset(-0xb5);
                    let pc8 =
                        FUN_004ddc10((*local_44.offset(-99) as u8 as i32).wrapping_add(0x261));
                    sprintf(
                        buf!(),
                        b"%u. %c%s %s: %c\0".as_ptr().cast(),
                        local_40.wrapping_add(1) as u32,
                        b9,
                        pc8,
                        pc6,
                        1u32,
                    );
                    let mut src: *mut i8 = local_44;
                    if *local_44 == 0 {
                        src = FUN_004ddc10(0x272);
                    }
                    let mut dst = buf!();
                    while *dst != 0 {
                        dst = dst.add(1);
                    }
                    loop {
                        let c = *src;
                        *dst = c;
                        src = src.add(1);
                        dst = dst.add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    i5 = -1;
                    i16 = 0;
                    FUN_004db7d0(0xc, buf!(), local_34.as_mut_ptr(), 0, 1, 0xffff);
                    if 0 < e10!() {
                        loop {
                            let mut i12 = i16.wrapping_add(s8!());
                            if at!(DAT_00aa5ac0, i32, i12) == i13 {
                                let mut i15: i32 = 0;
                                let mut p = (&raw const DAT_00aa615e).cast::<u8>();
                                let limit = (&raw const DAT_00aa62ae).cast::<u8>();
                                loop {
                                    i5 = i16;
                                    if p.cast::<i32>().read_unaligned()
                                        == at!(DAT_00aa5cb0, u32, i12) as i32
                                    {
                                        let k = i15.wrapping_mul(0x2a);
                                        let name = rect!(DAT_00aa6150, k).cast::<i8>();
                                        let pc =
                                            FUN_004ddc10(rdat!(DAT_00aa6162, k).wrapping_add(0x9a));
                                        sprintf(
                                            buf!(),
                                            b"%c%s %c%s\0".as_ptr().cast(),
                                            6u32,
                                            pc,
                                            1u32,
                                            name,
                                        );
                                        FUN_004d4cf0(local_10.as_mut_ptr());
                                        FUN_004d4d20(local_20.as_mut_ptr().cast());
                                        i12 = FUN_004d11e0();
                                        if i12 == rdat!(DAT_00aa615e, k) || DAT_00b69e30 == 2 {
                                            u18 = (&raw const DAT_00b6a270)
                                                .cast::<u8>()
                                                .byte_add(xcur!(param_1) as usize)
                                                .cast::<u32>()
                                                .read_unaligned();
                                        } else {
                                            u18 = DAT_00b6a2c0;
                                        }
                                        FUN_004db7d0(0xc, buf!(), local_20.as_mut_ptr(), 0, 1, u18);
                                        FUN_004d4d20(local_10.as_mut_ptr().cast());
                                        break;
                                    }
                                    p = p.add(0x2a);
                                    i15 = i15.wrapping_add(1);
                                    if !(p < limit) {
                                        break;
                                    }
                                }
                            }
                            i16 = i16.wrapping_add(1);
                            if !(i16 < e10!()) {
                                break;
                            }
                        }
                    }
                    if 1 < e10!() {
                        if i5 == -1 {
                            let b3 = FUN_004dc2e0(3);
                            if b3 != 0 && (FUN_004dc2a0(3) != 0 || FUN_004dc2a0(3) != 0) && {
                                i5 = FUN_004d6b90(local_20.as_mut_ptr(), local_38, local_3c);
                                i5 != 0
                            } {
                                if DAT_00b69e30 == 2 {
                                    if DAT_00bb73a7 != 0 {
                                        FUN_004de5a0(
                                            b"slotsound\0".as_ptr() as *mut i8,
                                            1,
                                            0,
                                            0xff,
                                        );
                                    }
                                    i5 = 0;
                                    if 0 < e10!() {
                                        loop {
                                            if at!(DAT_00aa5cb0, u32, i5.wrapping_add(s8!()))
                                                == DAT_00aa5e38
                                            {
                                                at!(DAT_00aa5ac0, i32, s8!().wrapping_add(i5)) =
                                                    i13;
                                                break;
                                            }
                                            i5 = i5.wrapping_add(1);
                                            if !(i5 < e10!()) {
                                                break;
                                            }
                                        }
                                    }
                                } else {
                                    DAT_00aa6af8 = 0;
                                    if 0 < e10!() {
                                        loop {
                                            if at!(
                                                DAT_00aa5cb0,
                                                u32,
                                                DAT_00aa6af8.wrapping_add(s8!() as u32)
                                            ) == DAT_00aa5e38
                                            {
                                                break;
                                            }
                                            DAT_00aa6af8 = DAT_00aa6af8.wrapping_add(1);
                                            if !((DAT_00aa6af8 as i32) < e10!()) {
                                                break;
                                            }
                                        }
                                    }
                                    DAT_00aa6af4 = sel!() as u32;
                                    DAT_00aa6af0 = 0x50;
                                    DAT_00aa6afc = i13 as u32;
                                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x10);
                                    DAT_00aa6af0 = 0x6a;
                                    DAT_00aa6af4 = DAT_00aa5e38;
                                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 8);
                                }
                                DAT_00aa5e38 = 0;
                                FUN_004dc1d0();
                            }
                        } else {
                            let b3 = FUN_004dc2e0(3);
                            if b3 == 0 {
                                if (local_24 != 0 || DAT_00b69e30 == 2)
                                    && (FUN_004dc1e0() != 0 || FUN_004dc200() != 0)
                                    && {
                                        i16 =
                                            FUN_004d6b90(local_20.as_mut_ptr(), local_38, local_3c);
                                        i16 != 0
                                    }
                                {
                                    if DAT_00b69e30 == 2 {
                                        DAT_00aa5e38 =
                                            at!(DAT_00aa5cb0, u32, i5.wrapping_add(s8!()));
                                        FUN_004dc1c0(3);
                                        at!(DAT_00aa5ac0, i32, i5.wrapping_add(s8!())) = -1;
                                    } else {
                                        i16 = 0;
                                        if 0 < DAT_00aa5e60 {
                                            let mut pu = (&raw mut DAT_00aa5e40).cast::<u32>();
                                            loop {
                                                if *pu
                                                    == at!(
                                                        DAT_00aa5cb0,
                                                        u32,
                                                        i5.wrapping_add(s8!())
                                                    )
                                                {
                                                    break;
                                                }
                                                pu = pu.add(1);
                                                i16 = i16.wrapping_add(1);
                                                if !(i16 < DAT_00aa5e60) {
                                                    break;
                                                }
                                            }
                                        }
                                        if i16 == DAT_00aa5e60 {
                                            DAT_00aa5e60 = DAT_00aa5e60.wrapping_add(1);
                                            DAT_00aa6af0 = 0x6b;
                                            DAT_00aa6af4 =
                                                at!(DAT_00aa5cb0, u32, i5.wrapping_add(s8!()));
                                            at!(DAT_00aa5e40, u32, i16) = DAT_00aa6af4;
                                            DAT_00aa6af8 = FUN_004d11e0() as u32;
                                            FUN_004cef70(0, &raw mut DAT_00aa6af0, 0xc);
                                            i16 = 0;
                                            DAT_00aa5e38 =
                                                at!(DAT_00aa5cb0, u32, i5.wrapping_add(s8!()));
                                            FUN_004dc1c0(3);
                                            u18 = DAT_00aa5e38;
                                            if 0 < e10!() {
                                                loop {
                                                    i5 = i16.wrapping_add(s8!());
                                                    if at!(DAT_00aa5cb0, u32, i5) == u18
                                                        && at!(DAT_00aa5ac0, i32, i5) != -1
                                                    {
                                                        DAT_00aa6af0 = 0x50;
                                                        DAT_00aa6af4 = sel!() as u32;
                                                        DAT_00aa6afc = 0xffffffff;
                                                        DAT_00aa6af8 = i16 as u32;
                                                        FUN_004cef70(
                                                            0,
                                                            &raw mut DAT_00aa6af0,
                                                            0x14,
                                                        );
                                                        u18 = DAT_00aa5e38;
                                                    }
                                                    i16 = i16.wrapping_add(1);
                                                    if !(i16 < e10!()) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if (FUN_004dc2a0(3) != 0 || FUN_004dc2c0(3) != 0) && {
                                i16 = FUN_004d6b90(local_20.as_mut_ptr(), local_38, local_3c);
                                i16 != 0
                            } {
                                if DAT_00b69e30 == 2 {
                                    if DAT_00bb73a7 != 0 {
                                        FUN_004de5a0(
                                            b"slotsound\0".as_ptr() as *mut i8,
                                            1,
                                            0,
                                            0xff,
                                        );
                                    }
                                    let mut i15: i32 = 0;
                                    let i12 = s8!();
                                    i16 = e10!();
                                    at!(DAT_00aa5ac0, i32, i5.wrapping_add(i12)) = -1;
                                    if 0 < i16 {
                                        loop {
                                            if at!(DAT_00aa5cb0, u32, i15.wrapping_add(i12))
                                                == DAT_00aa5e38
                                            {
                                                at!(DAT_00aa5ac0, i32, i15.wrapping_add(i12)) = i13;
                                                break;
                                            }
                                            i15 = i15.wrapping_add(1);
                                            if !(i15 < i16) {
                                                break;
                                            }
                                        }
                                    }
                                } else {
                                    DAT_00aa6af4 = sel!() as u32;
                                    DAT_00aa6af0 = 0x50;
                                    DAT_00aa6afc = 0xffffffff;
                                    DAT_00aa6af8 = i5 as u32;
                                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x14);
                                    DAT_00aa6af8 = 0;
                                    if 0 < e10!() {
                                        loop {
                                            if at!(
                                                DAT_00aa5cb0,
                                                u32,
                                                DAT_00aa6af8.wrapping_add(s8!() as u32)
                                            ) == DAT_00aa5e38
                                            {
                                                break;
                                            }
                                            DAT_00aa6af8 = DAT_00aa6af8.wrapping_add(1);
                                            if !((DAT_00aa6af8 as i32) < e10!()) {
                                                break;
                                            }
                                        }
                                    }
                                    DAT_00aa6af4 = sel!() as u32;
                                    DAT_00aa6af0 = 0x50;
                                    DAT_00aa6afc = i13 as u32;
                                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x10);
                                    DAT_00aa6af0 = 0x6a;
                                    DAT_00aa6af4 = DAT_00aa5e38;
                                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 8);
                                }
                                DAT_00aa5e38 = 0;
                                FUN_004dc1d0();
                            }
                        }
                    }
                    FUN_004d64a0(local_20.as_mut_ptr(), 0, 0x11);
                    FUN_004d64a0(local_34.as_mut_ptr(), 0, 0x11);
                    local_40 = local_40.wrapping_add(1);
                }
                local_44 = local_44.offset(0x562);
                i13 = i13.wrapping_add(1);
                if !(i13 < DAT_00aa5aac) {
                    break;
                }
            }
        }
        1
    }
}
