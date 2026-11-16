// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9683: u8;
    static mut DAT_009e9a5b: u8;
    static mut DAT_009e9a5f: u8;
    static mut DAT_009e9a63: u8;
    static mut DAT_009e9a67: u8;
    static mut DAT_009e9a69: u8;
    static mut DAT_009e9a6b: u8;
    static mut DAT_009e9a6d: u8;
    static mut DAT_009e9a6f: u8;
    static mut DAT_009e9a73: u8;
    static mut DAT_009e9a75: u8;
    static mut DAT_009e9a86: u8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a8070: i16;
    static mut DAT_00a080f0: i16;
    static mut DAT_00a004b0: u32;
    static mut DAT_009fe7f4: u32;
    static mut DAT_00a00490: u32;
    static mut DAT_00a07cd4: u32;
    static mut DAT_00a07cd8: u32;
    static mut DAT_00a07cdc: u32;
    static mut DAT_00523410: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427940(a: i16, b: i16, c: i16, d: i32, e: i16, f: i16, g: i32);
    fn FUN_0046a8d0(a: i32, b: i32, c: i32);
    fn FUN_00458e20(a: u32, b: i32);
}

unsafe fn r16(p: *const u8) -> i16 {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<i16>().read_unaligned() }
}

unsafe fn w16(p: *mut u8, v: i16) {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<i16>().write_unaligned(v) }
}

unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn w32(p: *mut u8, v: u32) {
    // SAFETY: p points into the game's own memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

fn step(mut v: i32, t: i32) -> i32 {
    if v < 1 {
        if v < -0x1000 {
            v = -0x1000;
        }
        if -0x10 < v {
            v = -0x10;
        }
        if v < t {
            v = t;
        }
    } else {
        if v > 0x1000 {
            v = 0x1000;
        }
        if v < 0x10 {
            v = 0x10;
        }
        if t < v {
            v = t;
        }
    }
    v
}

unsafe fn rec(idx: u16, off: u32) -> *const u8 {
    // SAFETY: reads the game's record table pointer.
    let base = unsafe { DAT_009a1fe8 } as u32;
    (u32::from(idx)
        .wrapping_mul(0x23)
        .wrapping_add(off)
        .wrapping_add(base)) as *const u8
}

unsafe fn sub_mul(dst: *mut u8, k: i32, m: u32) {
    let l = (k as i64).wrapping_mul(m as i32 as i64);
    let lo = l as u32;
    let hi = ((l as u64) >> 32) as u32;
    let d = lo >> 15 | hi.wrapping_shl(17);
    // SAFETY: dst points into the game's own memory.
    unsafe {
        let v = r32(dst).wrapping_sub(d);
        w32(dst, v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435c60(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; pointers computed
    // from param_1 index the game's tables exactly as the original does.
    unsafe {
        let o = (param_1 as u32).wrapping_mul(0x5bd) as i32 as isize;
        let p83 = (&raw mut DAT_009e9683 as *mut u8).wrapping_byte_offset(o);
        let p5b = (&raw mut DAT_009e9a5b as *mut u8).wrapping_byte_offset(o);
        let p5f = (&raw mut DAT_009e9a5f as *mut u8).wrapping_byte_offset(o);
        let p63 = (&raw mut DAT_009e9a63 as *mut u8).wrapping_byte_offset(o);
        let p67 = (&raw mut DAT_009e9a67 as *mut u8).wrapping_byte_offset(o);
        let p69 = (&raw mut DAT_009e9a69 as *mut u8).wrapping_byte_offset(o);
        let p6b = (&raw mut DAT_009e9a6b as *mut u8).wrapping_byte_offset(o);
        let p6d = (&raw mut DAT_009e9a6d as *mut u8).wrapping_byte_offset(o);
        let p6f = (&raw mut DAT_009e9a6f as *mut u8).wrapping_byte_offset(o);
        let p73 = (&raw mut DAT_009e9a73 as *mut u8).wrapping_byte_offset(o);
        let p75 = (&raw mut DAT_009e9a75 as *mut u8).wrapping_byte_offset(o);
        let p86 = (&raw mut DAT_009e9a86 as *mut u8).wrapping_byte_offset(o);

        let u_var1 = r16(p67) as u16;

        if *p83 < 2 {
            if u_var1 == 0xffff {
                let s75 = r16(p75);
                let s73 = r16(p73);
                if s75 != 0 || s73 != 0 {
                    let s6f = r16(p6f);
                    let s6b = r16(p6b);
                    let s6d = r16(p6d);
                    let s73 = r16(p73);
                    let s75 = r16(p75);
                    FUN_00427940(s6f, s6b, s6d, 0, s73, s75, 0);
                    FUN_0046a8d0(
                        DAT_00a004b0 as i32,
                        DAT_009fe7f4 as i32,
                        DAT_00a00490 as i32,
                    );
                    let s9 = DAT_00a080f0;
                    w16(p75, 0);
                    let ss1 = DAT_009a8070;
                    w16(p73, 0);
                    w16(p6b, s9);
                    w16(p6d, ss1);
                }
            } else {
                let ss1 = r16(rec(u_var1, 0x14));
                w16(p6f, ss1);
                let us = r16(p67) as u16;
                let ss1 = r16(rec(us, 0x12));
                w16(p6b, ss1);
                let us = r16(p67) as u16;
                let ss1 = r16(rec(us, 0x10));
                w16(p6d, ss1);
                let us = r16(p67) as u16;
                FUN_00458e20(u32::from(us), 0);
                let u5b = DAT_00a07cdc;
                let u5f = DAT_00a07cd8;
                let u63 = DAT_00a07cd4;
                w32(p5b, u5b);
                w32(p5f, u5f);
                w32(p63, u63);
            }
            let s9 = r16(p75);
            let s15 = r16(p73);
            let s14 = r16(p6d);
            let s13 = r16(p6b);
            let s12 = r16(p6f);
            FUN_00427940(s12, s13, s14, 0, s15, s9, 0);
            let u_var4 = DAT_009fe7f4;
            let us = r16(p67);
            if us != -1 {
                let k = r32(p86) as i32;
                sub_mul(p5b, k, DAT_00a004b0);
                let u_var5 = DAT_00a00490;
                let k = r32(p86) as i32;
                sub_mul(p5f, k, u_var4);
                let k = r32(p86) as i32;
                sub_mul(p63, k, u_var5);
            }
            let us = r16(p69) as u16;
            if us == 0xffff {
                return;
            }
            let i_var11 = rec(us, 0);
            let t4 = r32(i_var11.wrapping_byte_add(4));
            let u5b = r32(p5b);
            let t8 = r32(i_var11.wrapping_byte_add(8));
            let u5f = r32(p5f);
            let tc = r32(i_var11.wrapping_byte_add(0xc));
            let u63 = r32(p63);
            FUN_0046a8d0(
                t4.wrapping_sub(u5b) as i32,
                t8.wrapping_sub(u5f) as i32,
                tc.wrapping_sub(u63) as i32,
            );
            let s9 = DAT_00a080f0;
            let ss1 = DAT_009a8070;
            w16(p6f, 0);
            w16(p6b, s9);
            w16(p6d, ss1);
            let s6f = r16(p6f);
            let s6b = r16(p6b);
            FUN_00427940(s6f, s6b, ss1, 0, 0, 0, 0);
            return;
        }

        'end: {
            if u_var1 == 0xffff {
                w16(p6f, 0);
                w16(p6b, 0x4000);
                w16(p6d, 0);
                w16(p75, 0);
                let v = ((((*p83 as u32) & 0x7f) << 0xe) as i32 / -0x7f) as i16;
                w16(p73, v);
            } else {
                let ss1 = r16(rec(u_var1, 0x14));
                w16(p6f, ss1);
                let us = r16(p67) as u16;
                let ss1 = r16(rec(us, 0x12));
                w16(p6b, ss1);
                let us = r16(p67) as u16;
                let ss1 = r16(rec(us, 0x10));
                w16(p6d, ss1);
                let us = r16(p67) as u16;
                FUN_00458e20(u32::from(us), 0);
                let u5b = DAT_00a07cdc;
                let u5f = DAT_00a07cd8;
                let u63 = DAT_00a07cd4;
                let flag = DAT_00523410;
                w32(p5b, u5b);
                w32(p5f, u5f);
                w32(p63, u63);
                if flag != 0 {
                    break 'end;
                }

                let s9 = r16(p75);
                let half = (s9 as i32) / 2;
                let d = step(half.wrapping_neg(), -(s9 as i32));
                let ss1 = s9.wrapping_add(d as i16);
                w16(p75, ss1);
                if *p83 & 0x80 == 0 {
                    let s9 = r16(p73);
                    let half = (s9 as i32) / 2;
                    let d = step(half.wrapping_neg(), -(s9 as i32));
                    let ss1 = s9.wrapping_add(d as i16);
                    w16(p73, ss1);
                    break 'end;
                }
                w16(p6f, 0);
                w16(p6b, 0x4000);
                w16(p6d, 0);
                let s73 = r16(p73);
                let i10 = -0x4000i32 - (s73 as i32);
                let d = step(i10 / 2, i10);
                let s73 = r16(p73);
                let v = (d as i16).wrapping_add(s73);
                w16(p73, v);
            }
        }

        let s6f = r16(p6f);
        let s6b = r16(p6b);
        let s6d = r16(p6d);
        let s73 = r16(p73);
        let s75 = r16(p75);
        FUN_00427940(s6f, s6b, s6d, 0, s73, s75, 0);
        let u_var4 = DAT_009fe7f4;
        let us = r16(p67);
        if us == -1 {
            return;
        }
        let k = r32(p86) as i32;
        sub_mul(p5b, k, DAT_00a004b0);
        let u_var5 = DAT_00a00490;
        let k = r32(p86) as i32;
        sub_mul(p5f, k, u_var4);
        let k = r32(p86) as i32;
        sub_mul(p63, k, u_var5);
    }
}
