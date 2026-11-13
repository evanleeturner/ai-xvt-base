// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006691e6: i16;
    static mut _DAT_006691e8: i16;
    static mut DAT_00669204: i16;
    static mut DAT_00669206: i16;
    static mut DAT_00669208: i16;
    static mut DAT_0066920a: i16;
    static mut _DAT_00669214: u32;
    static mut DAT_0066920c: i16;
    static mut DAT_00669218: u32;
    static mut DAT_0066920e: i16;
    static mut DAT_00669210: i16;
    static mut DAT_00669212: i16;
    static mut DAT_0066921c: u32;
    static mut DAT_00a91cc0: u32;
    static mut DAT_00aa5aac: i8;
    static mut DAT_00aa4721: i8;
    static mut DAT_00aa471e: i8;
    static mut DAT_00669582: *mut core::ffi::c_void;
    static mut DAT_00669602: *mut core::ffi::c_void;
    static mut DAT_00669682: *mut core::ffi::c_void;
    static mut DAT_006696da: u32;
    static mut DAT_006696e4: u32;
    static mut DAT_00669716: u32;
    static mut DAT_00669220: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004f7340();
    fn FUN_004f7380();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004d6460(p1: *mut u32, p2: u32, p3: u32, p4: u32, p5: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f69b0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // array writes stay within the original's bounds.
    unsafe {
        DAT_006691e6 = 1;
        let mut s: i16 = 0;
        _DAT_006691e8 = 0;
        DAT_00669204 = 0;
        DAT_00669206 = 0;
        DAT_00669208 = 0;
        DAT_0066920a = 0;
        _DAT_00669214 = 0;
        DAT_0066920c = 0x20;
        DAT_00669218 = 0;
        DAT_0066920e = 0x20;
        DAT_00669210 = 0x20;
        DAT_00669212 = 0x20;
        DAT_0066921c = 0;
        let mut p = &raw mut DAT_00a91cc0;
        let mut n: i32 = 0x4f7d;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        DAT_00aa5aac = 1;
        DAT_00aa4721 = 0;
        DAT_00aa471e = 1;
        FUN_004f7340();
        FUN_004f7380();
        loop {
            let v = _malloc(0x28);
            let i = s as i32;
            s = s.wrapping_add(1);
            (&raw mut DAT_00669582)
                .offset(i as isize)
                .write_unaligned(v);
            if !(s < 0x20) {
                break;
            }
        }
        s = 0;
        loop {
            let v = _malloc(0x140);
            let i = s as i32;
            s = s.wrapping_add(1);
            (&raw mut DAT_00669602)
                .offset(i as isize)
                .write_unaligned(v);
            if !(s < 0x20) {
                break;
            }
        }
        s = 0;
        loop {
            let v = _malloc(0x400);
            let i = s as i32;
            s = s.wrapping_add(1);
            (&raw mut DAT_00669682)
                .offset(i as isize)
                .write_unaligned(v);
            if !(s < 0x14) {
                break;
            }
        }
        s = 0;
        loop {
            let i = s as i32;
            s = s.wrapping_add(1);
            (&raw mut DAT_006696da)
                .offset(i as isize)
                .write_unaligned(0);
            if !(s < 2) {
                break;
            }
        }
        s = 0;
        loop {
            let i = s as i32;
            s = s.wrapping_add(1);
            (&raw mut DAT_006696e4)
                .offset(i as isize)
                .write_unaligned(0);
            if !(s < 8) {
                break;
            }
        }
        s = 0;
        loop {
            let i = s as i32;
            s = s.wrapping_add(1);
            (&raw mut DAT_00669716)
                .offset(i as isize)
                .write_unaligned(0);
            if !(s < 8) {
                break;
            }
        }
        FUN_004d6460(&raw mut DAT_00669220, 0, 0, 0x168, 0xec);
    }
}
