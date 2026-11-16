// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005250b4: i8;
    static mut DAT_00524fd4: i8;
    static mut DAT_00703ca0: i8;
    static mut DAT_0070ad94: u8;
    static mut DAT_0070ad95: u8;
    static mut DAT_006d4a6a: u16;
    static mut DAT_006d9d56: [u8; 4];
    static mut DAT_006d9d60: u16;

    static mut DAT_005250a0: i8;
    static mut DAT_00525090: i8;
    static mut DAT_00525080: i8;
    static mut DAT_00524914: i8;
    static mut DAT_00525070: i8;
    static mut DAT_00525060: i8;
    static mut DAT_0052504c: i8;
    static mut DAT_00525038: i8;
    static mut DAT_00524904: i8;
    static mut DAT_00525028: i8;
    static mut DAT_00525014: i8;
    static mut DAT_00525000: i8;
    static mut DAT_00524fec: i8;
    static mut DAT_00523c18: i8;
    static mut DAT_00524f6c: i8;
    static mut DAT_00524f58: i8;
    static mut DAT_00524f90: i8;
    static mut DAT_00524f7c: i8;
    static mut DAT_00524f44: i8;
    static mut DAT_00524f30: i8;
    static mut DAT_00524f1c: i8;
    static mut DAT_00524fdc: i8;
    static mut DAT_00524fbc: i8;
    static mut DAT_00524fa4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044c7b0(param_1: i32, param_2: u32, param_3: u16) -> u8;
    fn FUN_0044fd90(param_1: u32) -> u8;
    fn FUN_00456330() -> u32;
    fn FUN_00456470() -> u32;
}

unsafe fn eq_n(a: *const i8, b: *const i8, n: u32) -> bool {
    let mut i = 0usize;
    while i < n as usize {
        // SAFETY: byte-wise compare stopping at the first mismatch, like repe cmpsb.
        if unsafe { a.add(i).read() != b.add(i).read() } {
            return false;
        }
        i += 1;
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042ab10(param_1: u32, param_2: u16) -> u8 {
    // SAFETY: all globals and callees are the game's own memory.
    unsafe {
        let u_var1: u32 = param_1 & 0xffff;
        let p6: *const i8 =
            (&raw mut DAT_00703ca0 as *const i8).wrapping_add(u_var1.wrapping_mul(0x55) as usize);
        let p2 = param_2 as u32;

        macro_rules! eq {
            ($d:ident, $n:expr) => {
                eq_n(p6, &raw const $d as *const i8, $n)
            };
        }

        let idx = (DAT_006d9d60 as u32)
            .wrapping_mul(0x566)
            .wrapping_add(p2.wrapping_mul(0x52));

        let mut u_var7: u16 = 0;
        let mut is_call = false;
        let mut is_final = false;
        let mut is_board = false;

        if eq!(DAT_005250b4, 0xc)
            || eq!(DAT_005250a0, 0x11)
            || eq!(DAT_00525090, 0x10)
            || eq!(DAT_00525080, 0xf)
            || eq!(DAT_00524914, 0xc)
            || eq!(DAT_00525070, 0x10)
        {
            is_final = true;
        } else {
            let mut n: u32 = 0;
            if eq!(DAT_00525060, 0xf) {
                n = 0xf;
            } else if eq!(DAT_0052504c, 0x14) {
                n = 0x14;
            } else if eq!(DAT_00525038, 0x12) {
                n = 0x12;
            } else if eq!(DAT_00524904, 0xe) {
                n = 0xe;
            } else if eq!(DAT_00525028, 0xf) {
                n = 0xf;
            } else if eq!(DAT_00525014, 0x13) {
                n = 0x13;
            } else if eq!(DAT_00525000, 0x12) {
                n = 0x12;
            } else if eq!(DAT_00524fec, 0x13) {
                n = 0x13;
            }
            if n != 0 {
                is_call = true;
                u_var7 = ((p6 as u32).wrapping_add(n) >> 16) as u16;
            } else if eq!(DAT_00523c18, 0xf)
                || eq!(DAT_00524f6c, 0xf)
                || eq!(DAT_00524f58, 0x13)
                || eq!(DAT_00524f90, 0x12)
                || eq!(DAT_00524f7c, 0x12)
                || eq!(DAT_00524f44, 0x11)
                || eq!(DAT_00524f30, 0x12)
                || eq!(DAT_00524f1c, 0x11)
            {
                is_board = true;
            }
        }

        if is_call {
            if FUN_0044c7b0(0, u_var1.wrapping_mul(5), param_2) as i16 != 0 {
                return 0;
            }
            if FUN_0044fd90(((u_var7 as u32) << 16) | p2) as i16 != 0 {
                return 0;
            }
            return 1;
        }

        let u_base = (&raw mut DAT_006d9d56 as *const u32).read_unaligned();
        let cur = ((u_base.wrapping_add(p2).wrapping_add(5)) as usize as *const u8).read();

        if is_final {
            let a = (&raw mut DAT_0070ad94 as *const u8)
                .byte_add(idx as usize)
                .read();
            return (a <= cur) as u8;
        }
        if is_board {
            let a = (&raw mut DAT_0070ad95 as *const u8)
                .byte_add(idx as usize)
                .read();
            return if cur < a { 0 } else { 1 };
        }

        if eq!(DAT_00524fdc, 0xf) {
            let b = (&raw mut DAT_0070ad95 as *const u8)
                .byte_add(idx as usize)
                .read();
            let k = (b as i32).wrapping_sub(1) as u16 as usize;
            let lim = (&raw mut DAT_006d4a6a as *const u16)
                .add(k.wrapping_mul(0x93))
                .read();
            return if (cur as u16) < lim { 0 } else { 1 };
        }

        if eq!(DAT_00524fd4, 8) {
            let v = (u_base.wrapping_add(0x36) as usize as *const i32).read_unaligned();
            return if v != 0 { 0 } else { 1 };
        }

        if eq!(DAT_00524fbc, 0x16) {
            let r = FUN_00456330();
            return if r as u16 as i16 == 0 { 0 } else { 1 };
        }

        if !eq!(DAT_00524fa4, 0x16) {
            return 0;
        }
        let r = FUN_00456470();
        if r as u16 as i16 == 0 { 0 } else { 1 }
    }
}
