// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040f050(param_1: u16);
}

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    // SAFETY: the address is a game-memory location the original writes.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: the address is a game-memory location the original writes.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (a as usize as *const u8).read() }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: the address is a game-memory location the original writes.
    unsafe { (a as usize as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn copy_dwords(mut dst: u32, mut src: u32, n: u32) -> (u32, u32) {
    let mut i = n;
    while i != 0 {
        // SAFETY: forward dword copy as in the original's loop.
        unsafe { wr32(dst, rd32(src)) };
        src = src.wrapping_add(4);
        dst = dst.wrapping_add(4);
        i -= 1;
    }
    (dst, src)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00422f60(param_1: i32, param_2: i32) {
    // SAFETY: all addresses are the game's own records, accessed as the original does.
    unsafe {
        let i4 = (param_1 as u32).wrapping_mul(0x23);
        let o2 = (param_2 as u32).wrapping_mul(0x23);

        let mut p7 = rd32(
            rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4)).wrapping_add(0xa7),
        );
        if p7 != 0 {
            let mut p6 = rd32(
                rd32(o2.wrapping_add(0x1f).wrapping_add(DAT_006d4988 as u32)).wrapping_add(0xa7),
            );
            if p6 != 0 {
                let (d, s) = copy_dwords(p7, p6, 0x10d);
                p7 = d;
                p6 = s;
                wr16(p7, rd16(p6));
            }
        }

        let p7 = rd32(
            rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4)).wrapping_add(0xa3),
        );
        if p7 != 0 {
            let p6 = rd32(
                rd32(o2.wrapping_add(0x1f).wrapping_add(DAT_006d4988 as u32)).wrapping_add(0xa3),
            );
            if p6 != 0 {
                wr32(p7, rd32(p6));
                wr32(p7.wrapping_add(4), rd32(p6.wrapping_add(4)));
            }
        }

        let p7 = rd32(
            rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4)).wrapping_add(0xab),
        );
        if p7 != 0 {
            let p6 = rd32(
                rd32(o2.wrapping_add(0x1f).wrapping_add(DAT_006d4988 as u32)).wrapping_add(0xab),
            );
            if p6 != 0 {
                copy_dwords(p7, p6, 0x13);
            }
        }

        let p7 = rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4));
        if p7 != 0 {
            let p6 = rd32(o2.wrapping_add(0x1f).wrapping_add(DAT_006d4988 as u32));
            if p6 != 0 {
                let u1 = rd32(p7.wrapping_add(0xa7));
                let u2 = rd32(p7.wrapping_add(0xa3));
                let u3 = rd32(p7.wrapping_add(0xab));
                let (d, s) = copy_dwords(p7, p6, 0x2b);
                wr16(d, rd16(s));
                wr8(d.wrapping_add(2), rd8(s.wrapping_add(2)));
                wr32(
                    rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4))
                        .wrapping_add(0xa7),
                    u1,
                );
                wr32(
                    rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4))
                        .wrapping_add(0xa3),
                    u2,
                );
                wr32(
                    rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4))
                        .wrapping_add(0xab),
                    u3,
                );
            }
        }

        let u1 = rd32((DAT_006d4988 as u32).wrapping_add(i4).wrapping_add(0x1f));
        let src = o2.wrapping_add(DAT_006d4988 as u32);
        let dst = (DAT_006d4988 as u32).wrapping_add(i4);
        let (d, s) = copy_dwords(dst, src, 8);
        wr16(d, rd16(s));
        wr8(d.wrapping_add(2), rd8(s.wrapping_add(2)));
        wr32(
            (DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i4),
            u1,
        );
        FUN_0040f050(param_1 as u16);
    }
}
