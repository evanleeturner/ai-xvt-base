// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b70c: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0064ef60: i32;
    static mut DAT_0081b6f8: i32;
    static mut DAT_00532ad8: i32;
    static mut DAT_0081bcd0: u32;
    static mut DAT_0085e6f0: i8;
    static mut DAT_0085e7c5: i8;
    static mut DAT_0085e78f: i8;
    static mut DAT_0085e790: i8;
    static mut DAT_0085e731: [i8; 1];
    static mut DAT_0085e6f1: [i8; 1];
    static mut DAT_0085e77f: [i8; 1];
    static mut DAT_005313a0: [i8; 1];
    static mut DAT_00531384: [i8; 1];
    static mut DAT_0053138c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004da920(a: i32, s: *mut i8, r: *mut i32, c: u32);
    fn FUN_004db2f0(s: *mut i8, a: i32) -> i32;
    fn FUN_004d0500(s: *mut i8, r: *mut i32);
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004daac0(a: i32, s: *mut i8, r: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004dd030(r: *mut i32, a: i32, b: i32);
    fn FUN_004dd000(a: *mut i32, b: *mut i32);
    fn FUN_004d52b0();
    fn FUN_004d0580(s: *mut i8, a: i32, b: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004dd7d0(r: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004de190(s: *mut i8, a: i32, b: i32, c: i32);
    fn FUN_004d0e60(s: *mut i8, a: i32, b: i32);
    fn FUN_004dd070(r: *mut i32, a: i32, b: i32);
    fn FUN_004dd130(r: *mut i32, a: i32, b: i32, c: i32);
    fn FUN_004d9660(r: *mut i32, s: *mut i8, a: i32, b: i32, c: i32, d: *mut i8) -> i32;
    fn FUN_004bc8b0(s: *mut i8, r: *mut i32, a: i32);
    fn FUN_004bc9e0(s: *mut i8, r: *mut i32, a: i32);
    fn FUN_004b9970(s: *mut i8, r: *mut i32, a: i32);
    fn FUN_004b9b40(s: *mut i8, r: *mut i32, a: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub unsafe extern "C" fn FUN_004bbd50(
    param_1: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: *mut i8,
    param_5: *mut i32,
    param_6: *mut i8,
    param_7: *mut i32,
    param_8: *mut i8,
    param_9: *mut i8,
    param_10: *mut u32,
    param_11: i32,
    param_12: *mut i8,
    param_13: *mut i32,
    param_14: *mut i32,
    param_15: *mut i32,
    param_16: u32,
    param_17: u32,
    param_18: i32,
    param_19: i32,
    param_20: u32,
    param_21: u32,
) {
    // SAFETY: all globals and callees are the game's own; the local arrays
    // outlive every call that receives a pointer to them.
    unsafe {
        let mut l18: [i32; 4] = [0; 4];
        let mut l28: [i32; 4] = [0; 4];
        let mut local_8: i32 = 0;
        let mut local_4: i32 = 0;
        let mut c_stack_29: i8;
        let s_off: *mut i8 = (&raw mut DAT_005313a0).cast::<i8>();
        let s_btn: *mut i8 = (&raw mut DAT_00531384).cast::<i8>();
        let s_snd: *mut i8 = (&raw mut DAT_0053138c).cast::<i8>();

        macro_rules! cond {
            () => {
                DAT_0081b70c != 0 || DAT_0081b850 == 2
            };
        }
        macro_rules! prehit {
            () => {
                if DAT_0064ef60 != 0 && (cond!()) {
                    FUN_004d52b0();
                    FUN_004d0580(s_off, l28[0], l28[1]);
                    FUN_004d5350(0);
                    FUN_004d0580(s_off, l28[0], l28[1]);
                }
            };
        }
        macro_rules! hit {
            ($v:expr, $f:expr) => {
                let hit = FUN_004dd7d0(l28.as_mut_ptr(), local_4, local_8);
                if hit != 0 && (cond!()) {
                    let mut go = true;
                    if FUN_004db570() == 0 {
                        if FUN_004db590() == 0 {
                            go = false;
                        }
                    }
                    if go {
                        if DAT_0085e6f0 != $v {
                            if DAT_0085e7c5 != 0 {
                                FUN_004de190(s_snd, 1, 0, 0xff);
                            }
                            if DAT_0085e6f0 != $v {
                                DAT_0085e78f = $f;
                            }
                        }
                        DAT_0085e6f0 = $v;
                    }
                }
                if DAT_0085e6f0 == $v {
                    FUN_004d0e60(s_btn, l28[0], l28[1]);
                }
            };
        }
        macro_rules! label {
            ($id:expr, $c:expr) => {
                let pc = FUN_004dcfa0($id);
                FUN_004daac0(0xc, pc, l28.as_mut_ptr(), 0, 1, $c);
            };
        }

        FUN_004dcfd0(l18.as_mut_ptr(), 0x54, 0x5a, 0x25c, 0x6a);
        let pc = FUN_004dcfa0(0x123);
        FUN_004da920(0xf, pc, l18.as_mut_ptr(), 0xffff);
        let pc = FUN_004dcfa0(0x127);
        let i3 = FUN_004db2f0(pc, 0xc);
        let pc = FUN_004dcfa0(0x128);
        let mut i8v = FUN_004db2f0(pc, 0xc);
        if i8v < i3 {
            i8v = i3;
        }
        FUN_004d0500(s_off, l28.as_mut_ptr());
        let i3 = l28[2].wrapping_sub(l28[0]).wrapping_add(1);
        FUN_004ddc70(&raw mut local_4, &raw mut local_8);
        FUN_004dcfd0(l18.as_mut_ptr(), 0x58, 0x6f, 0x14c, 0x7d);
        let u7: u32 = if DAT_0081b70c == 0 && DAT_0081b850 != 2 {
            0x2a6
        } else {
            0x1cf
        };
        let pc = FUN_004dcfa0(u7 as i32);
        FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x14);
        l18[2] = l18[0].wrapping_add(0x7f);
        FUN_004dd000(l28.as_mut_ptr(), l18.as_mut_ptr());
        l28[2] = l28[0].wrapping_add(i3);
        prehit!();
        hit!(0, 0);
        l28[0] = l28[2].wrapping_add(5);
        l28[2] = l28[2].wrapping_add(0x69);
        label!(0x124, DAT_0081bcd0);
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x19);
        l18[2] = l18[0].wrapping_add(0x7f);
        FUN_004dd000(l28.as_mut_ptr(), l18.as_mut_ptr());
        l28[2] = l28[0].wrapping_add(i3);
        prehit!();
        hit!(1, 1);
        l28[0] = l28[2].wrapping_add(5);
        l28[2] = l28[2].wrapping_add(0x69);
        label!(0x125, DAT_0081bcd0);
        FUN_004dd030(l28.as_mut_ptr(), 100, 0);
        label!(0x127, DAT_0081bcd0);
        l28[0] = i8v.wrapping_add(0xf).wrapping_add(l28[0]);
        l28[2] = l28[0].wrapping_add(200);
        FUN_004dd070(l28.as_mut_ptr(), 0, -2);
        FUN_004dd130(l28.as_mut_ptr(), 0, 0, DAT_0081b6f8);
        let r = FUN_004d9660(
            l28.as_mut_ptr(),
            (&raw mut DAT_0085e731).cast::<i8>(),
            0x40,
            0,
            0xc,
            core::ptr::null_mut(),
        );
        if r != 0 {
            DAT_00532ad8 = 1;
        }
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x19);
        l18[2] = l18[0].wrapping_add(0x7f);
        FUN_004dd000(l28.as_mut_ptr(), l18.as_mut_ptr());
        l28[2] = l28[0].wrapping_add(i3);
        prehit!();
        hit!(2, 0);
        l28[0] = l28[2].wrapping_add(5);
        l28[2] = l28[2].wrapping_add(0x69);
        label!(0x126, DAT_0081bcd0);
        FUN_004dd030(l28.as_mut_ptr(), 100, 0);
        label!(0x128, DAT_0081bcd0);
        l28[0] = l28[0].wrapping_add(i8v).wrapping_add(0xf);
        l28[2] = l28[0].wrapping_add(200);
        FUN_004dd070(l28.as_mut_ptr(), 0, -2);
        FUN_004dd130(l28.as_mut_ptr(), 0, 0, DAT_0081b6f8);
        let r = FUN_004d9660(
            l28.as_mut_ptr(),
            (&raw mut DAT_0085e6f1).cast::<i8>(),
            0x40,
            1,
            0xc,
            core::ptr::null_mut(),
        );
        if r != 0 {
            DAT_00532ad8 = 2;
        }
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x19);
        l18[2] = l18[0].wrapping_add(0x7f);
        FUN_004dd000(l28.as_mut_ptr(), l18.as_mut_ptr());
        l28[2] = i3.wrapping_add(l28[0]);
        prehit!();
        hit!(3, 0);
        l28[0] = l28[2].wrapping_add(5);
        l28[2] = l28[2].wrapping_add(0x69);
        label!(0x1b4, DAT_0081bcd0);
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x23);
        {
            let pc = FUN_004dcfa0(0x1d0);
            FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, 0xffff);
        }
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x14);
        FUN_004dd000(l28.as_mut_ptr(), l18.as_mut_ptr());
        label!(0x1d4, DAT_0081bcd0);
        let pc = FUN_004dcfa0(0x1d4);
        let w = FUN_004db2f0(pc, 0xc);
        l28[0] = l28[0].wrapping_add(w).wrapping_add(0xf);
        l28[2] = l28[0].wrapping_add(200);
        FUN_004dd070(l28.as_mut_ptr(), 0, -2);
        FUN_004dd130(l28.as_mut_ptr(), 0, 0, DAT_0081b6f8);
        let r = FUN_004d9660(
            l28.as_mut_ptr(),
            (&raw mut DAT_0085e77f).cast::<i8>(),
            0x10,
            2,
            0xc,
            core::ptr::null_mut(),
        );
        if r != 0 {
            DAT_00532ad8 = 0;
        }
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x23);
        if DAT_0081b850 == 3 {
            let pc = FUN_004dcfa0(0x2dd);
            FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, 0xffff);
            FUN_004dd030(l18.as_mut_ptr(), 0, 0x14);
            let pc = FUN_004dcfa0(0x1d1);
            FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, DAT_0081bcd0);
            FUN_004dd030(l18.as_mut_ptr(), 0, 0x11);
            FUN_004bc8b0(
                (&raw mut DAT_0085e78f).cast::<i8>(),
                l18.as_mut_ptr(),
                0x1d2,
            );
            FUN_004dd030(l18.as_mut_ptr(), 0, 0x14);
            let pc = FUN_004dcfa0(0x2d9);
            FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, DAT_0081bcd0);
            FUN_004dd030(l18.as_mut_ptr(), 0, 0x11);
            c_stack_29 = (((DAT_0085e790 as i32) >> 1).wrapping_sub(2)) as i8;
            FUN_004bc9e0(&raw mut c_stack_29, l18.as_mut_ptr(), 0x2da);
            return;
        }
        let pc = FUN_004dcfa0(0x2d8);
        FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, 0xffff);
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x14);
        let pc = FUN_004dcfa0(0x1d1);
        FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, DAT_0081bcd0);
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x11);
        FUN_004b9970(
            (&raw mut DAT_0085e78f).cast::<i8>(),
            l18.as_mut_ptr(),
            0x1d2,
        );
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x14);
        let pc = FUN_004dcfa0(0x2d9);
        FUN_004daac0(0xc, pc, l18.as_mut_ptr(), 0, 1, DAT_0081bcd0);
        FUN_004dd030(l18.as_mut_ptr(), 0, 0x11);
        c_stack_29 = (((DAT_0085e790 as i32) >> 1).wrapping_sub(2)) as i8;
        FUN_004b9b40(&raw mut c_stack_29, l18.as_mut_ptr(), 0x2da);
        DAT_0085e790 = ((c_stack_29 as i32).wrapping_mul(2).wrapping_add(4)) as i8;
    }
}
