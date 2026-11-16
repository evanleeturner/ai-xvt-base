// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665688: i8;
    static mut DAT_00665810: i8;
    static mut DAT_00665890: i8;
    static mut DAT_00665990: i8;
    static mut DAT_00665708: i8;
    static mut DAT_00665808: i32;
    static mut DAT_0066580c: i32;
    static mut DAT_00aa62a0: i32;
    static mut DAT_0052c0b0: i8;
    static mut DAT_0052b2c0: i8;
    static mut DAT_0052c0a4: i8;
    static mut DAT_0052c084: i8;
    static mut DAT_0052c094: i8;
    static mut DAT_0052a68c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcb30() -> u32;
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004d4e20();
    fn FUN_004b4d40(name: *mut i8, a: i32, b: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004d6460(rect: *mut i32, left: i32, top: i32, right: i32, bottom: i32);
    fn FUN_004db640(a: i32, text: *mut i8, rect: *mut i32, color: i32);
    fn FUN_004d64a0(rect: *mut i32, dx: i32, dy: i32);
    fn FUN_004ddc10(id: i32) -> *mut i8;
    fn FUN_004da780(
        rect: *mut i32,
        s1: *mut i8,
        s2: *mut i8,
        text: *mut i8,
        a: i32,
        b: i32,
        c: i32,
        snd: *mut i8,
    ) -> i32;
    fn FUN_004dcad0() -> u32;
    fn FUN_004dcb10() -> u32;
    fn FUN_004dcb50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd620(param_1: i32) -> u32 {
    let mut rect: [i32; 4] = [0; 4];
    let mut b_var3: i32 = 0;
    let mut i_var5: i32;
    // SAFETY: all callees and globals are the game's own; rect is a local
    // array passed by pointer as the original does.
    unsafe {
        if param_1 == 0 {
            FUN_004dcb30();
            if DAT_00665688 == 0 && DAT_00665810 != 0 {
                i_var5 = 0x1f0;
            } else {
                i_var5 = 0xb8;
            }
            FUN_004de0b0(i_var5, 0xff);
            FUN_004d4e20();
            FUN_004b4d40(&raw mut DAT_0052c0b0, 0, 0);
            FUN_004b4d40(&raw mut DAT_0052c0b0, 0, 0);
            FUN_004b4d40(&raw mut DAT_0052c0b0, 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
            return 0;
        }
        let r = rect.as_mut_ptr();
        FUN_004d6460(r, 0xa6, 0xe1, 0x206, 0xf5);
        rect[3] = rect[1].wrapping_add(0x14);
        FUN_004db640(0xf, &raw mut DAT_00665890, r, 0xffff);
        FUN_004d64a0(r, 0, 0x14);
        FUN_004db640(0xf, &raw mut DAT_00665990, r, 0xffff);
        FUN_004d64a0(r, 0, 0x14);
        FUN_004db640(0xf, &raw mut DAT_00665708, r, 0xffff);
        'end: {
            if DAT_00665688 == 0 {
                if DAT_00665810 == 0 {
                    FUN_004d6460(r, 0xab, 0xf0, 0xca, 0x113);
                    let pc = FUN_004ddc10(0x20b);
                    i_var5 = FUN_004da780(
                        r,
                        &raw mut DAT_0052b2c0,
                        &raw mut DAT_0052c0a4,
                        pc,
                        0xc,
                        0xffff,
                        0x14,
                        &raw mut DAT_0052a68c,
                    );
                    let u = FUN_004dcad0();
                    if u as u8 == 0x0d {
                        i_var5 = 1;
                    }
                    if i_var5 != 0 {
                        b_var3 = 1;
                        DAT_00aa62a0 = 1;
                        FUN_004de0b0(DAT_00665808, DAT_0066580c);
                    }
                    break 'end;
                }
                FUN_004d6460(r, 0x1e1, 0xf0, 0x201, 0x113);
                i_var5 = FUN_004da780(
                    r,
                    &raw mut DAT_0052c084,
                    &raw mut DAT_0052c094,
                    &raw mut DAT_00665810,
                    0xc,
                    0xffff,
                    0x15,
                    &raw mut DAT_0052a68c,
                );
                let u = FUN_004dcad0();
                if u as u8 == 0x1b {
                    i_var5 = 1;
                }
            } else {
                if DAT_00665810 == 0 {
                    FUN_004d6460(r, 0xab, 0xf0, 0xca, 0x113);
                    i_var5 = FUN_004da780(
                        r,
                        &raw mut DAT_0052b2c0,
                        &raw mut DAT_0052c0a4,
                        &raw mut DAT_00665688,
                        0xc,
                        0xffff,
                        0x14,
                        &raw mut DAT_0052a68c,
                    );
                    let u = FUN_004dcad0();
                    if u as u8 == 0x0d {
                        i_var5 = 1;
                    }
                    if i_var5 != 0 {
                        b_var3 = 1;
                        DAT_00aa62a0 = 1;
                    }
                    break 'end;
                }
                FUN_004d6460(r, 0xab, 0xf0, 0xca, 0x113);
                i_var5 = FUN_004da780(
                    r,
                    &raw mut DAT_0052b2c0,
                    &raw mut DAT_0052c0a4,
                    &raw mut DAT_00665688,
                    0xc,
                    0xffff,
                    0x14,
                    &raw mut DAT_0052a68c,
                );
                let u = FUN_004dcb10();
                if u as u8 == 0x0d {
                    i_var5 = 1;
                    FUN_004dcad0();
                }
                b_var3 = (i_var5 != 0) as i32;
                if b_var3 != 0 {
                    DAT_00aa62a0 = 1;
                }
                FUN_004d6460(r, 0x1e1, 0xf0, 0x201, 0x113);
                i_var5 = FUN_004da780(
                    r,
                    &raw mut DAT_0052c084,
                    &raw mut DAT_0052c094,
                    &raw mut DAT_00665810,
                    0xc,
                    0xffff,
                    0x15,
                    &raw mut DAT_0052a68c,
                );
                let u = FUN_004dcad0();
                if u as u8 == 0x1b {
                    i_var5 = 1;
                    FUN_004dcb50();
                }
            }
            if i_var5 != 0 {
                DAT_00aa62a0 = 0;
                b_var3 = 1;
            }
        }
    }
    if b_var3 == 0 {
        return 0;
    }
    1
}
