// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004af000(param_1: i32, param_2: i32) -> i32;
    fn __ftol() -> i64;
    fn FUN_0046a6d0(param_1: i16) -> i64;
    fn FUN_0046a3c0(param_1: i16) -> i64;
}

#[inline(always)]
fn cl(x: i32) -> i32 {
    let mut v = x;
    if 0x3fffffff < v {
        v = 0x3fffffff;
    }
    if v < -0x3fffffff {
        v = -0x3fff0000;
    }
    v
}

#[inline(always)]
fn sh(a: u32, b: u32) -> u32 {
    ((a.wrapping_mul(b)) as i32 >> 0xf) as u32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004285a0(
    param_1: i16,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    // SAFETY: the globals and callees are the game's own; __ftol takes its
    // operand from the x87 stack exactly as in the original.
    unsafe {
        DAT_009d8a58 = param_4;
        DAT_009d8a54 = param_5;
        DAT_009d8b60 = param_6;
        let r = FUN_004af000(param_2, param_3);
        if r != 0 {
            let iv1 = __ftol() as i32;
            let iv3 = __ftol() as i32;
            let iv4 = __ftol() as i32;
            let iv2 = (FUN_0046a6d0(param_1) as i16) as i32;
            let iv5 = (FUN_0046a3c0(param_1) as i16) as i32;
            let u1 = iv1 as u32;
            let u2 = iv2 as u32;
            let u3 = iv3 as u32;
            let u4 = iv4 as u32;
            let u5 = iv5 as u32;
            let m: u32 = 0xffff8000;

            let (lac, la4, l9c, lb0, la8, la0, lb4, i1, i3);
            if iv2 < 0 {
                let u6 = 0u32.wrapping_sub(u2);
                lac = cl(sh(u1, u1)
                    .wrapping_mul(u6)
                    .wrapping_add(u1.wrapping_mul(u1))
                    .wrapping_add(u2.wrapping_mul(0x8000)) as i32);
                la4 = cl(sh(u1, u3)
                    .wrapping_mul(u6)
                    .wrapping_add(u1.wrapping_mul(u3))
                    .wrapping_add(u5.wrapping_mul(u4) & m) as i32);
                l9c = cl(sh(u1, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u1.wrapping_mul(u4))
                    .wrapping_add((u5.wrapping_mul(u3) >> 0xf).wrapping_mul(m))
                    as i32);
                lb0 = cl(sh(u1, u3)
                    .wrapping_mul(u6)
                    .wrapping_add(u1.wrapping_mul(u3))
                    .wrapping_add((u5.wrapping_mul(u4) >> 0xf).wrapping_mul(m))
                    as i32);
                la8 = cl(sh(u3, u3)
                    .wrapping_mul(u6)
                    .wrapping_add(u3.wrapping_mul(u3))
                    .wrapping_add(u2.wrapping_mul(0x8000)) as i32);
                la0 = cl(sh(u3, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u3.wrapping_mul(u4))
                    .wrapping_add(u5.wrapping_mul(u1) & m) as i32);
                lb4 = cl(sh(u1, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u1.wrapping_mul(u4))
                    .wrapping_add(u5.wrapping_mul(u3) & m) as i32);
                i1 = cl(sh(u3, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u3.wrapping_mul(u4))
                    .wrapping_add((u5.wrapping_mul(u1) >> 0xf).wrapping_mul(m))
                    as i32);
                i3 = cl(sh(u4, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u4.wrapping_mul(u4))
                    .wrapping_add(u2.wrapping_mul(0x8000)) as i32);
            } else {
                let u6 = 0x7fffu32.wrapping_sub(u2);
                lac = cl(sh(u1, u1)
                    .wrapping_mul(u6)
                    .wrapping_add(u2.wrapping_mul(0x8000)) as i32);
                la4 = cl(sh(u1, u3)
                    .wrapping_mul(u6)
                    .wrapping_add(u5.wrapping_mul(u4) & m) as i32);
                l9c = cl(sh(u1, u4)
                    .wrapping_mul(u6)
                    .wrapping_add((u5.wrapping_mul(u3) >> 0xf).wrapping_mul(m))
                    as i32);
                lb0 = cl(sh(u1, u3)
                    .wrapping_mul(u6)
                    .wrapping_add((u5.wrapping_mul(u4) >> 0xf).wrapping_mul(m))
                    as i32);
                la8 = cl(sh(u3, u3)
                    .wrapping_mul(u6)
                    .wrapping_add(u2.wrapping_mul(0x8000)) as i32);
                la0 = cl(sh(u3, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u5.wrapping_mul(u1) & m) as i32);
                lb4 = cl(sh(u1, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u5.wrapping_mul(u3) & m) as i32);
                i1 = cl(sh(u3, u4)
                    .wrapping_mul(u6)
                    .wrapping_add((u5.wrapping_mul(u1) >> 0xf).wrapping_mul(m))
                    as i32);
                i3 = cl(sh(u4, u4)
                    .wrapping_mul(u6)
                    .wrapping_add(u2.wrapping_mul(0x8000)) as i32);
            }
            let l9c = l9c >> 0xf;
            let la0 = la0 >> 0xf;
            let la4 = la4 >> 0xf;
            let la8 = la8 >> 0xf;
            let lac = lac >> 0xf;
            let lb0 = lb0 >> 0xf;
            let lb4 = lb4 >> 0xf;

            let f1 = __ftol() as i32;
            let d2 = param_4.wrapping_sub(f1);
            let f2 = __ftol() as i32;
            let d5 = param_5.wrapping_add(f2);
            let f3 = __ftol() as i32;
            let d6 = param_6.wrapping_sub(f3);

            let o4 = cl(d2
                .wrapping_mul(lac)
                .wrapping_add(d5.wrapping_mul(lb0))
                .wrapping_add(d6.wrapping_mul(lb4)));
            let o1 = cl(d2
                .wrapping_mul(la4)
                .wrapping_add(d5.wrapping_mul(la8))
                .wrapping_add(d6.wrapping_mul(i1 >> 0xf)));
            let o3 = cl(d2
                .wrapping_mul(l9c)
                .wrapping_add(d5.wrapping_mul(la0))
                .wrapping_add(d6.wrapping_mul(i3 >> 0xf)));

            let g7 = __ftol() as i32;
            let g8 = __ftol() as i32;
            let g9 = __ftol() as i32;
            DAT_009d8a58 = (o4 >> 0xf).wrapping_add(g9);
            DAT_009d8a54 = (o1 >> 0xf).wrapping_sub(g7);
            DAT_009d8b60 = (o3 >> 0xf).wrapping_add(g8);
        }
    }
}
