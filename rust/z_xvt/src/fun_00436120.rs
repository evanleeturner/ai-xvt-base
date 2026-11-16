// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b743: u8;
    static mut DAT_0071bb1b: u8;
    static mut DAT_0071bb1f: u8;
    static mut DAT_0071bb23: u8;
    static mut DAT_0071bb27: u8;
    static mut DAT_0071bb29: u8;
    static mut DAT_0071bb2b: u8;
    static mut DAT_0071bb2d: u8;
    static mut DAT_0071bb2f: u8;
    static mut DAT_0071bb33: u8;
    static mut DAT_0071bb35: u8;
    static mut DAT_0052686c: u8;
    static mut DAT_006d4988: i32;
    static mut DAT_006da9f0: i16;
    static mut DAT_00732e40: i16;
    static mut DAT_006db594: i32;
    static mut DAT_006da538: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_0070acc4: i32;
    static mut DAT_0071e53c: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00483180(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i32);
    fn FUN_00418cd0(a: i32, b: i32, c: i32);
    fn FUN_004340e0(a: u32, b: u32, c: i32) -> u8;
}

unsafe fn idiv(dividend: i32, divisor: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv; faults as the original does.
    unsafe {
        asm!("cdq", "idiv {d:e}", d = in(reg) divisor, inout("eax") dividend => q, out("edx") _);
    }
    q
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: caller supplies an address in the game's memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: caller supplies an address in the game's memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: caller supplies an address in the game's memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn calc(iv2: i32, idx: u32) -> u32 {
    // SAFETY: globals are the game's own memory.
    unsafe {
        let mut dividend = (DAT_006db7ac as u32).wrapping_shl((DAT_0070acc4 & 0x1f) as u32) as i32;
        let i5 = idiv(dividend, DAT_006db7b4);
        dividend = (DAT_006db7b0 as u32).wrapping_shl((DAT_0070acc4 & 0x1f) as u32) as i32;
        let i6 = idiv(dividend, DAT_006db7b4);
        let mut u7 = (i6 as u32)
            .wrapping_mul(i6 as u32)
            .wrapping_add((i5 as u32).wrapping_mul(i5 as u32));
        let us1 = rd16((&raw mut DAT_0071bb29 as usize as u32).wrapping_add(iv2 as u32));
        let us2 = rd16((&raw mut DAT_0071bb27 as usize as u32).wrapping_add(iv2 as u32));
        if us1 as u32 == idx || us2 as u32 == idx {
            let a = (DAT_006db594 >> 4) as u32;
            let b = (DAT_006da538 >> 4) as u32;
            u7 = u7
                .wrapping_add(a.wrapping_mul(a))
                .wrapping_add(b.wrapping_mul(b));
        }
        u7
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00436120(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; addresses are
    // computed and read where the original does.
    unsafe {
        let iv2 = (param_1 as u32).wrapping_mul(0x5bd) as i32;
        let off = iv2 as u32;
        let s4v: i16;
        let s11: i16;
        let s10: i16;
        let mut skip = false;
        if rd8((&raw mut DAT_0071b743 as usize as u32).wrapping_add(off)) < 2 {
            let s0 = rd16((&raw mut DAT_0071bb2f as usize as u32).wrapping_add(off)) as i16;
            let s1 = rd16((&raw mut DAT_0071bb2b as usize as u32).wrapping_add(off)) as i16;
            let s2 = rd16((&raw mut DAT_0071bb2d as usize as u32).wrapping_add(off)) as i16;
            let s3 = rd16((&raw mut DAT_0071bb33 as usize as u32).wrapping_add(off)) as i16;
            let s4 = rd16((&raw mut DAT_0071bb35 as usize as u32).wrapping_add(off)) as i16;
            FUN_00483180(s0, s1, s2, 0, s3, s4, 0);
            let us0 = rd16((&raw mut DAT_0071bb29 as usize as u32).wrapping_add(off));
            if us0 == 0xffff {
                skip = true;
                s4v = 0;
                s11 = 0;
                s10 = 0;
            } else {
                let iv9 = (us0 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988 as u32);
                let i0 = rd32(iv9.wrapping_add(4));
                let i1 = rd32(iv9.wrapping_add(8));
                let i2 = rd32(iv9.wrapping_add(0xc));
                let j0 = rd32((&raw mut DAT_0071bb1b as usize as u32).wrapping_add(off));
                let j1 = rd32((&raw mut DAT_0071bb1f as usize as u32).wrapping_add(off));
                let j2 = rd32((&raw mut DAT_0071bb23 as usize as u32).wrapping_add(off));
                FUN_00418cd0(
                    i0.wrapping_sub(j0) as i32,
                    i1.wrapping_sub(j1) as i32,
                    i2.wrapping_sub(j2) as i32,
                );
                s4v = 0;
                s11 = DAT_006da9f0;
                s10 = DAT_00732e40;
            }
        } else {
            let b = (rd8((&raw mut DAT_0071b743 as usize as u32).wrapping_add(off)) & 0x7f) as i32;
            s4v = ((b << 0xe) / -0x7f) as i16;
            s11 = 0;
            s10 = 0x4000;
        }
        if !skip {
            FUN_00483180(0, s10, s11, 0, s4v, 0, 0);
        }

        let mut local_4: u32 = 0xffff;
        let mut iv9: i32 = 0;
        let mut u8_: u32 = 0;
        let mut local_8: u32 = (DAT_006db594 as u32)
            .wrapping_mul(DAT_006db594 as u32)
            .wrapping_add((DAT_006da538 as u32).wrapping_mul(DAT_006da538 as u32))
            >> 3;
        if 0 < DAT_0071e53c {
            loop {
                let base = (DAT_006d4988 as u32).wrapping_add(iv9 as u32);
                let u7 = rd8(base.wrapping_add(3)) as u32;
                if u7 != 0 && rd8(base.wrapping_add(2)) <= 5 {
                    let us0 = rd16(
                        (&raw mut DAT_0052686c as usize as u32).wrapping_add(u7.wrapping_mul(0x16)),
                    );
                    let b3 = FUN_004340e0(u8_, us0 as u32, param_1);
                    if b3 != 0 {
                        let v = calc(iv2, u8_);
                        if (v as i32) < (local_8 as i32) {
                            local_8 = v;
                            local_4 = u8_;
                        }
                    }
                }
                iv9 = (iv9 as u32).wrapping_add(0x23) as i32;
                u8_ = u8_.wrapping_add(1);
                if !((u8_ as i32) < DAT_0071e53c) {
                    break;
                }
            }
        }
        if DAT_00732a38 < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32 {
            let mut iv9 = (DAT_00732a38 as u32).wrapping_mul(0x23) as i32;
            let mut u8_ = DAT_00732a38 as u32;
            loop {
                let base = (DAT_006d4988 as u32).wrapping_add(iv9 as u32);
                let u7 = rd8(base.wrapping_add(3)) as u32;
                if u7 != 0 {
                    let b1 = rd8(base.wrapping_add(2));
                    if 7 < b1 && b1 < 10 {
                        let us0 = rd16(
                            (&raw mut DAT_0052686c as usize as u32)
                                .wrapping_add(u7.wrapping_mul(0x16)),
                        );
                        let b3 = FUN_004340e0(u8_, us0 as u32, param_1);
                        if b3 != 0 {
                            let v = calc(iv2, u8_);
                            if (v as i32) < (local_8 as i32) {
                                local_8 = v;
                                local_4 = u8_;
                            }
                        }
                    }
                }
                iv9 = (iv9 as u32).wrapping_add(0x23) as i32;
                u8_ = u8_.wrapping_add(1);
                if !((u8_ as i32) < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32)
                {
                    break;
                }
            }
        }
        local_4
    }
}
