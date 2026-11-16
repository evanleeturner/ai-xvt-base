// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b70c: i32;
    static mut DAT_0085e6f0: i8;
    static mut DAT_0053308c: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0085e731: u8;
    static mut DAT_0085e6f1: u8;
    static mut DAT_00757ce0: [u32; 0x54];
    static mut DAT_0064f658: [u32; 8];
    static mut DAT_0081b700: *mut u32;
    static mut DAT_0081bcd4: i32;
    static mut DAT_00757e40: [u32; 0x1d8];
    static mut DAT_00758824: i32;
    static mut DAT_005330fc: i32;
    static mut DAT_00757cd0: i32;
    static mut DAT_00758828: i32;
    static mut DAT_00758618: u32;
    static mut DAT_00532a34: i32;
    static mut DAT_0081bcb8: i32;
    static mut DAT_00757cc4: *mut u8;
    static mut s_frontres_joinback_bmp_00532a8c: [i8; 1];
    static mut s_background_00532a4c: [i8; 1];
    static mut s_frame_00531358: [i8; 1];
    static mut DAT_0081bc68: i32;
    static mut s_clientactive_00531eac: [i8; 1];
    static mut s_allactive_00531ebc: [i8; 1];
    static mut s_chatbox_00532a84: [i8; 1];
    static mut s_joinoverlay_00532a78: [i8; 1];
    static mut DAT_00757e70: [u8; 1];
    static mut DAT_00757e74: [u8; 1];
    static mut DAT_00757e78: [u8; 1];
    static mut DAT_00757e79: [u8; 1];
    static mut DAT_007585a0: i32;
    static mut DAT_007585a4: u8;
    static mut DAT_0081bfba: [u8; 1];
    static mut DAT_0081bfbe: [u8; 1];
    static mut DAT_00757cc0: *mut u8;
    static mut DAT_00757cc8: i32;
    static mut DAT_00757ccc: i32;
    static mut DAT_00758820: i32;
    static mut DAT_0081b740: [i8; 1];
    static mut s_v___d__d_00531e88: [i8; 1];
    static mut DAT_0081bcf0: i8;
    static mut DAT_0081e082: [i8; 1];
    static mut s__c_s__c_s_00531334: [i8; 1];
    static mut DAT_0081bcd0: i32;
    static mut s_rebtiny_d_00531328: [i8; 1];
    static mut s_imptiny_d_0053131c: [i8; 1];
    static mut DAT_0085e7c6: i8;
    static mut s_buttonsound_00531310: [i8; 1];
    static mut s_leaveup_005312fc: [i8; 1];
    static mut s_leavedown_00531304: [i8; 1];
    static mut s_flysound_00532a6c: [i8; 1];
    static mut s_nextup_00532a58: [i8; 1];
    static mut s_nextdown_00532a60: [i8; 1];
    static mut DAT_00757cd8: i32;
    static mut DAT_0081bfd2: [i8; 1];
    static mut DAT_00758620: u32;
    static mut DAT_00758624: i32;
    static mut DAT_00758628: [u8; 1];
    static mut DAT_0075862c: [u8; 1];
    static mut DAT_00758630: [u8; 1];
    static mut DAT_00758634: [u8; 1];
    static mut DAT_0085e77f: [u8; 1];
    static mut DAT_0085e783: [u8; 1];
    static mut DAT_0085e787: [u8; 1];
    static mut DAT_0085e78b: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004bef20();
    fn LAB_004bee00();
    fn FUN_004b8a50();
    fn FUN_004d7150();
    fn FUN_004db670(a: *mut c_void, b: *mut c_void);
    fn FUN_004d7e90(a: u32) -> u32;
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004dbf00() -> i32;
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004db6c0(a: *mut c_void, b: *mut i32);
    fn FUN_004ddc90(a: i32, b: i32);
    fn FUN_004cde20();
    fn _malloc(a: u32) -> *mut c_void;
    fn FUN_004d0290(a: *mut i8, b: *mut i8);
    fn FUN_004d52b0();
    fn FUN_004d1750(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004db490(a: i32);
    fn FUN_004da920(a: i32, b: *mut i8, c: *mut i32, d: i32);
    fn FUN_004d8520();
    fn FUN_004e0750() -> i32;
    fn FUN_004ee3d0();
    fn FUN_004e43d0(a: i32);
    fn FUN_004e4820(a: *mut u8);
    fn FUN_004ca200();
    fn FUN_004d6320(a: u32) -> i32;
    fn FUN_004d8a70(a: i32) -> i32;
    fn FUN_004d8900();
    fn FUN_004db650();
    fn FUN_004d77b0();
    fn FUN_004d75c0();
    fn FUN_004d7900(a: u32);
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004d83f0();
    fn FUN_004c0750(a: i32) -> i32;
    fn FUN_004da030();
    fn FUN_004da060(a: *mut i8);
    fn FUN_004d9a80(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: i32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004cdbb0() -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: i32);
    fn FUN_004da050();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn clr8() {
    // SAFETY: the game's own 8-word buffer.
    unsafe {
        (&raw mut DAT_0064f658).cast::<u32>().write_bytes(0, 8);
    }
}

unsafe fn clr_buf() {
    // SAFETY: the game's own heap buffer of 0x400 bytes, written as the original does.
    unsafe {
        DAT_00757cc4.write_bytes(0, 0x400);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d66c0(param_1: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used where the original uses them.
    unsafe {
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        let mut i_var3: i32;

        DAT_0081b70c = 0;
        if DAT_0085e6f0 != 0 {
            if DAT_0053308c != 0 {
                DAT_0081b850 = 0;
                FUN_004db670(
                    FUN_004bef20 as unsafe extern "C" fn() as *mut c_void,
                    LAB_004bee00 as unsafe extern "C" fn() as *mut c_void,
                );
                return 0;
            }
            if DAT_0081b850 == 3 {
                if DAT_0085e6f0 == 1 {
                    if DAT_0085e731 == 0 {
                        FUN_004dcfa0(0x13);
                        FUN_004dcfa0(0x1fd);
                        FUN_004dcfa0(0x2a9);
                        FUN_004dcfa0(0x2a8);
                        FUN_004dcfa0(0x2a7);
                        i_var3 = FUN_004dbf00();
                        if i_var3 == 0 {
                            DAT_0081b850 = 0;
                            FUN_004db670(
                                FUN_004bef20 as unsafe extern "C" fn() as *mut c_void,
                                LAB_004bee00 as unsafe extern "C" fn() as *mut c_void,
                            );
                            return 0;
                        }
                        FUN_004dcfd0(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                        FUN_004db6c0(
                            FUN_004b8a50 as unsafe extern "C" fn() as *mut c_void,
                            local_10.as_mut_ptr(),
                        );
                        return 0;
                    }
                } else if DAT_0085e6f0 == 2 && DAT_0085e6f1 == 0 {
                    FUN_004dcfa0(0x13);
                    FUN_004dcfa0(0x1fd);
                    FUN_004dcfa0(0x2ac);
                    FUN_004dcfa0(0x2ab);
                    FUN_004dcfa0(0x2aa);
                    i_var3 = FUN_004dbf00();
                    if i_var3 == 0 {
                        DAT_0081b850 = 0;
                        FUN_004db670(
                            FUN_004bef20 as unsafe extern "C" fn() as *mut c_void,
                            LAB_004bee00 as unsafe extern "C" fn() as *mut c_void,
                        );
                        return 0;
                    }
                    FUN_004dcfd0(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                    FUN_004db6c0(
                        FUN_004b8a50 as unsafe extern "C" fn() as *mut c_void,
                        local_10.as_mut_ptr(),
                    );
                    return 0;
                }
            }
            return FUN_004d7e90(param_1);
        }
        if param_1 == 0 {
            FUN_004ddc90(0x19f, 0x79);
            (&raw mut DAT_00757ce0).cast::<u32>().write_bytes(0, 0x54);
            FUN_004cde20();
            clr8();
            if !DAT_0081b700.is_null() {
                DAT_0081b700.write_bytes(0, 0x100);
                DAT_0081bcd4 = 0;
            }
            if DAT_0053308c == 0 {
                (&raw mut DAT_00757e40).cast::<u32>().write_bytes(0, 0x1d8);
                DAT_00758824 = 0;
            }
            DAT_005330fc = 0;
            DAT_0081b70c = 0;
            DAT_00757cd0 = 0;
            DAT_00758828 = 0;
            DAT_00758618 = 0xffffffff;
            DAT_00532a34 = -1;
            DAT_0081bcb8 = 0;
            DAT_0053308c = 0;
            if DAT_00757cc4.is_null() {
                DAT_00757cc4 = _malloc(0x400).cast::<u8>();
            }
            FUN_004d0290(
                (&raw mut s_frontres_joinback_bmp_00532a8c).cast::<i8>(),
                (&raw mut s_background_00532a4c).cast::<i8>(),
            );
            FUN_004d52b0();
            FUN_004d1750((&raw mut s_background_00532a4c).cast::<i8>(), 0, 0);
            FUN_004d0e60((&raw mut s_frame_00531358).cast::<i8>(), 0, 0);
            let pc: *mut i8 = if DAT_0081bc68 == 0 {
                (&raw mut s_clientactive_00531eac).cast::<i8>()
            } else {
                (&raw mut s_allactive_00531ebc).cast::<i8>()
            };
            FUN_004d0e60(pc, 0, 0);
            FUN_004d0580((&raw mut s_chatbox_00532a84).cast::<i8>(), 0, 0);
            FUN_004d0580((&raw mut s_joinoverlay_00532a78).cast::<i8>(), 0, 0);
            FUN_004d5350(1);
            FUN_004db490(0x14);
        }
        FUN_004dcfd0(local_20.as_mut_ptr(), 0x9e, 0x34, 0x1eb, 0x44);
        FUN_004da920(
            0xc,
            (&raw mut DAT_0064f658).cast::<i8>(),
            local_20.as_mut_ptr(),
            0xffff,
        );
        let u_var6: u32 = ((param_1 as i32) >> 0x1f) as u32;

        'cf7: {
            if (param_1 as i32) % 0xa0 == 0 {
                if DAT_00532a34 == -1 {
                    FUN_004d8520();
                    if DAT_00532a34 == -1 {
                        break 'cf7;
                    }
                }
            } else if DAT_00532a34 == -1 {
                break 'cf7;
            }
            i_var3 = FUN_004e0750();
            if i_var3 == 0x41 {
                let dvar4 = GetTickCount();
                let idx = (DAT_00532a34 as u32).wrapping_mul(0x3b) as usize;
                (&raw mut DAT_00757e74)
                    .cast::<u8>()
                    .byte_add(idx)
                    .cast::<u32>()
                    .write_unaligned(dvar4);
                let i3 = DAT_00532a34;
                let idx2 = (i3 as u32).wrapping_mul(0x3b) as usize;
                (&raw mut DAT_00757e70)
                    .cast::<u8>()
                    .byte_add(((DAT_00532a34 as u32).wrapping_mul(0x3b)) as usize)
                    .cast::<i32>()
                    .write_unaligned(DAT_007585a0);
                (&raw mut DAT_00757e79)
                    .cast::<u8>()
                    .byte_add(idx2)
                    .write(DAT_007585a4);
                FUN_004ee3d0();
                (&raw mut DAT_0081bfba)
                    .cast::<i32>()
                    .write_unaligned(DAT_00758828);
                (&raw mut DAT_0081bfbe)
                    .cast::<u8>()
                    .byte_add((DAT_00758828 as u32).wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .write_unaligned(DAT_00758618);
                let tmp: i32 = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
                FUN_004e43d0(tmp);
                FUN_004e4820(DAT_00757cc4);
                if !DAT_00757cc0.is_null() && {
                    DAT_00757cc8 = 0;
                    DAT_00757ccc != 0
                } {
                    let mut pi_var7 = DAT_00757cc0.byte_add(0x140).cast::<i32>();
                    loop {
                        let a: i32 = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
                        let b: i32 = (&raw mut DAT_0081bfbe)
                            .cast::<u8>()
                            .byte_add((a as u32).wrapping_mul(4) as usize)
                            .cast::<i32>()
                            .read_unaligned();
                        if pi_var7.read_unaligned() == b {
                            break;
                        }
                        pi_var7 = pi_var7.wrapping_add(0x51);
                        DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                        if !(DAT_00757cc8 < DAT_00757ccc) {
                            break;
                        }
                    }
                }
                DAT_005330fc = 0;
                break 'cf7;
            }
            if i_var3 == 0x46
                || i_var3 == 0x49
                || i_var3 == 0x4b
                || i_var3 == 0x4c
                || i_var3 == 0x5a
                || i_var3 == 0x66
                || i_var3 == 0x76
            {
                FUN_004ca200();
                FUN_004d8520();
                DAT_00758820 = 0;
                DAT_00532a34 = -1;
                DAT_00758618 = 0xffffffff;
            } else {
                if i_var3 == 0x69 {
                    FUN_004ca200();
                    DAT_00758820 = 0;
                    DAT_00532a34 = -1;
                    DAT_00758618 = 0xffffffff;
                    FUN_004d8520();
                } else {
                    if i_var3 != 0x44 && i_var3 != 0x5e && i_var3 != 0x5f && i_var3 != 0x60 {
                        break 'cf7;
                    }
                    let u_var5: Option<i32> = match i_var3 {
                        0x44 => {
                            FUN_004dcfa0(0x213);
                            FUN_004dcfa0(0x212);
                            Some(0x211)
                        }
                        0x5e => {
                            FUN_004dcfa0(0x216);
                            FUN_004dcfa0(0x215);
                            Some(0x214)
                        }
                        0x5f => {
                            FUN_004dcfa0(0x219);
                            FUN_004dcfa0(0x218);
                            Some(0x217)
                        }
                        0x60 => {
                            FUN_004dcfa0(0x21c);
                            FUN_004dcfa0(0x21b);
                            Some(0x21a)
                        }
                        _ => None,
                    };
                    if let Some(v) = u_var5 {
                        FUN_004dcfa0(v);
                        FUN_004dbf00();
                    }
                    FUN_004ca200();
                    DAT_00758820 = 0;
                    DAT_00532a34 = -1;
                    DAT_00758618 = 0xffffffff;
                    FUN_004d8520();
                }
            }
            clr8();
            clr_buf();
        }

        i_var3 = FUN_004d6320(param_1);
        if i_var3 != -1 {
            if i_var3 == DAT_00532a34 {
                DAT_00532a34 = -1;
                DAT_00758618 = 0xffffffff;
                DAT_00758820 = 0;
                clr8();
                clr_buf();
                FUN_004ca200();
            } else {
                DAT_00532a34 = i_var3;
                i_var3 = FUN_004d8a70(i_var3);
                if i_var3 == 0 {
                    FUN_004ca200();
                    DAT_00758820 = 0;
                    DAT_00532a34 = -1;
                    DAT_00758618 = 0xffffffff;
                    FUN_004d8520();
                    clr8();
                    clr_buf();
                } else {
                    FUN_004d8900();
                    let v: i32 = (&raw mut DAT_00757e70)
                        .cast::<u8>()
                        .byte_add((DAT_00532a34 as u32).wrapping_mul(0x3b) as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if v != 0 {
                        FUN_004db650();
                        FUN_004ddc90(0x25, 0x1bd);
                    }
                }
            }
        }
        FUN_004d77b0();
        FUN_004d75c0();
        if DAT_00532a34 != -1 {
            FUN_004d7900(param_1);
        }
        FUN_004dcfd0(local_20.as_mut_ptr(), 0x1fb, 0x1c4, 0x232, 0x1d0);
        _sprintf(
            (&raw mut DAT_0081b740).cast::<i8>(),
            (&raw mut s_v___d__d_00531e88).cast::<i8>(),
            1i32,
            1i32,
        );
        FUN_004da920(
            0xc,
            (&raw mut DAT_0081b740).cast::<i8>(),
            local_20.as_mut_ptr(),
            0xffff,
        );
        FUN_004dcfd0(local_20.as_mut_ptr(), 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_0081bcf0 != 0 {
            _sprintf(
                (&raw mut DAT_0081b740).cast::<i8>(),
                (&raw mut s__c_s__c_s_00531334).cast::<i8>(),
                6i32,
                (&raw mut DAT_0081e082).cast::<i8>(),
                1i32,
                &raw mut DAT_0081bcf0,
            );
            FUN_004da920(
                0xc,
                (&raw mut DAT_0081b740).cast::<i8>(),
                local_20.as_mut_ptr(),
                DAT_0081bcd0,
            );
            let i3: i32 = ((((param_1 ^ u_var6).wrapping_sub(u_var6) & 0x1f) ^ u_var6)
                .wrapping_sub(u_var6) as i32)
                >> 1;
            _sprintf(
                (&raw mut DAT_0081b740).cast::<i8>(),
                (&raw mut s_rebtiny_d_00531328).cast::<i8>(),
                i3,
            );
            FUN_004d0e60((&raw mut DAT_0081b740).cast::<i8>(), 0xcc, 0x1c5);
            _sprintf(
                (&raw mut DAT_0081b740).cast::<i8>(),
                (&raw mut s_imptiny_d_0053131c).cast::<i8>(),
                i3,
            );
            FUN_004d0e60((&raw mut DAT_0081b740).cast::<i8>(), 0x1a4, 0x1c5);
        }
        FUN_004d83f0();
        i_var3 = FUN_004c0750(0);
        if i_var3 == 1 {
            return 1;
        }
        FUN_004dcfd0(local_20.as_mut_ptr(), 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_0085e7c6 != 0 {
            FUN_004da030();
        }
        let pc = FUN_004dcfa0(0x22b);
        FUN_004da060(pc);
        let pc12 = (&raw mut s_buttonsound_00531310).cast::<i8>();
        let pc10 = FUN_004dcfa0(0xf5);
        i_var3 = FUN_004d9a80(
            local_20.as_mut_ptr(),
            (&raw mut s_leaveup_005312fc).cast::<i8>(),
            (&raw mut s_leavedown_00531304).cast::<i8>(),
            pc10,
            0xc,
            0,
            8,
            pc12,
        );
        if i_var3 != 0 {
            DAT_0081b850 = 0;
            FUN_004ca200();
            FUN_004db670(
                FUN_004bef20 as unsafe extern "C" fn() as *mut c_void,
                LAB_004bee00 as unsafe extern "C" fn() as *mut c_void,
            );
        }
        FUN_004dcfd0(local_20.as_mut_ptr(), 8, 0x195, 0x47, 0x1d6);
        if DAT_00532a34 != -1 {
            let v: i32 = (&raw mut DAT_00757e70)
                .cast::<u8>()
                .byte_add((DAT_00532a34 as u32).wrapping_mul(0x3b) as usize)
                .cast::<i32>()
                .read_unaligned();
            if v != 0
                && (&raw mut DAT_00757e78)
                    .cast::<u8>()
                    .byte_add((DAT_00532a34 as u32).wrapping_mul(0x3b) as usize)
                    .read()
                    == 2
            {
                let pc = FUN_004dcfa0(0x12);
                FUN_004da060(pc);
                let pc12 = (&raw mut s_flysound_00532a6c).cast::<i8>();
                let pc10 = FUN_004dcfa0(0x12);
                i_var3 = FUN_004d9a80(
                    local_20.as_mut_ptr(),
                    (&raw mut s_nextup_00532a58).cast::<i8>(),
                    (&raw mut s_nextdown_00532a60).cast::<i8>(),
                    pc10,
                    0xc,
                    0,
                    7,
                    pc12,
                );
                if i_var3 != 0 {
                    DAT_00757cd8 = 0;
                    let mut src = (&raw mut DAT_00757e40)
                        .cast::<i8>()
                        .byte_add((DAT_00532a34 as u32).wrapping_mul(0x3b) as usize);
                    let mut len: u32 = 0;
                    loop {
                        let c = src.read();
                        src = src.wrapping_add(1);
                        len = len.wrapping_add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    let mut s = src.wrapping_sub(len as usize);
                    let mut d = (&raw mut DAT_0081bfd2).cast::<i8>();
                    let mut n = len >> 2;
                    while n != 0 {
                        d.cast::<u32>()
                            .write_unaligned(s.cast::<u32>().read_unaligned());
                        s = s.wrapping_add(4);
                        d = d.wrapping_add(4);
                        n -= 1;
                    }
                    let mut n = len & 3;
                    while n != 0 {
                        d.write(s.read());
                        s = s.wrapping_add(1);
                        d = d.wrapping_add(1);
                        n -= 1;
                    }
                    i_var3 = FUN_004cdbb0();
                    DAT_00758620 = 0x42;
                    DAT_00758624 = 2;
                    (&raw mut DAT_00758628)
                        .cast::<u32>()
                        .write_unaligned((&raw mut DAT_0085e77f).cast::<u32>().read_unaligned());
                    (&raw mut DAT_0075862c)
                        .cast::<u32>()
                        .write_unaligned((&raw mut DAT_0085e783).cast::<u32>().read_unaligned());
                    (&raw mut DAT_00758630)
                        .cast::<u32>()
                        .write_unaligned((&raw mut DAT_0085e787).cast::<u32>().read_unaligned());
                    (&raw mut DAT_00758634)
                        .cast::<u32>()
                        .write_unaligned((&raw mut DAT_0085e78b).cast::<u32>().read_unaligned());
                    FUN_004cb830(i_var3, &raw mut DAT_00758620, 0x18);
                    FUN_004db670(
                        FUN_004d7150 as unsafe extern "C" fn() as *mut c_void,
                        core::ptr::null_mut(),
                    );
                }
            }
        }
        FUN_004da050();
        0
    }
}
