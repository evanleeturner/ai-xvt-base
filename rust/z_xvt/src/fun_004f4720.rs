// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757870: [i32; 1];
    static mut DAT_007579d0: [i32; 1];
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_0081b850: i32;
    static mut DAT_00650406: i16;
    static mut DAT_00650438: i32;
    static mut DAT_0065043c: i32;
    static mut DAT_0085e7c5: i8;

    static mut DAT_00531990: [i8; 1];
    static mut DAT_00533ecc: [i8; 1];
    static mut DAT_00533ed4: [i8; 1];
    static mut DAT_00533ec4: [i8; 1];
    static mut DAT_00533ebc: [i8; 1];
    static mut DAT_00533eac: [i8; 1];
    static mut DAT_00533eb4: [i8; 1];
    static mut DAT_005337ec: [i8; 1];
    static mut DAT_005337f4: [i8; 1];
    static mut DAT_005337dc: [i8; 1];
    static mut DAT_005337e4: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cdbc0() -> i32;
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc70(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd7d0(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d9ea0(param_1: *mut i32);
    fn FUN_004dcfa0(param_1: i32) -> *mut i8;
    fn FUN_004d9a80(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
    fn FUN_004f2c10();
    fn FUN_004d9d20(param_1: *mut i32, param_2: *mut i8, param_3: *mut i8, param_4: i32);
    fn FUN_004de190(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004f2d10();
    fn FUN_004f5ac0();
    fn FUN_004f5990();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f4720(_param_1: u32, _param_2: i32) -> u32 {
    let mut local_34: [i32; 4] = [0; 4];
    let mut local_24: i32 = 0;
    let mut local_20: [i32; 8] = [0; 8];
    let l34 = local_34.as_mut_ptr();
    let l20 = local_20.as_mut_ptr();
    // SAFETY: globals and callees are the game's own; all pointers refer to
    // live locals or the game's data.
    unsafe {
        let idx: i32 = (&raw const DAT_0081bfb6).cast::<i32>().read_unaligned();
        let s_jewel = (&raw mut DAT_00531990).cast::<i8>();

        let iv = FUN_004cdbc0();
        let a = (&raw mut DAT_00757870)
            .cast::<i32>()
            .offset((idx as u32).wrapping_mul(8) as i32 as isize)
            .read();
        let b5 = iv != a;
        let b6 = DAT_0081b850 == 2;
        l20.add(1).write(1);
        l20.add(2).write(1);
        l20.add(3).write(0);
        l20.add(4).write(1);
        l20.add(5).write(0);
        let d = (&raw mut DAT_007579d0)
            .cast::<i32>()
            .offset(idx as isize)
            .read();
        if (!b6 && b5) || d < 2 {
            l20.add(6).write(0);
            l20.add(7).write(0);
        } else {
            l20.add(6).write(1);
            l20.add(7).write(1);
        }
        let k = ((DAT_00650406 ^ 1) as i32).wrapping_add(1);
        l20.offset(k as isize).write(2);
        FUN_004dcfd0(l34, 0x16, 0xc6, 0x2a, 0xde);
        FUN_004ddc70(l20, &raw mut local_24);
        let mut r = FUN_004dd7d0(l34, l20.read(), local_24);
        if r != 0
            && ((FUN_004db530() != 0 || FUN_004db550() != 0)
                || FUN_004db570() != 0
                || FUN_004db590() != 0)
        {
            l20.add(4).write(2);
        }
        if (b6 || !b5) && l20.add(6).read() != 0 {
            FUN_004dcfd0(l34, 0x16, 0x132, 0x2a, 0x14a);
            let mut p = l20.add(6);
            loop {
                r = FUN_004dd7d0(l34, l20.read(), local_24);
                if r != 0
                    && ((FUN_004db530() != 0 || FUN_004db550() != 0)
                        || (FUN_004db570() != 0 || FUN_004db590() != 0))
                {
                    p.write(2);
                }
                p = p.add(1);
                FUN_004dd030(l34, 0, 0x1c);
                if p >= l20.add(8) {
                    break;
                }
            }
        }
        FUN_004d9ea0(l20.add(1));
        FUN_004dcfd0(l34, 0x16, 0xc6, 0x2a, 0xde);
        let mut p3 = FUN_004dcfa0(0xc5);
        r = FUN_004d9a80(
            l34,
            (&raw mut DAT_00533ecc).cast(),
            (&raw mut DAT_00533ed4).cast(),
            p3,
            0xc,
            0,
            0xe,
            s_jewel,
        );
        if r != 0 {
            DAT_00650438 = 0;
            DAT_0065043c = 0;
            FUN_004f2c10();
        }
        FUN_004dd030(l34, 0, -0x38);
        if DAT_00650406 == 0 {
            p3 = FUN_004dcfa0(0xc3);
            FUN_004d9d20(l34, (&raw mut DAT_00533ec4).cast(), p3, 0xc);
        } else {
            p3 = FUN_004dcfa0(0xc3);
            r = FUN_004d9a80(
                l34,
                (&raw mut DAT_00533ebc).cast(),
                (&raw mut DAT_00533ebc).cast(),
                p3,
                0xc,
                0,
                0xc,
                s_jewel,
            );
            if r != 0 {
                DAT_00650406 = 0;
            }
        }
        FUN_004dd030(l34, 0, -0x1c);
        if DAT_00650406 == 0 {
            p3 = FUN_004dcfa0(0x223);
            r = FUN_004d9a80(
                l34,
                (&raw mut DAT_00533eac).cast(),
                (&raw mut DAT_00533eac).cast(),
                p3,
                0xc,
                0,
                0xb,
                s_jewel,
            );
            if r != 0 {
                DAT_00650406 = 1;
            }
        } else {
            r = FUN_004dd7d0(l34, l20.read(), local_24);
            if r != 0 && (FUN_004db570() != 0 || FUN_004db590() != 0) {
                if DAT_0085e7c5 != 0 {
                    FUN_004de190(s_jewel, 1, 0, 0xff);
                }
                FUN_004f2d10();
            }
            p3 = FUN_004dcfa0(0xc4);
            FUN_004d9d20(l34, (&raw mut DAT_00533eb4).cast(), p3, 0xc);
        }
        if (b6 || !b5) && l20.add(6).read() != 0 {
            FUN_004dcfd0(l34, 0x16, 0x14e, 0x2a, 0x166);
            p3 = FUN_004dcfa0(0xca);
            r = FUN_004d9a80(
                l34,
                (&raw mut DAT_005337ec).cast(),
                (&raw mut DAT_005337f4).cast(),
                p3,
                0xc,
                0,
                0xf,
                s_jewel,
            );
            if r != 0 {
                FUN_004f5ac0();
            }
            FUN_004dd030(l34, 0, -0x1c);
            p3 = FUN_004dcfa0(0xc9);
            r = FUN_004d9a80(
                l34,
                (&raw mut DAT_005337dc).cast(),
                (&raw mut DAT_005337e4).cast(),
                p3,
                0xc,
                0,
                0x11,
                s_jewel,
            );
            if r != 0 {
                FUN_004f5990();
            }
        }
    }
    0
}
