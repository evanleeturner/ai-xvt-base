// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759663: i32;
    static mut DAT_00759653: i32;
    static mut DAT_00759657: i32;
    static mut DAT_00759667: i32;
    static mut DAT_0075966b: i32;
    static mut DAT_0075966f: i32;
    static mut DAT_007596c7: i32;
    static mut DAT_007596cb: i32;
    static mut DAT_00759645: i8;
    static mut DAT_00759646: i8;
    static mut DAT_0075965c: i8;
    static mut DAT_00759ad3: i8;
    static mut DAT_0081b6e4: i32;
    static mut DAT_0081b6f0: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0051a4c0();
    fn FUN_004d4d90();
    fn FUN_0051a4e0();
    fn FUN_004d50a0();
    fn FUN_004dbd90();
    fn FUN_004d3c60() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_004df5d0(param_1: i32, param_2: u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut u32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

type Pfn0x2c = unsafe extern "stdcall" fn(*mut i32, *mut i32, i32) -> i32;
type Pfn0x14 =
    unsafe extern "stdcall" fn(*mut i32, *mut u32, *mut i32, *mut u32, u32, *mut u32) -> i32;
type Pfn0x64 = unsafe extern "stdcall" fn(*mut i32, i32, *mut u32, i32, i32) -> i32;
type Pfn0x6c = unsafe extern "stdcall" fn(*mut i32) -> i32;
type Pfn0x80 = unsafe extern "stdcall" fn(*mut i32, i32) -> i32;
type Pfn0x1c = unsafe extern "stdcall" fn(*mut i32, i32, i32, *mut i32, *mut u32, i32) -> i32;

macro_rules! rd {
    ($g:ident) => {
        (&raw const $g).read_unaligned()
    };
}

unsafe fn slot(obj: *mut i32, off: i32) -> *const c_void {
    // SAFETY: obj is a live COM-style object; its first word is the vtable address.
    unsafe {
        let vt = obj.read();
        (vt.wrapping_add(off) as usize as *const *const c_void).read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4dc0() {
    // SAFETY: globals, callees and the vtable objects belong to the game; the
    // accesses replicate the original's order and widths.
    unsafe {
        if rd!(DAT_00759663) == 0 {
            return;
        }
        FUN_0051a4c0();
        if DAT_00759645 != 0 {
            FUN_004d4d90();
        }
        let mut local_f0: [u32; 4] = [0, 0, 0x280, 0x1e0];
        'lab: {
            if DAT_0081b6e4 == 0 && DAT_0081b6f0 == 0 {
                let i_tmp: i32 = rd!(DAT_00759657);
                if DAT_00759ad3 == 1 && i_tmp == 8 {
                    FUN_004df5d0(0, 0);
                    DAT_00759ad3 = 0;
                }
                loop {
                    let mut i_var1: i32;
                    loop {
                        let o1 = rd!(DAT_0075966b) as usize as *mut i32;
                        let o2 = rd!(DAT_0075966f) as usize as *mut i32;
                        let f: Pfn0x2c = core::mem::transmute(slot(o1, 0x2c));
                        i_var1 = f(o1, o2, 1);
                        if i_var1 == 0 {
                            break 'lab;
                        }
                        if i_var1 != -0x7789fe3e {
                            break;
                        }
                        i_var1 = FUN_004d3c60();
                        if i_var1 != 0 {
                            break 'lab;
                        }
                    }
                    if i_var1 != -0x7789fde4 {
                        break;
                    }
                }
            } else {
                let mut local_64: [u32; 25] = [0; 25];
                let mut local_e0: [u32; 4] = [0; 4];
                local_64[0] = 100;
                local_64[2] = 0xcc0020;
                FUN_004dcfd0(local_e0.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                loop {
                    let o1 = rd!(DAT_0075966b) as usize as *mut i32;
                    let o2 = rd!(DAT_0075966f) as usize as *mut i32;
                    let f: Pfn0x14 = core::mem::transmute(slot(o1, 0x14));
                    let i_var1 = f(
                        o1,
                        local_e0.as_mut_ptr(),
                        o2,
                        local_e0.as_mut_ptr(),
                        0x1020000,
                        local_64.as_mut_ptr(),
                    );
                    if i_var1 == 0 {
                        break;
                    }
                    if i_var1 == -0x7789fe3e {
                        if FUN_004d3c60() != 0 {
                            return;
                        }
                    } else if i_var1 != -0x7789fde4 {
                        return;
                    }
                }
            }
        }
        if DAT_0075965c != 0 {
            if rd!(DAT_007596cb) != 0 {
                (&raw mut DAT_007596cb).write_unaligned(0);
                if rd!(DAT_007596c7) != 0 {
                    let mut p59667 = rd!(DAT_00759667) as usize as *mut i32;
                    if !p59667.is_null() {
                        let mut au: [u32; 27] = [0; 27];
                        au[0] = 0x6c;
                        loop {
                            let mut i_var1: i32;
                            loop {
                                p59667 = rd!(DAT_00759667) as usize as *mut i32;
                                let f: Pfn0x64 = core::mem::transmute(slot(p59667, 100));
                                i_var1 = f(p59667, 0, au.as_mut_ptr(), 0, 0);
                                if i_var1 == 0 {
                                    break;
                                }
                                if i_var1 != -0x7789fe3e {
                                    break;
                                }
                                p59667 = rd!(DAT_00759667) as usize as *mut i32;
                                let g: Pfn0x6c = core::mem::transmute(slot(p59667, 0x6c));
                                g(p59667);
                            }
                            if i_var1 == 0 || i_var1 != -0x7789fde4 {
                                break;
                            }
                        }
                        let mut dst = au[9] as usize as *mut u32;
                        if !dst.is_null() {
                            let mut src = rd!(DAT_007596c7) as usize as *const u32;
                            let u_count = rd!(DAT_00759653) as u32;
                            let mut n = ((u_count.wrapping_mul(0xf)) & 0x7ffffff) << 3;
                            while n != 0 {
                                dst.write_unaligned(src.read_unaligned());
                                src = src.add(1);
                                dst = dst.add(1);
                                n -= 1;
                            }
                            p59667 = rd!(DAT_00759667) as usize as *mut i32;
                            let f: Pfn0x80 = core::mem::transmute(slot(p59667, 0x80));
                            f(p59667, 0);
                        }
                    }
                }
            }
            if DAT_0081b6e4 == 0 && DAT_0081b6f0 == 0 {
                loop {
                    let o2 = rd!(DAT_0075966f) as usize as *mut i32;
                    let o3 = rd!(DAT_00759667) as usize as *mut i32;
                    let f: Pfn0x1c = core::mem::transmute(slot(o2, 0x1c));
                    let i_var1 = f(o2, 0, 0, o3, local_f0.as_mut_ptr(), 0);
                    if i_var1 == 0 {
                        break;
                    }
                    if i_var1 == -0x7789fe3e {
                        if FUN_004d3c60() != 0 {
                            FUN_0051a4e0();
                            return;
                        }
                    } else if i_var1 != -0x7789fde4 {
                        FUN_0051a4e0();
                        return;
                    }
                }
            } else {
                FUN_004dbd90();
            }
        }
        if DAT_00759646 != 0 {
            FUN_004d50a0();
        }
        FUN_0051a4e0();
    }
}
