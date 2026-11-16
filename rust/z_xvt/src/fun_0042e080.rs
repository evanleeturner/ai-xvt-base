// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db5a8: i32;
    static mut DAT_0071bce5: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006a89a4: *mut i8;
    static mut DAT_005373d0: u32;
    static mut DAT_005373d4: i32;
    static mut DAT_005373d8: i32;
    static mut DAT_00525560: i32;
    static mut DAT_005250d8: i32;
    static mut DAT_005397fc: i32;
    static mut DAT_006a89b0: *mut u8;
    static mut DAT_006a89bc: *mut u8;
    static mut DAT_00536fb8: u8;
    static mut DAT_00536f78: u32;
    static mut DAT_005375f8: u8;
    static mut DAT_005377f8: i32;
    static mut DAT_005377fc: i32;
    static mut DAT_00537800: i32;
    static mut DAT_00537804: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f530() -> i32;
    fn FUN_004a7510();
    fn FUN_00438a40();
    fn FUN_00432c10(param_1: i32) -> i32;
    fn FUN_00432cc0(param_1: i32) -> *mut i32;
    fn FUN_00438c30(param_1: i32, param_2: *mut u8, param_3: i32);
    fn FUN_004310c0(param_1: *mut u32, param_2: i32);
    fn FUN_0049a1f0(param_1: *mut i32, param_2: *mut i8) -> i32;
    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
    fn FUN_004a53f0() -> i32;
    fn FUN_004a5470() -> u32;
    fn FUN_00438b70();
    fn FUN_004a74f0();
    fn FUN_0049a190(param_1: *mut u32, param_2: *mut u8, param_3: i32) -> i32;
    fn FUN_0049a180(param_1: i32) -> i32;
    fn FUN_0042e660(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0042c710();
}

unsafe fn read_flag() -> i32 {
    // SAFETY: the game's own packed table, indexed as the original does.
    unsafe {
        let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        (&raw mut DAT_0071bce5)
            .byte_add(off)
            .cast::<i32>()
            .read_unaligned()
    }
}

unsafe fn cstrlen(p: *const u8) -> usize {
    let mut n = 0usize;
    // SAFETY: caller guarantees a NUL-terminated string.
    unsafe {
        while *p.add(n) != 0 {
            n += 1;
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e080(param_1: i32, param_2: *mut u8, param_3: i32) -> u32 {
    let mut local_100: [u32; 64] = [0; 64];
    // SAFETY: all globals and callees are the game's own; pointers are
    // used exactly as the original uses them.
    unsafe {
        let buf = local_100.as_mut_ptr().cast::<u8>();
        let mut local_104: u32 = 1;
        let iv = FUN_0048f530();
        DAT_006db5a8 = DAT_006db5a8.wrapping_add(iv);
        if read_flag() != 0 {
            FUN_004a7510();
        }
        let mut local_118: i32 = 0;
        FUN_00438a40();
        let src = DAT_006a89a4 as *const u8;
        let len = cstrlen(src) + 1;
        core::ptr::copy_nonoverlapping(src, buf, len);
        let iv = FUN_00432c10(param_1);
        let p3 = FUN_00432cc0(iv);
        if !p3.is_null() {
            let s = p3 as *const u8;
            let l2 = cstrlen(s) + 1;
            let end = cstrlen(buf as *const u8);
            core::ptr::copy_nonoverlapping(s, buf.add(end), l2);
        }
        FUN_00438c30(1, buf, 0x30);
        DAT_005373d0 = 0x17;
        DAT_005373d4 = param_1;
        FUN_004310c0(&raw mut DAT_005373d0, 8);
        DAT_005373d0 = 0x3d;
        DAT_005373d4 = DAT_00525560;
        let iv = FUN_0049a1f0(&raw mut DAT_005373d8, param_2 as *mut i8);
        FUN_00431110(param_1, &raw mut DAT_005373d0, iv.wrapping_add(8) as u32);
        let mut local_124: i32 = 10;
        DAT_005250d8 = 1;
        let mut u7: u32 = local_100[0];
        loop {
            let mut i2: i32 = 0;
            DAT_005397fc = 0;
            loop {
                if 0xeb < i2 {
                    break;
                }
                let i4 = FUN_004a53f0();
                if i4 != 0 && FUN_004a5470() == 0x1b {
                    local_124 = 1;
                    i2 = 0xec;
                    break;
                }
                let i4 = DAT_006db5a8;
                FUN_0042c710();
                let i5 = FUN_0048f530();
                i2 = i2
                    .wrapping_sub(i4)
                    .wrapping_add(DAT_006db5a8)
                    .wrapping_add(i5);
                DAT_006db5a8 = i4;
                if DAT_005397fc != 0 {
                    break;
                }
            }
            local_118 = local_118.wrapping_add(i2);
            if 0xeb < local_118 {
                local_118 = 0;
                DAT_005373d0 = 0x1d;
                FUN_00431110(param_1, &raw mut DAT_005373d0, 4);
                FUN_004310c0(&raw mut DAT_005373d0, 4);
                u7 = (u7 == 0) as u32;
                let mut pb = DAT_006a89b0;
                if u7 != 0 {
                    pb = DAT_006a89bc;
                }
                FUN_00438c30(3, pb, 0x30);
            }
            let mut go = DAT_005397fc != 0;
            if !go {
                local_124 = local_124.wrapping_sub(1);
                go = local_124 == 0;
            }
            if !go {
                continue;
            }
            if DAT_005397fc == 0 {
                DAT_005373d4 = FUN_00432c10(param_1);
                DAT_005373d0 = 0x15;
                FUN_004310c0(&raw mut DAT_005373d0, 8);
                FUN_00438b70();
                FUN_0048f530();
                if read_flag() != 0 {
                    FUN_004a74f0();
                }
                DAT_005250d8 = 0;
                return 0;
            }
            let mut i2: i32 = 0;
            let i4 = FUN_0049a190(&raw mut DAT_00536fb8 as *mut u32, param_2, param_3);
            core::ptr::write_bytes(&raw mut DAT_00536f78, 0, 0x10);
            let mut s120: i32 = 0;
            let i5 = FUN_0049a180(param_3);
            DAT_005377f8 = 0x3f;
            let mut u8v: u32 = 0x1ec;
            DAT_005377fc = DAT_00525560;
            let mut p3: *mut i32 = &raw mut DAT_00537804;
            DAT_00537800 = 0;
            let mut s108: i32 = 0;
            if 0 < i4 {
                let mut s110: i32 = 0;
                loop {
                    let a = (&raw mut DAT_00536fb8)
                        .byte_add(s110 as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    let b = (&raw mut DAT_005375f8)
                        .byte_add(s110 as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if a == b {
                        s120 = s120.wrapping_add(i5);
                    } else {
                        let mut s11c = i5;
                        if param_3 < i5.wrapping_add(s120) {
                            s11c = param_3.wrapping_sub(s120);
                            if s11c < 0 {
                                s11c = 0;
                            }
                        }
                        while s11c != 0 {
                            let mut s114: u32 = (s11c as u32).wrapping_add(8);
                            if (u8v as i32) < (s114 as i32) {
                                s114 = u8v;
                            }
                            p3.write_unaligned(s120);
                            let u9 = s114.wrapping_sub(8);
                            p3.byte_add(4).write_unaligned(u9 as i32);
                            core::ptr::copy_nonoverlapping(
                                param_2.offset(s120 as isize) as *const u8,
                                p3.byte_add(8).cast::<u8>(),
                                u9 as usize,
                            );
                            let l1 = p3.byte_add(4).read_unaligned();
                            s11c = s11c.wrapping_sub(l1);
                            s120 = s120.wrapping_add(l1);
                            u8v = u8v.wrapping_sub(s114);
                            p3 = p3.byte_add(s114 as usize);
                            if u8v < 0x20 {
                                p3.write_unaligned(-1);
                                let mut i11 = i2.wrapping_add(1);
                                FUN_00431110(
                                    param_1,
                                    (&raw mut DAT_005377f8).offset(i2 as isize * 0x80) as *mut u32,
                                    0x1fc_u32.wrapping_sub(u8v),
                                );
                                if i11 == 0x10 {
                                    let r = FUN_0042e660(param_1, 0x10);
                                    if r == 0 {
                                        FUN_00438b70();
                                        FUN_0048f530();
                                        if read_flag() != 0 {
                                            FUN_004a74f0();
                                        }
                                        DAT_005250d8 = 0;
                                        return 0;
                                    }
                                    DAT_005373d0 = 0x1d;
                                    FUN_004310c0(&raw mut DAT_005373d0, 4);
                                    u7 = (u7 == 0) as u32;
                                    let mut pb = DAT_006a89b0;
                                    if u7 != 0 {
                                        pb = DAT_006a89bc;
                                    }
                                    i11 = 0;
                                    FUN_00438c30(3, pb, 0x30);
                                    core::ptr::write_bytes(&raw mut DAT_00536f78, 0, 0x10);
                                }
                                u8v = 0x1ec;
                                let o = i11 as isize * 0x80;
                                (&raw mut DAT_005377f8).offset(o).write(0x3f);
                                p3 = (&raw mut DAT_00537804).offset(o);
                                (&raw mut DAT_005377fc).offset(o).write(DAT_00525560);
                                (&raw mut DAT_00537800).offset(o).write(i11);
                                i2 = i11;
                            }
                        }
                    }
                    s110 = s110.wrapping_add(4);
                    s108 = s108.wrapping_add(1);
                    if !(s108 < i4) {
                        break;
                    }
                }
            }
            if u8v < 0x1f0 {
                p3.write_unaligned(-1);
                FUN_00431110(
                    param_1,
                    (&raw mut DAT_005377f8).offset(i2 as isize * 0x80) as *mut u32,
                    0x1fc_u32.wrapping_sub(u8v),
                );
                let r = FUN_0042e660(param_1, i2.wrapping_add(1));
                if r == 0 {
                    local_104 = 0;
                }
            }
            FUN_00438b70();
            FUN_0048f530();
            if read_flag() != 0 {
                FUN_004a74f0();
            }
            DAT_005250d8 = 0;
            return local_104;
        }
    }
}
