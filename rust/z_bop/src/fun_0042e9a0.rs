// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009ecc34: i32;
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
    static mut DAT_009e9a5b: u8;
    static mut DAT_009e9a5f: u8;
    static mut DAT_009e9a63: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(param_1: u32, param_2: i32);
    fn FUN_0046abf0(param_1: i32, param_2: i32) -> u32;
}

#[inline(always)]
fn clamp(v: i32) -> i32 {
    let mut r = v;
    if 0x3fffffff < r {
        r = 0x3fffffff;
    }
    if r < -0x3fffffff {
        r = -0x3fff0000;
    }
    r
}

#[inline(always)]
fn sub16(a: i32, b: i32) -> i16 {
    ((a as i16 as i32).wrapping_sub(b as i16 as i32)) as i16
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e9a0(param_1: u32, param_2: *mut i32) -> i32 {
    if param_1 == 0xffffffff {
        return 0x40;
    }
    // SAFETY: all globals and callees belong to the game; pointers are
    // read and written where the original does.
    unsafe {
        let mut iv1: i32 = ((param_1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32)) as usize as *const i32)
            .read_unaligned();
        let s5: i16;
        let s4: i16;
        let s7: i16;
        if iv1 == 0 {
            FUN_00458e20(param_1, 0);
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
            let a = (&raw mut DAT_009e9a63 as *mut u8)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            s7 = sub16(DAT_00a07cd4, a);
            let b = (&raw mut DAT_009e9a5b as *mut u8)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            s5 = sub16(DAT_00a07cdc, b);
            let c = (&raw mut DAT_009e9a5f as *mut u8)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            s4 = sub16(DAT_00a07cd8, c);
        } else {
            let p = iv1 as usize as *const u8;
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
            let a = p.byte_add(6).cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_009e9a5b as *mut u8)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            s5 = sub16(a, b);
            let a = p.byte_add(10).cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_009e9a5f as *mut u8)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            s4 = sub16(a, b);
            let a = p.byte_add(0xe).cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_009e9a63 as *mut u8)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            s7 = sub16(a, b);
        }
        iv1 = s7 as i32;
        let iv6 = s4 as i32;
        let iv8 = s5 as i32;

        let t = iv8
            .wrapping_mul(DAT_00a004a8)
            .wrapping_add(iv6.wrapping_mul(DAT_00a004cc))
            .wrapping_add(iv1.wrapping_mul(DAT_00a00488));
        let s44 = (clamp(t) >> 15) as i16;
        let t = iv8
            .wrapping_mul(DAT_00a004b0)
            .wrapping_add(iv6.wrapping_mul(DAT_009fe7f4))
            .wrapping_add(iv1.wrapping_mul(DAT_00a00490));
        let s48 = (clamp(t) >> 15) as i16;
        let u3 = FUN_0046abf0(s44 as i32, s48 as i32);
        let mut sv4 = u3 as i16;
        if 0x3fff < sv4 || sv4 < -0x3fff {
            let t = iv8
                .wrapping_mul(DAT_00a004a4)
                .wrapping_add(iv6.wrapping_mul(DAT_00a004b8))
                .wrapping_add(iv1.wrapping_mul(DAT_00a00480));
            let s50 = (clamp(t) >> 15) as i16;
            let u3 = FUN_0046abf0(s50 as i32, s48 as i32);
            let mut sv5 = (-0x8000i32).wrapping_sub(u3 as i16 as i32) as i16;
            sv4 = (-0x8000i32).wrapping_sub(sv4 as i32) as i16;
            if sv5 < 0 {
                sv5 = (-(sv5 as i32)) as i16;
            }
            let mut sv7 = sv4;
            if sv4 < 0 {
                sv7 = (-(sv4 as i32)) as i16;
            }
            let x = ((0x4000i32 - sv5 as i32) as i16) >> 8;
            let y = ((0x4000i32 - sv7 as i32) as i16) >> 8;
            let mut r = x.wrapping_mul(y) as i32;
            let q = ((r.wrapping_add((r >> 31) & 0x3f)) >> 6) as i16;
            let pv = param_2.read();
            r = q.wrapping_mul(pv as i16) as i32;
            let d = ((r.wrapping_add((r >> 31) & 0x7f)) >> 7) as i16 as i32;
            param_2.write(pv.wrapping_sub(d));
        }
        sv4 >>= 7;
        if sv4 < -0x40 {
            sv4 = -0x40;
        }
        if 0x3f < sv4 {
            sv4 = 0x3f;
        }
        sv4.wrapping_add(0x40) as i32
    }
}
