// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00524170: u8;
    static mut DAT_00524171: u8;
    static mut DAT_009d8c82: u8;
    static mut DAT_009d8c84: u8;
    static mut DAT_009d8c87: u8;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d8c89: u8;
    static mut DAT_009d8c8f: u8;
    static mut DAT_009d8cd2: u8;
    static mut DAT_009d9156: u8;
    static mut DAT_009d9192: u8;
    static mut DAT_00521f1a: u8;
    static mut DAT_00521f1b: u8;
    static mut DAT_005241e8: u8;
    static mut DAT_005241f4: u8;
}

#[inline(always)]
unsafe fn rd(p: *mut u8, i: u32) -> u8 {
    // SAFETY: p is the address of one of the game's own byte tables; the
    // index is the one the original uses.
    unsafe { p.add(i as usize).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00432fa0(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    // SAFETY: all globals are the game's own tables, read where the original reads.
    unsafe {
        let i1: u32 = (param_1 as u32).wrapping_mul(0x566);
        let u3: u32 = rd(&raw mut DAT_00524170, rd(&raw mut DAT_009d8c82, i1) as u32) as u32;
        let p3 = param_3 as u32;
        match param_2 {
            0 => return 0,
            1 => {
                if param_3 == param_1 {
                    return 1;
                }
            }
            2 => {
                if rd(&raw mut DAT_00524171, p3) as u32 == u3 {
                    return 1;
                }
            }
            3 => {
                if rd(&raw mut DAT_00521f1b, u3.wrapping_mul(0x1a)) == rd(&raw mut DAT_005241e8, p3)
                {
                    return 1;
                }
            }
            4 => {
                if rd(&raw mut DAT_00521f1a, u3.wrapping_mul(0x1a)) == rd(&raw mut DAT_005241f4, p3)
                {
                    return 1;
                }
            }
            5 => {
                if rd(&raw mut DAT_009d8c87, i1) as u32 == p3 {
                    return 1;
                }
            }
            6 => {
                if rd(&raw mut DAT_009d8cd2, i1) as u32 == p3 {
                    return 1;
                }
            }
            7 => {
                let local_val = (&raw mut DAT_009d9192)
                    .add(i1 as usize)
                    .cast::<i32>()
                    .read_unaligned();
                if param_3 == 9 {
                    if local_val != -1 {
                        return 1;
                    }
                } else {
                    if param_3 != 10 {
                        return 1;
                    }
                    if local_val == -1 {
                        return 1;
                    }
                }
            }
            8 => {
                if rd(&raw mut DAT_009d8c8f, i1) as u32 == p3 {
                    return 1;
                }
            }
            9 => {
                if rd(&raw mut DAT_009d8c89, i1) as u32 == p3 {
                    return 1;
                }
            }
            10 => {
                if rd(&raw mut DAT_009d8c84, i1) as u32 == p3 {
                    return 1;
                }
            }
            0xb => return 1,
            0xc => {
                if rd(&raw mut DAT_009d8c88, i1) as u32 == p3 {
                    return 1;
                }
            }
            0xf => {
                if param_3 != param_1 {
                    return 1;
                }
            }
            0x10 => {
                if rd(&raw mut DAT_00524171, p3) as u32 != u3 {
                    return 1;
                }
            }
            0x11 => {
                if rd(&raw mut DAT_00521f1b, u3.wrapping_mul(0x1a)) != rd(&raw mut DAT_005241e8, p3)
                {
                    return 1;
                }
            }
            0x12 => {
                if rd(&raw mut DAT_00521f1a, u3.wrapping_mul(0x1a)) != rd(&raw mut DAT_005241f4, p3)
                {
                    return 1;
                }
            }
            0x13 => {
                if rd(&raw mut DAT_009d8c87, i1) as u32 != p3 {
                    return 1;
                }
            }
            0x14 => {
                if rd(&raw mut DAT_009d8c8f, i1) as u32 != p3 {
                    return 1;
                }
            }
            0x15 => {
                if rd(&raw mut DAT_009d8c88, i1) as u32 != p3 {
                    return 1;
                }
            }
            0x17 => {
                if rd(&raw mut DAT_009d9156, i1) as u32 == p3 {
                    return 1;
                }
            }
            0x18 => {
                if rd(&raw mut DAT_009d9156, i1) as u32 != p3 {
                    return 1;
                }
            }
            _ => {}
        }
        0
    }
}
