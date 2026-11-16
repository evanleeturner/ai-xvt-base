// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759aec: *mut i32;
    static mut DAT_00759af4: i32;
    static mut DAT_00759af8: *mut i32;
    static mut DAT_00759b00: i32;
    static mut DAT_00759b04: i32;
    static mut DAT_00759645: u8;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dee40(param_1: *mut i8) -> i32;
    fn FUN_004d4d90();
    fn FUN_004d4d00() -> i32;
    fn FUN_00417c20(param_1: *mut i32, param_2: *mut i8) -> i32;
}

unsafe fn slot(obj: *mut i32, off: usize) -> *const c_void {
    // SAFETY: obj is a COM-style object whose first word is its vtable.
    unsafe {
        let vt = obj.read() as u32 as usize as *const u8;
        vt.add(off).cast::<*const c_void>().read_unaligned()
    }
}

type F0 = unsafe extern "stdcall" fn(*mut i32) -> i32;
type F1 = unsafe extern "stdcall" fn(*mut i32, i32) -> i32;
type FP = unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32;
type F3 = unsafe extern "stdcall" fn(*mut i32, i32, i32, u32) -> i32;
type FC = unsafe extern "stdcall" fn(*mut i32, i32, *mut *mut i32) -> i32;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004de190(
    param_1: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut param_3 = param_3;
    // SAFETY: all globals, callees and COM objects are the game's own memory.
    unsafe {
        if DAT_00759aec.is_null() {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var1 = FUN_004dee40(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let u_stack_8 = DAT_00759645 as u32;
        FUN_004d4d90();
        let mut i_var5: i32;
        let mut i_var6: i32;
        let mut u_stack_4: u32 = 0;
        if DAT_00759b00 == 0xc {
            i_var5 = 0;
            i_var6 = 0;
            loop {
                let base = DAT_00759af8;
                if base.byte_add(i_var6 as usize).read() != -1 {
                    let p3 =
                        base.byte_add(i_var6 as usize).add(2).read() as u32 as usize as *mut i32;
                    let f: FP = core::mem::transmute(slot(p3, 0x24));
                    let i_var2 = f(p3, &raw mut u_stack_4);
                    if i_var2 != 0 || ((u_stack_4 & 1) == 0 && (u_stack_4 & 4) == 0) {
                        let o = DAT_00759af8.add((i_var5 as usize) * 3 + 2).read() as u32 as usize
                            as *mut i32;
                        let f: F0 = core::mem::transmute(slot(o, 8));
                        f(o);
                        DAT_00759af8.add((i_var5 as usize) * 3).write(-1);
                        DAT_00759af8.add((i_var5 as usize) * 3 + 2).write(0);
                        DAT_00759b00 = DAT_00759b00.wrapping_sub(1);
                        break;
                    }
                }
                i_var6 = i_var6.wrapping_add(0xc);
                i_var5 += 1;
                if !(i_var6 < 0x90) {
                    break;
                }
            }
            if i_var5 == 0xc {
                i_var5 = 0xc;
                let mut u_stack_c = param_4;
                i_var6 = 0;
                let mut p3 = DAT_00759af8;
                loop {
                    let addr = (p3.read())
                        .wrapping_mul(0x145)
                        .wrapping_add(0x144)
                        .wrapping_add(DAT_00759af4);
                    let u_var4 = (addr as u32 as usize as *const u8).read() as u32;
                    if (u_var4 as i32) < (u_stack_c as i32) {
                        i_var5 = i_var6;
                        u_stack_c = u_var4;
                    }
                    p3 = p3.add(3);
                    i_var6 += 1;
                    if !(i_var6 < 0xc) {
                        break;
                    }
                }
                if i_var5 == 0xc {
                    if param_2 != 0 {
                        let mut i5 = 0i32;
                        let mut p3 = DAT_00759af8;
                        loop {
                            if p3.read() == i_var1 {
                                let o = DAT_00759af8.add((i5 as usize) * 3 + 2).read() as u32
                                    as usize as *mut i32;
                                let f: F1 = core::mem::transmute(slot(o, 0x34));
                                f(o, 0);
                                DAT_00759af8.add((i5 as usize) * 3 + 1).write(DAT_00759b04);
                                DAT_00759b04 = DAT_00759b04.wrapping_add(1);
                                if u_stack_8 != 0 {
                                    DAT_00758830 = FUN_004d4d00();
                                }
                                return 1;
                            }
                            p3 = p3.add(3);
                            i5 += 1;
                            if !(i5 < 0xc) {
                                break;
                            }
                        }
                    }
                    if u_stack_8 != 0 {
                        DAT_00758830 = FUN_004d4d00();
                    }
                    return 0;
                }
                let o =
                    DAT_00759af8.add((i_var5 as usize) * 3 + 2).read() as u32 as usize as *mut i32;
                let f: F0 = core::mem::transmute(slot(o, 0x48));
                f(o);
                let o =
                    DAT_00759af8.add((i_var5 as usize) * 3 + 2).read() as u32 as usize as *mut i32;
                let f: F0 = core::mem::transmute(slot(o, 8));
                f(o);
                DAT_00759af8.add((i_var5 as usize) * 3).write(-1);
                DAT_00759af8.add((i_var5 as usize) * 3 + 2).write(0);
                DAT_00759b00 = DAT_00759b00.wrapping_sub(1);
            }
        } else {
            i_var5 = 0;
            let mut p3 = DAT_00759af8;
            loop {
                if p3.read() == -1 {
                    break;
                }
                p3 = p3.add(3);
                i_var5 += 1;
                if !(i_var5 < 0xc) {
                    break;
                }
            }
        }
        let mut pi_var8: *mut i32 = core::ptr::null_mut();
        let dev = DAT_00759aec;
        let f: FC = core::mem::transmute(slot(dev, 0x14));
        let arg = (DAT_00759af4
            .wrapping_add(0x140)
            .wrapping_add(i_var1.wrapping_mul(0x145)) as u32 as usize
            as *const i32)
            .read_unaligned();
        f(dev, arg, &raw mut pi_var8);
        if !pi_var8.is_null() {
            let p8 = pi_var8;
            let f: F1 = core::mem::transmute(slot(p8, 0x34));
            f(p8, 0);
            if param_3 < 0x80 {
                if param_3 < 0 {
                    param_3 = 0;
                }
            } else {
                param_3 = 0x7f;
            }
            let f: F1 = core::mem::transmute(slot(p8, 0x3c));
            let v = ((param_3 as u32).wrapping_mul(5).wrapping_sub(0x27b) as i32).wrapping_mul(400)
                / 0x7f;
            f(p8, v);
            u_stack_4 = param_5 as u32;
            if 0x7f < (u_stack_4 as i32) {
                u_stack_4 = 0x7f;
            }
            if (u_stack_4 as i32) < 0 {
                u_stack_4 = 0;
            }
            let f: F1 = core::mem::transmute(slot(p8, 0x40));
            let v = (u_stack_4.wrapping_mul(5).wrapping_sub(0x13b) as i32).wrapping_mul(400) / 0x3f;
            f(p8, v);
            let u_var4: u32 = (param_6 == 1) as u32;
            let f: F3 = core::mem::transmute(slot(p8, 0x30));
            i_var6 = f(p8, 0, 0, u_var4);
            if i_var6 == -0x7787ff6a {
                let b = DAT_00759af4.wrapping_add(i_var1.wrapping_mul(0x145));
                i_var6 = FUN_00417c20(
                    (b.wrapping_add(0x140) as u32 as usize as *const i32).read_unaligned() as u32
                        as usize as *mut i32,
                    b.wrapping_add(0x40) as u32 as usize as *mut i8,
                );
                if i_var6 == 1 {
                    let f: F1 = core::mem::transmute(slot(p8, 0x34));
                    f(p8, 0);
                    let f: F3 = core::mem::transmute(slot(p8, 0x30));
                    i_var6 = f(p8, 0, 0, u_var4);
                    if i_var6 == 0 {
                        let k = (i_var5 as usize) * 3;
                        DAT_00759af8.add(k).write(i_var1);
                        DAT_00759af8.add(k + 2).write(p8 as usize as u32 as i32);
                        DAT_00759af8.add(k + 1).write(DAT_00759b04);
                        DAT_00759b04 = DAT_00759b04.wrapping_add(1);
                        DAT_00759b00 = DAT_00759b00.wrapping_add(1);
                        i_var6 = 0;
                    } else {
                        i_var6 = 0;
                    }
                }
            } else if i_var6 == 0 {
                let k = (i_var5 as usize) * 3;
                DAT_00759af8.add(k).write(i_var1);
                DAT_00759af8.add(k + 2).write(p8 as usize as u32 as i32);
                DAT_00759af8.add(k + 1).write(DAT_00759b04);
                DAT_00759b04 = DAT_00759b04.wrapping_add(1);
                DAT_00759b00 = DAT_00759b00.wrapping_add(1);
                i_var6 = 1;
            }
            if u_stack_8 != 0 {
                DAT_00758830 = FUN_004d4d00();
            }
            return i_var6;
        }
        if u_stack_8 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        0
    }
}
