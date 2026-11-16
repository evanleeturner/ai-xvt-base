// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1320: [i8; 1];
    static mut DAT_0051a3a8: [i8; 1];
    static mut DAT_0051a2bc: [i8; 1];
    static mut DAT_0051a394: [i8; 1];
    static mut DAT_0051a384: [i8; 1];
    static mut DAT_0051a374: [i8; 1];
    static mut DAT_0051a368: [i8; 1];
    static mut DAT_0051a358: [i8; 1];
    static mut DAT_0051a348: [i8; 1];
    static mut DAT_0051a334: [i8; 1];
    static mut DAT_0051a320: [i8; 1];
    static mut DAT_0051a1e4: [i8; 1];
    static mut DAT_0051a310: [i8; 1];
    static mut DAT_0051a2fc: [i8; 1];
    static mut DAT_0051a2e8: [i8; 1];
    static mut DAT_0051a2d4: [i8; 1];
    static mut DAT_0051a254: [i8; 1];
    static mut DAT_0051a244: [i8; 1];
    static mut DAT_0051a230: [i8; 1];
    static mut DAT_0051a278: [i8; 1];
    static mut DAT_0051a264: [i8; 1];
    static mut DAT_0051a21c: [i8; 1];
    static mut DAT_0051a208: [i8; 1];
    static mut DAT_0051a1f4: [i8; 1];
    static mut DAT_0051a2c4: [i8; 1];
    static mut DAT_0051a2a4: [i8; 1];
    static mut DAT_0051a28c: [i8; 1];
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009a73c0: u16;
    static mut DAT_009d8cd4: [u8; 1];
    static mut DAT_009d8cd5: [u8; 1];
    static mut DAT_009a20ca: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_00468cf0(param_1: u32, param_2: u32) -> u64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00468e40() -> u32;
    fn FUN_0045ddf0(param_1: u16) -> u16;
    fn FUN_00462830(param_1: u32) -> u16;
}

unsafe fn eq(a: *const i8, b: *const i8, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        // SAFETY: bytes are read only up to the first mismatch, as repe cmpsb does.
        if unsafe { a.add(i).read() != b.add(i).read() } {
            return false;
        }
        i += 1;
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00403fe0(param_1: u32, param_2: u16) -> u8 {
    let uvar1 = param_1 & 0xffff;
    // SAFETY: all globals are the game's own memory; pointers derived from
    // them are read where the original reads.
    unsafe {
        let p6 = (&raw mut DAT_009d1320 as *mut i8).add((uvar1.wrapping_mul(0x55)) as usize)
            as *const i8;
        let base_ptr = &raw mut DAT_009a73b6 as *const u32;
        let idx = (DAT_009a73c0 as u32)
            .wrapping_mul(0x566)
            .wrapping_add((param_2 as u32).wrapping_mul(0x52)) as usize;

        // category: 0 final, 1 check, 2 board
        let finals: [(*const i8, usize); 6] = [
            (&raw mut DAT_0051a3a8 as *const i8, 0xc),
            (&raw mut DAT_0051a394 as *const i8, 0x11),
            (&raw mut DAT_0051a384 as *const i8, 0x10),
            (&raw mut DAT_0051a374 as *const i8, 0xf),
            (&raw mut DAT_0051a368 as *const i8, 0xc),
            (&raw mut DAT_0051a358 as *const i8, 0x10),
        ];
        let checks: [(*const i8, usize); 9] = [
            (&raw mut DAT_0051a348 as *const i8, 0xf),
            (&raw mut DAT_0051a334 as *const i8, 0x14),
            (&raw mut DAT_0051a320 as *const i8, 0x12),
            (&raw mut DAT_0051a1e4 as *const i8, 0xe),
            (&raw mut DAT_0051a310 as *const i8, 0xf),
            (&raw mut DAT_0051a2fc as *const i8, 0x13),
            (&raw mut DAT_0051a2e8 as *const i8, 0x12),
            (&raw mut DAT_0051a2d4 as *const i8, 0x13),
            (core::ptr::null(), 0),
        ];
        let boards: [(*const i8, usize); 8] = [
            (&raw mut DAT_0051a254 as *const i8, 0xf),
            (&raw mut DAT_0051a244 as *const i8, 0xf),
            (&raw mut DAT_0051a230 as *const i8, 0x13),
            (&raw mut DAT_0051a278 as *const i8, 0x12),
            (&raw mut DAT_0051a264 as *const i8, 0x12),
            (&raw mut DAT_0051a21c as *const i8, 0x11),
            (&raw mut DAT_0051a208 as *const i8, 0x12),
            (&raw mut DAT_0051a1f4 as *const i8, 0x11),
        ];

        for (s, n) in finals.iter() {
            if eq(p6, *s, *n) {
                let base = base_ptr.read_unaligned() as usize;
                let v = ((base.wrapping_add(5).wrapping_add(param_2 as usize)) as *const u8).read();
                let t = (&raw mut DAT_009d8cd4 as *const u8).add(idx).read();
                return (t <= v) as u8;
            }
        }
        for (s, n) in checks.iter() {
            if *n == 0 {
                break;
            }
            if eq(p6, *s, *n) {
                if FUN_0045ddf0(param_2) != 0 {
                    return 0;
                }
                if FUN_00462830(param_2 as u32) != 0 {
                    return 0;
                }
                return 1;
            }
        }
        for (s, n) in boards.iter() {
            if eq(p6, *s, *n) {
                let base = base_ptr.read_unaligned() as usize;
                let v = ((base.wrapping_add(5).wrapping_add(param_2 as usize)) as *const u8).read();
                let t = (&raw mut DAT_009d8cd5 as *const u8).add(idx).read();
                return (v >= t) as u8;
            }
        }

        if eq(p6, &raw mut DAT_0051a2c4 as *const i8, 0xf) {
            let base = base_ptr.read_unaligned() as usize;
            let v =
                ((base.wrapping_add(5).wrapping_add(param_2 as usize)) as *const u8).read() as u16;
            let b = (&raw mut DAT_009d8cd5 as *const u8).add(idx).read();
            let i2 = (b.wrapping_sub(1) as u16 as u32).wrapping_mul(0x93) as usize;
            let t = (&raw mut DAT_009a20ca as *const u16).add(i2).read();
            if v < t {
                return 0;
            }
            return 1;
        }

        if eq(p6, &raw mut DAT_0051a2bc as *const i8, 8) {
            let base = base_ptr.read_unaligned() as usize;
            let t = (base.wrapping_add(0x36) as *const i32).read_unaligned();
            if t != 0 {
                return 0;
            }
            return 1;
        }

        if eq(p6, &raw mut DAT_0051a2a4 as *const i8, 0x16) {
            let r = FUN_00468cf0(0, uvar1.wrapping_mul(5));
            if (r as u16) == 0 {
                return 0;
            }
            return 1;
        }

        if !eq(p6, &raw mut DAT_0051a28c as *const i8, 0x16) {
            return 0;
        }
        let r = FUN_00468e40();
        if (r as u16) == 0 {
            return 0;
        }
        1
    }
}
