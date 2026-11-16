// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665d38: i32;
    static mut DAT_00665d3c: i32;
    static mut DAT_00665d40: i32;
    static mut DAT_00665d44: i32;
    static mut DAT_00665d48: i32;
    static mut DAT_00665d4c: i32;
    static mut DAT_00665d50: i32;
    static mut DAT_00665d54: i32;
    static mut DAT_00665d58: i32;
    static mut DAT_00665d5c: i32;
    static mut DAT_00aa6110: i32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00528118: [i8; 1];
    static mut DAT_00523d40: [i8; 1];
    static mut s__c_s__c_s_0052a6b0: [i8; 1];
    static mut s__c_s__c__1f__s_0052c508: [i8; 1];
    static mut s__c_s__c_d_0052c518: [i8; 1];
    static mut s__c_s__c_0052c524: [i8; 1];
    static mut s__c_s__c_d__s_0052c52c: [i8; 1];
    static mut _DAT_00518308: f64;
    static mut _DAT_00518310: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dbbd0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_0042b270() -> i64;
}

unsafe fn buf() -> *mut i8 {
    // SAFETY: address of the game's own text buffer.
    unsafe { (&raw mut DAT_00b69d20).cast::<i8>() }
}

unsafe fn cat(src: *const i8) {
    // SAFETY: src is a NUL-terminated string and the game buffer is large
    // enough, as in the original inlined strcat.
    unsafe {
        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut d = buf();
        while *d != 0 {
            d = d.add(1);
        }
        let mut i = 0;
        while i < n {
            *d.add(i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ea090() -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers passed
    // refer to local arrays and the game's buffers.
    unsafe {
        let mut local_28: [i32; 4] = [0; 4];
        let mut rect: [i32; 4] = [0; 4];
        let l28 = local_28.as_mut_ptr();
        let fmt3 = (&raw mut s__c_s__c_s_0052a6b0).cast::<i8>();
        let fmtds = (&raw mut s__c_s__c_d__s_0052c52c).cast::<i8>();
        let sep = (&raw mut DAT_00523d40).cast::<i8>();

        FUN_004d6460(l28, 0x58, 0x6f, 0x14c, 0x7d);
        let mut iv7 = (DAT_00665d38 as u32).wrapping_sub(1) as i32;
        if iv7 < 0 {
            iv7 = 0;
        }
        iv7 = iv7.wrapping_mul(0x200);
        _sprintf(
            buf(),
            (&raw mut DAT_00528118).cast::<i8>(),
            DAT_00aa6110.wrapping_add(iv7),
        );
        FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
        FUN_004d64a0(l28, 0, 0x1e);
        let p2 = FUN_004ddc10(DAT_00665d3c.wrapping_add(0x1f0));
        let p3 = FUN_004ddc10(0x1e3);
        _sprintf(buf(), fmt3, 4i32, p3, 1u32, p2);
        FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
        FUN_004d64a0(l28, 0, 0xf);
        let i4 = DAT_00aa6110.wrapping_add(iv7).wrapping_add(0x40);
        let p2 = FUN_004ddc10(0x1e4);
        _sprintf(buf(), fmt3, 4i32, p2, 1u32, i4);
        FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
        FUN_004d64a0(l28, 0, 0xf);
        let i4 = DAT_00aa6110.wrapping_add(iv7).wrapping_add(0x80);
        let p2 = FUN_004ddc10(0x1e5);
        _sprintf(buf(), fmt3, 4i32, p2, 1u32, i4);
        FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
        FUN_004d64a0(l28, 0, 0xf);
        let u5 = DAT_00b6a2c0;
        let p2 = FUN_004ddc10(0x1e6);
        FUN_004db7d0(0xc, p2, l28, 0, 1, u5);
        FUN_004d64a0(l28, 0, 0xf);
        FUN_004d6480(rect.as_mut_ptr(), l28);
        rect[3] = rect[1].wrapping_add(0x78);
        rect[0] = rect[0].wrapping_add(0x14);
        let i4 = FUN_004dbbd0(
            0xc,
            DAT_00aa6110.wrapping_add(iv7).wrapping_add(0xc0) as usize as *mut i8,
            rect.as_mut_ptr(),
            0xffff,
            3,
            0,
        );
        FUN_004d64a0(l28, 0, i4.wrapping_mul(3).wrapping_add(6).wrapping_mul(5));

        if DAT_00665d3c == 0 {
            let p2 = FUN_004ddc10(0x201);
            let p3 = FUN_004ddc10(0x1e7);
            _sprintf(buf(), fmtds, 4i32, p3, 1u32, DAT_00665d40, p2);
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
            let p2 = FUN_004ddc10(0x202);
            let p3 = FUN_004ddc10(0x1e8);
            _sprintf(buf(), fmtds, 4i32, p3, 1u32, DAT_00665d44, p2);
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
            let p2 = FUN_004ddc10(0x2ca);
            let p3 = FUN_004ddc10(0x1e9);
            _sprintf(buf(), fmtds, 4i32, p3, 1u32, DAT_00665d48, p2);
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
            let p2 = FUN_004ddc10(0x1ea);
            _sprintf(
                buf(),
                (&raw mut s__c_s__c_0052c524).cast::<i8>(),
                4i32,
                p2,
                1u32,
            );

            let do58f = if DAT_00665d4c == 0 {
                true
            } else {
                cat(FUN_004ddc10(DAT_00665d4c.wrapping_add(0x205)));
                cat(sep);
                cat(FUN_004ddc10(0x204));
                if DAT_00665d50 != 0 {
                    cat(sep);
                    cat(FUN_004ddc10(0x203));
                    cat(sep);
                    true
                } else {
                    false
                }
            };
            if do58f && DAT_00665d50 != 0 {
                cat(FUN_004ddc10(DAT_00665d50.wrapping_add(0x205)));
                cat(sep);
                cat(FUN_004ddc10(0x205));
            }
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
            let p2 = FUN_004ddc10(0x1eb);
            _sprintf(
                buf(),
                (&raw mut s__c_s__c_d_0052c518).cast::<i8>(),
                4i32,
                p2,
                1u32,
                DAT_00665d54,
            );
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
        } else if DAT_00665d3c != 8 && DAT_00665d3c != 9 {
            let l9 = FUN_0042b270();
            let mut d = (l9 as i32) as f64;
            let u5: i32;
            if d < _DAT_00518308 {
                u5 = 0x2c8;
            } else {
                d *= _DAT_00518310;
                u5 = 0x20a;
            }
            let p2 = FUN_004ddc10(u5);
            let p3 = FUN_004ddc10(0x1ec);
            _sprintf(
                buf(),
                (&raw mut s__c_s__c__1f__s_0052c508).cast::<i8>(),
                4i32,
                p3,
                1u32,
                d,
                p2,
            );
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
            let i7 = DAT_00aa6110.wrapping_add(iv7).wrapping_add(0x1c0);
            let p2 = FUN_004ddc10(0x1ed);
            _sprintf(buf(), fmt3, 4i32, p2, 1u32, i7);
            FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
            FUN_004d64a0(l28, 0, 0xf);
        }

        let p2 = FUN_004ddc10(0x2cb);
        let p3 = FUN_004ddc10(0x1ee);
        _sprintf(buf(), fmtds, 4i32, p3, 1u32, DAT_00665d58, p2);
        FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
        FUN_004d64a0(l28, 0, 0xf);
        let p2 = FUN_004ddc10(0x2c9);
        let p3 = FUN_004ddc10(0x1ef);
        _sprintf(buf(), fmtds, 4i32, p3, 1u32, DAT_00665d5c, p2);
        FUN_004db7d0(0xc, buf(), l28, 0, 1, 0xffff);
        FUN_004d64a0(l28, 0, 0xf);
        1
    }
}
