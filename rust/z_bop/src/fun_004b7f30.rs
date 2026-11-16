// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664e88: i32;
    static mut DAT_00664e8c: i32;
    static mut DAT_00664e90: i32;
    static mut DAT_00664e94: i32;
    static mut DAT_00664e98: i32;
    static mut DAT_00664e9c: i32;
    static mut DAT_00bb72a0: i32;
    static mut DAT_00bb2820: [i8; 1];
    static mut DAT_00bb7380: [i8; 1];
    static mut DAT_00b6a2e0: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b6bc9e: i8;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00bb73a8: i8;
    static mut DAT_00b69e30: i32;
    static mut DAT_0052c000: i32;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: u32;
    static mut DAT_00aa6af8: u32;
    static mut DAT_00aa6afc: u32;
    static mut DAT_00aa6b00: u32;
    static mut DAT_00aa6b04: u32;
    static mut DAT_00aa6b08: u32;
    static mut DAT_00aa6b0c: u32;
    static mut DAT_00aa6b10: u32;
    static mut DAT_00aa6b14: u32;
    static mut DAT_00aa6b18: u32;
    static mut DAT_00aa6b1c: u32;
    static mut DAT_00aa6b20: u32;
    static mut DAT_00aa6b24: u32;
    static mut DAT_00aa6b28: u32;
    static mut DAT_00aa6b2c: u32;
    static mut DAT_00aa6b30: u32;
    static mut DAT_00aa6b34: u32;
    static mut DAT_00aa6b38: u32;
    static mut DAT_00bb7394: u8;
    static mut DAT_00bb7395: u8;
    static mut DAT_00bb7396: u8;
    static mut DAT_00bb7397: u8;
    static mut DAT_00bb7398: u8;
    static mut DAT_00bb7399: u8;
    static mut DAT_00bb739a: u8;
    static mut DAT_00bb739b: u8;
    static mut DAT_00bb739c: u8;
    static mut DAT_00bb739d: u8;
    static mut DAT_00bb739e: u8;
    static mut DAT_00bb739f: u8;
    static mut DAT_00bb736f: u8;
    static mut DAT_00bb73a4: u8;
    static mut DAT_00bb7370: u8;
    static mut DAT_00bb73a5: u8;
    static mut DAT_00bb73a6: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004d9d10();
    fn FUN_004d9ca0();
    fn FUN_004dcb30();
    fn FUN_004bd5f0();
    fn FUN_004b4a60(a: *const i8, b: *const i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *const i8, b: i32, c: i32);
    fn FUN_004b5570(a: *const i8, b: i32, c: i32);
    fn FUN_004d6460(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004b4d40(a: *const i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004bb680();
    fn FUN_004b83a0();
    fn FUN_004bc520();
    fn FUN_004bcc10();
    fn FUN_004bda30();
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004baaf0();
    fn FUN_004bf9b0(a: i32) -> i32;
    fn FUN_004dad40();
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004dad70(a: *mut i8);
    fn FUN_004da780(
        r: *mut i32,
        a: *const i8,
        b: *const i8,
        c: *mut i8,
        d: i32,
        e: i32,
        f: i32,
        g: *const i8,
    ) -> u32;
    fn FUN_004dad60();
    fn FUN_004dd0f0() -> u32;
    fn FUN_004cffb0() -> u32;
    fn FUN_004ba3c0();
    fn FUN_004dc7e0();
    fn FUN_004dc1d0();
    fn FUN_004b4bf0(a: *const i8);
    fn FUN_004dc170();
    fn FUN_004d9d40();
    fn rand() -> i32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7f30(param_1: u32) -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all callees and globals are the game's own memory; pointers
    // are used exactly as the original does.
    unsafe {
        let mut i_var5: i32 = 0;
        if param_1 == 0 {
            FUN_004de0b0(0x20, 0x7f);
            FUN_004d9d10();
            FUN_004d9ca0();
            DAT_00664e88 = 0;
            DAT_00664e8c = 0;
            DAT_00664e9c = 0;
            DAT_00664e90 = 0;
            DAT_00664e98 = 1;
            FUN_004dcb30();
            FUN_004bd5f0();
            if 0 < DAT_00bb72a0 {
                let mut pc = (&raw mut DAT_00bb2820).cast::<i8>();
                loop {
                    let idx = DAT_00664e90 as isize;
                    if pc.read() == (&raw mut DAT_00bb7380).cast::<i8>().offset(idx).read() {
                        DAT_00664e94 = i_var5;
                    }
                    pc = pc.byte_add(0x95);
                    i_var5 = i_var5.wrapping_add(1);
                    if !(i_var5 < DAT_00bb72a0) {
                        break;
                    }
                }
            }
            FUN_004b4a60(c"frontres\\configb.bmp".as_ptr(), c"backconfig".as_ptr());
            FUN_004d4e20();
            FUN_004b5de0(c"backconfig".as_ptr(), 0, 0);
            FUN_004b5570(c"frame".as_ptr(), 0, 0);
            FUN_004b5570(c"alloff".as_ptr(), 0, 0);
            FUN_004d6460(local_10.as_mut_ptr(), 0x54, 0x6b, 0x25c, 0x1b1);
            FUN_004b4d40(c"configoverlay".as_ptr(), 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
        }
        'sd: {
            'lab: {
                'aft: {
                    match DAT_00664e9c {
                        0 => {
                            FUN_004bb680();
                            break 'aft;
                        }
                        1 | 2 => {
                            FUN_004b83a0();
                            if DAT_00664e98 == 0 {
                                break 'sd;
                            }
                            break 'lab;
                        }
                        3 => {
                            FUN_004bc520();
                            break 'aft;
                        }
                        4 => {
                            FUN_004bcc10();
                            break 'sd;
                        }
                        5 => {
                            FUN_004bda30();
                            break 'aft;
                        }
                        _ => break 'sd,
                    }
                }
                if DAT_00664e98 == 0 {
                    break 'sd;
                }
            }
            FUN_004d4e20();
            FUN_004d4ec0(1);
        }
        FUN_004d6460(local_10.as_mut_ptr(), 200, 0x1c4, 0x1b4, 0x1d0);
        if (&raw mut DAT_00b6a2e0).cast::<i8>().read() != 0 {
            sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                c"%c%s %c%s".as_ptr(),
                6i32,
                &raw mut DAT_00b6bc9e,
                1i32,
                (&raw mut DAT_00b6a2e0).cast::<i8>(),
            );
            FUN_004db640(
                0xc,
                (&raw mut DAT_00b69d20).cast::<i8>(),
                local_10.as_mut_ptr(),
                DAT_00b6a2c0,
            );
            let u_var4: u32 = ((param_1 as i32) >> 0x1f) as u32;
            i_var5 = ((((param_1 ^ u_var4).wrapping_sub(u_var4) & 0x1f) ^ u_var4)
                .wrapping_sub(u_var4) as i32)
                >> 1;
            sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                c"rebtiny%d".as_ptr(),
                i_var5,
            );
            FUN_004b5570((&raw mut DAT_00b69d20).cast::<i8>(), 0xcc, 0x1c5);
            sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                c"imptiny%d".as_ptr(),
                i_var5,
            );
            FUN_004b5570((&raw mut DAT_00b69d20).cast::<i8>(), 0x1a4, 0x1c5);
        }
        DAT_00664e98 = 0;
        FUN_004baaf0();
        i_var5 = FUN_004bf9b0(2);
        if i_var5 == 1 {
            return 1;
        }
        FUN_004d6460(local_10.as_mut_ptr(), 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_00bb73a8 != 0 {
            FUN_004dad40();
        }
        let pc = FUN_004ddc10(0xce);
        FUN_004dad70(pc);
        let pc = FUN_004ddc10(0xce);
        let mut u_var4: u32 = FUN_004da780(
            local_10.as_mut_ptr(),
            c"leaveup".as_ptr(),
            c"leavedown".as_ptr(),
            pc,
            0xc,
            0,
            8,
            c"buttonsound".as_ptr(),
        );
        FUN_004dad60();
        u_var4 |= FUN_004dd0f0();
        if DAT_00b69e30 == 4 {
            let u_var3 = FUN_004cffb0();
            u_var4 |= u_var3;
        }
        if u_var4 != 0 {
            FUN_004ba3c0();
            DAT_0052c000 = 0;
            FUN_004dcb30();
            FUN_004dc7e0();
            FUN_004dc1d0();
            FUN_004b4bf0(c"backconfig".as_ptr());
            FUN_004dc170();
            FUN_004d9d40();
            if DAT_00b69e30 == 4 {
                DAT_00aa6af0 = 0x59;
                DAT_00aa6af4 = DAT_00bb7394 as u32;
                DAT_00aa6af8 = DAT_00bb7395 as u32;
                DAT_00aa6afc = DAT_00bb7396 as u32;
                DAT_00aa6b00 = DAT_00bb7397 as u32;
                DAT_00aa6b04 = DAT_00bb7398 as u32;
                DAT_00aa6b08 = DAT_00bb7399 as u32;
                DAT_00aa6b0c = DAT_00bb739a as u32;
                DAT_00aa6b10 = DAT_00bb739b as u32;
                DAT_00aa6b14 = DAT_00bb739c as u32;
                DAT_00aa6b18 = DAT_00bb739d as u32;
                DAT_00aa6b1c = DAT_00bb739e as u32;
                DAT_00aa6b20 = DAT_00bb739f as u32;
                DAT_00aa6b24 = rand() as u32;
                DAT_00aa6b28 = DAT_00bb736f as u32;
                DAT_00aa6b2c = DAT_00bb73a4 as u32;
                DAT_00aa6b30 = DAT_00bb7370 as u32;
                DAT_00aa6b34 = DAT_00bb73a5 as u32;
                DAT_00aa6b38 = DAT_00bb73a6 as u32;
                FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x4c);
            }
        }
        0
    }
}
