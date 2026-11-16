// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9683: [u8; 1];
    static mut DAT_009e9684: [u8; 1];
    static mut DAT_009e9685: [u8; 1];
    static mut DAT_009e9a67: [u8; 1];
    static mut DAT_009e9a84: [u8; 1];
    static mut DAT_009ecc34: i32;
    static mut DAT_00527ed0: i32;
    static mut DAT_00a0813f: i8;
    static mut DAT_00527e90: i32;
    static mut DAT_00527e94: i32;
    static mut DAT_009cd264: i16;
    static mut DAT_009a8062: i16;
    static mut DAT_009ed664: i16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a8e24: i32;
    static mut DAT_009a73f4: i8;
    static mut DAT_009d6941: u8;
    static mut DAT_009cd274: i16;
    static mut DAT_00a00860: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a8d9c: u32;
    static mut DAT_00521f1c: [u8; 1];
    static mut DAT_00a08104: u32;
    static mut DAT_009a8e34: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_009a8c1c: *mut u8;
    static mut DAT_009ec476: i8;
    static mut DAT_005235f0: i32;
    static mut DAT_0051a520: i32;
    static mut DAT_009a8c2c: i32;
    static mut DAT_009a7ba8: i16;
    static mut DAT_009e9660: i16;
    static mut DAT_00523408: u8;
    static mut DAT_005233c8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435bc0();
    fn FUN_0040b7b0();
    fn FUN_00438ef0(a: i32);
    fn FUN_004aba50();
    fn FUN_00441c30();
    fn FUN_004abe50();
    fn FUN_004244e0();
    fn FUN_004865e0();
    fn FUN_004244d0();
    fn thunk_FUN_004b3070();
    fn FUN_00426080();
    fn FUN_00410930();
    fn FUN_00436310();
    fn FUN_0044ff10(a: u32, b: u32) -> u8;
    fn FUN_004362a0(a: u32, b: i32);
    fn FUN_00450020(a: i32, b: i32, c: i32, d: u32) -> u8;
    fn FUN_00436680();
    fn FUN_0044fe40(a: u32);
    fn FUN_00427a60(a: i16, b: i16, c: i16, d: i32, e: *mut u8);
    fn FUN_0042b380(a: u16);
    fn FUN_0044f880(a: *mut u8);
    fn FUN_0041fc20(a: u32);
    fn FUN_00472400(a: *mut u8);
    fn FUN_0041ff70(a: u16);
    fn FUN_00401000(a: i32);
    fn FUN_004012c0(a: i32);
    fn FUN_00482be0();
    fn FUN_004ac760() -> i32;
    fn FUN_00486200();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_00452960(a: u32, b: u32);
    fn FUN_00485f00(a: *mut core::ffi::c_void, b: i32);
    fn FUN_00446550(a: *mut core::ffi::c_void, b: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0044f140() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers read from game structures are used where the original does.
    unsafe {
        let mut i_var8: i32 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32;
        if (&raw mut DAT_009e9683)
            .cast::<u8>()
            .offset(i_var8 as isize)
            .read()
            != 0
        {
            DAT_00527ed0 = 0;
            DAT_00a0813f = 0;
            FUN_00435bc0();
            if DAT_00527e90 != 0 {
                FUN_0040b7b0();
            }
            FUN_00438ef0(DAT_009ecc34);
            FUN_004aba50();
            FUN_00452960(0, 0);
            FUN_00441c30();
            FUN_004abe50();
            DAT_00527ed0 = 1;
            return;
        }
        let c_var2: i8 = (&raw mut DAT_009e9684)
            .cast::<u8>()
            .offset(i_var8 as isize)
            .read() as i8;
        let mut this: u32 = (((DAT_009ecc34 as u32) >> 8) << 8) | (c_var2 as u8 as u32);
        if c_var2 == 2 {
            this = (&raw mut DAT_009e9685)
                .cast::<u8>()
                .offset(i_var8 as isize)
                .cast::<u32>()
                .read_unaligned();
            if this < 0x213 {
                DAT_00527ed0 = 0;
                DAT_00a0813f = 0;
                FUN_00485f00(this as usize as *mut core::ffi::c_void, 1);
                FUN_004244e0();
                FUN_004865e0();
                if DAT_00527e90 != 0 {
                    FUN_0040b7b0();
                }
                FUN_00438ef0(DAT_009ecc34);
                FUN_004aba50();
                FUN_00452960(0, 0);
                FUN_00441c30();
                FUN_004abe50();
                DAT_00527ed0 = 1;
                return;
            }
        } else if c_var2 == 1 {
            FUN_004244d0();
            this = 0;
        }
        DAT_00527ed0 = 0;
        DAT_00a0813f = 0;
        if DAT_009cd264 != 0 {
            this = 0;
            if DAT_00527e90 != 0 {
                thunk_FUN_004b3070();
                this = 0;
            }
            DAT_009cd264 = 0;
        }
        FUN_00485f00(this as usize as *mut core::ffi::c_void, 1);
        DAT_009a8062 = 0;
        if DAT_00527e90 != 0 {
            DAT_009ed664 = 0x3000;
            FUN_00426080();
            FUN_004aba50();
            FUN_00410930();
            FUN_004abe50();
        }
        let mut s_var10: i16 = 0;
        let mut unaff_edi: u32 = 0;
        let mut local_4: u32 = 0;
        let this_00: *mut core::ffi::c_void = core::ptr::null_mut();
        FUN_00436310();
        if 0 < DAT_00a07ce8 {
            'outer: loop {
                'next1: {
                    if s_var10 as i32 == DAT_009a8e24 {
                        let mut c1 = false;
                        if DAT_009a73f4 == 0 {
                            c1 = true;
                        } else if DAT_009d6941 != 0 {
                            c1 = true;
                        } else if (&raw mut DAT_009e9684)
                            .cast::<u8>()
                            .offset(((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32) as isize)
                            .read()
                            == 2
                        {
                            c1 = true;
                        }
                        if c1 {
                            s_var10 = DAT_009cd274;
                            if DAT_009cd274 as i32 == DAT_00a07ce8 {
                                break 'outer;
                            }
                        }
                    }
                    unaff_edi = s_var10 as i32 as u32;
                    let idx = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
                    let a1: u16 = (&raw mut DAT_009e9a67)
                        .cast::<u8>()
                        .offset(idx)
                        .cast::<u16>()
                        .read_unaligned();
                    let a2: i16 = (&raw mut DAT_009e9a84)
                        .cast::<u8>()
                        .offset(idx)
                        .cast::<i16>()
                        .read_unaligned();
                    if a1 as u32 != unaff_edi || a2 != 0 || DAT_00a00860 != 0 {
                        let p_ent =
                            DAT_009a1fe8.offset(unaff_edi.wrapping_mul(0x23) as i32 as isize);
                        if p_ent.add(3).read() != 0 {
                            DAT_009a8d9c = (&raw mut DAT_00521f1c)
                                .cast::<u8>()
                                .offset(
                                    (p_ent.add(3).read() as u32).wrapping_mul(0x1a) as i32 as isize
                                )
                                .cast::<u32>()
                                .read_unaligned();
                            let b_var7: u8;
                            match p_ent.add(2).read() {
                                0 | 1 | 2 | 3 | 4 | 5 | 0xe => {
                                    let p9: *mut u8 =
                                        p_ent.add(0x1f).cast::<*mut u8>().read_unaligned();
                                    DAT_00a08104 = p9.add(0xa9).cast::<u32>().read_unaligned();
                                    b_var7 = FUN_0044ff10(unaff_edi, DAT_009a8d9c);
                                }
                                6 | 7 => {
                                    b_var7 = FUN_0044ff10(unaff_edi, DAT_009a8d9c);
                                }
                                0xb => {
                                    b_var7 = FUN_0044ff10(unaff_edi, DAT_009a8d9c);
                                }
                                0xd => {
                                    b_var7 = FUN_0044ff10(unaff_edi, DAT_009a8d9c);
                                }
                                _ => break 'next1,
                            }
                            i_var8 = b_var7 as i32;
                            if i_var8 != 0 {
                                FUN_004362a0(unaff_edi, DAT_009a8e34);
                            }
                        }
                    }
                }
                s_var10 = s_var10.wrapping_add(1);
                if !((s_var10 as i32) < DAT_00a07ce8) {
                    break 'outer;
                }
            }
        }
        s_var10 = DAT_00a07ce8 as i16;
        if (s_var10 as i32) < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32 {
            loop {
                unaff_edi = s_var10 as i32 as u32;
                let p_ent = DAT_009a1fe8.offset(unaff_edi.wrapping_mul(0x23) as i32 as isize);
                if p_ent.add(3).read() != 0 {
                    DAT_009a8d9c = (&raw mut DAT_00521f1c)
                        .cast::<u8>()
                        .offset((p_ent.add(3).read() as u32).wrapping_mul(0x1a) as i32 as isize)
                        .cast::<u32>()
                        .read_unaligned();
                    let t = p_ent.add(2).read();
                    if 7 < t && t < 0xc {
                        let p4: i32 = p_ent.add(4).cast::<i32>().read_unaligned();
                        let p8: i32 = p_ent.add(8).cast::<i32>().read_unaligned();
                        let pc: i32 = p_ent.add(0xc).cast::<i32>().read_unaligned();
                        let b_var7 = FUN_00450020(p4, p8, pc, DAT_009a8d9c);
                        if b_var7 != 0 {
                            FUN_004362a0(unaff_edi, DAT_009a8e34);
                        }
                    }
                }
                s_var10 = s_var10.wrapping_add(1);
                if !((s_var10 as i32)
                    < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32)
                {
                    break;
                }
            }
        }
        FUN_00436680();
        let mut node: *mut u8 = DAT_009a8c1c;
        while !node.is_null() {
            let u_var4: u16 = node.add(4).cast::<u16>().read_unaligned();
            unaff_edi = (unaff_edi & 0xffff0000) | (u_var4 as u32);
            let i_var9: i32 = u_var4 as i16 as i32;
            let i_var5: i32 = (i_var9 as u32).wrapping_mul(0x23) as i32;
            let b_var3: u8 = DAT_009a1fe8.offset(i_var5 as isize).add(2).read();
            if i_var9 < DAT_00a07ce8 {
                match b_var3 {
                    0 | 1 | 2 | 3 | 4 | 5 | 0xe => {
                        let p9: *mut u8 = DAT_009a1fe8
                            .offset(i_var5 as isize)
                            .add(0x1f)
                            .cast::<*mut u8>()
                            .read_unaligned();
                        DAT_00a08104 = p9.add(0xa9).cast::<u32>().read_unaligned();
                        FUN_0044fe40(unaff_edi);
                        let u_var6: u32 = DAT_00527e94 as u32;
                        if b_var3 == 0xe {
                            DAT_009ec476 = 0;
                        }
                        let p_ent = DAT_009a1fe8.offset(i_var5 as isize);
                        if p_ent.add(3).read() == 0x24 {
                            DAT_00527e94 = 0;
                            local_4 = u_var6;
                        }
                        let s14: i16 = p_ent.add(0x14).cast::<i16>().read_unaligned();
                        let s12: i16 = p_ent.add(0x12).cast::<i16>().read_unaligned();
                        let s10: i16 = p_ent.add(0x10).cast::<i16>().read_unaligned();
                        FUN_00427a60(s14, s12, s10, 0, p_ent);
                        if b_var3 == 0xe {
                            FUN_0042b380(u_var4);
                        } else {
                            FUN_0044f880(DAT_009a1fe8.offset(i_var5 as isize));
                            FUN_0041fc20(unaff_edi);
                            FUN_00472400(DAT_009a1fe8.offset(i_var5 as isize));
                            DAT_005235f0 = 0;
                        }
                        if DAT_009a1fe8.offset(i_var5 as isize).add(3).read() == 0x24 {
                            DAT_00527e94 = local_4 as i32;
                        }
                        DAT_009ec476 = 1;
                    }
                    6 | 7 => {
                        FUN_0044fe40(unaff_edi);
                        let p_ent = DAT_009a1fe8.offset(i_var5 as isize);
                        let s14: i16 = p_ent.add(0x14).cast::<i16>().read_unaligned();
                        let s12: i16 = p_ent.add(0x12).cast::<i16>().read_unaligned();
                        let s10: i16 = p_ent.add(0x10).cast::<i16>().read_unaligned();
                        FUN_00427a60(s14, s12, s10, 0, p_ent);
                        FUN_0041ff70(u_var4);
                    }
                    0xb | 0xd => {
                        FUN_0044fe40(unaff_edi);
                        let p_ent = DAT_009a1fe8.offset(i_var5 as isize);
                        let s14: i16 = p_ent.add(0x14).cast::<i16>().read_unaligned();
                        let s12: i16 = p_ent.add(0x12).cast::<i16>().read_unaligned();
                        let s10: i16 = p_ent.add(0x10).cast::<i16>().read_unaligned();
                        FUN_00427a60(s14, s12, s10, 0, p_ent);
                        FUN_00401000(i_var9);
                    }
                    _ => {}
                }
            } else if 7 < b_var3 && b_var3 < 0xc {
                FUN_0044fe40(unaff_edi);
                let p_ent = DAT_009a1fe8.offset(i_var5 as isize);
                let s14: i16 = p_ent.add(0x14).cast::<i16>().read_unaligned();
                let s12: i16 = p_ent.add(0x12).cast::<i16>().read_unaligned();
                let s10: i16 = p_ent.add(0x10).cast::<i16>().read_unaligned();
                FUN_00427a60(s14, s12, s10, 0, core::ptr::null_mut());
                FUN_0044f880(DAT_009a1fe8.offset(i_var5 as isize));
                FUN_00446550(this_00, u_var4 as u32);
                DAT_005235f0 = 0;
            }
            let next: *mut u8 = node.add(8).cast::<*mut u8>().read_unaligned();
            node = next;
        }
        if DAT_00527e90 == 0 {
            DAT_009ed664 = 0x3000;
            FUN_00426080();
            FUN_004aba50();
            FUN_00410930();
            FUN_004abe50();
        }
        DAT_0051a520 = 1;
        FUN_004865e0();
        DAT_0051a520 = 0;
        if DAT_00527e90 == 0 {
            FUN_004012c0(1);
            FUN_00482be0();
        }
        DAT_009a7ba8 = 0;
        DAT_009e9660 = 0;
        i_var8 = FUN_004ac760();
        DAT_009a8c2c = (DAT_009a8c2c as u32).wrapping_add(i_var8 as u32) as i32;
        DAT_009a7ba8 = DAT_009a8c2c as i16;
        FUN_00486200();
        if DAT_00527e90 != 0 {
            FUN_0040b7b0();
        }
        FUN_00438ef0(DAT_009ecc34);
        DAT_00523408 = 0;
        i_var8 = FUN_004ac760();
        DAT_009a8c2c = (DAT_009a8c2c as u32).wrapping_add(i_var8 as u32) as i32;
        DAT_00523408 = DAT_005233c8;
        DAT_009a7ba8 = (DAT_009a8c2c as i16).wrapping_sub(DAT_009a7ba8);
        FUN_004aba50();
        FUN_00452960(0, 0);
        FUN_00441c30();
        FUN_004abe50();
        DAT_00527ed0 = 1;
    }
}
