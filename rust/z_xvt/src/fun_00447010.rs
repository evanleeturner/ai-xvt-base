// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0071b758: i8;
    static mut DAT_0071e6be: i16;
    static mut DAT_0071b757: i8;
    static mut DAT_0071bcbc: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_00732950: i16;
    static mut DAT_00732952: i16;
    static mut DAT_00525564: i32;
    static mut DAT_0052b0b4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_00447740(a: i32, b: u32);
    fn FUN_00447240(a: i32, b: u32);
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: address is inside the game's own memory.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}
unsafe fn r16(a: i32) -> i16 {
    // SAFETY: address is inside the game's own memory.
    unsafe { (a as u32 as usize as *const i16).read_unaligned() }
}
unsafe fn r8(a: i32) -> u8 {
    // SAFETY: address is inside the game's own memory.
    unsafe { (a as u32 as usize as *const u8).read() }
}
unsafe fn w32(a: i32, v: i32) {
    // SAFETY: address is inside the game's own memory.
    unsafe { (a as u32 as usize as *mut i32).write_unaligned(v) }
}
unsafe fn w16(a: i32, v: i16) {
    // SAFETY: address is inside the game's own memory.
    unsafe { (a as u32 as usize as *mut i16).write_unaligned(v) }
}
unsafe fn w8(a: i32, v: u8) {
    // SAFETY: address is inside the game's own memory.
    unsafe { (a as u32 as usize as *mut u8).write(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00447010(param_1: i32) {
    // SAFETY: all globals and addresses are the game's own memory.
    unsafe {
        let iv3 = param_1.wrapping_mul(0x5bd);
        let a730 = (&raw mut DAT_0071b730) as usize as i32;
        let a758 = (&raw mut DAT_0071b758) as usize as i32;
        let a757 = (&raw mut DAT_0071b757) as usize as i32;
        let abcbc = (&raw mut DAT_0071bcbc) as usize as i32;
        let ab0b4 = (&raw mut DAT_0052b0b4) as usize as i32;

        let mut iv1 = r32(a730.wrapping_add(iv3));
        if iv1 == -1 {
            return;
        }
        let iv2 = r32(r32(iv1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988))
        .wrapping_add(0xa7));
        if r32(iv2.wrapping_add(0x1d)) != 0
            && (r8(iv2.wrapping_add(0x23c)) != 1 || r16(iv2.wrapping_add(0x23e)) == 0)
        {
            FUN_004a1d70(0x15f, param_1);
            return;
        }
        let d = DAT_0071e6be as i32;
        let half = (d >> 1).wrapping_add(d) as i16;
        let b757 = r8(a757.wrapping_add(iv3)) as u32;
        if r8(a758.wrapping_add(iv3)) != 0 {
            if half <= r16(iv2.wrapping_add(0x22d).wrapping_add((b757 * 2) as i32)) {
                return;
            }
            if r8(iv2.wrapping_add(0x1df)) & 8 != 0 {
                FUN_00447740(iv1, b757);
                iv1 = r32(r32(iv1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_006d4988))
                .wrapping_add(0xa7));
                let b757b = r8(a757.wrapping_add(iv3)) as i32;
                let idx = r8(ab0b4
                    .wrapping_add(b757b)
                    .wrapping_add((r8(iv1.wrapping_add(4)) as i32).wrapping_mul(0xef)))
                    as i32;
                let s = (r8(iv1.wrapping_add(0x34a).wrapping_add(idx.wrapping_mul(4))) as u32)
                    .wrapping_add(
                        r8(iv1.wrapping_add(0x346).wrapping_add(idx.wrapping_mul(4))) as u32,
                    );
                if s != 0 {
                    return;
                }
                w8(a758.wrapping_add(iv3), 0);
                w8(a757.wrapping_add(iv3), 0);
                w16(iv1.wrapping_add(0x21c), 0x76);
                w32(
                    iv1.wrapping_add(0x220),
                    r32(abcbc.wrapping_add(iv3)).wrapping_add(0x76),
                );
                return;
            }
            if param_1 != DAT_0071ecf4 {
                return;
            }
            DAT_00732950 = 0x5d;
            DAT_00732952 = 0x57;
            FUN_004a1d70(0x56, param_1);
            return;
        }
        let u4 = b757 as i32;
        let mut s5 = r16(iv2.wrapping_add(0x21c).wrapping_add(u4.wrapping_mul(2)));
        'lab: {
            if DAT_00525564 != 0 {
                if s5 != 0 {
                    if r32(abcbc.wrapping_add(iv3))
                        <= r32(iv2.wrapping_add(0x220).wrapping_add(u4.wrapping_mul(4)))
                    {
                        s5 = DAT_0071e6be.wrapping_mul(2);
                        break 'lab;
                    }
                    s5 = 0;
                }
                w32(
                    iv2.wrapping_add(0x220).wrapping_add(u4.wrapping_mul(4)),
                    r32(abcbc.wrapping_add(iv3)),
                );
            }
        }
        if s5 < half {
            if r8(iv2.wrapping_add(0x1df)) & 0x10 != 0 {
                FUN_00447240(
                    r32(a730.wrapping_add(iv3)),
                    r8(a757.wrapping_add(iv3)) as u32,
                );
                return;
            }
            if param_1 == DAT_0071ecf4 {
                DAT_00732952 = 0x57;
                DAT_00732950 = (r8(a757.wrapping_add(iv3)) as i32).wrapping_add(0x5b) as i16;
                FUN_004a1d70(0x56, param_1);
                return;
            }
        }
    }
}
