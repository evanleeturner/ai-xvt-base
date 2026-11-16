// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081c012: [u8; 4];
    static mut DAT_0081d326: [u8; 4];
    static mut DAT_0081fb96: [u8; 4];

    static mut DAT_00757680: [i32; 1];
    static mut DAT_00757870: [i32; 1];

    static mut DAT_00757c80: i32;
    static mut DAT_00757c84: i32;
    static mut DAT_00757c88: i32;
    static mut DAT_00757c8c: i32;
    static mut DAT_00757c90: i32;
    static mut DAT_00757c94: u32;
    static mut DAT_00757c98: u32;
    static mut DAT_00757c9c: i32;
    static mut DAT_00757ca4: i32;
    static mut DAT_00757cac: i32;
    static mut DAT_00757cb0: i32;
    static mut DAT_00757cb4: i32;
    static mut DAT_00757cb8: i32;

    static mut DAT_007438d2: [u8; 1];
    static mut DAT_007438d3: [u8; 1];
    static mut DAT_007438d5: [u8; 1];
    static mut DAT_007438d6: [u8; 1];
    static mut DAT_007438e1: [u8; 1];
    static mut DAT_00743da3: [u8; 1];
    static mut DAT_00743db0: [u8; 1];
    static mut DAT_00743db8: [u8; 1];
    static mut DAT_00743dbe: [u8; 1];
    static mut DAT_00743dc2: [u8; 1];
    static mut DAT_00743dc3: [u8; 1];
    static mut DAT_00743dcc: [u8; 1];
    static mut DAT_00743dd6: [u8; 1];

    static mut DAT_005333e8: [u8; 1];
    static mut DAT_005333fc: [u32; 1];
    static mut DAT_0053340c: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cdbc0() -> i32;
    fn FUN_004ecce0(param_1: i32) -> u32;
}

unsafe fn rb(p: *mut u8, i: i32) -> u8 {
    // SAFETY: p points into the game's own table; the index is the original's.
    unsafe { p.offset(i as isize).read() }
}

unsafe fn ri(p: *mut i32, i: i32) -> i32 {
    // SAFETY: p points into the game's own table; the index is the original's.
    unsafe { p.offset(i as isize).read() }
}

unsafe fn count(p: *mut u8, base: i32, n: i32) -> i32 {
    let mut c = 0i32;
    let mut i = 0i32;
    loop {
        // SAFETY: reads the game's own table at the original's offsets.
        if unsafe { rb(p, i.wrapping_add(base)) } != 0 {
            c = c.wrapping_add(1);
        }
        i += 1;
        if i >= n {
            break;
        }
    }
    c
}

unsafe fn reset() {
    // SAFETY: writes the game's own globals.
    unsafe {
        DAT_00757c84 = 0;
        DAT_00757c90 = 0;
        DAT_00757ca4 = 0;
        DAT_00757cac = 0;
        DAT_00757cb8 = 0;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ec560() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // table indices follow the original's arithmetic.
    unsafe {
        let local_bfb6 = i32::from_ne_bytes(DAT_0081bfb6);
        let row = ((local_bfb6 as u32).wrapping_mul(8)) as i32;

        if DAT_0081b850 == 2 {
            let idx = ((local_bfb6 as u32).wrapping_mul(0x20) as i32) / 4;
            DAT_00757cb0 = ri((&raw mut DAT_00757680).cast::<i32>(), idx);
        } else {
            let mut i5: i32 = 0;
            loop {
                let i6 = ri((&raw mut DAT_00757870).cast::<i32>(), i5.wrapping_add(row));
                let i3 = FUN_004cdbc0();
                if i6 == i3 {
                    DAT_00757cb0 = ri((&raw mut DAT_00757680).cast::<i32>(), i5.wrapping_add(row));
                }
                i5 += 1;
                if i5 >= 8 {
                    break;
                }
            }
        }
        DAT_00757ca4 = 0;
        DAT_00757cac = 0;
        DAT_00757c84 = 0;
        DAT_00757c90 = 0;
        DAT_00757cb8 = 0;
        let i5 = (DAT_00757cb0 as u32).wrapping_mul(0x562) as i32;
        DAT_00757c94 = rb((&raw mut DAT_007438e1).cast::<u8>(), i5) as u32;
        DAT_00757c98 = rb((&raw mut DAT_007438d3).cast::<u8>(), i5) as u32;
        match rb((&raw mut DAT_00743dc2).cast::<u8>(), i5) {
            0 => {
                DAT_00757cb4 = 0;
                DAT_00757c9c = 1;
            }
            1 => {
                DAT_00757cb4 = 10;
                DAT_00757c9c = 0;
            }
            2 => {
                DAT_00757cb4 = 5;
                DAT_00757c9c = 0;
            }
            3 => {
                DAT_00757cb4 = 6;
                DAT_00757c9c = 0;
            }
            4 => {
                DAT_00757c9c = 1;
                DAT_00757cb4 = 0;
                let c = count((&raw mut DAT_00743dc3).cast::<u8>(), i5, 10);
                DAT_00757c9c = DAT_00757c9c.wrapping_add(c);
            }
            _ => {}
        }
        DAT_00757c80 = count((&raw mut DAT_00743db0).cast::<u8>(), i5, 8);
        if rb((&raw mut DAT_007438d5).cast::<u8>(), i5) != 0 || DAT_00757c80 != 0 {
            DAT_00757c80 = DAT_00757c80.wrapping_add(1);
        }
        DAT_00757c88 = count((&raw mut DAT_00743db8).cast::<u8>(), i5, 6);
        if rb((&raw mut DAT_007438d6).cast::<u8>(), i5) != 0 || DAT_00757c88 != 0 {
            DAT_00757c88 = DAT_00757c88.wrapping_add(1);
        }
        DAT_00757c8c = count((&raw mut DAT_00743dbe).cast::<u8>(), i5, 4);
        if rb((&raw mut DAT_00743da3).cast::<u8>(), i5) != 0 || DAT_00757c8c != 0 {
            DAT_00757c8c = DAT_00757c8c.wrapping_add(1);
        }

        let local_c012 = i32::from_ne_bytes(DAT_0081c012);
        if local_c012 != 1 {
            reset();
            return;
        }
        let local_bfba = i32::from_ne_bytes(DAT_0081bfba);
        if local_bfba != 1 && local_bfba != 2 {
            reset();
            return;
        }
        let local_d326 = i32::from_ne_bytes(DAT_0081d326);
        if local_d326 == 0 {
            reset();
            return;
        }
        let mut u7: u32 = 0;
        let u4 = FUN_004ecce0(-1);
        let local_fb96 = i32::from_ne_bytes(DAT_0081fb96);
        let skip = if (u4 as i32) < 1 || (3 < (u4 as i32) && u4 != 0xe) {
            local_fb96 != 0
        } else {
            local_fb96 != 1
        };
        if !skip {
            u7 = (&raw mut DAT_005333e8).cast::<u8>().add(u4 as usize).read() as u32;
        }
        if u7 != 0 {
            loop {
                let u4 = FUN_004ecce0(-1);
                if u7 == u4 {
                    break;
                }
                if DAT_00757cb4 == 0 {
                    DAT_00757ca4 = DAT_00757ca4.wrapping_add(1);
                    if DAT_00757c9c <= DAT_00757ca4 {
                        DAT_00757ca4 = 0;
                    }
                    let b1: u8;
                    let b2: u8;
                    if DAT_00757ca4 == 0 {
                        let k = (DAT_00757cb0 as u32).wrapping_mul(0x562) as i32;
                        b1 = rb((&raw mut DAT_007438d3).cast::<u8>(), k);
                        b2 = rb((&raw mut DAT_007438e1).cast::<u8>(), k);
                    } else {
                        let k = ((DAT_00757cb0 as u32).wrapping_mul(0x562) as i32)
                            .wrapping_add(DAT_00757ca4);
                        b1 = rb((&raw mut DAT_00743dcc).cast::<u8>(), k);
                        b2 = rb((&raw mut DAT_00743dd6).cast::<u8>(), k);
                    }
                    DAT_00757c98 = b1 as u32;
                    DAT_00757c94 = b2 as u32;
                } else {
                    DAT_00757cac = DAT_00757cac.wrapping_add(1);
                    if DAT_00757cb4 <= DAT_00757cac {
                        DAT_00757cac = 0;
                    }
                    if DAT_00757cac != 0 {
                        let k = (DAT_00757cb0 as u32).wrapping_mul(0x562) as i32;
                        let b1 = rb((&raw mut DAT_00743dc2).cast::<u8>(), k);
                        if b1 != 0 {
                            let d2 = rb((&raw mut DAT_007438d2).cast::<u8>(), k) as u32;
                            if b1 < 3 {
                                let v = (&raw mut DAT_005333fc)
                                    .cast::<u32>()
                                    .offset(DAT_00757cac as isize)
                                    .read();
                                if v == d2 {
                                    DAT_00757cac = DAT_00757cac.wrapping_add(1);
                                }
                            } else if b1 == 3 {
                                let v = (&raw mut DAT_0053340c)
                                    .cast::<u32>()
                                    .offset(DAT_00757cac as isize)
                                    .read();
                                if v == d2 {
                                    DAT_00757cac = DAT_00757cac.wrapping_add(1);
                                }
                            }
                        }
                    }
                    if DAT_00757cb4 <= DAT_00757cac {
                        DAT_00757cac = 0;
                    }
                }
            }
        }
    }
}
