// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0077dc4e: [u8; 4];
    static mut DAT_00759ad4: i32;
    static mut DAT_00759ad8: i32;
    static mut DAT_00759adc: i32;
    static mut DAT_00759ae0: i32;
    static mut DAT_00759ae4: i32;
    static mut DAT_00759ae8: i32;
    static mut DAT_00759673: [u8; 4];
    static mut DAT_00780736: [u8; 4];
    static mut DAT_0075dc46: [u8; 4];
    static mut DAT_0075dc4e: [u8; 0x20000];
    static mut DAT_0077e0a2: [u8; 0];
    static mut DAT_0077dc66: [u8; 4];
    static mut DAT_0077dc6a: [u8; 4];
    static mut DAT_007806f2: [u8; 4];
    static mut DAT_0077dc52: [u8; 4];
    static mut DAT_0077dc56: [u8; 0];
    static mut DAT_00758d59: u8;
    static mut DAT_00758fd0: [u8; 0];
    static mut DAT_00758830: i32;
    static mut DAT_00758f2e: [u8; 32];
    static mut DAT_00758f4e: [u8; 32];
    static mut DAT_00759b08: i32;
    static mut DAT_00759b0c: i32;
    static mut DAT_00759b14: i32;
    static mut DAT_00759b34: i32;
    static mut DAT_0075965e: [u8; 4];
    static mut DAT_00758ee0: u8;
    static mut DAT_00758ee1: u8;
    static mut DAT_00759b2c: i32;
    static mut DAT_00759b28: u32;
    static mut DAT_00759b18: u32;
    static mut DAT_00759b1c: i32;
    static mut DAT_007806ee: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4600(param_1: *mut c_void) -> i32;
    fn FUN_004d3ca0(param_1: i32);
    fn FUN_004df760(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004ca990();
    fn FUN_004d4d00() -> i32;
    fn FUN_004d4d90();
    fn FUN_004db790(param_1: i32, param_2: *mut u8) -> i32;
    fn FUN_004dd860();
    fn FUN_004d00f0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_004cfd60();
    fn FUN_004cfff0();
    fn FUN_004cfbc0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_004d4dc0();
    fn FUN_004df050(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
    fn PeekMessageA(msg: *mut u32, hwnd: *mut c_void, min: u32, max: u32, remove: u32) -> i32;
    fn GetMessageA(msg: *mut u32, hwnd: *mut c_void, min: u32, max: u32) -> i32;
    fn TranslateMessage(msg: *const u32) -> i32;
    fn DispatchMessageA(msg: *const u32) -> i32;
    fn GetKeyboardState(keys: *mut u8) -> i32;
    fn PostMessageA(hwnd: *mut c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
}

unsafe fn rd32(p: *const u8) -> i32 {
    // SAFETY: caller passes the address of a 4-byte game global.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr32(p: *mut u8, v: i32) {
    // SAFETY: caller passes the address of a 4-byte game global.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d4250(param_1: *mut c_void) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers are
    // addresses of game globals accessed with the original's widths.
    unsafe {
        let mut i_var6: i32 = 0;
        let mut b_var2: i32 = 0;
        let mut msg: [u32; 7] = [0; 7];

        if FUN_004d4600(param_1) == 0 {
            return 0;
        }
        let init = rd32((&raw mut DAT_0077dc4e).cast::<u8>());
        if init != 0 {
            let f: unsafe extern "C" fn() -> i32 =
                core::mem::transmute(init as usize as *const c_void);
            if f() != 0 {
                FUN_004d3ca0(1);
                return 0;
            }
        }
        let mut local_24: u32 = GetTickCount();
        DAT_00759ad4 = 0xffff;
        DAT_00759ad8 = FUN_004df760(0x40, 0xc4, 0x40);
        DAT_00759adc = FUN_004df760(0xff, 0, 0);
        DAT_00759ae0 = FUN_004df760(0xff, 0xff, 0);
        DAT_00759ae4 = FUN_004df760(0, 0, 0xff);
        DAT_00759ae8 = FUN_004df760(0x80, 0x80, 0xff);
        let mut local_20: u32 = local_24;
        let table = (&raw mut DAT_0077e0a2).cast::<u8>();
        loop {
            loop {
                loop {
                    if rd32((&raw mut DAT_00759673).cast::<u8>()) != 0 {
                        break;
                    }
                    if PeekMessageA(msg.as_mut_ptr(), core::ptr::null_mut(), 0, 0, 0) != 0 {
                        if GetMessageA(msg.as_mut_ptr(), core::ptr::null_mut(), 0, 0) == 0 {
                            return msg[2];
                        }
                        TranslateMessage(msg.as_ptr());
                        DispatchMessageA(msg.as_ptr());
                    }
                }
                if b_var2 != 0 && i_var6 == 0 {
                    b_var2 = 0;
                    wr32((&raw mut DAT_00780736).cast::<u8>(), 0);
                    if rd32((&raw mut DAT_0075dc46).cast::<u8>()) != 0 {
                        core::ptr::write_bytes((&raw mut DAT_0075dc4e).cast::<u8>(), 0, 0x20000);
                    }
                    FUN_004ca990();
                    let idx = rd32((&raw mut DAT_0077dc66).cast::<u8>()) as u32;
                    let off = idx.wrapping_mul(0x440) as usize;
                    if rd32(table.byte_add(off)) != 0 {
                        GetKeyboardState((&raw mut DAT_00758fd0).cast::<u8>());
                        DAT_00758830 = FUN_004d4d00();
                        let idx = rd32((&raw mut DAT_0077dc66).cast::<u8>()) as u32;
                        let pc = rd32(table.byte_add(4).byte_add(idx.wrapping_mul(0x440) as usize));
                        let idx = rd32((&raw mut DAT_0077dc66).cast::<u8>()) as u32;
                        let pf = rd32(table.byte_add(idx.wrapping_mul(0x440) as usize));
                        let p = rd32((&raw mut DAT_007806f2).cast::<u8>());
                        let frame: unsafe extern "C" fn(i32) -> i32 =
                            core::mem::transmute(pf as usize as *const c_void);
                        i_var6 = frame(p);
                        let a = rd32((&raw mut DAT_0077dc6a).cast::<u8>());
                        if a == 1 || i_var6 == 1 {
                            wr32((&raw mut DAT_0077dc6a).cast::<u8>(), 0);
                            if pc != 0 {
                                let p = rd32((&raw mut DAT_007806f2).cast::<u8>());
                                let h: unsafe extern "C" fn(i32) =
                                    core::mem::transmute(pc as usize as *const c_void);
                                h(p);
                            }
                        }
                        FUN_004d4d90();
                        if rd32((&raw mut DAT_0077dc52).cast::<u8>()) != 0 {
                            let t = rd32((&raw mut DAT_0077dc52).cast::<u8>());
                            FUN_004db790(t, (&raw mut DAT_0077dc56).cast::<u8>());
                            wr32((&raw mut DAT_0077dc52).cast::<u8>(), 0);
                        }
                        if DAT_00758d59 == 1 {
                            FUN_004dd860();
                        }
                        core::ptr::write_bytes((&raw mut DAT_00758f2e).cast::<u8>(), 0, 32);
                        core::ptr::write_bytes((&raw mut DAT_00758f4e).cast::<u8>(), 0, 32);
                        let p = rd32((&raw mut DAT_007806f2).cast::<u8>());
                        wr32((&raw mut DAT_007806f2).cast::<u8>(), p.wrapping_add(1));
                        if i_var6 == 1 {
                            if DAT_00759b08 != 0 && DAT_00759b0c != 0 && DAT_00759b14 == 0 {
                                FUN_004d00f0(DAT_00759b34, 0x200, 2000);
                            }
                            FUN_004cfd60();
                            let hwnd =
                                rd32((&raw mut DAT_0075965e).cast::<u8>()) as usize as *mut c_void;
                            PostMessageA(hwnd, 0x10, 0, 0);
                        }
                        let t = rd32((&raw mut DAT_0075dc46).cast::<u8>());
                        if t != 0 {
                            wr32((&raw mut DAT_0075dc46).cast::<u8>(), t.wrapping_sub(1));
                        }
                        DAT_00758ee0 = 0;
                        DAT_00758ee1 = 0;
                        if DAT_00759b2c == 2 {
                            let d = GetTickCount();
                            if DAT_00759b28 < d {
                                FUN_004cfff0();
                            }
                        }
                        if DAT_00759b0c != 0 {
                            let d = GetTickCount();
                            if DAT_00759b18 < d {
                                if DAT_00759b1c == 0 {
                                    DAT_00759b14 = 1;
                                } else {
                                    FUN_004cfbc0(DAT_00759b0c, 0, 0);
                                }
                            }
                        }
                    }
                }
                if PeekMessageA(msg.as_mut_ptr(), core::ptr::null_mut(), 0, 0, 0) != 0 {
                    break;
                }
                if i_var6 == 0 {
                    let d = GetTickCount();
                    let b = rd32((&raw mut DAT_007806ee).cast::<u8>());
                    if b <= d.wrapping_sub(local_24) as i32 {
                        b_var2 = 1;
                        FUN_004d4dc0();
                        local_24 = d;
                    }
                    if 99 < d.wrapping_sub(local_20) as i32 {
                        FUN_004df050(0);
                        FUN_004df050(1);
                        local_20 = d;
                    }
                }
            }
            if GetMessageA(msg.as_mut_ptr(), core::ptr::null_mut(), 0, 0) == 0 {
                break;
            }
            TranslateMessage(msg.as_ptr());
            DispatchMessageA(msg.as_ptr());
        }
        msg[2]
    }
}
