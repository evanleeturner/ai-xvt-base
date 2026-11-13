// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041bb00(param_1: u16);
}

unsafe fn rd32(a: i32) -> i32 {
    // SAFETY: the address is a game pointer, read where the original reads.
    unsafe { (a as *const i32).read_unaligned() }
}

unsafe fn wr32(a: i32, v: i32) {
    // SAFETY: the address is a game pointer, written where the original writes.
    unsafe { (a as *mut i32).write_unaligned(v) }
}

unsafe fn rd16(a: i32) -> i16 {
    // SAFETY: the address is a game pointer, read where the original reads.
    unsafe { (a as *const i16).read_unaligned() }
}

unsafe fn wr16(a: i32, v: i16) {
    // SAFETY: the address is a game pointer, written where the original writes.
    unsafe { (a as *mut i16).write_unaligned(v) }
}

unsafe fn cp32(dst: i32, src: i32, n: i32) {
    let mut i = 0i32;
    while i < n {
        let off = i.wrapping_mul(4);
        // SAFETY: forward dword copy between the game's own buffers.
        unsafe { wr32(dst.wrapping_add(off), rd32(src.wrapping_add(off))) };
        i = i.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00459f30(param_1: i32, param_2: i32) {
    // SAFETY: all addresses derive from the game's own table in DAT_009a1fe8,
    // accessed exactly as the original does.
    unsafe {
        let i_var4 = param_1.wrapping_mul(0x23);
        let p2 = param_2.wrapping_mul(0x23);

        let d = DAT_009a1fe8;
        let dst = rd32(rd32(d.wrapping_add(0x1f).wrapping_add(i_var4)).wrapping_add(0xa9));
        if dst != 0 {
            let src =
                rd32(rd32(p2.wrapping_add(0x1f).wrapping_add(DAT_009a1fe8)).wrapping_add(0xa9));
            if src != 0 {
                cp32(dst, src, 0x118);
                wr16(
                    dst.wrapping_add(0x118 * 4),
                    rd16(src.wrapping_add(0x118 * 4)),
                );
            }
        }

        let dst =
            rd32(rd32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4)).wrapping_add(0xa5));
        if dst != 0 {
            let src =
                rd32(rd32(p2.wrapping_add(0x1f).wrapping_add(DAT_009a1fe8)).wrapping_add(0xa5));
            if src != 0 {
                wr32(dst, rd32(src));
                wr32(dst.wrapping_add(4), rd32(src.wrapping_add(4)));
                wr16(dst.wrapping_add(8), rd16(src.wrapping_add(8)));
            }
        }

        let dst =
            rd32(rd32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4)).wrapping_add(0xad));
        if dst != 0 {
            let src =
                rd32(rd32(p2.wrapping_add(0x1f).wrapping_add(DAT_009a1fe8)).wrapping_add(0xad));
            if src != 0 {
                cp32(dst, src, 0x13);
            }
        }

        let dst = rd32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4));
        if dst != 0 {
            let src = rd32(p2.wrapping_add(0x1f).wrapping_add(DAT_009a1fe8));
            if src != 0 {
                let u1 = rd32(dst.wrapping_add(0xa9));
                let u2 = rd32(dst.wrapping_add(0xa5));
                let u3 = rd32(dst.wrapping_add(0xad));
                cp32(dst, src, 0x2c);
                (dst.wrapping_add(0x2c * 4) as *mut u8)
                    .write((src.wrapping_add(0x2c * 4) as *const u8).read());
                wr32(
                    rd32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4)).wrapping_add(0xa9),
                    u1,
                );
                wr32(
                    rd32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4)).wrapping_add(0xa5),
                    u2,
                );
                wr32(
                    rd32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4)).wrapping_add(0xad),
                    u3,
                );
            }
        }

        let u1 = rd32(DAT_009a1fe8.wrapping_add(i_var4).wrapping_add(0x1f));
        let src = p2.wrapping_add(DAT_009a1fe8);
        let dst = DAT_009a1fe8.wrapping_add(i_var4);
        cp32(dst, src, 8);
        wr16(dst.wrapping_add(32), rd16(src.wrapping_add(32)));
        (dst.wrapping_add(34) as *mut u8).write((src.wrapping_add(34) as *const u8).read());
        wr32(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var4), u1);
        FUN_0041bb00(param_1 as u16);
    }
}
