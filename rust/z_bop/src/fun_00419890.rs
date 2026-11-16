// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a1fe0: i32;
    static mut DAT_00a07cec: i32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d1308: i32;
    static mut DAT_009d6940: [u8; 2];
    static mut DAT_00a00854: i32;
    static mut DAT_009a8e18: *mut u8;
    static mut DAT_009e9640: u32;
    static mut DAT_0051a44f: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041b830(param_1: *mut u8, param_2: u16, param_3: u16);
}

#[inline(always)]
unsafe fn rd_i32(p: *const u8, off: usize) -> i32 {
    // SAFETY: caller supplies a pointer into the game's own records.
    unsafe { p.byte_add(off).cast::<i32>().read_unaligned() }
}

#[inline(always)]
unsafe fn rd_ptr(p: *const u8, off: usize) -> *mut u8 {
    // SAFETY: caller supplies a pointer into the game's own records.
    unsafe { p.byte_add(off).cast::<u32>().read_unaligned() as usize as *mut u8 }
}

#[inline(always)]
unsafe fn rd_u16(p: *const u8, off: usize) -> u16 {
    // SAFETY: caller supplies a pointer into the game's own records.
    unsafe { p.byte_add(off).cast::<u16>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419890(param_1: *mut u8, param_2: u16) {
    // SAFETY: all globals and the records they point at are the game's own
    // memory; reads and calls happen in the original's order.
    unsafe {
        let ivar9: *mut u8 = DAT_009a1fe8.byte_add((param_2 as u32).wrapping_mul(0x23) as usize);
        let ivar5 = rd_i32(ivar9, 0x1b);

        if ivar5 == -1 {
            let puvar1 = rd_ptr(ivar9, 0x1f);
            let uvar12: u16 = rd_u16(puvar1, 0x85);

            match *ivar9.byte_add(2) {
                0..=2 => {
                    if DAT_009d6940[1] == 0 {
                        let mut uvar10 = DAT_00a07cec as u16;
                        if ((DAT_00a07cec & 0xffff) as i32) < DAT_009a1fe0 {
                            loop {
                                let ivar4 = (uvar10 as u32).wrapping_mul(0x23) as usize;
                                let ivar6 = DAT_009a1fe8.byte_add(ivar4);
                                if *DAT_009a1fe8.byte_add(0x16 + ivar4) != *ivar9.byte_add(0x16)
                                    && *ivar6.byte_add(3) != 0
                                {
                                    let ivar7 = rd_i32(ivar6, 0x1b);
                                    let puvar2 = rd_ptr(ivar6, 0x1f);
                                    if ivar7 != -1 && !puvar2.is_null() {
                                        FUN_0041b830(puvar2.byte_add(0x12), uvar10, param_2);
                                    }
                                    let cvar2 = *DAT_009a1fe8.byte_add(2 + ivar4) as i8;
                                    if cvar2 == 4 || cvar2 == 3 || cvar2 == 5 {
                                        let puvar2 = rd_ptr(DAT_009a1fe8.byte_add(ivar4), 0x1f);
                                        if !puvar2.is_null() {
                                            let puvar3 = rd_ptr(puvar2, 0xa9);
                                            if !puvar3.is_null()
                                                && *puvar3.byte_add(0x5e) != 0x1e
                                                && rd_u16(puvar3, 0x68) as i16 == -1
                                            {
                                                FUN_0041b830(
                                                    puvar2.byte_add(0x12),
                                                    uvar10,
                                                    param_2,
                                                );
                                            }
                                        }
                                    }
                                }
                                uvar10 = uvar10.wrapping_add(1);
                                if !((uvar10 as i32) < DAT_009a1fe0) {
                                    break;
                                }
                            }
                        }
                        uvar10 = DAT_00a07ce8 as u16;
                        if ((DAT_00a07ce8 & 0xffff) as i32)
                            < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32
                        {
                            loop {
                                let bvar1 = *DAT_009a1fe8
                                    .byte_add((uvar10 as u32).wrapping_mul(0x23) as usize + 3);
                                if 99 < bvar1 && bvar1 < 0x6a {
                                    FUN_0041b830(param_1, param_2, uvar10);
                                }
                                uvar10 = uvar10.wrapping_add(1);
                                if !((uvar10 as i32)
                                    < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32)
                                {
                                    break;
                                }
                            }
                            return;
                        }
                    }
                }
                3..=5 => {
                    let puvar1 = rd_ptr(ivar9, 0x1f);
                    let puvar2 = rd_ptr(puvar1, 0xa9);
                    if *puvar2.byte_add(0x5e) != 0x1e
                        && rd_u16(puvar2, 0x68) as i16 == -1
                        && ((DAT_00a07cec & 0xffff) as i32) < DAT_009a1fe0
                    {
                        let mut uvar10 = DAT_00a07cec as u16;
                        loop {
                            if param_2 != uvar10 {
                                let ivar4 = (uvar10 as u32).wrapping_mul(0x23) as usize;
                                let ivar6 = DAT_009a1fe8.byte_add(ivar4);
                                if *ivar6.byte_add(3) != 0 {
                                    let ivar7 = rd_i32(ivar6, 0x1b);
                                    if ivar7 == -1 {
                                        if *ivar6.byte_add(2) != 0xd {
                                            FUN_0041b830(param_1, param_2, uvar10);
                                            let cvar2 = *DAT_009a1fe8.byte_add(2 + ivar4) as i8;
                                            if cvar2 == 4 || cvar2 == 3 || cvar2 == 5 {
                                                let puvar1 =
                                                    rd_ptr(DAT_009a1fe8.byte_add(ivar4), 0x1f);
                                                if !puvar1.is_null() {
                                                    let puvar2 = rd_ptr(puvar1, 0xa9);
                                                    if !puvar2.is_null()
                                                        && *puvar2.byte_add(0x5e) != 0x1e
                                                        && rd_u16(puvar2, 0x68) as i16 == -1
                                                    {
                                                        FUN_0041b830(
                                                            puvar1.byte_add(0x12),
                                                            uvar10,
                                                            param_2,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let puvar1 = rd_ptr(ivar6, 0x1f);
                                        if !puvar1.is_null() {
                                            FUN_0041b830(puvar1.byte_add(0x12), uvar10, param_2);
                                        }
                                    }
                                }
                            }
                            uvar10 = uvar10.wrapping_add(1);
                            if DAT_009a1fe0 <= uvar10 as i32 {
                                return;
                            }
                        }
                    }
                }
                6 | 7 => {
                    let mut uvar10 = DAT_00a07cec as u16;
                    let uvar3: u16 = rd_u16(
                        DAT_009a8e18,
                        4usize.wrapping_add(
                            (param_2 as u32).wrapping_sub(DAT_009e9640).wrapping_mul(10) as usize,
                        ),
                    );
                    if ((DAT_00a07cec & 0xffff) as i32) < DAT_00a00854 {
                        loop {
                            let ivar6 =
                                DAT_009a1fe8.byte_add((uvar10 as u32).wrapping_mul(0x23) as usize);
                            let uvar8 = uvar10;
                            let mut bvar6 = 0i32;
                            if *ivar6.byte_add(3) != 0 {
                                if (uvar10 as i32) < DAT_009a1fe0 {
                                    bvar6 = 1;
                                } else if *(&raw mut DAT_0051a44f)
                                    .cast::<u8>()
                                    .add(*ivar6.byte_add(3) as usize)
                                    != 0
                                    && param_2 != uvar8
                                {
                                    let puvar1 = rd_ptr(ivar6, 0x1f);
                                    let uvar11 = rd_u16(puvar1, 0x85);
                                    if uvar11 != uvar12 {
                                        bvar6 = 1;
                                    }
                                }
                            }
                            if bvar6 != 0 && uvar12 != uvar8 && *ivar6.byte_add(2) != 0xd {
                                let mut bvar5 = 0i32;
                                let uvar7 = uvar3 as u32;
                                if (uvar7 as i32) < DAT_009a1fe0 && DAT_00a07cec <= uvar7 as i32 {
                                    let ivar7 = rd_i32(
                                        DAT_009a1fe8,
                                        uvar7.wrapping_mul(0x23).wrapping_add(0x1b) as usize,
                                    );
                                    if ivar7 != -1 {
                                        bvar5 = 1;
                                    }
                                }
                                if bvar5 == 0 {
                                    let ivar7 = rd_i32(
                                        DAT_009a1fe8,
                                        (uvar12 as u32).wrapping_mul(0x23).wrapping_add(0x1b)
                                            as usize,
                                    );
                                    if ivar7 != -1 || uvar3 == uvar8 {
                                        bvar5 = 1;
                                    }
                                }
                                if bvar5 != 0 {
                                    FUN_0041b830(param_1, param_2, uvar10);
                                }
                            }
                            uvar10 = uvar8.wrapping_add(1);
                            if !((uvar8.wrapping_add(1) as i32) < DAT_00a00854) {
                                break;
                            }
                        }
                    }
                    if DAT_009d6940[1] == 0 {
                        uvar10 = DAT_00a07ce8 as u16;
                        if ((DAT_00a07ce8 & 0xffff) as i32)
                            < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32
                        {
                            loop {
                                if *DAT_009a1fe8
                                    .byte_add((uvar10 as u32).wrapping_mul(0x23) as usize + 3)
                                    != 0
                                {
                                    FUN_0041b830(param_1, param_2, uvar10);
                                }
                                uvar10 = uvar10.wrapping_add(1);
                                if !((uvar10 as i32)
                                    < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32)
                                {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            let puvar1 = rd_ptr(ivar9, 0x1f);
            let puvar2 = rd_ptr(puvar1, 0xa9);
            let uvar11 = rd_u16(puvar2, 0x1e5);
            let mut uvar10 = DAT_00a07cec as u16;
            if uvar11 != 0 && ((DAT_00a07cec & 0xffff) as i32) < DAT_009a1fe0 {
                loop {
                    let ivar6 = DAT_009a1fe8.byte_add((uvar10 as u32).wrapping_mul(0x23) as usize);
                    if *ivar6.byte_add(3) != 0
                        && param_2 != uvar10
                        && *ivar6.byte_add(2) != 0xd
                        && *ivar9.byte_add(0x16) != *ivar6.byte_add(0x16)
                    {
                        FUN_0041b830(param_1, param_2, uvar10);
                    }
                    uvar10 = uvar10.wrapping_add(1);
                    if !((uvar10 as i32) < DAT_009a1fe0) {
                        break;
                    }
                }
            }
            if DAT_009d6940[1] == 0 {
                uvar10 = DAT_00a07ce8 as u16;
                if ((DAT_00a07ce8 & 0xffff) as i32)
                    < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32
                {
                    loop {
                        if *DAT_009a1fe8.byte_add((uvar10 as u32).wrapping_mul(0x23) as usize + 3)
                            != 0
                        {
                            FUN_0041b830(param_1, param_2, uvar10);
                        }
                        uvar10 = uvar10.wrapping_add(1);
                        if !((uvar10 as i32)
                            < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32)
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
}
