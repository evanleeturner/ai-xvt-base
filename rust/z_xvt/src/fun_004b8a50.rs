// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ef50: i32;
    static mut DAT_0064ef54: i32;
    static mut DAT_0064ef58: i32;
    static mut DAT_0064ef5c: i32;
    static mut DAT_0064ef60: i32;
    static mut DAT_0064ef64: i32;
    static mut DAT_0085e6c0: i32;
    static mut DAT_00859c40: [i8; 0];
    static mut DAT_0085e7a0: [i8; 0];
    static mut DAT_0081bcf0: [i8; 0];
    static mut DAT_0081e082: [i8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_0081bcd0: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0085e7c6: i8;
    static mut DAT_00532ad8: i32;
    static mut DAT_00758620: u32;
    static mut DAT_00758624: u32;
    static mut DAT_00758628: u32;
    static mut DAT_0075862c: u32;
    static mut DAT_00758630: u32;
    static mut DAT_00758634: u32;
    static mut DAT_00758638: u32;
    static mut DAT_0075863c: u32;
    static mut DAT_00758640: u32;
    static mut DAT_00758644: u32;
    static mut DAT_00758648: u32;
    static mut DAT_0075864c: u32;
    static mut DAT_00758650: u32;
    static mut DAT_00758654: u32;
    static mut DAT_00758658: u32;
    static mut DAT_0075865c: u32;
    static mut DAT_00758660: u32;
    static mut DAT_0085e7b4: u8;
    static mut DAT_0085e7b5: u8;
    static mut DAT_0085e7b6: u8;
    static mut DAT_0085e7b7: u8;
    static mut DAT_0085e7b8: u8;
    static mut DAT_0085e7b9: u8;
    static mut DAT_0085e7ba: u8;
    static mut DAT_0085e7bb: u8;
    static mut DAT_0085e7bc: u8;
    static mut DAT_0085e7bd: u8;
    static mut DAT_0085e7be: u8;
    static mut DAT_0085e7bf: u8;
    static mut DAT_0085e78f: u8;
    static mut DAT_0085e7c4: u8;
    static mut DAT_0085e790: u8;

    static mut DAT_00531360: [i8; 0];
    static mut DAT_00531378: [i8; 0];
    static mut DAT_00531358: [i8; 0];
    static mut DAT_00531350: [i8; 0];
    static mut DAT_00531340: [i8; 0];
    static mut DAT_00531334: [i8; 0];
    static mut DAT_00531328: [i8; 0];
    static mut DAT_0053131c: [i8; 0];
    static mut DAT_00531310: [i8; 0];
    static mut DAT_005312fc: [i8; 0];
    static mut DAT_00531304: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ddc90(a: i32, b: i32);
    fn FUN_004d8fb0();
    fn FUN_004d8f30();
    fn FUN_004dbea0();
    fn FUN_004bdd20();
    fn FUN_004d0290(a: *mut i8, b: *mut i8);
    fn FUN_004d52b0();
    fn FUN_004d1750(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004dcfd0(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004db490(a: i32);
    fn FUN_004bbd50();
    fn FUN_004b8ed0();
    fn FUN_004bcc20();
    fn FUN_004bd330();
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004da920(a: i32, b: *mut i8, c: *mut i32, d: i32);
    fn FUN_004bb3c0();
    fn FUN_004c0750(a: i32) -> i32;
    fn FUN_004da030();
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004da060(a: *mut i8);
    fn FUN_004d9a80(
        r: *mut i32,
        a: *mut i8,
        b: *mut i8,
        c: *mut i8,
        d: i32,
        e: i32,
        f: i32,
        g: *mut i8,
    ) -> u32;
    fn FUN_004da050();
    fn FUN_004dc470() -> u8;
    fn FUN_004cc8e0() -> u32;
    fn FUN_004bacf0();
    fn FUN_004dbb40();
    fn FUN_004db520();
    fn FUN_004d0430(a: *mut i8);
    fn FUN_004db4c0();
    fn FUN_004d8fe0();
    fn rand() -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8a50(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: *mut i8,
    param_5: *mut i32,
    param_6: *mut i32,
    param_7: *mut i32,
    param_8: u32,
    param_9: u32,
    param_10: i32,
    param_11: i32,
    param_12: u32,
    param_13: u32,
) -> u32 {
    let _ = (
        param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9, param_10, param_11,
        param_12, param_13,
    );
    let mut local_rect: [i32; 4] = [0; 4];
    let mut i_var5: i32 = 0;
    // SAFETY: all globals and callees are the game's own; pointers passed
    // are to the game's buffers and to the local rect, as in the original.
    unsafe {
        if param_1.is_null() {
            FUN_004ddc90(0x20, 0x7f);
            FUN_004d8fb0();
            FUN_004d8f30();
            DAT_0064ef50 = 0;
            DAT_0064ef54 = 0;
            DAT_0064ef64 = 0;
            DAT_0064ef58 = 0;
            DAT_0064ef60 = 1;
            FUN_004dbea0();
            FUN_004bdd20();
            if 0 < DAT_0085e6c0 {
                let mut pc = (&raw mut DAT_00859c40).cast::<i8>();
                loop {
                    let idx = DAT_0064ef58 as isize;
                    if pc.read() == (&raw mut DAT_0085e7a0).cast::<i8>().offset(idx).read() {
                        DAT_0064ef5c = i_var5;
                    }
                    pc = pc.byte_add(0x95);
                    i_var5 = i_var5.wrapping_add(1);
                    if !(i_var5 < DAT_0085e6c0) {
                        break;
                    }
                }
            }
            FUN_004d0290(
                (&raw mut DAT_00531360).cast(),
                (&raw mut DAT_00531378).cast(),
            );
            FUN_004d52b0();
            FUN_004d1750((&raw mut DAT_00531378).cast(), 0, 0);
            FUN_004d0e60((&raw mut DAT_00531358).cast(), 0, 0);
            FUN_004d0e60((&raw mut DAT_00531350).cast(), 0, 0);
            FUN_004dcfd0(local_rect.as_mut_ptr(), 0x54, 0x6b, 0x25c, 0x1b1);
            FUN_004d0580((&raw mut DAT_00531340).cast(), 0, 0);
            FUN_004d5350(1);
            FUN_004db490(0x14);
        }
        match DAT_0064ef64 {
            0 => {
                FUN_004bbd50();
                if DAT_0064ef60 != 0 {
                    FUN_004d52b0();
                    FUN_004d5350(1);
                }
            }
            1 | 2 => {
                FUN_004b8ed0();
                if DAT_0064ef60 != 0 {
                    FUN_004d52b0();
                    FUN_004d5350(1);
                }
            }
            3 => {
                FUN_004bcc20();
                if DAT_0064ef60 != 0 {
                    FUN_004d52b0();
                    FUN_004d5350(1);
                }
            }
            4 => {
                FUN_004bd330();
            }
            _ => {}
        }
        FUN_004dcfd0(local_rect.as_mut_ptr(), 200, 0x1c4, 0x1b4, 0x1d0);
        if (&raw mut DAT_0081bcf0).cast::<i8>().read() != 0 {
            sprintf(
                (&raw mut DAT_0081b740).cast(),
                (&raw const DAT_00531334).cast::<i8>(),
                6i32,
                (&raw mut DAT_0081e082).cast::<i8>(),
                1i32,
                (&raw mut DAT_0081bcf0).cast::<i8>(),
            );
            FUN_004da920(
                0xc,
                (&raw mut DAT_0081b740).cast(),
                local_rect.as_mut_ptr(),
                DAT_0081bcd0,
            );
            let u4 = ((param_1 as usize as i32) >> 0x1f) as u32;
            let p = param_1 as usize as u32;
            let t = (p ^ u4).wrapping_sub(u4) & 0x1f;
            i_var5 = ((t ^ u4).wrapping_sub(u4) as i32) >> 1;
            sprintf(
                (&raw mut DAT_0081b740).cast(),
                (&raw const DAT_00531328).cast::<i8>(),
                i_var5,
            );
            FUN_004d0e60((&raw mut DAT_0081b740).cast(), 0xcc, 0x1c5);
            sprintf(
                (&raw mut DAT_0081b740).cast(),
                (&raw const DAT_0053131c).cast::<i8>(),
                i_var5,
            );
            FUN_004d0e60((&raw mut DAT_0081b740).cast(), 0x1a4, 0x1c5);
        }
        DAT_0064ef60 = 0;
        FUN_004bb3c0();
        i_var5 = FUN_004c0750(2);
        if i_var5 != 1 {
            FUN_004dcfd0(local_rect.as_mut_ptr(), 0x55, 0x1bf, 0xb0, 0x1d7);
            if DAT_0085e7c6 != 0 {
                FUN_004da030();
            }
            let pc = FUN_004dcfa0(0xc1);
            FUN_004da060(pc);
            let pc8 = (&raw mut DAT_00531310).cast::<i8>();
            let pc2 = FUN_004dcfa0(0xc1);
            let mut u4 = FUN_004d9a80(
                local_rect.as_mut_ptr(),
                (&raw mut DAT_005312fc).cast(),
                (&raw mut DAT_00531304).cast(),
                pc2,
                0xc,
                0,
                8,
                pc8,
            );
            FUN_004da050();
            let b1 = FUN_004dc470();
            u4 |= b1 as u32;
            if DAT_0081b850 == 4 {
                let u3 = FUN_004cc8e0();
                u4 |= u3;
            }
            if u4 != 0 {
                FUN_004bacf0();
                DAT_00532ad8 = 0;
                FUN_004dbea0();
                FUN_004dbb40();
                FUN_004db520();
                FUN_004d0430((&raw mut DAT_00531378).cast());
                FUN_004db4c0();
                FUN_004d8fe0();
                if DAT_0081b850 == 4 {
                    DAT_00758620 = 0x59;
                    DAT_00758624 = DAT_0085e7b4 as u32;
                    DAT_00758628 = DAT_0085e7b5 as u32;
                    DAT_0075862c = DAT_0085e7b6 as u32;
                    DAT_00758630 = DAT_0085e7b7 as u32;
                    DAT_00758634 = DAT_0085e7b8 as u32;
                    DAT_00758638 = DAT_0085e7b9 as u32;
                    DAT_0075863c = DAT_0085e7ba as u32;
                    DAT_00758640 = DAT_0085e7bb as u32;
                    DAT_00758644 = DAT_0085e7bc as u32;
                    DAT_00758648 = DAT_0085e7bd as u32;
                    DAT_0075864c = DAT_0085e7be as u32;
                    DAT_00758650 = DAT_0085e7bf as u32;
                    DAT_00758654 = rand() as u32;
                    DAT_00758658 = DAT_0085e78f as u32;
                    DAT_0075865c = DAT_0085e7c4 as u32;
                    DAT_00758660 = DAT_0085e790 as u32;
                    FUN_004cb830(0, &raw mut DAT_00758620, 0x44);
                }
            }
            return 0;
        }
        1
    }
}
