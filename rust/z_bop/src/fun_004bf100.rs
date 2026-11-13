// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69cc0: *mut c_void;
    static mut DAT_00b69e28: *mut *mut c_void;
    static mut DAT_0052b35c: i8;
    static mut DAT_0051a854: i32;
    static mut DAT_00b6a2e0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _free(p: *mut c_void);
    fn FUN_004dfc30(p: *mut c_void);
    fn FUN_004dfa10(p: *const i8) -> *mut c_void;
    fn _malloc(n: u32) -> *mut c_void;
    fn FUN_004cc4a0(name: *const i8, p: *mut c_void) -> i32;
    fn FUN_004cc6c0(h: i32, buf: *mut c_void, n: u32) -> i32;
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_004cc590(h: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ccf50();
    fn FUN_004ccf70();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bf100(param_1: *mut i32) -> i32 {
    // SAFETY: all globals and callees are the game's own; pointers read from
    // the game's structures are used exactly as the original does.
    unsafe {
        if !DAT_00b69cc0.is_null() {
            _free(DAT_00b69cc0);
            DAT_00b69cc0 = core::ptr::null_mut();
        }
        if !DAT_00b69e28.is_null() {
            FUN_004dfc30(DAT_00b69e28 as *mut c_void);
            DAT_00b69e28 = core::ptr::null_mut();
        }
        FUN_004ccf50();
        DAT_00b69e28 = FUN_004dfa10(&raw mut DAT_0052b35c as *const i8) as *mut *mut c_void;
        FUN_004ccf70();
        if DAT_00b69e28.is_null() {
            return 0;
        }
        let mut pu5 = DAT_00b69e28.read_unaligned() as *mut *mut c_void;
        if (DAT_00b69e28.add(1).read_unaligned() as i32) < 1 {
            DAT_00b69cc0 = core::ptr::null_mut();
        } else {
            DAT_00b69cc0 = _malloc((DAT_00b69e28.add(1).read_unaligned() as u32).wrapping_mul(0xe));
            let i_var4 = DAT_00b69e28.add(1).read_unaligned() as i32;
            let mut p6 = DAT_00b69cc0 as *mut u8;
            let n = i_var4.wrapping_mul(0xe);
            let mut u_var3 = (n as u32) >> 2;
            while u_var3 != 0 {
                (p6 as *mut i32).write_unaligned(0);
                p6 = p6.byte_add(4);
                u_var3 = u_var3.wrapping_sub(1);
            }
            let mut u_var3 = (n & 3) as u32;
            while u_var3 != 0 {
                p6.write(0);
                p6 = p6.byte_add(1);
                u_var3 = u_var3.wrapping_sub(1);
            }
        }
        if DAT_00b69cc0.is_null() {
            return 0;
        }
        let mut i_var4: i32 = 0;
        if !pu5.is_null() {
            let mut i_var7: i32 = 0;
            loop {
                let h = FUN_004cc4a0(
                    pu5.read_unaligned() as *const i8,
                    &raw mut DAT_0051a854 as *mut c_void,
                );
                if h != 0 {
                    let base = (DAT_00b69cc0 as *mut u8).byte_add(i_var7 as usize);
                    FUN_004cc6c0(h, base as *mut c_void, 0xc);
                    base.byte_add(0xc).write(0);
                    let r = __strcmpi(base as *const i8, &raw mut DAT_00b6a2e0 as *const i8);
                    if r == 0 {
                        param_1.write_unaligned(i_var4);
                    }
                    i_var7 = i_var7.wrapping_add(0xe);
                    i_var4 = i_var4.wrapping_add(1);
                    FUN_004cc590(h);
                }
                pu5 = pu5.add(1).read_unaligned() as *mut *mut c_void;
                if pu5.is_null() {
                    break;
                }
            }
        }
        1
    }
}
