// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066ddd4: *mut i32;
    static mut DAT_0066e700: *mut i32;
    static mut DAT_00803f80: *mut i32;
    static mut DAT_00a68748: *mut i32;
    static mut DAT_0066ddd0: i32;
    static mut DAT_00527eac: i32;
    static mut DAT_00527ea8: i32;
    static mut DAT_00527ea4: i32;
    static mut DAT_00527f7c: u32;
    static mut DAT_00527f80: i32;
    static mut DAT_00622cb0: i32;
    static mut DAT_00803b70: u32;
    static mut DAT_00803f90: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ab970(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004aafe0() -> i32;
    fn FUN_004aba10() -> i32;
    fn FUN_004ac460() -> i32;
    fn timeGetTime() -> u32;
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

type F1 = unsafe extern "stdcall" fn(*mut i32) -> i32;
type FPu = unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32;
type FPi = unsafe extern "stdcall" fn(*mut i32, *mut i32) -> i32;
type F3 = unsafe extern "stdcall" fn(*mut i32, i32, i32) -> i32;
type F18 = unsafe extern "stdcall" fn(*mut i32, *mut u32, *mut *mut i32, i32) -> i32;
type F64 = unsafe extern "stdcall" fn(*mut i32, i32, *mut u32, i32, i32) -> i32;

unsafe fn vt(o: *mut i32, off: usize) -> *const c_void {
    // SAFETY: o points at a COM-style object whose first word is its vtable.
    unsafe {
        let table = (o as *const *const u8).read_unaligned();
        table.byte_add(off).cast::<*const c_void>().read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004abee0() {
    // SAFETY: the globals, vtables and callees are the game's own memory and code.
    unsafe {
        let mut au: [u32; 27] = [0; 27];
        let mut i_stack_8: i32 = 0;
        let mut i_stack_c: i32;

        if DAT_00527eac == 0 {
            if DAT_00527ea8 == 0 {
                au = [0; 27];
                au[0] = 0x6c;
                au[1] = 1;
                au[26] = 0x200;
                let f = core::mem::transmute::<*const c_void, F18>(vt(DAT_0066ddd4, 0x18));
                let r = f(DAT_0066ddd4, au.as_mut_ptr(), &raw mut DAT_0066e700, 0);
                if r != 0 {
                    FUN_004ab970(4);
                    return;
                }
                if FUN_004aafe0() == 0 {
                    FUN_004ab970(0xc);
                    return;
                }
            }
            au = [0; 27];
            au[0] = 0x6c;
            let f = core::mem::transmute::<*const c_void, FPu>(vt(DAT_0066e700, 0x58));
            f(DAT_0066e700, au.as_mut_ptr());
            DAT_00803b70 = au[4];
            au = [0; 27];
            au[0] = 0x6c;
            loop {
                let f = core::mem::transmute::<*const c_void, F64>(vt(DAT_0066e700, 100));
                let r = f(DAT_0066e700, 0, au.as_mut_ptr(), 0, 0);
                if r == 0 {
                    break;
                }
                if r != -0x7789fde4i32 {
                    return;
                }
            }
            let n = FUN_004aba10();
            let mut src = &raw mut DAT_00803f90;
            let mut dst = au[9] as *mut u32;
            let mut cnt = ((n as u32).wrapping_mul(0xf) & 0x7ffffff) << 3;
            while cnt != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                cnt -= 1;
            }
            let f = core::mem::transmute::<*const c_void, FPu>(vt(DAT_0066e700, 0x80));
            f(DAT_0066e700, au[9] as *mut u32);
            if DAT_00527ea8 == 0 && !DAT_0066e700.is_null() {
                let f = core::mem::transmute::<*const c_void, F1>(vt(DAT_0066e700, 8));
                f(DAT_0066e700);
                DAT_0066e700 = core::ptr::null_mut();
            }
        } else {
            if DAT_00527ea4 != 0 {
                if DAT_00527f7c == 0 {
                    let f = core::mem::transmute::<*const c_void, FPi>(vt(DAT_0066ddd4, 0x44));
                    let mut r = f(DAT_0066ddd4, &mut i_stack_8);
                    if r == 0 {
                        loop {
                            if i_stack_8 == 0 {
                                break;
                            }
                            let f =
                                core::mem::transmute::<*const c_void, FPi>(vt(DAT_0066ddd4, 0x44));
                            r = f(DAT_0066ddd4, &mut i_stack_8);
                            if r != 0 {
                                break;
                            }
                        }
                        loop {
                            if i_stack_8 != 0 {
                                break;
                            }
                            let f =
                                core::mem::transmute::<*const c_void, FPi>(vt(DAT_0066ddd4, 0x44));
                            r = f(DAT_0066ddd4, &mut i_stack_8);
                            if r != 0 {
                                break;
                            }
                        }
                    }
                    DAT_00527f7c = timeGetTime();
                    let f = core::mem::transmute::<*const c_void, FPi>(vt(DAT_0066ddd4, 0x3c));
                    r = f(DAT_0066ddd4, &raw mut DAT_00622cb0);
                    if r != 0 {
                        let mut i = 0;
                        while i < 100 {
                            loop {
                                if i_stack_8 == 0 {
                                    break;
                                }
                                let f = core::mem::transmute::<*const c_void, FPi>(vt(
                                    DAT_0066ddd4,
                                    0x44,
                                ));
                                let r4 = f(DAT_0066ddd4, &mut i_stack_8);
                                if r4 != 0 {
                                    break;
                                }
                            }
                            loop {
                                if i_stack_8 != 0 {
                                    break;
                                }
                                let f = core::mem::transmute::<*const c_void, FPi>(vt(
                                    DAT_0066ddd4,
                                    0x44,
                                ));
                                let r4 = f(DAT_0066ddd4, &mut i_stack_8);
                                if r4 != 0 {
                                    break;
                                }
                            }
                            i += 1;
                        }
                        let d = timeGetTime();
                        let divisor = d.wrapping_sub(DAT_00527f7c) as i32;
                        if divisor == 0 {
                            RaiseException(0xC0000094, 0, 0, core::ptr::null());
                        } else {
                            DAT_00622cb0 = 10000000 / divisor;
                        }
                    }
                } else {
                    let d = timeGetTime();
                    let v = ((DAT_00622cb0 as u32).wrapping_mul(d.wrapping_sub(DAT_00527f7c))
                        as i32)
                        % 100000;
                    if (v < DAT_00527f80
                        || (100000u32.wrapping_sub(DAT_00527f80 as u32) as i32) < v)
                    {
                        let f = core::mem::transmute::<*const c_void, FPi>(vt(DAT_0066ddd4, 0x44));
                        let r = f(DAT_0066ddd4, &mut i_stack_8);
                        if r == 0 && i_stack_8 == 0 {
                            loop {
                                if i_stack_8 != 0 {
                                    break;
                                }
                                let f = core::mem::transmute::<*const c_void, FPi>(vt(
                                    DAT_0066ddd4,
                                    0x44,
                                ));
                                let r3 = f(DAT_0066ddd4, &mut i_stack_8);
                                if r3 != 0 {
                                    break;
                                }
                            }
                            DAT_00527f7c = timeGetTime();
                        }
                    }
                }
            }
            let f = core::mem::transmute::<*const c_void, F3>(vt(DAT_0066e700, 0x2c));
            i_stack_c = f(DAT_0066e700, 0, 1);
            if i_stack_c == -0x7789ff1fi32 {
                let f = core::mem::transmute::<*const c_void, F3>(vt(DAT_0066ddd4, 0x50));
                let mut r = f(DAT_0066ddd4, DAT_0066ddd0, 0x51);
                if r != 0 {
                    core::arch::asm!("int3");
                    return;
                }
                let f = core::mem::transmute::<*const c_void, F3>(vt(DAT_0066e700, 0x2c));
                r = f(DAT_0066e700, 0, 1);
                if r == -0x7789fe3ei32 {
                    let f = core::mem::transmute::<*const c_void, F1>(vt(DAT_0066e700, 0x6c));
                    f(DAT_0066e700);
                    let f = core::mem::transmute::<*const c_void, F1>(vt(DAT_00803f80, 0x6c));
                    f(DAT_00803f80);
                    let f = core::mem::transmute::<*const c_void, F1>(vt(DAT_00a68748, 0x6c));
                    f(DAT_00a68748);
                    let f = core::mem::transmute::<*const c_void, F3>(vt(DAT_0066e700, 0x2c));
                    r = f(DAT_0066e700, 0, 1);
                }
                if r != 0 {
                    core::arch::asm!("int3");
                    return;
                }
                let f = core::mem::transmute::<*const c_void, F3>(vt(DAT_0066ddd4, 0x50));
                i_stack_c = f(DAT_0066ddd4, DAT_0066ddd0, 8);
                if i_stack_c != 0 {
                    core::arch::asm!("int3");
                    return;
                }
            }
            if i_stack_c == -0x7789fe3ei32 && FUN_004ac460() != 0 {
                FUN_004aafe0();
                return;
            }
        }
    }
}
