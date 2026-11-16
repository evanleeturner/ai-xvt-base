// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut _DAT_00732a3c: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_007092c0: u8;
    static mut _DAT_00732950: i16;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071b740: [u8; 1];
    static mut DAT_0071e528: [u8; 1];
    static mut DAT_0070b252: [u8; 1];
    static mut DAT_0070aaf0: [i32; 1];
    static mut DAT_0070ab10: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041e890(a: u16);
    fn FUN_00423200(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_004441e0(a: i32);
    fn FUN_00432cc0(a: i32) -> *mut i32;
    fn FUN_004a3490(a: i32, b: *mut i32);
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_004a41d0(a: i32);
    fn FUN_00423890(a: i32, b: i32, c: i32);
    fn FUN_00438970();
    fn FUN_00432ca0() -> i32;
    fn FUN_0042c670(a: i32);
    fn FUN_00431d30() -> i32;
    fn FUN_0042c580();
}

/// Returns true when the scan finds no matching record (iVar6 == 0).
unsafe fn not_found(i_var8: i32) -> bool {
    // SAFETY: reads the game's own globals and record tables as the original does.
    unsafe {
        let mut i_var2: i32 = -1;
        let mut i_var6: i32 = DAT_006d4980.wrapping_sub(_DAT_00732a3c);
        while i_var6 != 0 {
            i_var2 = i_var2.wrapping_add(1);
            if DAT_006d4980 <= i_var2 {
                i_var2 = _DAT_00732a3c;
            }
            let rec = ((i_var2 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_006d4988 as u32)) as usize as *const u8;
            if rec.byte_add(3).read() != 0 {
                let base = (&raw mut DAT_0070b252).cast::<u8>();
                let val = base
                    .byte_add((rec.byte_add(0x16).read() as usize).wrapping_mul(0x566))
                    .cast::<i32>()
                    .read_unaligned();
                if val == i_var8 {
                    let tmp32 = rec.byte_add(0x1f).cast::<u32>().read_unaligned();
                    let tmp32b = (tmp32.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
                    let c_var1 = (tmp32b.wrapping_add(7) as usize as *const i8).read();
                    if c_var1 != 3 && c_var1 != 5 && c_var1 != 4 {
                        break;
                    }
                }
            }
            i_var6 = i_var6.wrapping_sub(1);
        }
        i_var6 == 0
    }
}

unsafe fn tail() {
    // SAFETY: counts and reads the game's own player table, then calls its functions.
    unsafe {
        let mut i_var2: i32 = 0;
        let start = (&raw mut DAT_0071b740).cast::<u8>();
        let end = (&raw mut DAT_0071e528).cast::<u8>();
        let mut pcvar4 = start;
        loop {
            if pcvar4.read() == 1 {
                i_var2 = i_var2.wrapping_add(1);
            }
            pcvar4 = pcvar4.byte_add(0x5bd);
            if !((pcvar4 as usize) < (end as usize)) {
                break;
            }
        }
        let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        if start.byte_add(idx).read() == 1 {
            if i_var2 != 1 {
                _DAT_00732950 = i_var2 as i16;
                i_var2 = 0x174;
            } else {
                i_var2 = 0x175;
            }
            FUN_004a1d70(i_var2, DAT_0071ecf4);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00444630() {
    // SAFETY: all globals, tables and callees are the game's own memory;
    // pointers are read and written where the original does.
    unsafe {
        let mut i_var8: i32 = 0;
        let mut pivar9 = (&raw mut DAT_0071b730).cast::<u8>();
        loop {
            if pivar9.byte_add(0x10).read() != 0 {
                if pivar9.byte_add(0x11).read() == 0 {
                    if pivar9.byte_add(0x13).read() != 0
                        && pivar9.byte_add(0x10).read() != 2
                        && DAT_00525550 == 0
                    {
                        if not_found(i_var8) {
                            let us = pivar9.byte_add(0xe).cast::<u16>().read_unaligned();
                            FUN_0041e890(us);
                            if not_found(i_var8) {
                                FUN_004441e0(i_var8);
                                if i_var8 != DAT_0071ecf4 {
                                    let pivar3 = FUN_00432cc0(i_var8);
                                    FUN_004a3490(0, pivar3);
                                    FUN_004a1d70(0x173, DAT_0071ecf4);
                                    tail();
                                }
                            }
                        }
                    }
                } else if DAT_00525550 == 0 {
                    let first = pivar9.cast::<i32>().read_unaligned();
                    if first != -1 {
                        let second = pivar9.byte_add(4).cast::<i32>().read_unaligned();
                        let rec = ((first as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32))
                            as usize as *const u8;
                        let us = rec.cast::<u16>().read_unaligned();
                        if rec.byte_add(3).read() == 0 || (second as u32) != (us as u32) {
                            let us2 = pivar9.byte_add(0xe).cast::<u16>().read_unaligned();
                            FUN_0041e890(us2);
                            let i_var2 = FUN_00423200(i_var8, -1, 0, 0);
                            if i_var2 == 0 {
                                if i_var8 == DAT_0071ecf4 {
                                    FUN_004a41d0(0x118);
                                }
                            } else {
                                FUN_004441e0(i_var8);
                                if i_var8 != DAT_0071ecf4 {
                                    let pivar3 = FUN_00432cc0(i_var8);
                                    FUN_004a3490(0, pivar3);
                                    FUN_004a1d70(0x173, DAT_0071ecf4);
                                    tail();
                                }
                            }
                        }
                    }
                }
            }
            pivar9 = pivar9.byte_add(0x5bd);
            i_var8 = i_var8.wrapping_add(1);
            if 0x71e517usize < (pivar9 as usize) {
                if DAT_00525550 == 0 {
                    i_var8 = 0;
                    let mut pcvar4 = (&raw mut DAT_0071b740).cast::<u8>();
                    let mut pvar_a = (&raw mut DAT_0070aaf0).cast::<i32>();
                    let limit = (&raw mut DAT_0070ab10).cast::<i32>();
                    loop {
                        if pvar_a.read() != 0 && pcvar4.read() != 0 {
                            let a = pcvar4.byte_sub(0x10).cast::<i32>().read_unaligned();
                            if a != -1 {
                                FUN_00423890(i_var8, 0, 1);
                            }
                            if i_var8 == DAT_0071ecf4 {
                                DAT_007092c0 = 1;
                            }
                            pcvar4.write(0);
                            FUN_00438970();
                            let i_var2 = FUN_00432ca0();
                            let b = pcvar4.byte_add(0x578).cast::<i32>().read_unaligned();
                            if i_var2 != b {
                                FUN_0042c670(i_var8);
                            }
                            let pivar3 = FUN_00432cc0(i_var8);
                            FUN_004a3490(0, pivar3);
                            FUN_004a1d70(0x172, DAT_0071ecf4);
                            if i_var8 == DAT_0071ecf4 && FUN_00431d30() != 0 {
                                FUN_0042c580();
                            }
                        }
                        pcvar4 = pcvar4.byte_add(0x5bd);
                        pvar_a = pvar_a.add(1);
                        i_var8 = i_var8.wrapping_add(1);
                        if !((pvar_a as usize) < (limit as usize)) {
                            break;
                        }
                    }
                }
                return;
            }
        }
    }
}
