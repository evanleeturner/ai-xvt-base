// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb2818: i32;
    static mut DAT_00b6a250: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00bb737f: i8;
    static mut DAT_00bb737e: u8;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00b6a5aa: u8;
    static mut DAT_00b6a5ae: u8;
    static mut DAT_00b6bc32: u8;
    static mut DAT_00aa6114: *mut core::ffi::c_void;
    static mut DAT_00aa6138: u32;
    static mut DAT_00aa6144: u32;
    static mut DAT_0052c1e8: [*mut i8; 1];
    static mut DAT_00b6d58a: i8;
    static mut DAT_00b6a24c: i32;
    static mut DAT_00b6a2e0: i8;
    static mut DAT_00b6a5c6: i8;
    static mut DAT_00b6a2c8: u32;
    static mut DAT_0066da78: i8;
    static mut DAT_00b6bc9e: i8;
    static mut DAT_0052d598: [i8; 1];
    static mut DAT_0052d5cc: [i8; 1];
    static mut DAT_0052b12c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ba3c0();
    fn FUN_004bf650(a: i32);
    fn FUN_004de100();
    fn FUN_004cc9f0(a: i32) -> i32;
    fn FUN_004d2e50();
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8) -> i32;
    fn FUN_004de110();
    fn FUN_004c9ee0();
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
    fn FUN_004cd9d0();
    fn FUN_004dc330(a: *const core::ffi::c_void, b: *const core::ffi::c_void);
    fn FUN_004d32a0() -> i32;
    fn FUN_004d3520(a: u32, b: u32, c: i32);
    fn FUN_004d5bc0();
    fn FUN_004d56b0();
    fn FUN_004d5b70(a: i32);
    fn FUN_004e5000(a: i32);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn free(p: *mut core::ffi::c_void);
    fn FUN_004a9c00(a: *mut i8) -> i32;
    fn FUN_004d5760();
    fn FUN_004df700(a: *const i8);
    fn FUN_004d3330();
    fn FUN_004d3020(a: i32, b: i32, c: i32);
    fn FUN_004d34a0(a: u32);
    fn FUN_004be1b0();
    fn FUN_004be050();
    fn FUN_004fe160();
    fn FUN_004fe100();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_005066a0(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly as the original does.
    unsafe {
        let mut i_var11: i32 = 0;
        if param_1 == 0 {
            FUN_004ba3c0();
            FUN_004bf650(0);
            loop {
                FUN_004de100();
                let i_var2 = FUN_004cc9f0(DAT_00bb2818);
                if i_var2 != 0 {
                    break;
                }
                FUN_004d2e50();
                let pc3 = FUN_004ddc10(0x13);
                let pc4 = FUN_004ddc10(0x20b);
                let pc5 = FUN_004ddc10(0x2c4);
                let pc6 = FUN_004ddc10(0x2c3);
                let pc7 = FUN_004ddc10(0x2c2);
                i_var11 = FUN_004dcb90(pc7, pc6, pc5, pc4, pc3);
                if i_var11 == 0 {
                    FUN_004de100();
                    return 1;
                }
            }
            FUN_004de110();
            FUN_004c9ee0();
            if DAT_00b6a250 == 0 && DAT_00b69e30 != 3 {
                FUN_004de100();
                let pc7: *mut i8 = core::ptr::null_mut();
                let pc6: *mut i8 = core::ptr::null_mut();
                let pc3 = FUN_004ddc10(0x2ef);
                let pc4 = FUN_004ddc10(0x2ee);
                let pc5 = FUN_004ddc10(0x2ed);
                FUN_004dcb90(pc5, pc4, pc3, pc6, pc7);
                if DAT_00b69e30 != 2 {
                    DAT_00aa6af0 = 0x46;
                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 4);
                    FUN_004cd9d0();
                }
                DAT_00b69e30 = 0;
                FUN_004dc330(
                    FUN_004be1b0 as *const core::ffi::c_void,
                    FUN_004be050 as *const core::ffi::c_void,
                );
                return 0;
            }
            if DAT_00bb737f != 0 {
                let i_var2 = FUN_004d32a0();
                if i_var2 == 0 {
                    let u8_ = (DAT_00bb737e as u32).wrapping_mul(0xffff) / 9;
                    FUN_004d3520(u8_, u8_ >> 3, 1000);
                }
            }
            FUN_004d5bc0();
            FUN_004d56b0();
            FUN_004d5b70(1);
            let i_train: i32 = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();
            FUN_004e5000(i_train);
            if !DAT_00aa6114.is_null() {
                let mut u_var8: u32 = 0;
                if DAT_00aa6138 != 0 {
                    let mut pi10 = DAT_00aa6114.byte_add(0x140).cast::<i32>();
                    loop {
                        let i_tmp: i32 = (&raw mut DAT_00b6a5ae)
                            .byte_add((i_train as u32).wrapping_mul(4) as usize)
                            .cast::<i32>()
                            .read_unaligned();
                        if pi10.read_unaligned() == i_tmp {
                            break;
                        }
                        pi10 = pi10.wrapping_add(0x52);
                        u_var8 = u_var8.wrapping_add(1);
                        if !(u_var8 < DAT_00aa6138) {
                            break;
                        }
                    }
                }
                let base = &raw mut DAT_0052c1e8 as *mut *mut i8;
                let pu12 = base
                    .byte_add((i_train as u32).wrapping_mul(4) as usize)
                    .read_unaligned();
                let pc3: *const i8 = if DAT_00b69cb0 == 0 {
                    (&raw const DAT_0052d598).cast::<i8>()
                } else {
                    (&raw const DAT_0052d5cc).cast::<i8>()
                };
                let rec = (u_var8.wrapping_mul(0x148).wrapping_add(DAT_00aa6114 as u32))
                    as *mut core::ffi::c_void;
                let name = (&raw mut DAT_00b6d58a)
                    .byte_add((DAT_00b6a24c as u32).wrapping_mul(0x16) as usize);
                sprintf(
                    &raw mut DAT_0066da78,
                    pc3,
                    pu12,
                    rec,
                    name,
                    &raw mut DAT_00b6a2e0,
                    DAT_00aa6144,
                    &raw mut DAT_00b6a5c6,
                    DAT_00b6a2c8,
                );
                free(DAT_00aa6114);
                DAT_00aa6114 = core::ptr::null_mut();
                i_var11 = FUN_004a9c00(&raw mut DAT_0066da78);
            }
            FUN_004d5760();
            FUN_004d5b70(0);
            FUN_004df700((&raw const DAT_0052b12c).cast::<i8>());
            FUN_004ba3c0();
            let i_tmp: i32 = (&raw mut DAT_00b6bc32).cast::<i32>().read_unaligned();
            let src = FUN_004ddc10((i_tmp as u32).wrapping_add(0x7a) as i32);
            let mut len: u32 = 0;
            while src.add(len as usize).read() != 0 {
                len = len.wrapping_add(1);
            }
            len = len.wrapping_add(1);
            let mut ps = src as *const i8;
            let mut pd = &raw mut DAT_00b6bc9e;
            let mut n = len >> 2;
            while n != 0 {
                pd.cast::<u32>()
                    .write_unaligned(ps.cast::<u32>().read_unaligned());
                ps = ps.add(4);
                pd = pd.add(4);
                n -= 1;
            }
            let mut n = len & 3;
            while n != 0 {
                *pd = *ps;
                ps = ps.add(1);
                pd = pd.add(1);
                n -= 1;
            }
            FUN_004bf650(0);
            FUN_004d2e50();
            FUN_004d3330();
            if DAT_00bb737f != 0 {
                FUN_004d3020(7, 0, 0);
                FUN_004d34a0((DAT_00bb737e as u32).wrapping_mul(0xffff) / 9);
            }
        }
        if i_var11 == 0 {
            FUN_004de100();
            FUN_004cd9d0();
            FUN_004dc330(
                FUN_004be1b0 as *const core::ffi::c_void,
                FUN_004be050 as *const core::ffi::c_void,
            );
            return 0;
        }
        FUN_004dc330(
            FUN_004fe160 as *const core::ffi::c_void,
            FUN_004fe100 as *const core::ffi::c_void,
        );
        0
    }
}
