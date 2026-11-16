// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b3e: [u8; 4];
    static mut DAT_00aa7b26: [u8; 4];
    static mut DAT_00aa7b96: i8;
    static mut DAT_00aa7aa0: i8;
    static mut DAT_00aa739e: i8;
    static mut DAT_00aa739f: i8;
    static mut DAT_00aa73a0: i8;
    static mut DAT_00aa73a1: i8;
    static mut DAT_00aa7209: [u8; 4];
    static mut DAT_00aa720d: [u8; 4];
    static mut DAT_00aa7598: [u32; 0x40];
    static mut DAT_00aa7698: [i8; 0x400];
    static mut DAT_00aa7a98: u32;
    static mut DAT_00aa7a9c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3350();
    fn FUN_004d3400();
    fn FUN_004d3820(param_1: i32);
    fn FUN_004d5380();
    fn FUN_004de150();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ReleaseCapture() -> i32;
    fn SetCapture(h: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn PostQuitMessage(code: i32);
    fn PostMessageA(h: *mut core::ffi::c_void, msg: u32, w: u32, l: i32) -> i32;
    fn SetCursor(h: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn SetCursorPos(x: i32, y: i32) -> i32;
    fn DefWindowProcA(h: *mut core::ffi::c_void, msg: u32, w: u32, l: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d3a00(
    param_1: *mut core::ffi::c_void,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> i32 {
    // SAFETY: the globals and callees are the game's own; the unaligned
    // globals are accessed with unaligned reads and writes.
    unsafe {
        'def: {
            if param_2 < 0x1d {
                if param_2 == 0x1c {
                    (&raw mut DAT_00aa7b3e)
                        .cast::<u32>()
                        .write_unaligned(param_3);
                    if param_3 == 0 {
                        FUN_004d3350();
                        ReleaseCapture();
                    } else {
                        FUN_004d3400();
                        let h = (&raw mut DAT_00aa7b26)
                            .cast::<*mut core::ffi::c_void>()
                            .read_unaligned();
                        if !h.is_null() {
                            let h2 = (&raw mut DAT_00aa7b26)
                                .cast::<*mut core::ffi::c_void>()
                                .read_unaligned();
                            SetCapture(h2);
                        }
                        DAT_00aa7b96 = 1;
                        FUN_004de150();
                    }
                    break 'def;
                }
                if param_2 == 2 {
                    FUN_004d3820(1);
                    PostQuitMessage(0);
                    return 0;
                }
                break 'def;
            }
            if param_2 < 0x101 {
                if param_2 == 0x100 {
                    if param_3 == 0x1b && DAT_00aa7aa0 != 0 {
                        PostMessageA(param_1, 0x10, 0, 0);
                    }
                    break 'def;
                }
                if param_2 == 0x20 {
                    SetCursor(core::ptr::null_mut());
                    return 1;
                }
                break 'def;
            }
            if 0x200 < param_2 {
                match param_2 {
                    0x201 => {
                        DAT_00aa739e = 1;
                        if (param_3 & 2) != 0 {
                            DAT_00aa739f = 1;
                        }
                    }
                    0x202 => {
                        DAT_00aa739e = 0;
                        DAT_00aa73a0 = 1;
                        if (param_3 & 2) != 0 {
                            DAT_00aa739f = 0;
                            DAT_00aa73a1 = 1;
                        }
                    }
                    0x204 => {
                        DAT_00aa739f = 1;
                        if (param_3 & 1) != 0 {
                            DAT_00aa739e = 1;
                        }
                    }
                    0x205 => {
                        DAT_00aa739f = 0;
                        DAT_00aa73a1 = 1;
                        if (param_3 & 1) != 0 {
                            DAT_00aa739e = 0;
                            DAT_00aa73a0 = 1;
                        }
                    }
                    _ => {}
                }
                break 'def;
            }
            if param_2 == 0x200 {
                let u_var2 = param_4 & 0xffff;
                let mut local_y = param_4 >> 0x10;
                let mut local_x = u_var2;
                (&raw mut DAT_00aa720d)
                    .cast::<u32>()
                    .write_unaligned(local_y);
                (&raw mut DAT_00aa7209)
                    .cast::<u32>()
                    .write_unaligned(local_x);
                if 0x280 < u_var2 {
                    local_x = 0x280;
                    (&raw mut DAT_00aa7209)
                        .cast::<u32>()
                        .write_unaligned(local_x);
                }
                let b_var1 = 0x1e0 < local_y;
                if b_var1 {
                    local_y = 0x1e0;
                    (&raw mut DAT_00aa720d)
                        .cast::<u32>()
                        .write_unaligned(local_y);
                }
                if b_var1 || 0x280 < u_var2 {
                    local_y = (&raw mut DAT_00aa720d).cast::<u32>().read_unaligned();
                    local_x = (&raw mut DAT_00aa7209).cast::<u32>().read_unaligned();
                    SetCursorPos(local_x as i32, local_y as i32);
                }
                break 'def;
            }
            match param_2 {
                0x101 => {
                    let p = (&raw mut DAT_00aa7598).cast::<u32>();
                    let mut i = 0;
                    while i < 0x40 {
                        p.add(i).write(0);
                        i += 1;
                    }
                    break 'def;
                }
                0x102 => {
                    if (DAT_00aa7a9c.wrapping_sub(DAT_00aa7a98) == 1
                        || (DAT_00aa7a98 == 0x3ff && DAT_00aa7a9c == 0))
                    {
                        DAT_00aa7a9c = DAT_00aa7a9c.wrapping_add(1);
                        if DAT_00aa7a9c == 0x400 {
                            DAT_00aa7a9c = 0;
                        }
                    }
                    (&raw mut DAT_00aa7698)
                        .cast::<i8>()
                        .add(DAT_00aa7a98 as usize)
                        .write(param_3 as i8);
                    if DAT_00aa7a98 == 0x3ff {
                        DAT_00aa7a98 = 0;
                    } else {
                        DAT_00aa7a98 = DAT_00aa7a98.wrapping_add(1);
                    }
                    break 'def;
                }
                0x104 => {}
                0x105 => {
                    if param_3 == 0x4f {
                        FUN_004d5380();
                        return 0;
                    }
                    if param_3 == 0x73 {
                        return 0;
                    }
                }
                _ => break 'def,
            }
            if param_3 == 0x73 {
                return 0;
            }
        }
        DefWindowProcA(param_1, param_2, param_3, param_4)
    }
}
