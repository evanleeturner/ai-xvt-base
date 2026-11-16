// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c0b0: [i8; 1];
    static mut DAT_0052a68c: [i8; 1];
    static mut DAT_0052b2c0: [i8; 1];
    static mut DAT_0052c0a4: [i8; 1];
    static mut DAT_0052c084: [i8; 1];
    static mut DAT_0052c094: [i8; 1];
    static mut DAT_00665688: [i8; 1];
    static mut DAT_00665810: [i8; 1];
    static mut DAT_00665890: [i8; 1];
    static mut DAT_00665990: [i8; 1];
    static mut DAT_00665708: [i8; 1];
    static mut DAT_00665808: i32;
    static mut DAT_0066580c: i32;
    static mut DAT_00aa62a0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcb30();
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004d4e20();
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004dd0f0() -> i32;
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: i32);
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004da780(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: i32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004dcad0() -> u32;
    fn FUN_004dcb10() -> u32;
    fn FUN_004dcb50() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dcd30(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers passed
    // are addresses of game buffers or of our local rect.
    unsafe {
        let mut local_10: [i32; 4] = [0; 4];
        let r = local_10.as_mut_ptr();
        let box_s = (&raw mut DAT_0052c0b0).cast::<i8>();
        let snd = (&raw mut DAT_0052a68c).cast::<i8>();
        let ok = (&raw mut DAT_0052b2c0).cast::<i8>();
        let okd = (&raw mut DAT_0052c0a4).cast::<i8>();
        let can = (&raw mut DAT_0052c084).cast::<i8>();
        let cand = (&raw mut DAT_0052c094).cast::<i8>();
        let d688 = (&raw mut DAT_00665688).cast::<i8>();
        let d810 = (&raw mut DAT_00665810).cast::<i8>();

        if param_1 == 0 {
            FUN_004dcb30();
            let i5 = if DAT_00665688[0] == 0 && DAT_00665810[0] != 0 {
                0x1f0
            } else {
                0xb8
            };
            FUN_004de0b0(i5, 0xff);
            FUN_004d4e20();
            FUN_004b4d40(box_s, 0, 0);
            FUN_004b4d40(box_s, 0, 0);
            FUN_004b4d40(box_s, 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
            return 0;
        }
        let mut b1: u8 = (FUN_004dd0f0() != 0) as u8;
        if b1 != 0 {
            DAT_00aa62a0 = 0;
        }
        FUN_004d6460(r, 0xa6, 0xe1, 0x206, 0xf5);
        local_10[3] = local_10[1].wrapping_add(0x14);
        FUN_004db640(0xf, (&raw mut DAT_00665890).cast::<i8>(), r, 0xffff);
        FUN_004d64a0(r, 0, 0x14);
        FUN_004db640(0xf, (&raw mut DAT_00665990).cast::<i8>(), r, 0xffff);
        FUN_004d64a0(r, 0, 0x14);
        FUN_004db640(0xf, (&raw mut DAT_00665708).cast::<i8>(), r, 0xffff);
        let mut i5: i32;
        'end: {
            if DAT_00665688[0] == 0 {
                if DAT_00665810[0] == 0 {
                    FUN_004d6460(r, 0xab, 0xf0, 0xca, 0x113);
                    let p2 = FUN_004ddc10(0x20b);
                    i5 = FUN_004da780(r, ok, okd, p2, 0xc, 0xffff, 0x14, snd);
                    let u3 = FUN_004dcad0();
                    if (u3 as u8) == 13 {
                        i5 = 1;
                    }
                    if i5 != 0 {
                        b1 = 1;
                        DAT_00aa62a0 = 1;
                        FUN_004de0b0(DAT_00665808, DAT_0066580c);
                    }
                    break 'end;
                }
                FUN_004d6460(r, 0x1e1, 0xf0, 0x201, 0x113);
                i5 = FUN_004da780(r, can, cand, d810, 0xc, 0xffff, 0x15, snd);
                let u3 = FUN_004dcad0();
                if (u3 as u8) == 0x1b {
                    i5 = 1;
                }
            } else {
                if DAT_00665810[0] == 0 {
                    FUN_004d6460(r, 0xab, 0xf0, 0xca, 0x113);
                    i5 = FUN_004da780(r, ok, okd, d688, 0xc, 0xffff, 0x14, snd);
                    let u3 = FUN_004dcad0();
                    if (u3 as u8) == 13 {
                        i5 = 1;
                    }
                    if i5 != 0 {
                        b1 = 1;
                        DAT_00aa62a0 = 1;
                    }
                    break 'end;
                }
                FUN_004d6460(r, 0xab, 0xf0, 0xca, 0x113);
                i5 = FUN_004da780(r, ok, okd, d688, 0xc, 0xffff, 0x14, snd);
                let u3 = FUN_004dcb10();
                if (u3 as u8) == 13 {
                    i5 = 1;
                    FUN_004dcad0();
                }
                if i5 != 0 {
                    DAT_00aa62a0 = 1;
                }
                b1 = ((i5 != 0) || b1 != 0) as u8;
                FUN_004d6460(r, 0x1e1, 0xf0, 0x201, 0x113);
                i5 = FUN_004da780(r, can, cand, d810, 0xc, 0xffff, 0x15, snd);
                let u3 = FUN_004dcad0();
                if (u3 as u8) == 0x1b {
                    i5 = 1;
                    FUN_004dcb50();
                }
            }
            if i5 != 0 {
                DAT_00aa62a0 = 0;
                b1 = 1;
            }
        }
        if b1 == 0 {
            return 0;
        }
        1
    }
}
