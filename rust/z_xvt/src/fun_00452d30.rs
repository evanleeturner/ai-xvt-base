// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: u16;
    static mut DAT_007091ae: u16;
    static mut DAT_006ffbf2: u16;
    static mut DAT_00732e44: i16;
    static mut DAT_006d4984: u16;
    static mut DAT_00703be4: u16;
    static mut _DAT_006da4fc: i32;
    static mut DAT_00732f3c: i32;
    static mut _DAT_006d4994: i32;
    static mut DAT_0070acc4: i32;
    static mut DAT_0071ed00: i32;
    static mut DAT_006d4a42: i16;
    static mut _DAT_0071f2e4: i32;
    static mut DAT_0070acc8: i32;
    static mut _DAT_006db7bc: i32;
    static mut DAT_0071ecf4: i32;

    static mut DAT_0071bb2f: [u8; 0];
    static mut DAT_0071bb2b: [u8; 0];
    static mut DAT_0071bb2d: [u8; 0];
    static mut DAT_0071bb1b: [u8; 0];
    static mut DAT_0071bb1f: [u8; 0];
    static mut DAT_0071bb23: [u8; 0];

    static mut DAT_005f0e34: i16;
    static mut DAT_005f0e32: i16;
    static mut DAT_005f0e30: i16;
    static mut DAT_00525e08: i16;
    static mut DAT_005f0e24: i32;
    static mut DAT_005f0e28: i32;
    static mut DAT_005f0e2c: i32;
    static mut _DAT_00520168: f32;

    static mut DAT_0073255c: i32;
    static mut DAT_00732558: i32;
    static mut DAT_00732568: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732584: i32;
    static mut DAT_0073253c: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
    static mut DAT_00732538: i32;
    static mut DAT_0070a138: i32;
    static mut DAT_0070a134: i32;
    static mut DAT_0070a130: i32;
    static mut DAT_0070a12c: i32;
    static mut DAT_0070a010: i32;
    static mut DAT_0070a00c: i32;
    static mut _DAT_0070a008: i32;
    static mut _DAT_0070a124: i32;
    static mut _DAT_0070a120: i32;

    static mut _DAT_006ccae0: f32;
    static mut _DAT_006ccae4: f32;
    static mut _DAT_006ccae8: f32;
    static mut _DAT_006ccaec: f32;
    static mut _DAT_006ccaf0: f32;
    static mut _DAT_006ccaf4: f32;
    static mut _DAT_006ccaf8: f32;
    static mut _DAT_006ccafc: f32;
    static mut _DAT_006ccb00: f32;
    static mut _DAT_006ccb04: f32;
    static mut _DAT_006ccb08: f32;
    static mut _DAT_006ccb0c: f32;
    static mut _DAT_006ccb10: f32;
    static mut _DAT_006ccb14: f32;
    static mut _DAT_006ccb18: f32;
    static mut _DAT_006ccb1c: f32;
    static mut _DAT_006ccb20: f32;
    static mut _DAT_006ccb24: f32;
    static mut _DAT_006ccb28: f32;
    static mut _DAT_006ccb2c: f32;
    static mut _DAT_006ccb30: f32;
    static mut _DAT_006ccb34: f32;
    static mut _DAT_006ccb38: f32;
    static mut _DAT_006ccb3c: f32;

    static mut DAT_00525e14: i32;
    static mut DAT_00525e18: u16;
    static mut DAT_00525e1c: i32;
    static mut DAT_00527e74: u16;
    static mut DAT_006da9f4: u32;
    static mut DAT_006db7b4: i32;
    static mut DAT_005254e0: u8;
    static mut DAT_005f0e3f: u32;
    static mut DAT_005254f8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00483180(a: i16, b: i16, c: i16, d: i32, e: i32, f: i32, g: i32);
    fn FUN_004832a0(a: i16, b: i16, c: i16, d: i16, e: i32);
    fn FUN_00414fe0(a: *mut f32, b: *mut f32);
    fn FUN_00465740();
    fn FUN_0048f4b0(a: u32);
    fn FUN_0048f3c0(a: u32, b: i32) -> u32;
    fn FUN_0048f4f0(a: u32) -> u32;
    fn __ftol() -> i64;
    fn FUN_00486310(a: i32);
    fn FUN_00466020();
    fn FUN_00465c40();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_00465970(a: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00452d30(
    mut param_1: i32,
    param_2: i32,
    mut param_3: i32,
    mut param_4: i32,
) -> u32 {
    // SAFETY: all globals, arrays and callees are the game's own memory and
    // code; pointers are read and written where the original does.
    unsafe {
        if DAT_006da850 == 0 {
            return 0;
        }
        if param_1 < 0 {
            param_3 = param_3.wrapping_add(param_1);
            param_1 = 0;
        }
        if param_2 < 0 {
            param_4 = param_4.wrapping_add(param_2);
        }
        if 0x3ff < param_1 {
            return 0;
        }
        if 0x2ff < param_2 {
            return 0;
        }
        if 0x300 < param_4.wrapping_add(param_2) {
            param_4 = 0x300i32.wrapping_sub(param_2);
        }
        if 0x400 < param_3.wrapping_add(param_1) {
            param_3 = 0x400i32.wrapping_sub(param_1);
        }
        DAT_007091ae = param_3 as u16;
        DAT_006ffbf2 = DAT_007091ae.wrapping_sub(1);
        DAT_00732e44 = (param_3 / 2) as i16;
        DAT_006d4984 = param_4 as u16;
        DAT_00703be4 = DAT_006d4984.wrapping_sub(1);
        _DAT_006da4fc = param_2;
        DAT_00732f3c = 0x200;
        _DAT_006d4994 = 0x100;
        DAT_0070acc4 = 9;
        DAT_0071ed00 = 0;
        DAT_006d4a42 = (param_4 / 2) as i16;
        _DAT_0071f2e4 = (DAT_0070acc8 as u32)
            .wrapping_mul(param_2 as u32)
            .wrapping_add(param_1 as u32) as i32;
        _DAT_006db7bc = param_1;

        let mut idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let s0 = (&raw const DAT_0071bb2f)
            .cast::<u8>()
            .add(idx)
            .cast::<i16>()
            .read_unaligned();
        let s1 = (&raw const DAT_0071bb2b)
            .cast::<u8>()
            .add(idx)
            .cast::<i16>()
            .read_unaligned();
        let s2 = (&raw const DAT_0071bb2d)
            .cast::<u8>()
            .add(idx)
            .cast::<i16>()
            .read_unaligned();
        FUN_00483180(s0, s1, s2, 0, 0, 0, 0);
        FUN_004832a0(DAT_005f0e34, DAT_005f0e32, DAT_005f0e30, DAT_00525e08, 0);

        idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let i0 = (&raw const DAT_0071bb1b)
            .cast::<u8>()
            .add(idx)
            .cast::<i32>()
            .read_unaligned();
        _DAT_006ccae0 = DAT_005f0e24.wrapping_sub(i0) as f32;
        idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let i1 = (&raw const DAT_0071bb1f)
            .cast::<u8>()
            .add(idx)
            .cast::<i32>()
            .read_unaligned();
        _DAT_006ccae4 = DAT_005f0e28.wrapping_sub(i1) as f32;
        _DAT_006ccaec = (DAT_0073255c as f32) * _DAT_00520168;
        _DAT_006ccaf0 = (DAT_00732558 as f32) * _DAT_00520168;
        _DAT_006ccaf4 = (DAT_00732568 as f32) * _DAT_00520168;
        _DAT_006ccafc = (DAT_00732578 as f32) * _DAT_00520168;
        _DAT_006ccaf8 = (DAT_00732584 as f32) * _DAT_00520168;
        _DAT_006ccb04 = (DAT_0073253c as f32) * _DAT_00520168;
        _DAT_006ccb00 = (DAT_00732530 as f32) * _DAT_00520168;
        _DAT_006ccb0c = (DAT_00732548 as f32) * _DAT_00520168;
        _DAT_006ccb08 = (DAT_00732538 as f32) * _DAT_00520168;
        idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let i2 = (&raw const DAT_0071bb23)
            .cast::<u8>()
            .add(idx)
            .cast::<i32>()
            .read_unaligned();
        _DAT_006ccae8 = DAT_005f0e2c.wrapping_sub(i2) as f32;
        FUN_00414fe0(&raw mut _DAT_006ccae0, &raw mut _DAT_006ccaec);

        _DAT_006ccaf0 = (DAT_0070a138 as f32) * _DAT_00520168;
        _DAT_006ccaf4 = (DAT_0070a134 as f32) * _DAT_00520168;
        _DAT_006ccaf8 = (DAT_0070a130 as f32) * _DAT_00520168;
        _DAT_006ccaec = (DAT_0070a12c as f32) * _DAT_00520168;
        _DAT_006ccafc = (DAT_0070a010 as f32) * _DAT_00520168;
        _DAT_006ccb00 = (DAT_0070a00c as f32) * _DAT_00520168;
        _DAT_006ccb04 = (_DAT_0070a008 as f32) * _DAT_00520168;
        _DAT_006ccb08 = (_DAT_0070a124 as f32) * _DAT_00520168;
        _DAT_006ccb0c = (_DAT_0070a120 as f32) * _DAT_00520168;
        _DAT_006ccb10 = -_DAT_006ccae0;
        _DAT_006ccb14 = -_DAT_006ccae4;
        _DAT_006ccb18 = -_DAT_006ccae8;
        _DAT_006ccb1c = _DAT_006ccaec;
        _DAT_006ccb20 = _DAT_006ccaf8;
        _DAT_006ccb24 = _DAT_006ccb04;
        _DAT_006ccb28 = _DAT_006ccaf0;
        _DAT_006ccb2c = _DAT_006ccafc;
        _DAT_006ccb30 = _DAT_006ccb08;
        _DAT_006ccb34 = _DAT_006ccaf4;
        _DAT_006ccb38 = _DAT_006ccb00;
        _DAT_006ccb3c = _DAT_006ccb0c;
        FUN_00414fe0(&raw mut _DAT_006ccb10, &raw mut _DAT_006ccb1c);

        if DAT_00525e14 == 0 {
            FUN_00465740();
            let n = ((((DAT_007091ae as u32) >> 7).wrapping_add(2))
                .wrapping_mul(DAT_006d4984 as u32))
            .wrapping_add(DAT_00527e74 as u32);
            if DAT_00525e1c < n as i32 {
                if DAT_00525e18 != 0 {
                    FUN_0048f4b0(DAT_00525e18 as u32);
                }
                let n2 = ((((DAT_007091ae as u32) >> 7).wrapping_add(2))
                    .wrapping_mul(DAT_006d4984 as u32))
                .wrapping_add(DAT_00527e74 as u32);
                DAT_00525e18 = FUN_0048f3c0(n2, 0) as u16;
                let n3 = ((((DAT_007091ae as u32) >> 7).wrapping_add(2))
                    .wrapping_mul(DAT_006d4984 as u32))
                .wrapping_add(DAT_00527e74 as u32);
                DAT_00525e1c = n3 as i32;
            }
            DAT_006da9f4 = FUN_0048f4f0(DAT_00525e18 as u32);
            let mut u_var2: u32 = 0;
            let mut p4 = (DAT_00527e74 as u32).wrapping_add(DAT_006da9f4) as *mut u8;
            if DAT_006d4984 != 0 {
                loop {
                    *p4 = 1;
                    let mut p5 = p4.wrapping_add(1);
                    let u_var6 = DAT_007091ae as u32;
                    let mut u_var7 = u_var6;
                    if 0xff < u_var6 {
                        *p5 = 0;
                        p5 = p4.wrapping_add(2);
                        u_var7 = u_var6.wrapping_sub(0xff);
                        if 0xff < u_var7 {
                            *p5 = 0;
                            p5 = p4.wrapping_add(3);
                            u_var7 = u_var6.wrapping_sub(0x1ff);
                        }
                    }
                    *p5 = u_var7 as u8;
                    p4 = p5.wrapping_add(1);
                    u_var2 = u_var2.wrapping_add(1);
                    if !(u_var2 < DAT_006d4984 as u32) {
                        break;
                    }
                }
            }
            DAT_00525e14 = 1;
        }
        let l_var8 = __ftol();
        DAT_006db7b4 = l_var8 as i32;
        let u_var3 = DAT_005254e0 as u32;
        let p = (&raw const DAT_005f0e3f).read_unaligned();
        *(p.wrapping_add(0x88) as *mut u8) = DAT_005254e0;
        let u_var1 = DAT_005254f8;
        DAT_005254f8 = 0;
        FUN_00465970(u_var3);
        FUN_00486310(0x5f0e20);
        FUN_00466020();
        FUN_00465c40();
        DAT_005254f8 = u_var1;
        1
    }
}
