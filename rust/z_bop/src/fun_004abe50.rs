// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527ea0: i32;
    static mut DAT_00527f78: i32;
    static mut DAT_00527eac: i32;
    static mut _DAT_009ed23b: i16;
    static mut DAT_00527ed0: i32;
    static mut DAT_0066ddcc: *mut *mut c_void;
    static mut DAT_00803f80: *mut *mut c_void;
    static mut DAT_0066e700: *mut *mut c_void;
    static mut DAT_00527ecc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004abe50() {
    // SAFETY: the globals are the game's own memory; the object pointers are
    // the game's live objects whose first word is a vtable pointer, and the
    // slot at offset 0x80 holds the function the original calls.
    unsafe {
        if DAT_00527ea0 != 1 {
            if DAT_00527f78 > 1 {
                DAT_00527f78 = DAT_00527f78.wrapping_sub(1);
                return;
            }
            if DAT_00527f78 < 1 {
                DAT_00527f78 = 0;
                return;
            }
            DAT_00527f78 = DAT_00527f78.wrapping_sub(1);
            if DAT_00527eac != 0 {
                if _DAT_009ed23b != 0 {
                    if DAT_00527ed0 != 0 {
                        let obj = DAT_0066ddcc;
                        let vt = (*obj) as *const u8;
                        let f = vt.byte_add(0x80).cast::<*const c_void>().read_unaligned();
                        let f: unsafe extern "C" fn(*mut *mut c_void) = core::mem::transmute(f);
                        f(obj);
                        return;
                    }
                    let obj = DAT_00803f80;
                    let vt = (*obj) as *const u8;
                    let f = vt.byte_add(0x80).cast::<*const c_void>().read_unaligned();
                    let f: unsafe extern "C" fn(*mut *mut c_void, i32) = core::mem::transmute(f);
                    f(obj, DAT_00527ecc);
                    return;
                }
                let obj = DAT_0066e700;
                let vt = (*obj) as *const u8;
                let f = vt.byte_add(0x80).cast::<*const c_void>().read_unaligned();
                let f: unsafe extern "C" fn(*mut *mut c_void, i32) = core::mem::transmute(f);
                f(obj, DAT_00527ecc);
            }
        }
    }
}
