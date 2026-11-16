// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007092c0: i8;
    static mut DAT_00525550: i32;
    static mut DAT_0071bb27: [u8; 1];
    static mut DAT_0071b743: [u8; 1];
    static mut DAT_0071bb4a: [u8; 1];
    static mut DAT_0071bb44: [u8; 1];
    static mut DAT_0071bb40: [u8; 1];
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071bb33: [u8; 1];
    static mut DAT_0071bb35: [u8; 1];
    static mut DAT_0071bb29: [u8; 1];
    static mut DAT_0071b741: [u8; 1];
    static mut DAT_0071b744: [u8; 1];
    static mut DAT_0071b73e: [u8; 1];
    static mut DAT_0071b781: [u8; 1];
    static mut DAT_0071b783: [u8; 1];
    static mut DAT_0071bb1a: [u8; 1];
    static mut DAT_0071b740: [u8; 1];
    static mut DAT_0071e528: [u8; 1];
    static mut DAT_006d4988: i32;
    static mut DAT_0071ecf4: u32;
    static mut DAT_007092b0: i32;
    static mut DAT_00525518: i32;
    static mut DAT_00525540: u32;
    static mut DAT_006db5a0: *const c_void;
    static mut _DAT_00732950: i16;
    static mut DAT_00525538: u8;
    static mut _DAT_0071b010: i32;
    static mut DAT_00525ad0: [i32; 1];
    static mut DAT_00732e4c: u8;
    static mut DAT_00525ae8: [u8; 1];
    static mut DAT_0052553c: u16;
    static mut DAT_00525ae0: [u16; 1];
    static mut DAT_006d9d94: u8;
    static mut DAT_00525af0: [u8; 1];
    static mut DAT_0071e536: u8;
    static mut DAT_00525544: u32;
    static mut DAT_00732a1e: i8;
    static mut DAT_00732e48: i32;
    static mut DAT_006db5a8: i32;
    static mut DAT_0052ae54: i32;
    static mut DAT_006db592: i16;
    static mut DAT_007327e0: i32;
    static mut DAT_0070accc: i8;
    static mut DAT_00525580: u8;
    static mut DAT_007329cc: i8;
    static mut DAT_0071e534: u16;
    static mut DAT_0071e6be: u16;
    static mut DAT_007092c1: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040cbf0(a: i32, b: u32);
    fn FUN_00405cf0(a: u32) -> u32;
    fn FUN_004054e0();
    fn FUN_004a1d70(a: u32, b: u32);
    fn FUN_0040a1a0(a: i32, b: i32, c: u32);
    fn FUN_0048f530() -> i32;
    fn FUN_004a6a80();
    fn FUN_004a6f90();
    fn FUN_004a7040();
    fn FUN_00491150();
    fn FUN_004651e0();
    fn FUN_00465210();
    fn FUN_0041e890(a: u32);
    fn FUN_00423200(a: u32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_004441e0(a: u32);
    fn FUN_00432cc0(a: u32) -> *mut i32;
    fn FUN_004a3490(a: i32, b: *mut i32);
    fn FUN_004a41d0(a: i32);
    fn FUN_00447010(a: u32);
    fn FUN_00436120(a: u32) -> u32;
    fn FUN_00441930(a: u32) -> u32;
    fn FUN_00442bf0(a: u32, b: u32);
    fn FUN_0043a080(a: u32, b: u32, c: u32, d: u32);
    fn FUN_00440d20(a: u32);
    fn FUN_0043fe30(a: u32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_004a42b0(a: u32, b: u32);
}

#[inline(always)]
fn at(base: *mut u8, off: i32) -> *mut u8 {
    base.wrapping_byte_add(off as usize)
}

#[inline(always)]
unsafe fn flag(idx: u32) -> u8 {
    // SAFETY: the table pointer is the game's own; read where the original reads.
    unsafe {
        idx.wrapping_mul(0x23)
            .wrapping_add(3)
            .wrapping_add(DAT_006d4988 as u32) as *const u8
    }
    .read()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables)]
pub unsafe extern "C" fn FUN_00439a30(param_1: u32) {
    // SAFETY: all globals and callees are the game's own memory and code; the
    // per-player block is accessed unaligned at the original's offsets.
    unsafe {
        let extraout_ecx: u32 = 0;
        let extraout_edx: u32 = 0;
        let unaff_ebx: u32 = 0;
        let unaff_esi: u32 = 0;
        let unaff_edi: u32 = 0;

        if DAT_007092c0 == 1 {
            return;
        }
        let mut i_var3: i32 = param_1.wrapping_mul(0x5bd) as i32;
        if DAT_00525550 == 0 {
            let u_var2 = at((&raw mut DAT_0071bb27).cast::<u8>(), i_var3)
                .cast::<u16>()
                .read_unaligned();
            if u_var2 != 0xffff && flag(u_var2 as u32) == 0 {
                if at((&raw mut DAT_0071b743).cast::<u8>(), i_var3).read() == 0 {
                    at((&raw mut DAT_0071bb4a).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0);
                    at((&raw mut DAT_0071bb44).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0);
                    at((&raw mut DAT_0071bb40).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0);
                    let tmp32 = at((&raw mut DAT_0071b730).cast::<u8>(), i_var3)
                        .cast::<i32>()
                        .read_unaligned();
                    at((&raw mut DAT_0071bb27).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(tmp32 as u16);
                    FUN_0040cbf0(0, param_1);
                    at((&raw mut DAT_0071bb33).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0);
                    at((&raw mut DAT_0071bb35).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0);
                } else {
                    at((&raw mut DAT_0071bb27).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0xffff);
                }
            }
            if at((&raw mut DAT_0071b743).cast::<u8>(), i_var3).read() != 0 {
                let u_var2 = at((&raw mut DAT_0071bb29).cast::<u8>(), i_var3)
                    .cast::<u16>()
                    .read_unaligned();
                if u_var2 != 0xffff && flag(u_var2 as u32) == 0 {
                    at((&raw mut DAT_0071bb29).cast::<u8>(), i_var3)
                        .cast::<u16>()
                        .write_unaligned(0xffff);
                }
            }
        }
        FUN_00405cf0(param_1);
        FUN_004054e0();
        if param_1 == DAT_0071ecf4 && DAT_00525550 == 0 {
            match DAT_007092b0 {
                0x7d => {
                    if DAT_00525518 == 1 {
                        DAT_00525540 = DAT_00525540.wrapping_add(0x40);
                        if DAT_00525540 == 0x300 {
                            DAT_00525540 = 0x100;
                        }
                        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_006db5a0);
                        f();
                        _DAT_00732950 =
                            ((DAT_00525540.wrapping_sub(0x100) >> 6) as i16).wrapping_add(1);
                        FUN_004a1d70(0x115, param_1);
                        FUN_0040a1a0(0x2b, -1, param_1);
                    }
                }
                0x7f => {
                    DAT_00525538 = DAT_00525538.wrapping_add(1);
                    if 3 < DAT_00525538 {
                        DAT_00525538 = 0;
                    }
                    let u_var6 = DAT_00525538 as u32;
                    _DAT_0071b010 = (&raw mut DAT_00525ad0)
                        .cast::<i32>()
                        .add(u_var6 as usize)
                        .read();
                    DAT_00732e4c = (&raw mut DAT_00525ae8)
                        .cast::<u8>()
                        .add(u_var6 as usize)
                        .read();
                    DAT_0052553c = (&raw mut DAT_00525ae0)
                        .cast::<u16>()
                        .add(u_var6 as usize)
                        .read();
                    DAT_006d9d94 = (&raw mut DAT_00525af0)
                        .cast::<u8>()
                        .add((u_var6 as usize).wrapping_mul(2))
                        .read();
                    DAT_0071e536 = 1;
                    FUN_004a1d70(u_var6.wrapping_add(0x65), param_1);
                    FUN_0040a1a0(0x2b, -1, param_1);
                }
                0x84 => {
                    DAT_00525544 = (DAT_00525544 == 0) as u32;
                }
                0x88 => {
                    if DAT_00732a1e == 0 {
                        DAT_00732a1e = 1;
                    } else {
                        DAT_00732a1e = 0;
                    }
                }
                0x8b => {
                    if DAT_00732e48 == 1 {
                        let r = FUN_0048f530();
                        DAT_006db5a8 = DAT_006db5a8.wrapping_add(r);
                        FUN_0040a1a0(0x2b, -1, param_1);
                        FUN_004a1d70(1, param_1);
                        DAT_0052ae54 = 0;
                        FUN_004a6a80();
                        FUN_004a42b0(extraout_ecx, extraout_edx);
                        FUN_004a6f90();
                        FUN_004a7040();
                        DAT_0052ae54 = 1;
                        FUN_00491150();
                        loop {
                            let u_var6 = FUN_00405cf0(0xffff_fffe);
                            if (u_var6 as i16) != 0 {
                                break;
                            }
                        }
                        FUN_0048f530();
                        FUN_004a1d70(2, param_1);
                        DAT_006db592 = 0;
                        DAT_007327e0 = 0;
                        FUN_004651e0();
                    }
                }
                0x8e => {
                    if DAT_0070accc == 0 {
                        DAT_0070accc = 1;
                    } else {
                        DAT_0070accc = 0;
                    }
                }
                0x91 => {
                    FUN_0040a1a0(0x2b, -1, param_1);
                    FUN_004a1d70(0, param_1);
                }
                0xe4 => {
                    FUN_0040a1a0(0x2b, -1, param_1);
                    FUN_00465210();
                }
                _ => {}
            }
        }
        if 1 < DAT_00525580 && DAT_007329cc != 0 {
            return;
        }
        i_var3 = param_1.wrapping_mul(0x5bd) as i32;
        if at((&raw mut DAT_0071b741).cast::<u8>(), i_var3).read() != 0 {
            if DAT_00525550 != 0 {
                return;
            }
            let tmp32 = at((&raw mut DAT_0071b730).cast::<u8>(), i_var3)
                .cast::<i32>()
                .read_unaligned();
            if tmp32 == -1 {
                return;
            }
            if flag(tmp32 as u32) != 0 {
                return;
            }
            let tmp16 = at((&raw mut DAT_0071b73e).cast::<u8>(), i_var3)
                .cast::<u16>()
                .read_unaligned();
            FUN_0041e890(tmp16 as u32);
            let r = FUN_00423200(param_1, -1, 0, 0);
            if r != 0 {
                FUN_004441e0(param_1);
                if param_1 == DAT_0071ecf4 {
                    return;
                }
                let pi = FUN_00432cc0(param_1);
                FUN_004a3490(0, pi);
                FUN_004a1d70(0x173, DAT_0071ecf4);
                let mut count: i32 = 0;
                let mut pc: *mut u8 = (&raw mut DAT_0071b740).cast::<u8>();
                let end: *mut u8 = (&raw mut DAT_0071e528).cast::<u8>();
                loop {
                    if pc.read() == 1 {
                        count = count.wrapping_add(1);
                    }
                    pc = pc.wrapping_byte_add(0x5bd);
                    if !(pc < end) {
                        break;
                    }
                }
                let idx = DAT_0071ecf4.wrapping_mul(0x5bd) as i32;
                if at((&raw mut DAT_0071b740).cast::<u8>(), idx).read() != 1 {
                    return;
                }
                if count != 1 {
                    _DAT_00732950 = count as i16;
                    FUN_004a1d70(0x174, DAT_0071ecf4);
                    return;
                }
                FUN_004a1d70(0x175, DAT_0071ecf4);
                return;
            }
            if param_1 != DAT_0071ecf4 {
                return;
            }
            FUN_004a41d0(0x118);
            return;
        }
        'lab: {
            if at((&raw mut DAT_0071b744).cast::<u8>(), i_var3).read() != 0 {
                break 'lab;
            }
            let tmp16 = at((&raw mut DAT_0071bb40).cast::<u8>(), i_var3)
                .cast::<u16>()
                .read_unaligned();
            if (DAT_0071e534 as u8 & 0xd) == 1
                && tmp16 == 0
                && at((&raw mut DAT_0071b743).cast::<u8>(), i_var3).read() == 0
            {
                FUN_00447010(param_1);
            }
            let mut u_var2 = at((&raw mut DAT_0071b781).cast::<u8>(), i_var3)
                .cast::<u16>()
                .read_unaligned()
                & 0xe;
            if (DAT_0071e534 as u8 & 0xe) == 2 {
                let mut t = at((&raw mut DAT_0071b783).cast::<u8>(), i_var3)
                    .cast::<u16>()
                    .read_unaligned();
                if u_var2 == 2 {
                    t = t.wrapping_add(DAT_0071e6be);
                } else {
                    t = DAT_0071e6be;
                }
                at((&raw mut DAT_0071b783).cast::<u8>(), i_var3)
                    .cast::<u16>()
                    .write_unaligned(t);
                at((&raw mut DAT_0071b781).cast::<u8>(), i_var3)
                    .cast::<u16>()
                    .write_unaligned(DAT_0071e534);
                if t < 0x3b {
                    DAT_0071e534 &= 0xfffd;
                }
                break 'lab;
            }
            let t = at((&raw mut DAT_0071b783).cast::<u8>(), i_var3)
                .cast::<u16>()
                .read_unaligned();
            'fa7: {
                if u_var2 == 2 && t < 0x3b {
                    if at((&raw mut DAT_0071b743).cast::<u8>(), i_var3).read() == 0 {
                        let t16b = at((&raw mut DAT_0071bb40).cast::<u8>(), i_var3)
                            .cast::<u16>()
                            .read_unaligned();
                        if DAT_007092c1 != 0 || t16b != 0 {
                            break 'fa7;
                        }
                        u_var2 = FUN_00441930(param_1) as u16;
                    } else {
                        u_var2 = FUN_00436120(param_1) as u16;
                    }
                    if u_var2 != 0xffff {
                        FUN_00442bf0(u_var2 as u32, param_1);
                    }
                }
            }
            at((&raw mut DAT_0071b781).cast::<u8>(), i_var3)
                .cast::<u16>()
                .write_unaligned(DAT_0071e534);
            at((&raw mut DAT_0071b783).cast::<u8>(), i_var3)
                .cast::<u16>()
                .write_unaligned(0);
        }
        if at((&raw mut DAT_0071bb1a).cast::<u8>(), i_var3).read() == 0 {
            FUN_0043a080(param_1, unaff_edi, unaff_esi, unaff_ebx);
        } else {
            FUN_00440d20(param_1);
        }
        if at((&raw mut DAT_0071b740).cast::<u8>(), i_var3).read() != 0 {
            FUN_0043fe30(param_1);
        }
    }
}
