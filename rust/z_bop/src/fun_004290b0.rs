// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0048c: i32;
    static mut DAT_00a00494: i32;
    static mut DAT_00a0049c: i32;
    static mut DAT_00a004ac: i32;
    static mut DAT_00a004b4: i32;
    static mut DAT_00a004d0: i32;
    static mut DAT_00a07c68: i32;
    static mut DAT_00a07c6c: i32;
    static mut DAT_00a07cc4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a6d0(param_1: i16) -> i64;
    fn FUN_0046a3c0(param_1: i16) -> i64;
}

fn clamp(mut x: i32) -> i32 {
    if 0x3fffffff < x {
        x = 0x3fffffff;
    }
    if x < -0x3fffffff {
        x = -0x3fff0000;
    }
    x
}

fn row(x: i32, a: i32, y: i32, b: i32, z: i32, c: i32) -> i32 {
    clamp(
        x.wrapping_mul(a)
            .wrapping_add(y.wrapping_mul(b))
            .wrapping_add(z.wrapping_mul(c)),
    )
}

fn lsr(x: i32) -> i32 {
    ((x as u32) >> 0xf) as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004290b0(param_1: i32, param_2: i32, param_3: i32, param_4: i16) {
    if param_4 == 0 {
        return;
    }
    // SAFETY: the callees and the globals are the game's own memory.
    unsafe {
        let l8 = FUN_0046a6d0(param_4);
        let l9 = FUN_0046a3c0(param_4);
        let iv1 = (l8 as i16) as i32;
        let iv7 = (l9 as i16) as i32;
        let neg = (l8 as i16) < 0;
        let iv6: i32 = if neg {
            iv1.wrapping_neg()
        } else {
            0x7fff - iv1
        };
        let mask: i32 = 0xffff8000u32 as i32;
        let e = iv1.wrapping_mul(0x8000);
        let f = |pq: i32| -> i32 {
            let t = (pq >> 0xf).wrapping_mul(iv6);
            if neg { t.wrapping_add(pq) } else { t }
        };
        let p11 = param_1.wrapping_mul(param_1);
        let p12 = param_1.wrapping_mul(param_2);
        let p13 = param_1.wrapping_mul(param_3);
        let p22 = param_2.wrapping_mul(param_2);
        let p23 = param_2.wrapping_mul(param_3);
        let p33 = param_3.wrapping_mul(param_3);
        let m = -0x8000i32;

        let mut local_28 = clamp(f(p11).wrapping_add(e));
        let mut local_1c = clamp(f(p12).wrapping_add(iv7.wrapping_mul(param_3) & mask));
        let mut local_10 =
            clamp(f(p13).wrapping_add(lsr(iv7.wrapping_mul(param_2)).wrapping_mul(m)));
        let mut local_2c =
            clamp(f(p12).wrapping_add(lsr(iv7.wrapping_mul(param_3)).wrapping_mul(m)));
        let mut local_20 = clamp(f(p22).wrapping_add(e));
        let mut local_14 = clamp(f(p23).wrapping_add(iv7.wrapping_mul(param_1) & mask));
        let mut local_30 = clamp(f(p13).wrapping_add(iv7.wrapping_mul(param_2) & mask));
        let mut local_24 =
            clamp(f(p23).wrapping_add(lsr(iv7.wrapping_mul(param_1)).wrapping_mul(m)));
        let mut iv1n = clamp(f(p33).wrapping_add(e));

        local_10 >>= 0xf;
        local_14 >>= 0xf;
        local_1c >>= 0xf;
        local_20 >>= 0xf;
        local_24 >>= 0xf;
        local_28 >>= 0xf;
        local_2c >>= 0xf;
        local_30 >>= 0xf;
        iv1n >>= 0xf;

        let iv6 = row(
            DAT_00a0049c,
            local_28,
            DAT_00a0048c,
            local_2c,
            DAT_00a00494,
            local_30,
        );
        let iv7 = row(
            DAT_00a0049c,
            local_1c,
            DAT_00a0048c,
            local_20,
            DAT_00a00494,
            local_24,
        );
        let iv2 = row(
            DAT_00a0049c,
            local_10,
            DAT_00a0048c,
            local_14,
            DAT_00a00494,
            iv1n,
        );
        DAT_00a00494 = iv2 >> 0xf;

        let iv2 = row(
            DAT_00a004d0,
            local_28,
            DAT_00a004ac,
            local_2c,
            DAT_00a004b4,
            local_30,
        );
        let iv3 = row(
            DAT_00a004d0,
            local_1c,
            DAT_00a004ac,
            local_20,
            DAT_00a004b4,
            local_24,
        );
        let iv4 = row(
            DAT_00a004d0,
            local_10,
            DAT_00a004ac,
            local_14,
            DAT_00a004b4,
            iv1n,
        );
        DAT_00a004b4 = iv4 >> 0xf;

        let iv4 = row(
            DAT_00a07c68,
            local_28,
            DAT_00a07c6c,
            local_2c,
            DAT_00a07cc4,
            local_30,
        );
        let iv5 = row(
            DAT_00a07c68,
            local_1c,
            DAT_00a07c6c,
            local_20,
            DAT_00a07cc4,
            local_24,
        );
        let iv1 = row(
            DAT_00a07c68,
            local_10,
            DAT_00a07c6c,
            local_14,
            DAT_00a07cc4,
            iv1n,
        );
        DAT_00a07cc4 = iv1 >> 0xf;

        DAT_00a0048c = iv7 >> 0xf;
        DAT_00a0049c = iv6 >> 0xf;
        DAT_00a004ac = iv3 >> 0xf;
        DAT_00a004d0 = iv2 >> 0xf;
        DAT_00a07c68 = iv4 >> 0xf;
        DAT_00a07c6c = iv5 >> 0xf;
    }
}
