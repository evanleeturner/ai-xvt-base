// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007092b0: u16;
    static mut DAT_0071bb19: [u8; 1];
    static mut DAT_0071bb1a: [u8; 1];
    static mut DAT_0071bae7: [u8; 1];
    static mut DAT_0070ac48: [u8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b73c: [u8; 1];
    static mut DAT_0071e522: u8;
    static mut DAT_0072f508: [u8; 1];
    static mut DAT_00732fe2: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a3490(param_1: i32, param_2: *mut core::ffi::c_void);
    fn FUN_004a1d70(param_1: i32, param_2: i32);
    fn FUN_00432cc0(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00440d20(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets are computed exactly as the original does.
    unsafe {
        let b19 = (&raw mut DAT_0071bb19).cast::<u8>();
        let b1a = (&raw mut DAT_0071bb1a).cast::<u8>();
        let ae7 = (&raw mut DAT_0071bae7).cast::<u8>();
        let sel = DAT_007092b0;
        let i2 = (param_1 as u32).wrapping_mul(0x5bd) as i32 as isize;

        match sel {
            8 => {
                if *b19.offset(i2) != 0 {
                    let b4 = (*b19.offset(i2)).wrapping_sub(1);
                    *b19.offset(i2) = b4;
                    *ae7.offset(i2 + b4 as isize) = 0x5f;
                    *ae7.offset(1 + *b19.offset(i2) as isize + i2) = 0;
                }
                FUN_004a3490(0, ae7.offset(i2).cast());
                FUN_004a1d70((*b1a.offset(i2) as u32).wrapping_add(0x16c) as i32, param_1);
                return;
            }
            9 => {
                *b1a.offset(i2) = (*b1a.offset(i2)).wrapping_add(1);
                if 3 < *b1a.offset(i2) {
                    *b1a.offset(i2) = 1;
                }
                FUN_004a3490(0, ae7.offset(i2).cast());
                FUN_004a1d70((*b1a.offset(i2) as u32).wrapping_add(0x16c) as i32, param_1);
                return;
            }
            0xd => {
                *ae7.offset(i2 + *b19.offset(i2) as isize) = 0;
                let p5 = FUN_00432cc0(param_1);
                FUN_004a3490(0, p5.cast());
                let base = (&raw mut DAT_0071b73c).cast::<u8>();
                let mut p6 = base;
                FUN_004a3490(1, ae7.offset(i2).cast());
                let mut local_8: i32 = 0;
                loop {
                    if *p6.add(4) != 0 {
                        let mut b3 = false;
                        DAT_00732fe2 = 3;
                        let m = *b1a.offset(i2);
                        if m == 1 {
                            let a = base.offset(i2).cast::<u16>().read_unaligned();
                            let b = p6.cast::<u16>().read_unaligned();
                            if a == b {
                                b3 = true;
                            }
                        } else {
                            let mut skip = false;
                            if m == 2 {
                                let a = base.offset(i2).cast::<u16>().read_unaligned();
                                let b = p6.cast::<u16>().read_unaligned();
                                let b7 = if a == b {
                                    false
                                } else {
                                    let idx =
                                        (a as u32).wrapping_add((b as u32).wrapping_mul(0x1e5));
                                    *(&raw mut DAT_0072f508).cast::<u8>().add(idx as usize) == 0
                                };
                                if !b7 {
                                    skip = true;
                                }
                            }
                            if !skip {
                                b3 = true;
                            }
                        }
                        if b3 {
                            FUN_004a1d70(0x16c, local_8);
                        }
                    }
                    p6 = p6.add(0x5bd);
                    local_8 = local_8.wrapping_add(1);
                    if ((&raw mut DAT_0071e522) as usize).wrapping_add(1) < p6 as usize {
                        *b1a.offset(i2) = 0;
                        FUN_004a1d70(0x170, param_1);
                        return;
                    }
                }
            }
            0x1b => {
                *b1a.offset(i2) = 0;
                FUN_004a1d70(0x171, param_1);
            }
            _ => {
                if sel != 0 {
                    if *b19.offset(i2) < 0x30 {
                        *ae7.offset(i2 + *b19.offset(i2) as isize) = sel as u8;
                        *ae7.offset(1 + *b19.offset(i2) as isize + i2) = 0x5f;
                        *ae7.offset(2 + *b19.offset(i2) as isize + i2) = 0;
                        *b19.offset(i2) = (*b19.offset(i2)).wrapping_add(1);
                    }
                    FUN_004a3490(0, ae7.offset(i2).cast());
                    FUN_004a1d70((*b1a.offset(i2) as u32).wrapping_add(0x16c) as i32, param_1);
                    return;
                }
                if param_1 == DAT_0071ecf4 {
                    let off = (param_1 as u32).wrapping_mul(0x10) as usize;
                    let v = (&raw mut DAT_0070ac48)
                        .cast::<u8>()
                        .add(off)
                        .cast::<i16>()
                        .read_unaligned();
                    if v < 0xec {
                        FUN_004a3490(0, ae7.offset(i2).cast());
                        FUN_004a1d70((*b1a.offset(i2) as u32).wrapping_add(0x16c) as i32, param_1);
                        return;
                    }
                }
            }
        }
    }
}
