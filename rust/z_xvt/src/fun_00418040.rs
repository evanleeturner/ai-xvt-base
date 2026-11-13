// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe fn vt(p: *mut i32, off: usize) -> *const c_void {
    // SAFETY: p is a COM-style object whose first word is its vtable address.
    unsafe {
        let table = p.read_unaligned() as u32 as usize;
        ((table + off) as *const u32).read_unaligned() as usize as *const c_void
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_00418040(param_1: *mut u32) -> *mut i32 {
    let mut unaff_esi: u32 = 0;
    let mut local_14: u32 = 0;
    let mut local_10: u32 = 0;
    let mut aux: [u8; 16] = [0; 16];

    if param_1.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: param_1 and the objects it references are the game's own memory;
    // the vtable slots are called with the signatures the original uses.
    unsafe {
        let idx = param_1.add(3).read_unaligned() as usize;
        let mut p4 = param_1.add(idx + 4).read_unaligned() as usize as *mut i32;
        if !p4.is_null() {
            let f: unsafe extern "C" fn(*mut i32, *mut u32) -> i32 =
                core::mem::transmute(vt(p4, 0x24));
            let iv2 = f(p4, &mut local_14);
            if iv2 < 0 {
                unaff_esi = 0;
            }
            if (unaff_esi & 1) != 0 {
                if (param_1.add(2).read_unaligned() as i32) < 2 {
                    p4 = core::ptr::null_mut();
                } else {
                    let i2 = param_1.add(3).read_unaligned() as i32;
                    param_1.add(3).write_unaligned(i2.wrapping_add(1) as u32);
                    if (param_1.add(2).read_unaligned() as i32) <= i2.wrapping_add(1) {
                        param_1.add(3).write_unaligned(0);
                    }
                    let idx = param_1.add(3).read_unaligned() as usize;
                    p4 = param_1.add(idx + 4).read_unaligned() as usize as *mut i32;
                    let f: unsafe extern "C" fn(*mut i32, *mut u32) -> i32 =
                        core::mem::transmute(vt(p4, 0x24));
                    let iv2 = f(p4, &mut local_14);
                    if -1 < iv2 && (unaff_esi & 1) != 0 {
                        let g: unsafe extern "C" fn(*mut i32) = core::mem::transmute(vt(p4, 0x48));
                        g(p4);
                        let h: unsafe extern "C" fn(*mut i32, i32) =
                            core::mem::transmute(vt(p4, 0x34));
                        h(p4, 0);
                    }
                }
            }
            if !p4.is_null() && (unaff_esi & 2) != 0 {
                let p11 = p4;
                let f50: unsafe extern "C" fn() -> i32 = core::mem::transmute(vt(p4, 0x50));
                let iv2 = f50();
                if -1 < iv2 {
                    let p1 = param_1.read_unaligned() as usize as *mut i32;
                    if !p4.is_null() && !p1.is_null() && param_1.add(1).read_unaligned() != 0 {
                        let p9 = local_14;
                        let p10 = core::ptr::null_mut::<u32>();
                        let f2c: unsafe extern "C" fn(
                            *mut i32,
                            i32,
                            u32,
                            *mut u8,
                            *mut u32,
                        ) -> i32 = core::mem::transmute(vt(p4, 0x2c));
                        let iv2 = f2c(
                            p4,
                            0,
                            param_1.add(1).read_unaligned(),
                            aux.as_mut_ptr(),
                            &mut local_10,
                        );
                        if -1 < iv2 {
                            let mut p5 = p1 as *mut u32;
                            let mut p7 = p11 as *mut u32;
                            let mut n = p9 >> 2;
                            while n != 0 {
                                p7.write_unaligned(p5.read_unaligned());
                                p5 = p5.add(1);
                                p7 = p7.add(1);
                                n = n.wrapping_sub(1);
                            }
                            let mut b5 = p5 as *mut u8;
                            let mut b7 = p7 as *mut u8;
                            let mut n = p9 & 3;
                            while n != 0 {
                                b7.write(b5.read());
                                b5 = b5.add(1);
                                b7 = b7.add(1);
                                n = n.wrapping_sub(1);
                            }
                            if local_14 != 0 {
                                let mut p6 = (p9 as usize).wrapping_add(p1 as usize) as *mut u32;
                                let mut p8 = p10;
                                let mut n = local_14 >> 2;
                                while n != 0 {
                                    p8.write_unaligned(p6.read_unaligned());
                                    p6 = p6.add(1);
                                    p8 = p8.add(1);
                                    n = n.wrapping_sub(1);
                                }
                                let mut b6 = p6 as *mut u8;
                                let mut b8 = p8 as *mut u8;
                                let mut n = local_14 & 3;
                                while n != 0 {
                                    b8.write(b6.read());
                                    b6 = b6.add(1);
                                    b8 = b8.add(1);
                                    n = n.wrapping_sub(1);
                                }
                            }
                            let f4c: unsafe extern "C" fn(*mut i32, *mut i32, u32, *mut u32, u32) =
                                core::mem::transmute(vt(p4, 0x4c));
                            f4c(p4, p11, p9, p10, local_14);
                            return p4;
                        }
                    }
                }
                p4 = core::ptr::null_mut();
            }
        }
        p4
    }
}
