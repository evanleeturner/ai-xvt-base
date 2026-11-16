// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665d28: i32;
    static mut DAT_00665d2c: i32;
    static mut DAT_00665d30: i32;
    static mut DAT_00665d38: [u32; 11];
    static mut DAT_00665d64: i32;
    static mut DAT_00665d68: i32;
    static mut DAT_00665d6c: i32;
    static mut DAT_00665d70: i32;
    static mut DAT_00665d74: i32;
    static mut DAT_00aa60d0: i32;
    static mut DAT_00aa60f4: *mut core::ffi::c_void;
    static mut DAT_00aa6110: *mut core::ffi::c_void;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00b6a2e0: [i8; 0];
    static mut DAT_00b6bc9e: [i8; 0];
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00bb73a8: i8;
    static mut DAT_0052c47c: [i8; 0];
    static mut DAT_0052b3b0: [i8; 0];
    static mut DAT_0052a6d4: [i8; 0];
    static mut DAT_0052a6cc: [i8; 0];
    static mut DAT_0052a6bc: [i8; 0];
    static mut DAT_0052a6b0: [i8; 0];
    static mut DAT_0052a6a4: [i8; 0];
    static mut DAT_0052a698: [i8; 0];
    static mut DAT_0052a68c: [i8; 0];
    static mut DAT_0052a678: [i8; 0];
    static mut DAT_0052a680: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004eea80();
    fn FUN_004ea8c0();
    fn FUN_0042b100();
    fn FUN_00426a00(a: *mut core::ffi::c_void);
    fn FUN_00429e70(a: *mut i8);
    fn FUN_0042b010(a: i32, b: i32, c: i32);
    fn FUN_0042b0e0(a: i32, b: i32, c: i32);
    fn FUN_0042b0d0(a: i32);
    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_0042b090(a: i32, b: i32, c: i32);
    fn FUN_0042b250(a: i32);
    fn FUN_0042a380(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004ea090();
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004bf9b0(a: i32) -> i32;
    fn FUN_004e9af0();
    fn FUN_004dad40();
    fn FUN_004dad70(a: *mut i8);
    fn FUN_004da780(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: u32,
        g: i32,
        h: *mut i8,
    ) -> u32;
    fn FUN_004dad60();
    fn FUN_004dd0f0() -> u8;
    fn FUN_004cffb0() -> u32;
    fn FUN_004b4bf0(a: *mut i8);
    fn FUN_004dcb30();
    fn FUN_004dc7e0();
    fn _free(a: *mut core::ffi::c_void);
    fn FUN_0042b1b0();
    fn FUN_004dc170();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e96b0(param_1: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly where the original uses them.
    unsafe {
        let mut local_10: [i32; 4] = [0; 4];
        let l = local_10.as_mut_ptr();
        let s_front = (&raw mut DAT_0052c47c) as *mut i8;
        let s_back = (&raw mut DAT_0052b3b0) as *mut i8;
        let buf = (&raw mut DAT_00b69d20) as *mut i8;

        if param_1 == 0 {
            FUN_004de0b0(0x20, 0x13f);
            DAT_00665d28 = 0;
            DAT_00665d70 = 0;
            DAT_00665d2c = 1;
            DAT_00665d68 = 1;
            DAT_00665d6c = 1;
            DAT_00665d30 = 0;
            DAT_00665d74 = 0x42dc0000;
            DAT_00665d64 = 0x43610000;
            FUN_004eea80();
            FUN_004ea8c0();
            if DAT_00aa60d0 != 0 {
                FUN_0042b100();
            }
            let p = (&raw mut DAT_00665d38) as *mut u32;
            let mut i = 0usize;
            while i < 11 {
                p.add(i).write(0);
                i += 1;
            }
            DAT_00665d38[0] = (DAT_00aa60f4 as *const u8)
                .byte_add((DAT_00665d28.wrapping_mul(0x44)) as usize)
                .byte_add(0x40)
                .cast::<u32>()
                .read_unaligned();
            FUN_00426a00(p as *mut core::ffi::c_void);
            FUN_00429e70(
                (DAT_00665d28
                    .wrapping_mul(0x44)
                    .wrapping_add(DAT_00aa60f4 as i32)) as *mut i8,
            );
            FUN_0042b010(0, 0, 0);
            let i4 = (DAT_00aa60f4 as *const u8)
                .byte_add((DAT_00665d28.wrapping_mul(0x44)) as usize)
                .byte_add(0x40)
                .cast::<i32>()
                .read_unaligned();
            let u6: u32 = if i4 == 6 || i4 == 7 { 100 } else { 0 };
            FUN_0042b0e0(0, u6 as i32, 0);
            FUN_0042b0d0(0);
            FUN_004b4a60(s_front, s_back);
            FUN_004d4e20();
            FUN_004b5de0(s_back, 0, 0);
            FUN_004b5570((&raw mut DAT_0052a6d4) as *mut i8, 0, 0);
            FUN_004b5570((&raw mut DAT_0052a6cc) as *mut i8, 0, 0);
            FUN_004b4d40((&raw mut DAT_0052a6bc) as *mut i8, 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
        }
        FUN_004d6460(l, 0x118, 0x6b, 0x25e, 0x1b1);
        FUN_0042b090(0, 0, 0);
        FUN_0042b250(0);
        FUN_0042a380(
            local_10[0],
            local_10[1],
            local_10[2].wrapping_sub(local_10[0]).wrapping_add(1),
            local_10[3].wrapping_sub(local_10[1]).wrapping_add(1),
            0,
        );
        FUN_004d6460(l, 0x54, 0x5a, 0x25c, 0x6a);
        let pc = FUN_004ddc10(1);
        FUN_004db640(0xf, pc, l, 0xffff);
        FUN_004ea090();
        FUN_004d6460(l, 200, 0x1c4, 0x1b4, 0x1d0);
        let b2e0 = (&raw mut DAT_00b6a2e0) as *mut i8;
        if b2e0.read() != 0 {
            _sprintf(
                buf,
                (&raw mut DAT_0052a6b0) as *const i8,
                6i32,
                (&raw mut DAT_00b6bc9e) as *mut i8,
                1i32,
                b2e0,
            );
            FUN_004db640(0xc, buf, l, DAT_00b6a2c0);
            let u9 = ((param_1 as i32) >> 0x1f) as u32;
            let i4 = ((((param_1 ^ u9).wrapping_sub(u9) & 0x1f) ^ u9).wrapping_sub(u9) as i32) >> 1;
            _sprintf(buf, (&raw mut DAT_0052a6a4) as *const i8, i4);
            FUN_004b5570(buf, 0xcc, 0x1c5);
            _sprintf(buf, (&raw mut DAT_0052a698) as *const i8, i4);
            FUN_004b5570(buf, 0x1a4, 0x1c5);
        }
        if FUN_004bf9b0(3) == 1 {
            return 1;
        }
        FUN_004e9af0();
        FUN_004d6460(l, 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_00bb73a8 != 0 {
            FUN_004dad40();
            let p = FUN_004ddc10(0xce);
            FUN_004dad70(p);
        }
        let pc = FUN_004ddc10(0xce);
        let mut u9 = FUN_004da780(
            l,
            (&raw mut DAT_0052a678) as *mut i8,
            (&raw mut DAT_0052a680) as *mut i8,
            pc,
            0xc,
            0,
            8,
            (&raw mut DAT_0052a68c) as *mut i8,
        );
        FUN_004dad60();
        u9 |= FUN_004dd0f0() as u32;
        if DAT_00b69e30 == 4 {
            u9 |= FUN_004cffb0();
        }
        if u9 != 0 {
            FUN_004b4bf0(s_back);
            FUN_004dcb30();
            FUN_004dc7e0();
            if DAT_00aa60d0 == 0 {
                if !DAT_00aa60f4.is_null() {
                    _free(DAT_00aa60f4);
                    DAT_00aa60f4 = core::ptr::null_mut();
                }
            } else {
                FUN_0042b1b0();
            }
            if !DAT_00aa6110.is_null() {
                _free(DAT_00aa6110);
                DAT_00aa6110 = core::ptr::null_mut();
            }
            FUN_004dc170();
        }
        0
    }
}
