// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00531990: [i8; 0];
    static mut DAT_0053335c: [i8; 0];
    static mut DAT_00533368: [i8; 0];
    static mut DAT_00533374: [i8; 0];
    static mut DAT_00533380: [i8; 0];
    static mut DAT_0053338c: [i8; 0];
    static mut DAT_00533398: [i8; 0];
    static mut DAT_0053332c: [i8; 0];
    static mut DAT_00533338: [i8; 0];
    static mut DAT_00533344: [i8; 0];
    static mut DAT_00533350: [i8; 0];

    static mut DAT_0064ff48: i32;
    static mut DAT_0064ff4c: i32;
    static mut DAT_0064ff58: [u32; 11];
    static mut DAT_0064ff84: f32;
    static mut DAT_0064ff88: i32;
    static mut DAT_0064ff8c: i32;
    static mut DAT_0064ff94: f32;
    static mut DAT_00757ca0: *mut i8;
    static mut DAT_00757ca8: i32;
    static mut _DAT_005202ec: f32;
    static mut _DAT_005202f0: f64;
    static mut _DAT_00520300: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004dcfd0(rect: *mut i32, l: i32, t: i32, r: i32, b: i32);
    fn FUN_004dd7d0(rect: *mut i32, x: i32, y: i32) -> i32;
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004dd030(rect: *mut i32, dx: i32, dy: i32);
    fn FUN_004d9ea0(p: *mut i32);
    fn FUN_004dcfa0(id: i32) -> *mut i8;
    fn FUN_004d9a80(
        rect: *mut i32,
        up: *mut i8,
        down: *mut i8,
        text: *mut i8,
        a: i32,
        b: i32,
        c: i32,
        snd: *mut i8,
    ) -> i32;
    fn FUN_004539c0();
    fn FUN_004a7ac0(p: *mut u32);
    fn FUN_004526c0(p: *mut i8);
    fn FUN_00453aa0(a: i32, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e7ec0() -> u32 {
    // SAFETY: all callees and globals are the game's own; local buffers are
    // valid for the accesses made, and the game's record array is indexed
    // as the original does.
    unsafe {
        let mut local_34: i32 = 0;
        let mut local_30: i32 = 0;
        let mut local_2c: [i32; 4] = [0; 4];
        let mut local_1c: [i32; 7] = [1, 1, 1, 0, 0, 1, 1];
        let rect = local_2c.as_mut_ptr();
        let base = local_1c.as_mut_ptr();

        FUN_004ddc70(&raw mut local_30, &raw mut local_34);
        FUN_004dcfd0(rect, 0x16, 0x72, 0x2a, 0x8a);
        FUN_004ddc70(&raw mut local_30, &raw mut local_34);
        let mut idx: usize = 0;
        loop {
            let p = base.add(idx);
            if *p != 0 {
                if FUN_004dd7d0(rect, local_30, local_34) != 0 {
                    if FUN_004db530() != 0
                        || FUN_004db550() != 0
                        || FUN_004db570() != 0
                        || FUN_004db590() != 0
                    {
                        *p = 2;
                    }
                }
            }
            idx += 1;
            FUN_004dd030(rect, 0, 0x1c);
            if idx >= 3 {
                break;
            }
        }

        FUN_004dcfd0(rect, 0x16, 0x132, 0x2a, 0x14a);
        idx = 5;
        loop {
            let p = base.add(idx);
            if *p != 0 {
                if FUN_004dd7d0(rect, local_30, local_34) != 0 {
                    if FUN_004db530() != 0
                        || FUN_004db550() != 0
                        || FUN_004db570() != 0
                        || FUN_004db590() != 0
                    {
                        *p = 2;
                    }
                }
            }
            idx += 1;
            FUN_004dd030(rect, 0, 0x1c);
            if 7 <= idx {
                break;
            }
        }

        FUN_004d9ea0(base);
        FUN_004dcfd0(rect, 0x16, 0xaa, 0x2a, 0xc2);
        let snd = &raw mut DAT_00531990 as *mut i8;
        let pc = FUN_004dcfa0(0x119);
        let r = FUN_004d9a80(
            rect,
            &raw mut DAT_0053338c as *mut i8,
            &raw mut DAT_00533398 as *mut i8,
            pc,
            0xc,
            0,
            0xd,
            snd,
        );
        if r != 0 {
            DAT_0064ff4c = DAT_0064ff4c.wrapping_sub(1);
            if DAT_0064ff4c == -2 {
                DAT_0064ff88 = DAT_0064ff88.wrapping_sub(1);
                DAT_0064ff4c = 1;
                if DAT_0064ff88 == -2 {
                    DAT_0064ff88 = 1;
                    DAT_0064ff8c = DAT_0064ff8c.wrapping_sub(1);
                    if DAT_0064ff8c == -2 {
                        DAT_0064ff8c = 1;
                    }
                }
            }
            FUN_004539c0();
        }

        FUN_004dd030(rect, 0, -0x1c);
        let pc = FUN_004dcfa0(0x117);
        FUN_004d9a80(
            rect,
            &raw mut DAT_00533374 as *mut i8,
            &raw mut DAT_00533380 as *mut i8,
            pc,
            0xc,
            0,
            0xc,
            snd,
        );
        if FUN_004dd7d0(rect, local_30, local_34) != 0 {
            if FUN_004db530() == 0 {
                if FUN_004db550() != 0 {
                    DAT_0064ff84 = DAT_0064ff84 + _DAT_005202ec;
                    if (_DAT_00520300 as f32) <= DAT_0064ff84 {
                        DAT_0064ff84 = 0.0;
                    }
                }
            } else {
                DAT_0064ff84 = DAT_0064ff84 - _DAT_005202ec;
                if DAT_0064ff84 <= (_DAT_005202f0 as f32) {
                    DAT_0064ff84 = 360.0;
                }
            }
        }

        FUN_004dd030(rect, 0, -0x1c);
        let pc = FUN_004dcfa0(0x118);
        FUN_004d9a80(
            rect,
            &raw mut DAT_0053335c as *mut i8,
            &raw mut DAT_00533368 as *mut i8,
            pc,
            0xc,
            0,
            0xb,
            snd,
        );
        if FUN_004dd7d0(rect, local_30, local_34) != 0 {
            if FUN_004db530() == 0 {
                if FUN_004db550() != 0 {
                    DAT_0064ff94 = DAT_0064ff94 + _DAT_005202ec;
                    if (_DAT_00520300 as f32) <= DAT_0064ff94 {
                        DAT_0064ff94 = 0.0;
                    }
                }
            } else {
                DAT_0064ff94 = DAT_0064ff94 - _DAT_005202ec;
                if DAT_0064ff94 <= (_DAT_005202f0 as f32) {
                    DAT_0064ff94 = 360.0;
                }
            }
        }

        FUN_004dcfd0(rect, 0x16, 0x14e, 0x2a, 0x166);
        let pc = FUN_004dcfa0(0x11b);
        let r = FUN_004d9a80(
            rect,
            &raw mut DAT_00533344 as *mut i8,
            &raw mut DAT_00533350 as *mut i8,
            pc,
            0xc,
            0,
            0x11,
            snd,
        );
        if r != 0 {
            DAT_0064ff48 = DAT_0064ff48.wrapping_sub(1);
            if DAT_0064ff48 < 0 {
                DAT_0064ff48 = DAT_00757ca8.wrapping_sub(1);
            }
            let arr = &raw mut DAT_0064ff58 as *mut u32;
            let mut i = 0usize;
            while i < 11 {
                arr.add(i).write(0);
                i += 1;
            }
            let off = (DAT_0064ff48 as u32).wrapping_mul(0x44);
            let rec = DAT_00757ca0.byte_add(off as usize);
            arr.write(rec.byte_add(0x40).cast::<u32>().read_unaligned());
            FUN_004a7ac0(arr);
            FUN_004526c0(rec);
            let v = rec.byte_add(0x40).cast::<i32>().read_unaligned();
            let u6 = if v == 6 || v == 7 { 100 } else { 0 };
            FUN_00453aa0(0, u6, 0);
        }

        FUN_004dd030(rect, 0, -0x1c);
        let pc = FUN_004dcfa0(0x11a);
        let r = FUN_004d9a80(
            rect,
            &raw mut DAT_0053332c as *mut i8,
            &raw mut DAT_00533338 as *mut i8,
            pc,
            0xc,
            0,
            0x10,
            snd,
        );
        if r != 0 {
            DAT_0064ff48 = DAT_0064ff48.wrapping_add(1);
            if DAT_00757ca8 <= DAT_0064ff48 {
                DAT_0064ff48 = 0;
            }
            let arr = &raw mut DAT_0064ff58 as *mut u32;
            let mut i = 0usize;
            while i < 11 {
                arr.add(i).write(0);
                i += 1;
            }
            let off = (DAT_0064ff48 as u32).wrapping_mul(0x44);
            let rec = DAT_00757ca0.byte_add(off as usize);
            arr.write(rec.byte_add(0x40).cast::<u32>().read_unaligned());
            FUN_004a7ac0(arr);
            FUN_004526c0(rec);
            let v = rec.byte_add(0x40).cast::<i32>().read_unaligned();
            if v != 6 && v != 7 {
                FUN_00453aa0(0, 0, 0);
                return 1;
            }
            FUN_00453aa0(0, 100, 0);
        }
        1
    }
}
