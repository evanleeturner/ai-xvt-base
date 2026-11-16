// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052d198: [i8; 1];
    static mut DAT_0052d1a4: [i8; 1];
    static mut DAT_0052d134: [i8; 1];
    static mut DAT_0052d0e0: [i8; 1];
    static mut DAT_0052d118: [i8; 1];
    static mut DAT_0052d0fc: [i8; 1];
    static mut DAT_0052bf74: [i8; 1];
    static mut DAT_0052d154: [i8; 1];
    static mut DAT_0052d178: [i8; 1];

    static mut DAT_006697cc: *mut c_void;
    static mut DAT_006697c8: u16;
    static mut DAT_0052d0c8: i32;
    static mut DAT_006697bc: i32;
    static mut DAT_006697f0: i32;
    static mut DAT_006697ec: i32;
    static mut DAT_006697d0: i32;
    static mut DAT_006697b0: i32;
    static mut DAT_006697a8: i32;
    static mut DAT_006697d4: i32;
    static mut DAT_006697b4: i32;
    static mut DAT_006697ac: i32;
    static mut DAT_006697b8: i32;
    static mut DAT_006697d8: i32;
    static mut DAT_006697c0: i32;
    static mut DAT_006697a0: i32;
    static mut DAT_006697dc: i32;
    static mut DAT_006697c4: i32;
    static mut DAT_006697a4: i32;
    static mut DAT_006697f8: u32;
    static mut DAT_006697e0: i32;
    static mut DAT_006697e4: i32;
    static mut DAT_006697e8: i32;
    static mut DAT_006697f4: i32;
    static mut DAT_00669800: [i8; 1];
    static mut DAT_0066b800: [i8; 1];
    static mut DAT_0066d800: [u16; 1];
    static mut DAT_0066d840: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcb30() -> i32;
    fn FUN_004cc590(f: *mut c_void);
    fn FUN_004cc4a0(name: *mut i8, mode: *mut i8) -> *mut c_void;
    fn FUN_004fbaf0(a: *mut u32, b: *mut i32, c: *mut i32, d: *mut i32) -> i32;
    fn FUN_004dc330(a: unsafe extern "C" fn(), b: unsafe extern "C" fn());
    fn FUN_004be1b0();
    fn FUN_004be050();
    fn FUN_004dc140(a: i32);
    fn FUN_004dca90(a: i32) -> u8;
    fn FUN_004b5570(name: *mut i8, x: i32, y: i32);
    fn FUN_004d4e20();
    fn FUN_004b5de0(name: *mut i8, x: i32, y: i32);
    fn FUN_004b4d40(name: *mut i8, x: i32, y: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc190();
    fn FUN_004db4e0(a: i32, s: *mut i8, x: i32, y: i32, c: u32);
    fn FUN_004dc1b0();
    fn FUN_004d3340();
    fn FUN_004d4f80();
    fn FUN_004d3520(a: i32, b: i32, c: i32);
    fn FUN_004dcad0() -> u32;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004d4c10();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004fb670(param_1: i32) -> u32 {
    let mut local_4: i32 = 0;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers handed on are addresses of the game's buffers.
    unsafe {
        if param_1 == 0 {
            FUN_004dcb30();
            if !DAT_006697cc.is_null() {
                FUN_004cc590(DAT_006697cc);
            }
            DAT_006697c8 = 0xffff;
            DAT_0052d0c8 = 0;
            DAT_006697bc = 0;
            DAT_006697f0 = 0;
            DAT_006697ec = 0;
            DAT_006697d0 = 0;
            DAT_006697b0 = 0;
            DAT_006697a8 = 0;
            DAT_006697d4 = 0;
            DAT_006697b4 = 0;
            DAT_006697ac = 0;
            DAT_006697cc = FUN_004cc4a0(
                (&raw mut DAT_0052d198).cast::<i8>(),
                (&raw mut DAT_0052d1a4).cast::<i8>(),
            );
            DAT_006697b8 = 0;
            DAT_006697d8 = 0;
            DAT_006697c0 = 0;
            DAT_006697a0 = 0;
            DAT_006697dc = 0;
            DAT_006697c4 = 0;
            DAT_006697a4 = 0;
            let i_var3 = FUN_004fbaf0(
                &raw mut DAT_006697f8,
                &raw mut DAT_006697b8,
                &raw mut DAT_006697e0,
                &raw mut DAT_006697e4,
            );
            if i_var3 == 0 {
                FUN_004dc330(FUN_004be1b0, FUN_004be050);
                return 0;
            }
            FUN_004dc140(200);
        }
        if DAT_0052d0c8 != 0 {
            FUN_004dc330(FUN_004be1b0, FUN_004be050);
            return 0;
        }
        if FUN_004dca90(0x10) != 0 && FUN_004dca90(0x12) != 0 && FUN_004dca90(0x7b) != 0 {
            let mut sel: Option<(i32, *mut i8)> = None;
            match DAT_006697bc {
                0 => sel = Some((0x9d, (&raw mut DAT_0052d134).cast::<i8>())),
                1 => sel = Some((0xa3, (&raw mut DAT_0052d0e0).cast::<i8>())),
                2 => sel = Some((0x98, (&raw mut DAT_0052d118).cast::<i8>())),
                4 => sel = Some((0x8f, (&raw mut DAT_0052d0fc).cast::<i8>())),
                _ => {}
            }
            if let Some((i, p)) = sel {
                FUN_004b5570(p, 0xa0, i);
            }
        }
        'l948: {
            if DAT_006697d0 == DAT_006697d8
                && DAT_006697a8 == DAT_006697a0
                && DAT_006697b0 == DAT_006697c0
                && DAT_006697d4 == DAT_006697dc
                && DAT_006697ac == DAT_006697a4
                && DAT_006697b4 == DAT_006697c4
            {
                break 'l948;
            }
            DAT_006697d8 = DAT_006697d0;
            DAT_006697a0 = DAT_006697a8;
            DAT_006697c0 = DAT_006697b0;
            DAT_006697dc = DAT_006697d4;
            DAT_006697a4 = DAT_006697ac;
            DAT_006697c4 = DAT_006697b4;
            FUN_004d4e20();
            FUN_004b5de0((&raw mut DAT_0052bf74).cast::<i8>(), 0, 0);
            if DAT_006697d0 == 1 {
                FUN_004b4d40(
                    (&raw mut DAT_0052d154).cast::<i8>(),
                    DAT_006697a8,
                    DAT_006697b0,
                );
            } else if DAT_006697d0 == 2 {
                FUN_004b4d40(
                    (&raw mut DAT_0052d178).cast::<i8>(),
                    DAT_006697a8,
                    DAT_006697b0,
                );
            }
            if DAT_006697d4 == 1 {
                FUN_004b4d40(
                    (&raw mut DAT_0052d154).cast::<i8>(),
                    DAT_006697ac,
                    DAT_006697b4,
                );
            } else if DAT_006697d4 == 2 {
                FUN_004b4d40(
                    (&raw mut DAT_0052d178).cast::<i8>(),
                    DAT_006697ac,
                    DAT_006697b4,
                );
            }
            FUN_004d4ec0(1);
        }
        if (DAT_006697f8 & 1) != 0 {
            FUN_004dc190();
        }
        let i_var3 = DAT_006697f0;
        let mut i_var5 = DAT_006697e8;
        let mut pc = (&raw mut DAT_00669800).cast::<i8>();
        let mut p6 = (&raw mut DAT_0066d800).cast::<u16>();
        let end1 = (&raw mut DAT_0066d840).cast::<u16>();
        loop {
            let p7 = p6.wrapping_add(1);
            FUN_004db4e0(0xf, pc, i_var3, i_var5, u32::from(p6.read_unaligned()));
            i_var5 = i_var5.wrapping_add(0x13);
            pc = pc.wrapping_add(0x100);
            p6 = p7;
            if !((p7 as usize) < (end1 as usize)) {
                break;
            }
        }
        if (DAT_006697f8 & 1) == 0 {
            FUN_004dc190();
        } else {
            FUN_004dc1b0();
        }
        let i_var3 = DAT_006697f4;
        let mut p6 = (&raw mut DAT_0066d840).cast::<u16>();
        let mut pc = (&raw mut DAT_0066b800).cast::<i8>();
        let mut i_var5 = DAT_006697ec;
        let end2 = end1.wrapping_add(32);
        loop {
            let p7 = p6.wrapping_add(1);
            FUN_004db4e0(0xf, pc, i_var3, i_var5, u32::from(p6.read_unaligned()));
            p6 = p7;
            pc = pc.wrapping_add(0x100);
            i_var5 = i_var5.wrapping_add(0x13);
            if !((p7 as usize) < (end2 as usize)) {
                break;
            }
        }
        FUN_004dc1b0();
        if DAT_006697b8 == 0 {
            if DAT_006697e0 <= param_1 {
                DAT_0052d0c8 = 1;
                FUN_004d4f80();
                FUN_004d3520(0x8000, 0x1000, 2000);
            }
        } else if DAT_006697e0 <= param_1 {
            FUN_004fbaf0(
                &raw mut DAT_006697f8,
                &raw mut DAT_006697b8,
                &raw mut local_4,
                &raw mut DAT_006697e4,
            );
            DAT_006697e0 = DAT_006697e0.wrapping_add(local_4);
            FUN_004dc140(DAT_006697e4);
            DAT_006697bc = DAT_006697bc.wrapping_add(1);
            if DAT_006697b8 == 0 {
                FUN_004d3340();
            }
        }
        let u_var4 = FUN_004dcad0() & 0xff;
        if FUN_004dc220() != 0
            || FUN_004dc240() != 0
            || u_var4 == 0x1b
            || u_var4 == 0xd
            || u_var4 == 0x20
        {
            DAT_0052d0c8 = 1;
            FUN_004d4f80();
            FUN_004d4c10();
            FUN_004d3520(0x8000, 0x1000, 2000);
        }
        0
    }
}
