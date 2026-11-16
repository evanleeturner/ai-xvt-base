// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_006d9980: i16;
    static mut DAT_00732e8c: i32;
    static mut DAT_0070acc8: i32;
    static mut DAT_00525534: i32;
    static mut DAT_00525518: i32;
    static mut DAT_007325e0: [u16; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a6a80();
    fn FUN_004a6f90();
}

unsafe fn fill_span(row: u32, start: i32, end: i32, c: u8) {
    // SAFETY: the globals are the game's own; the pointer written through is
    // the game's frame buffer address computed exactly as the original does.
    unsafe {
        if DAT_00525518 == 2 {
            let v = (&raw const DAT_007325e0)
                .cast::<u16>()
                .add(c as usize)
                .read();
            if start < end {
                let mut p = (DAT_0070acc8 as u32)
                    .wrapping_mul(row)
                    .wrapping_add(DAT_00525534 as u32)
                    .wrapping_add((start as u32).wrapping_mul(2))
                    as *mut u8;
                let diff = (end as u32).wrapping_sub(start as u32);
                let mut n = diff >> 1;
                while n != 0 {
                    p.cast::<u32>()
                        .write_unaligned(((v as u32) << 16) | (v as u32));
                    p = p.byte_add(4);
                    n -= 1;
                }
                let mut n = (diff & 1 != 0) as u32;
                while n != 0 {
                    p.cast::<u16>().write_unaligned(v);
                    p = p.byte_add(2);
                    n -= 1;
                }
            }
        } else if start < end {
            let mut p = (DAT_0070acc8 as u32)
                .wrapping_mul(row)
                .wrapping_add(DAT_00525534 as u32)
                .wrapping_add(start as u32) as *mut u8;
            let diff = (end as u32).wrapping_sub(start as u32);
            let mut n = diff >> 2;
            while n != 0 {
                p.cast::<u32>()
                    .write_unaligned((c as u32).wrapping_mul(0x01010101));
                p = p.byte_add(4);
                n -= 1;
            }
            let mut n = diff & 3;
            while n != 0 {
                p.write(c);
                p = p.byte_add(1);
                n -= 1;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004354e0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u8,
) {
    // SAFETY: the globals and callees are the game's own; spans are drawn
    // into the game's frame buffer exactly where the original does.
    unsafe {
        let uvar3 = (param_3 as u32).wrapping_add(param_1 as u32);
        if !((0 < param_2.wrapping_add(param_4))
            && (0 < uvar3 as i32)
            && (param_1 < DAT_007091ae as i32)
            && (param_2 < DAT_006d4984 as i32)
            && (0 < param_4)
            && (0 < param_3))
        {
            return;
        }
        let mut ivar7 = param_3 >> 3;
        let mut local_8 = param_4 >> 3;
        if ivar7 < 3 {
            ivar7 = 3;
        }
        if local_8 < 3 {
            local_8 = 3;
        }
        if param_3 < ivar7 {
            ivar7 = param_3;
        }
        if param_4 < local_8 {
            local_8 = param_4;
        }
        FUN_004a6a80();
        if -1 < param_2 {
            let mut uvar5 = (ivar7 as u32).wrapping_add(param_1 as u32);
            let uvar2 = DAT_007091ae as u32;
            if 0 < uvar5 as i32 && param_1 < uvar2 as i32 {
                let mut ivar4 = param_1;
                if param_1 < 0 {
                    ivar4 = 0;
                }
                if (uvar2 as i32) < uvar5 as i32 {
                    uvar5 = uvar2;
                }
                let s = (DAT_006d9980 as i32 as u32).wrapping_add(ivar4 as u32) as i32;
                let e = (DAT_006d9980 as i32 as u32).wrapping_add(uvar5) as i32;
                let row = (param_2 as u32).wrapping_add(DAT_00732e8c as u32);
                fill_span(row, s, e, param_5);
            }
            let mut ivar4 =
                (param_3 as u32).wrapping_add((param_1 as u32).wrapping_sub(ivar7 as u32)) as i32;
            let uvar5 = DAT_007091ae as u32;
            if 0 < uvar3 as i32 && ivar4 < uvar5 as i32 {
                if ivar4 < 0 {
                    ivar4 = 0;
                }
                let mut uvar2 = uvar3;
                if (uvar5 as i32) < uvar3 as i32 {
                    uvar2 = uvar5;
                }
                let s = (DAT_006d9980 as i32 as u32).wrapping_add(ivar4 as u32) as i32;
                let e = (DAT_006d9980 as i32 as u32).wrapping_add(uvar2) as i32;
                let row = (param_2 as u32).wrapping_add(DAT_00732e8c as u32);
                fill_span(row, s, e, param_5);
            }
        }
        if param_2.wrapping_add(param_4) <= DAT_006d4984 as i32 {
            let mut uvar5 = (ivar7 as u32).wrapping_add(param_1 as u32);
            let uvar2 = DAT_007091ae as u32;
            if 0 < uvar5 as i32 && param_1 < uvar2 as i32 {
                let mut ivar4 = param_1;
                if param_1 < 0 {
                    ivar4 = 0;
                }
                if (uvar2 as i32) < uvar5 as i32 {
                    uvar5 = uvar2;
                }
                let e = uvar5.wrapping_add(DAT_006d9980 as i32 as u32) as i32;
                let s = (DAT_006d9980 as i32 as u32).wrapping_add(ivar4 as u32) as i32;
                let row = (DAT_00732e8c as u32)
                    .wrapping_add(param_4 as u32)
                    .wrapping_sub(1)
                    .wrapping_add(param_2 as u32);
                fill_span(row, s, e, param_5);
            }
            let mut ivar7b =
                (param_3 as u32).wrapping_add((param_1 as u32).wrapping_sub(ivar7 as u32)) as i32;
            let uvar5 = DAT_007091ae as u32;
            if 0 < uvar3 as i32 && ivar7b < uvar5 as i32 {
                if ivar7b < 0 {
                    ivar7b = 0;
                }
                let mut uvar2 = uvar3;
                if (uvar5 as i32) < uvar3 as i32 {
                    uvar2 = uvar5;
                }
                let e = uvar2.wrapping_add(DAT_006d9980 as i32 as u32) as i32;
                let s = (DAT_006d9980 as i32 as u32).wrapping_add(ivar7b as u32) as i32;
                let row = (DAT_00732e8c as u32)
                    .wrapping_add(param_4 as u32)
                    .wrapping_sub(1)
                    .wrapping_add(param_2 as u32);
                fill_span(row, s, e, param_5);
            }
        }
        let mut ivar7 = 1i32;
        if 1 < local_8 {
            loop {
                let y = param_2.wrapping_add(ivar7);
                if -1 < y && y < DAT_006d4984 as i32 {
                    if -1 < param_1 {
                        let s = (param_1 as u32).wrapping_add(DAT_006d9980 as i32 as u32) as i32;
                        let row = (DAT_00732e8c as u32)
                            .wrapping_add(ivar7 as u32)
                            .wrapping_add(param_2 as u32);
                        fill_span(row, s, s.wrapping_add(1), param_5);
                    }
                    if uvar3 as i32 <= DAT_007091ae as i32 {
                        let row = (DAT_00732e8c as u32)
                            .wrapping_add(ivar7 as u32)
                            .wrapping_add(param_2 as u32);
                        let e = (param_3 as u32)
                            .wrapping_add(DAT_006d9980 as i32 as u32)
                            .wrapping_add(param_1 as u32) as i32;
                        fill_span(row, e.wrapping_sub(1), e, param_5);
                    }
                }
                ivar7 += 1;
                if !(ivar7 < local_8) {
                    break;
                }
            }
        }
        let mut ivar7 = param_4.wrapping_sub(local_8);
        while ivar7 < param_4 - 1 {
            let y = param_2.wrapping_add(ivar7);
            if local_8 <= ivar7 && -1 < y && y < DAT_006d4984 as i32 {
                if -1 < param_1 {
                    let s = (param_1 as u32).wrapping_add(DAT_006d9980 as i32 as u32) as i32;
                    let row = (DAT_00732e8c as u32)
                        .wrapping_add(ivar7 as u32)
                        .wrapping_add(param_2 as u32);
                    fill_span(row, s, s.wrapping_add(1), param_5);
                }
                if uvar3 as i32 <= DAT_007091ae as i32 {
                    let e = (param_3 as u32)
                        .wrapping_add(DAT_006d9980 as i32 as u32)
                        .wrapping_add(param_1 as u32) as i32;
                    let row = (DAT_00732e8c as u32)
                        .wrapping_add(ivar7 as u32)
                        .wrapping_add(param_2 as u32);
                    fill_span(row, e.wrapping_sub(1), e, param_5);
                }
            }
            ivar7 += 1;
        }
        FUN_004a6f90();
    }
}
