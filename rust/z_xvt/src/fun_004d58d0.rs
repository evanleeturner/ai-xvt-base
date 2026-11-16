// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759673: [u8; 4];
    static mut DAT_007806ee: [u8; 4];
    static mut DAT_0075dc46: [u8; 4];
    static mut DAT_0075dc4e: [u8; 0x20000];
    static mut DAT_0077dc66: [u8; 4];
    static mut DAT_0077dc6a: [u8; 4];
    static mut DAT_007806f2: [u8; 4];
    static mut DAT_0077e0a2: [u8; 0];
    static mut DAT_0077e0a6: [u8; 0];
    static mut DAT_00758fd0: [u8; 256];
    static mut DAT_00758830: i32;
    static mut DAT_00758d59: i8;
    static mut DAT_00758f2e: [u8; 32];
    static mut DAT_00758f4e: [u8; 32];
    static mut DAT_00758ee0: u8;
    static mut DAT_00758ee1: u8;
    static mut DAT_00759b0c: i32;
    static mut DAT_00759b18: u32;
    static mut DAT_00759b1c: i32;
    static mut DAT_00759b14: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4dc0();
    fn FUN_004df050(param_1: i32);
    fn FUN_004ca990();
    fn FUN_004d4d00() -> i32;
    fn FUN_004d4d90();
    fn FUN_004dd860();
    fn FUN_004cfbc0(param_1: i32, param_2: i32, param_3: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d5280() -> u32;
    fn GetTickCount() -> u32;
    fn PeekMessageA(msg: *mut c_void, hwnd: *mut c_void, min: u32, max: u32, remove: u32) -> i32;
    fn GetMessageA(msg: *mut c_void, hwnd: *mut c_void, min: u32, max: u32) -> i32;
    fn TranslateMessage(msg: *const c_void) -> i32;
    fn DispatchMessageA(msg: *const c_void) -> i32;
    fn GetKeyboardState(keys: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d58d0() -> u32 {
    // SAFETY: all globals, callees and Win32 imports are the game's own;
    // the unaligned table/field accesses are done where the original does.
    unsafe {
        let mut local_1c = [0u32; 7];
        let msg = local_1c.as_mut_ptr() as *mut c_void;
        let mut b_var2 = false;
        let mut local_24 = GetTickCount();
        let mut local_20 = local_24;
        loop {
            loop {
                let tmp = (&raw mut DAT_00759673).cast::<i32>().read_unaligned();
                if tmp != 0 {
                    break;
                }
                if PeekMessageA(msg, core::ptr::null_mut(), 0, 0, 0) != 0 {
                    if GetMessageA(msg, core::ptr::null_mut(), 0, 0) == 0 {
                        return 2;
                    }
                    TranslateMessage(msg);
                    DispatchMessageA(msg);
                }
            }
            if b_var2 {
                break;
            }
            if PeekMessageA(msg, core::ptr::null_mut(), 0, 0, 0) == 0 {
                let d = GetTickCount();
                let tmp = (&raw mut DAT_007806ee).cast::<i32>().read_unaligned();
                if tmp <= d.wrapping_sub(local_24) as i32 {
                    b_var2 = true;
                    FUN_004d4dc0();
                    local_24 = d;
                }
                if 99 < d.wrapping_sub(local_20) as i32 {
                    FUN_004df050(0);
                    FUN_004df050(1);
                    local_20 = d;
                }
            } else {
                if GetMessageA(msg, core::ptr::null_mut(), 0, 0) == 0 {
                    return 2;
                }
                TranslateMessage(msg);
                DispatchMessageA(msg);
            }
        }

        let zero: i32 = 0;
        let tmp = (&raw mut DAT_0075dc46).cast::<i32>().read_unaligned();
        if tmp != 0 {
            let p = (&raw mut DAT_0075dc4e).cast::<u8>();
            let mut i = 0usize;
            while i != 0x20000 {
                p.add(i).write(0);
                i += 1;
            }
        }
        FUN_004ca990();

        let tmp = (&raw mut DAT_0077dc66).cast::<i32>().read_unaligned();
        let mut idx = (tmp as u32).wrapping_mul(0x440);
        let mut pc8 = (&raw mut DAT_0077e0a2)
            .cast::<u8>()
            .byte_add(idx as usize)
            .cast::<*const c_void>()
            .read_unaligned();
        if !pc8.is_null() {
            GetKeyboardState((&raw mut DAT_00758fd0).cast::<u8>());
            DAT_00758830 = FUN_004d4d00();
            let tmp = (&raw mut DAT_0077dc66).cast::<i32>().read_unaligned();
            idx = (tmp as u32).wrapping_mul(0x440);
            let pc1 = (&raw mut DAT_0077e0a6)
                .cast::<u8>()
                .byte_add(idx as usize)
                .cast::<*const c_void>()
                .read_unaligned();
            pc8 = (&raw mut DAT_0077e0a2)
                .cast::<u8>()
                .byte_add(idx as usize)
                .cast::<*const c_void>()
                .read_unaligned();
            let tmp = (&raw mut DAT_007806f2).cast::<i32>().read_unaligned();
            let f8: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(pc8);
            let i_var5 = f8(tmp);
            let tmp = (&raw mut DAT_0077dc6a).cast::<i32>().read_unaligned();
            if tmp == 1 || i_var5 == 1 {
                (&raw mut DAT_0077dc6a).cast::<i32>().write_unaligned(zero);
                if !pc1.is_null() {
                    let tmp = (&raw mut DAT_007806f2).cast::<i32>().read_unaligned();
                    let f1: unsafe extern "C" fn(i32) = core::mem::transmute(pc1);
                    f1(tmp);
                }
            }
            FUN_004d4d90();
            if DAT_00758d59 == 1 {
                FUN_004dd860();
            }
            let p = (&raw mut DAT_00758f2e).cast::<u8>();
            let mut i = 0usize;
            while i != 32 {
                p.add(i).write(0);
                i += 1;
            }
            let p = (&raw mut DAT_00758f4e).cast::<u8>();
            let mut i = 0usize;
            while i != 32 {
                p.add(i).write(0);
                i += 1;
            }
            let u = (&raw mut DAT_007806f2).cast::<u32>().read_unaligned();
            (&raw mut DAT_007806f2)
                .cast::<u32>()
                .write_unaligned(u.wrapping_add(1));
            if i_var5 == 1 {
                return 1;
            }
            let u = (&raw mut DAT_0075dc46).cast::<u32>().read_unaligned();
            if u != 0 {
                (&raw mut DAT_0075dc46)
                    .cast::<u32>()
                    .write_unaligned(u.wrapping_sub(1));
            }
            DAT_00758ee0 = 0;
            DAT_00758ee1 = 0;
            if DAT_00759b0c != 0 {
                let d = FUN_004d5280();
                if DAT_00759b18 < d {
                    if DAT_00759b1c != 0 {
                        FUN_004cfbc0(DAT_00759b0c, 0, 0);
                        return 0;
                    }
                    DAT_00759b14 = 1;
                }
            }
        }
        0
    }
}
