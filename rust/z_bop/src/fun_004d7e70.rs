// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052bfb4: [i8; 1];
    static mut DAT_0052bf74: [i8; 1];
    static mut DAT_0052a6d4: [i8; 1];
    static mut DAT_0052b69c: [i8; 1];
    static mut DAT_0052b6ac: [i8; 1];
    static mut DAT_0052bfac: [i8; 1];
    static mut DAT_0052b1b4: [i8; 1];
    static mut DAT_0052a6b0: [i8; 1];
    static mut DAT_0052a6a4: [i8; 1];
    static mut DAT_0052a698: [i8; 1];
    static mut DAT_0052a68c: [i8; 1];
    static mut DAT_0052a678: [i8; 1];
    static mut DAT_0052a680: [i8; 1];

    static mut DAT_00b6a250: i32;
    static mut DAT_0052c184: i32;
    static mut DAT_00bb72d0: i8;
    static mut DAT_00bb73a8: i8;
    static mut DAT_00aa6150: [u32; 0x54];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b6a2e0: [i8; 1];
    static mut DAT_00b6bc9e: [i8; 1];
    static mut DAT_00b6a2c0: u32;

    static mut DAT_004d7370: u8;
    static mut DAT_004be050: u8;
    static mut DAT_004e1470: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d73c0();
    fn FUN_004be1b0();
    fn FUN_004e15d0();
    fn FUN_004b7f30();

    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004d6460(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004e0490() -> i32;
    fn FUN_004cd9e0();
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8);
    fn FUN_004dc380(f: *const core::ffi::c_void, p: *mut core::ffi::c_void);
    fn FUN_004dc330(f: *const core::ffi::c_void, p: *mut core::ffi::c_void);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004bf9b0(a: i32) -> i32;
    fn FUN_004dad40();
    fn FUN_004dad70(a: *mut i8);
    fn FUN_004da780(
        r: *mut i32,
        a: *mut i8,
        b: *mut i8,
        c: *mut i8,
        d: i32,
        e: i32,
        f: i32,
        g: *mut i8,
    ) -> i32;
    fn FUN_004dad60();
    fn FUN_004b4bf0(a: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d7e70(param_1: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers passed
    // are to the game's buffers or to locals that outlive the calls.
    unsafe {
        let s_frontres = (&raw mut DAT_0052bfb4).cast::<i8>();
        let s_background = (&raw mut DAT_0052bf74).cast::<i8>();
        let s_frame = (&raw mut DAT_0052a6d4).cast::<i8>();
        let s_clientactive = (&raw mut DAT_0052b69c).cast::<i8>();
        let s_allactive = (&raw mut DAT_0052b6ac).cast::<i8>();
        let s_chatbox = (&raw mut DAT_0052bfac).cast::<i8>();
        let s_v = (&raw mut DAT_0052b1b4).cast::<i8>();
        let s_ccc = (&raw mut DAT_0052a6b0).cast::<i8>();
        let s_reb = (&raw mut DAT_0052a6a4).cast::<i8>();
        let s_imp = (&raw mut DAT_0052a698).cast::<i8>();
        let s_buttonsound = (&raw mut DAT_0052a68c).cast::<i8>();
        let s_leaveup = (&raw mut DAT_0052a678).cast::<i8>();
        let s_leavedown = (&raw mut DAT_0052a680).cast::<i8>();
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();

        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];

        if param_1 == 0 {
            FUN_004b4a60(s_frontres, s_background);
            FUN_004d4e20();
            FUN_004b5de0(s_background, 0, 0);
            FUN_004b5570(s_frame, 0, 0);
            let pc = if DAT_00b6a250 == 0 {
                s_clientactive
            } else {
                s_allactive
            };
            FUN_004b5570(pc, 0, 0);
            FUN_004b4d40(s_chatbox, 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
        }
        FUN_004d6460(local_20.as_mut_ptr(), 0x54, 0x6b, 0x1b2, 0x1b1);
        let pc4 = FUN_004ddc10(0x14);
        FUN_004db640(0xf, pc4, local_20.as_mut_ptr(), 0xffff);
        let mut i_var1 = FUN_004e0490();

        'lab: {
            let pc_f: *const core::ffi::c_void;
            let pv: *mut core::ffi::c_void;
            if i_var1 == 0x44
                || i_var1 == 0x5e
                || i_var1 == 0x5f
                || i_var1 == 0x60
                || i_var1 == 0x46
                || i_var1 == 0x3a
            {
                FUN_004cd9e0();
                match i_var1 {
                    0x3a | 0x60 => {
                        let a = FUN_004ddc10(0x22a);
                        let b = FUN_004ddc10(0x229);
                        let c = FUN_004ddc10(0x228);
                        FUN_004dcb90(c, b, a, core::ptr::null_mut(), core::ptr::null_mut());
                    }
                    0x44 => {
                        let a = FUN_004ddc10(0x221);
                        let b = FUN_004ddc10(0x220);
                        let c = FUN_004ddc10(0x21f);
                        FUN_004dcb90(c, b, a, core::ptr::null_mut(), core::ptr::null_mut());
                    }
                    0x46 => {
                        let a = FUN_004ddc10(0x279);
                        let b = FUN_004ddc10(0x278);
                        let c = FUN_004ddc10(0x277);
                        FUN_004dcb90(c, b, a, core::ptr::null_mut(), core::ptr::null_mut());
                    }
                    0x5e => {
                        let a = FUN_004ddc10(0x224);
                        let b = FUN_004ddc10(0x223);
                        let c = FUN_004ddc10(0x222);
                        FUN_004dcb90(c, b, a, core::ptr::null_mut(), core::ptr::null_mut());
                    }
                    0x5f => {
                        let a = FUN_004ddc10(0x227);
                        let b = FUN_004ddc10(0x226);
                        let c = FUN_004ddc10(0x225);
                        FUN_004dcb90(c, b, a, core::ptr::null_mut(), core::ptr::null_mut());
                        FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                        FUN_004dc380(
                            FUN_004b7f30 as *const core::ffi::c_void,
                            local_10.as_mut_ptr().cast(),
                        );
                    }
                    _ => {}
                }
                if DAT_00bb72d0 == 0 {
                    DAT_0052c184 = 1;
                    pv = (&raw mut DAT_004d7370).cast();
                    pc_f = FUN_004d73c0 as *const core::ffi::c_void;
                } else {
                    pv = (&raw mut DAT_004be050).cast();
                    pc_f = FUN_004be1b0 as *const core::ffi::c_void;
                }
            } else {
                if i_var1 != 0x45 {
                    break 'lab;
                }
                pv = (&raw mut DAT_004e1470).cast();
                let mut p = (&raw mut DAT_00aa6150).cast::<u32>();
                let mut n = 0x54;
                while n != 0 {
                    *p = 0;
                    p = p.add(1);
                    n -= 1;
                }
                pc_f = FUN_004e15d0 as *const core::ffi::c_void;
            }
            FUN_004dc330(pc_f, pv);
        }

        FUN_004d6460(local_20.as_mut_ptr(), 0x1fb, 0x1c4, 0x232, 0x1d0);
        sprintf(buf, s_v, 2i32, 0i32);
        FUN_004db640(0xc, buf, local_20.as_mut_ptr(), 0xffff);
        FUN_004d6460(local_20.as_mut_ptr(), 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_00b6a2e0[0] != 0 {
            sprintf(
                buf,
                s_ccc,
                6i32,
                (&raw mut DAT_00b6bc9e).cast::<i8>(),
                1i32,
                (&raw mut DAT_00b6a2e0).cast::<i8>(),
            );
            FUN_004db640(0xc, buf, local_20.as_mut_ptr(), DAT_00b6a2c0);
            let u_var10 = ((param_1 as i32) >> 0x1f) as u32;
            i_var1 = ((((param_1 ^ u_var10).wrapping_sub(u_var10) & 0x1f) ^ u_var10)
                .wrapping_sub(u_var10) as i32)
                >> 1;
            sprintf(buf, s_reb, i_var1);
            FUN_004b5570(buf, 0xcc, 0x1c5);
            sprintf(buf, s_imp, i_var1);
            FUN_004b5570(buf, 0x1a4, 0x1c5);
        }
        i_var1 = FUN_004bf9b0(0);
        if i_var1 == 1 {
            return 1;
        }
        if DAT_00bb73a8 != 0 {
            FUN_004dad40();
        }
        let pc4 = FUN_004ddc10(0x13);
        FUN_004dad70(pc4);
        FUN_004d6460(local_20.as_mut_ptr(), 0x55, 0x1bf, 0xb0, 0x1d7);
        let pc4 = FUN_004ddc10(0x13);
        i_var1 = FUN_004da780(
            local_20.as_mut_ptr(),
            s_leaveup,
            s_leavedown,
            pc4,
            0xc,
            0,
            8,
            s_buttonsound,
        );
        FUN_004dad60();
        if i_var1 != 0 || param_1 == 0x1e0 {
            FUN_004cd9e0();
            DAT_0052c184 = 1;
            FUN_004dc330(
                FUN_004d73c0 as *const core::ffi::c_void,
                (&raw mut DAT_004d7370).cast(),
            );
            FUN_004b4bf0(s_background);
        }
        0
    }
}
