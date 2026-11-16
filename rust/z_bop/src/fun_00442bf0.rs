// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9683: [u8; 1];
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009e9a5b: [u8; 1];
    static mut DAT_009e9a5f: [u8; 1];
    static mut DAT_009e9a63: [u8; 1];
    static mut DAT_009e9a6b: [u8; 1];
    static mut DAT_009e9a6d: [u8; 1];
    static mut DAT_009e9a6f: [u8; 1];
    static mut DAT_009e9bf6: [u8; 1];
    static mut DAT_00521f1c: [u8; 1];
    static mut DAT_0051c659: [u8; 1];
    static mut DAT_0051c65b: [u8; 1];
    static mut DAT_0051c65d: [u8; 1];
    static mut DAT_0051c65f: [u8; 1];
    static mut DAT_00a08cbe: [u8; 1];
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a8070: i16;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a00480: i32;
    static mut DAT_00a00488: i32;
    static mut DAT_00a00490: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00a004a8: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_00a004b8: i32;
    static mut DAT_00a004cc: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a080f0: i16;
    static mut DAT_00a08374: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(a: u32, b: i32);
    fn FUN_00427940(a: i16, b: i16, c: i16, d: i32, e: i32, f: i32, g: i32);
    fn FUN_0046a8d0(a: i32, b: i32, c: i32);
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

macro_rules! ad {
    ($d:ident) => {
        ((&raw mut $d).cast::<u8>() as usize as u32)
    };
}

unsafe fn rd<T>(addr: u32) -> T {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (addr as usize as *const T).read_unaligned() }
}

unsafe fn wr<T>(addr: u32, v: T) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (addr as usize as *mut T).write_unaligned(v) }
}

fn clamp(v: i32) -> i32 {
    let mut v = v;
    if 0x3fffffff < v {
        v = 0x3fffffff;
    }
    if v < -0x3fffffff {
        v = -0x3fff0000;
    }
    v
}

fn dot(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    a.wrapping_mul(b)
        .wrapping_add(c.wrapping_mul(d))
        .wrapping_add(e.wrapping_mul(f))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442bf0(param_1: u32, param_2: i16, param_3: i32) {
    // SAFETY: globals and callees are the game's own; all addresses are
    // computed as the original computes them.
    unsafe {
        let g = DAT_009a1fe8 as u32;
        let a683 = ad!(DAT_009e9683);
        let a670 = ad!(DAT_009e9670);
        let a5b = ad!(DAT_009e9a5b);
        let a5f = ad!(DAT_009e9a5f);
        let a63 = ad!(DAT_009e9a63);
        let a6b = ad!(DAT_009e9a6b);
        let a6d = ad!(DAT_009e9a6d);
        let a6f = ad!(DAT_009e9a6f);
        let abf6 = ad!(DAT_009e9bf6);

        FUN_00458e20(param_1 & 0xffff, 0);
        let i2 = (param_3 as u32).wrapping_mul(0x5bd);
        let mut i15: i32;
        let mut i14: i32;
        let mut i17: i32;
        if rd::<u8>(a683.wrapping_add(i2)) == 0 {
            let t: i32 = rd(a670.wrapping_add(i2));
            let p = (t as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8 as u32);
            i15 = rd(p.wrapping_add(4));
            i14 = rd(p.wrapping_add(8));
            i17 = rd(p.wrapping_add(0xc));
        } else {
            i15 = rd(a5b.wrapping_add(i2));
            i14 = rd(a5f.wrapping_add(i2));
            i17 = rd(a63.wrapping_add(i2));
        }
        i15 = DAT_00a07cdc.wrapping_sub(i15).wrapping_mul(2);
        i14 = DAT_00a07cd8.wrapping_sub(i14).wrapping_mul(2);
        let mut u10: u32 = (i14 >> 0x10) as u32;
        i17 = DAT_00a07cd4.wrapping_sub(i17).wrapping_mul(2);
        let mut u4: u32 = (i15 >> 0x10) as u32;
        let mut u12: u32 = (i17 >> 0x10) as u32;
        if u4 & 0x8000 != 0 {
            u4 = ((((i15 as u32) >> 0x10) as i16).wrapping_neg()) as u16 as u32;
        }
        if u10 & 0x8000 != 0 {
            u10 = ((((i14 as u32) >> 0x10) as i16).wrapping_neg()) as u16 as u32;
        }
        if u12 & 0x8000 != 0 {
            u12 = ((((i17 as u32) >> 0x10) as i16).wrapping_neg()) as u16 as u32;
        }
        let mut i_stack8: i32;
        let mut i5: i32;
        let mut i16v: i32;
        loop {
            let u8v: u16;
            let u11: u16;
            loop {
                i_stack8 = i17;
                i5 = i15;
                i16v = i14;
                u4 = (u4 >> 1) & 0x7fff;
                i15 = i5 >> 1;
                i14 = i16v >> 1;
                i17 = i_stack8 >> 1;
                let v8 = (u10 as u16) >> 1;
                u10 = v8 as u32;
                let v11 = (u12 as u16) >> 1;
                u12 = v11 as u32;
                if (u4 as i16) == 0 {
                    u8v = v8;
                    u11 = v11;
                    break;
                }
            }
            if u8v == 0 && u11 == 0 {
                break;
            }
        }
        let mut s13: i16 = (i16v >> 2) as i16;
        let mut s9: i16 = (i5 >> 2) as i16;
        let s_stack8: i16 = (i_stack8 >> 2) as i16;
        let s3: i16;

        if rd::<u8>(a683.wrapping_add(i2)) == 0 {
            let t: i32 = rd(a670.wrapping_add(i2));
            let mut p17 = (t as u32).wrapping_mul(0x23).wrapping_add(g);
            let ps: u32 = rd(p17.wrapping_add(0x1f));
            let tb: u8 = rd(ps.wrapping_add(0x92));
            if tb != 0 {
                let t1: i16 = rd(p17.wrapping_add(0x12));
                let t2: i16 = rd(p17.wrapping_add(0x10));
                FUN_00427bd0(t1, t2, p17 as i32);
                let t: i32 = rd(a670.wrapping_add(i2));
                p17 = (t as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8 as u32);
                let t1: i16 = rd(p17.wrapping_add(0x14));
                FUN_00427d30(t1, 0, p17 as i32);
            }
            let t: i32 = rd(a670.wrapping_add(i2));
            let p17 = (t as u32).wrapping_mul(0x23);
            let ps: u32 = rd(p17.wrapping_add(0x1f).wrapping_add(DAT_009a1fe8 as u32));
            let v15 = s_stack8 as i32;
            let v16 = s9 as i32;
            let v14 = s13 as i32;
            let x1: i16 = rd(ps.wrapping_add(0x99));
            let x2: i16 = rd(ps.wrapping_add(0x9b));
            let x3: i16 = rd(ps.wrapping_add(0x9d));
            let r17 = clamp(dot(v16, x1 as i32, v14, x2 as i32, v15, x3 as i32));
            let x1: i16 = rd(ps.wrapping_add(0x93));
            let x2: i16 = rd(ps.wrapping_add(0x95));
            let x3: i16 = rd(ps.wrapping_add(0x97));
            let r5 = clamp(dot(v16, x1 as i32, v14, x2 as i32, v15, x3 as i32));
            let x1: i16 = rd(ps.wrapping_add(0x9f));
            let x2: i16 = rd(ps.wrapping_add(0xa1));
            let x3: i16 = rd(ps.wrapping_add(0xa3));
            let r14 = clamp(dot(v16, x1 as i32, v14, x2 as i32, v15, x3 as i32));
            FUN_0046a8d0(r17 >> 0xf, r5 >> 0xf, r14 >> 0xf);
            let t: i32 = rd(a670.wrapping_add(i2));
            let p17 = (t as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8 as u32);
            s13 = rd(p17.wrapping_add(0x10));
            s9 = rd(p17.wrapping_add(0x12));
            s3 = rd(p17.wrapping_add(0x14));
        } else {
            let t1: i16 = rd(a6f.wrapping_add(i2));
            let t2: i16 = rd(a6b.wrapping_add(i2));
            let t3: i16 = rd(a6d.wrapping_add(i2));
            FUN_00427940(t1, t2, t3, 0, 0, 0, 0);
            let v14 = s_stack8 as i32;
            let v15 = s13 as i32;
            let v16 = s9 as i32;
            let r17 = clamp(dot(v16, DAT_00a004a8, v15, DAT_00a004cc, v14, DAT_00a00488));
            let r5 = clamp(dot(v16, DAT_00a004b0, v15, DAT_009fe7f4, v14, DAT_00a00490));
            let r14 = clamp(dot(v16, DAT_00a004a4, v15, DAT_00a004b8, v14, DAT_00a00480));
            FUN_0046a8d0(r17 >> 0xf, r5 >> 0xf, r14 >> 0xf);
            s13 = rd(a6d.wrapping_add(i2));
            s9 = rd(a6b.wrapping_add(i2));
            s3 = rd(a6f.wrapping_add(i2));
        }
        FUN_00427940(
            s3,
            s9,
            s13,
            0,
            0x4000i32.wrapping_sub(DAT_00a080f0 as i32),
            DAT_009a8070 as i32,
            0,
        );
        let iv14 = DAT_009fe7f4;
        let p15 = (param_1 & 0xffff)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32);
        i17 = rd(p15.wrapping_add(0x1f));
        let cond = if i17 == 0 {
            true
        } else {
            i17 = rd((i17 as u32).wrapping_add(0xa9));
            i17 == 0
        };
        if cond {
            let tb: u8 = rd(p15.wrapping_add(3));
            i17 = rd(ad!(DAT_00521f1c).wrapping_add((tb as u32).wrapping_mul(0x1a)));
        } else {
            let tb: u8 = rd((i17 as u32).wrapping_add(4));
            let o = (tb as u32).wrapping_mul(0x101);
            let v13: i16 = rd(ad!(DAT_0051c65b).wrapping_add(o));
            let v9: i16 = rd(ad!(DAT_0051c65f).wrapping_add(o));
            let t1: i16 = rd(ad!(DAT_0051c65d).wrapping_add(o));
            let (m15, m16): (i32, i32);
            if v9 < v13 || t1 < v13 {
                if v13 < v9 || t1 < v9 {
                    m15 = v13 as i32;
                    m16 = v9 as i32;
                } else {
                    m15 = v13 as i32;
                    m16 = t1 as i32;
                }
            } else {
                m16 = t1 as i32;
                m15 = v9 as i32;
            }
            let tb2: u8 = rd(ad!(DAT_0051c659).wrapping_add(o));
            i17 = (((m15 as u32).wrapping_add(m16 as u32) >> 1).wrapping_shl((tb2 & 0x1f) as u32))
                as i32;
        }
        if param_2 == 0 {
            u4 = 0x3c;
            let t1: i16 = rd(abf6.wrapping_add(i2));
            if t1 != 0x13 {
                let m = (t1 == 0x112) as u16;
                let m = m.wrapping_neg() as u32 & 0xffd4;
                u4 = m.wrapping_add(0x90) as u16 as u32;
            }
        } else {
            let tu: u16 =
                rd(ad!(DAT_00a08cbe).wrapping_add((DAT_00a08374 as u32).wrapping_mul(0xc)));
            u4 = tu as u32;
        }
        s13 = rd(abf6.wrapping_add(i2));
        let s6: u32 = if s13 == 0x101 || s13 == 0x112 { 9 } else { 8 };
        let mut sv9: i16 = 0;
        u12 = (i17 as u32).wrapping_shl(s6);
        if u4 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q: u32;
        // SAFETY: unsigned divide as the machine does it, faulting on zero.
        core::arch::asm!("div {d}", d = in(reg) u4, inout("eax") u12 => q, inout("edx") 0u32 => _);
        u4 = q;
        while 0x3fff < u4 {
            sv9 = sv9.wrapping_add(1);
            u4 >>= 1;
        }
        if s13 == 0x101 {
            let w = u4 as u16;
            u4 = w.wrapping_add(w >> 2) as u32;
        }
        u4 &= 0xffff;
        let mut prod = (u4 as i32 as i64).wrapping_mul(DAT_00a004b0 as i64);
        let mut outv = ((prod as u32) >> 0xf) | (((prod as u64 >> 0x20) as u32).wrapping_shl(0x11));
        wr(a5b.wrapping_add(i2), outv);
        let i17b = DAT_00a00490;
        prod = (u4 as i32 as i64).wrapping_mul(iv14 as i64);
        outv = ((prod as u32) >> 0xf) | (((prod as u64 >> 0x20) as u32).wrapping_shl(0x11));
        wr(a5f.wrapping_add(i2), outv);
        prod = (u4 as i32 as i64).wrapping_mul(i17b as i64);
        outv = ((prod as u32) >> 0xf) | (((prod as u64 >> 0x20) as u32).wrapping_shl(0x11));
        wr(a63.wrapping_add(i2), outv);
        if sv9 != 0 {
            let b7 = (sv9 as u8 & 0x1f) as u32;
            let t: u32 = rd(a5b.wrapping_add(i2));
            wr(a5b.wrapping_add(i2), t.wrapping_shl(b7));
            let t: u32 = rd(a5f.wrapping_add(i2));
            wr(a5f.wrapping_add(i2), t.wrapping_shl(b7));
            let t: u32 = rd(a63.wrapping_add(i2));
            wr(a63.wrapping_add(i2), t.wrapping_shl(b7));
        }
        let t: i32 = rd(a5b.wrapping_add(i2));
        wr(a5b.wrapping_add(i2), DAT_00a07cdc.wrapping_sub(t));
        let t: i32 = rd(a5f.wrapping_add(i2));
        wr(a5f.wrapping_add(i2), DAT_00a07cd8.wrapping_sub(t));
        let t: i32 = rd(a63.wrapping_add(i2));
        wr(a63.wrapping_add(i2), DAT_00a07cd4.wrapping_sub(t));
    }
}
