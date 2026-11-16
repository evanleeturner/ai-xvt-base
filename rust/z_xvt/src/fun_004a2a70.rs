// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070ac40: i16;
    static mut DAT_0070ac46: i16;
    static mut DAT_0070ac48: i16;
    static mut DAT_0070ac4a: i16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d9990: i16;
    static mut DAT_00645b8c: i32;
    static mut DAT_006559b4: i16;
    static mut DAT_0070a178: u8;
    static mut DAT_006d99e4: i16;
    static mut DAT_006d9999: u8;
    static mut DAT_00525508: u8;
    static mut DAT_006da9ec: i32;
    static mut DAT_006da538: i32;
    static mut DAT_006db594: i32;
    static mut DAT_00525518: i32;
    static mut DAT_0052571c: i32;
    static mut DAT_00703ad0: i32;
    static mut DAT_006da840: i32;
    static mut DAT_006da4e4: i32;
    static mut DAT_00703c80: *const core::ffi::c_void;
    static mut DAT_00645ae0: i16;
    static mut DAT_006d9d30: i32;
    static mut DAT_00703c78: i32;
    static mut DAT_006db714: i32;
    static mut DAT_006db710: i32;
    static mut DAT_00645b38: i16;
    static mut DAT_0071e530: i32;
    static mut DAT_0071ecf8: i32;
    static mut DAT_00703c74: i32;
    static mut DAT_007092b4: i32;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071bb37: i8;
    static mut DAT_0070a014: u32;
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a2440(param_1: u16);
    fn FUN_0048eda0(param_1: u32);
    fn FUN_00461f20(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_0048ed00(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_004a3830(param_1: u32, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

macro_rules! redraw {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        FUN_0048eda0(DAT_00525508 as u32);
        FUN_00461f20(
            DAT_006da9ec,
            DAT_006da538,
            DAT_006db594,
            (DAT_00525518 as u32).wrapping_mul(DAT_006da538 as u32) as i32,
        );
        FUN_0048ed00($a as i16, $b as i16, $c as i16, $d as i16);
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
        f();
        FUN_00461f20(0, 0x140, 200, 0);
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a2a70() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets index the game's record tables exactly as the original does.
    unsafe {
        let idx = DAT_0071ecf4 as u32;
        let a = (&raw mut DAT_0070ac40)
            .byte_add(idx.wrapping_mul(0x10) as usize)
            .read_unaligned();
        if ((a == 0 && DAT_006d9990 != -1) || DAT_00645b8c != 0) && DAT_006559b4 == 0 {
            if DAT_0070a178 == 0 {
                redraw!(DAT_0052571c, DAT_00703ad0, DAT_006da840, DAT_006da4e4);
                DAT_006d9990 = -1;
            } else {
                let mut u6: u16 = 0;
                loop {
                    let u7 = u6.wrapping_add(1);
                    let mut p8 = (&raw mut DAT_006d99e4)
                        .byte_add((u6 as usize).wrapping_mul(0x54))
                        .cast::<u32>();
                    let mut p9 = (&raw mut DAT_006d9990)
                        .byte_add((u6 as usize).wrapping_mul(0x54))
                        .cast::<u32>();
                    let mut i = 0x15;
                    while i != 0 {
                        p9.write_unaligned(p8.read_unaligned());
                        p8 = p8.add(1);
                        p9 = p9.add(1);
                        i -= 1;
                    }
                    u6 = u7;
                    if !(u7 < DAT_0070a178 as u16) {
                        break;
                    }
                }
                DAT_0070a178 = DAT_0070a178.wrapping_sub(1);
                FUN_004a2440(DAT_006d9999 as u16);
            }
        }
        let idx = DAT_0071ecf4 as u32;
        let b = (&raw mut DAT_0070ac46)
            .byte_add(idx.wrapping_mul(0x10) as usize)
            .read_unaligned();
        if (b == 0 && DAT_00645ae0 != -1) || DAT_00645b8c != 0 {
            redraw!(DAT_006d9d30, DAT_00703c78, DAT_006db714, DAT_006db710);
            DAT_00645ae0 = -1;
        }
        let idx = DAT_0071ecf4 as u32;
        let c = (&raw mut DAT_0070ac48)
            .byte_add(idx.wrapping_mul(0x10) as usize)
            .read_unaligned();
        if (c == 0 && DAT_00645b38 != -1) || DAT_00645b8c != 0 {
            redraw!(DAT_0071e530, DAT_0071ecf8, DAT_00703c74, DAT_007092b4);
            DAT_00645b38 = -1;
        }
        let idx = DAT_0071ecf4 as u32;
        let d = (&raw mut DAT_0070ac4a)
            .byte_add(idx.wrapping_mul(0x10) as usize)
            .read_unaligned();
        if d == 0 {
            let s2 = (&raw mut DAT_0071b74a)
                .byte_add(idx.wrapping_mul(0x5bd) as usize)
                .cast::<i16>()
                .read_unaligned();
            if s2 != -1 {
                let u3 = FUN_004a3830(
                    ((idx.wrapping_mul(0x16f) >> 16) << 16) | (s2 as u16 as u32),
                    DAT_0071ecf4,
                    0,
                    0,
                );
                if u3 != DAT_0070a014 {
                    let idx = DAT_0071ecf4 as u32;
                    let st = (&raw mut DAT_0071b74a)
                        .byte_add(idx.wrapping_mul(0x5bd) as usize)
                        .cast::<i16>()
                        .read_unaligned();
                    let u3 =
                        FUN_004a3830(((idx >> 16) << 16) | (st as u16 as u32), DAT_0071ecf4, 0, 1);
                    if u3 != 0 {
                        let idx = DAT_0071ecf4 as u32;
                        let c1 = (&raw mut DAT_0071bb37)
                            .byte_add(idx.wrapping_mul(0x5bd) as usize)
                            .read();
                        if c1 == 0x13 || c1 == 0 || c1 == 0x14 {
                            let st = (&raw mut DAT_0071b74a)
                                .byte_add(idx.wrapping_mul(0x5bd) as usize)
                                .cast::<i16>()
                                .read_unaligned();
                            DAT_0070a014 = FUN_004a3830(
                                ((idx.wrapping_mul(0x5bd) >> 16) << 16) | (st as u16 as u32),
                                DAT_0071ecf4,
                                1,
                                0,
                            );
                            let idx = DAT_0071ecf4 as u32;
                            (&raw mut DAT_0070ac4a)
                                .byte_add(idx.wrapping_mul(0x10) as usize)
                                .write_unaligned(0x49c);
                        }
                    }
                }
            }
        }
        let mut pc = &raw mut DAT_0071b740;
        loop {
            if pc.read() != 0 {
                let iv = pc.byte_add(0x59d).cast::<i32>().read_unaligned();
                if iv == 0 {
                    pc.byte_add(0x36).write(0);
                }
            }
            pc = pc.byte_add(0x5bd);
            if !((pc as usize) < ((&raw mut DAT_0071e528) as usize)) {
                break;
            }
        }
        if DAT_00645b8c != 0 {
            DAT_00645b8c = 0;
        }
    }
}
