// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757cee: i32;
    static mut DAT_00757e3d: i32;
    static mut DAT_00757ce0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ee3d0() -> u32 {
    // SAFETY: all globals are the game's own memory; pointers are formed and
    // dereferenced at the same byte offsets as the original, unaligned reads
    // and writes being used throughout.
    unsafe {
        let base_cee = &raw mut DAT_00757cee as *mut i32;
        let base_ce0 = &raw mut DAT_00757ce0 as *mut u8;
        let end = (&raw mut DAT_00757e3d) as usize;

        let mut p4: *mut i32 = base_cee;
        let mut i5: i32 = 0;

        loop {
            if p4.read_unaligned() == 0 {
                p4 = p4.wrapping_byte_add(0x2a);
                i5 = i5.wrapping_add(1);
                if end < p4 as usize {
                    return 1;
                }
                continue;
            }
            let mut u2: u32 = (i5 as u32).wrapping_add(1);
            if u2 < 8 {
                p4 = p4.wrapping_byte_add(0x2a);
                i5 = i5.wrapping_add(1);
                if end < p4 as usize {
                    return 1;
                }
                continue;
            }

            let mut p1: *mut i32 = base_cee.wrapping_byte_add(u2.wrapping_mul(0x2a) as usize);
            while p1.read_unaligned() == 0 {
                p1 = p1.wrapping_byte_add(0x2a);
                u2 = u2.wrapping_add(1);
                if end < p1 as usize {
                    p4 = p4.wrapping_byte_add(0x2a);
                    i5 = i5.wrapping_add(1);
                    if end < p4 as usize {
                        return 1;
                    }
                    continue;
                }
            }

            let mut src = base_ce0.wrapping_byte_add(u2.wrapping_mul(0x2a) as usize) as *const u32;
            let mut dst = p4.wrapping_byte_sub(0xe) as *mut u32;
            let mut n = 10;
            while n != 0 {
                dst.write_unaligned(src.read_unaligned());
                dst = dst.wrapping_add(1);
                src = src.wrapping_add(1);
                n -= 1;
            }
            (dst as *mut u16).write_unaligned((src as *const u16).read_unaligned());
            base_cee
                .wrapping_byte_add(u2.wrapping_mul(0x2a) as usize)
                .write_unaligned(0);

            p4 = p4.wrapping_byte_add(0x2a);
            i5 = i5.wrapping_add(1);
            if end < p4 as usize {
                return 1;
            }
        }
    }
}
