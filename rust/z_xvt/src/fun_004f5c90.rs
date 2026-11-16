// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_00757d0a: [u8; 0];
    static mut DAT_00757cee: [u8; 0];
    static mut DAT_00757e2d: [u8; 0];
    static mut DAT_00757e3e: [u8; 0];
    static mut DAT_0081e0a2: [u8; 0];
    static mut DAT_0081bc64: i32;
    static mut DAT_0081e0a6: [u8; 0];
    static mut DAT_0081e0c6: [u8; 0];
    static mut DAT_0081e0e6: [u8; 0];
    static mut DAT_0081e1a6: [u8; 0];
    static mut DAT_0081e266: [u8; 0];
    static mut DAT_0081e286: [u8; 0];
    static mut DAT_0081e2a6: [u8; 0];
    static mut DAT_0081e366: [u8; 0];
    static mut DAT_0081e4e6: [u8; 0];
    static mut DAT_0081fa7e: [u8; 0];
    static mut DAT_0081bfba: [u8; 0];
    static mut DAT_0081c012: [u8; 0];
    static mut DAT_0081d326: [u8; 0];
    static mut DAT_0081d32e: [u8; 0];
    static mut DAT_0081d3f6: [u8; 0];
    static mut DAT_0081d3fa: [u8; 0];
    static mut DAT_0081f802: [u8; 0];
    static mut DAT_0081f7da: [u8; 0];
    static mut DAT_0081f7fe: [u8; 0];
    static mut DAT_00757870: [i32; 0];
    static mut DAT_00757680: [u8; 0];
    static mut DAT_00743dc3: [u8; 0];
    static mut DAT_007438d7: [u8; 0];
    static mut DAT_007438d2: [u8; 0];
    static mut DAT_007438e4: [u8; 0];
    static mut DAT_0081e00e: [u8; 0];
    static mut DAT_00757a28: i32;
    static mut DAT_007579d0: [i32; 0];
    static mut DAT_0075766c: i16;
    static mut DAT_0085e7c4: i8;
    static mut DAT_0081fb96: [u8; 0];
    static mut DAT_0081fbda: [u8; 0];
    static mut DAT_0082e3fe: [u8; 0];
    static mut DAT_0081fb9e: [u8; 0];
    static mut DAT_0081fba2: [u8; 0];
    static mut DAT_0081fba6: [u8; 0];
    static mut DAT_0081fbaa: [u8; 0];
    static mut DAT_0081fbae: [u8; 0];
    static mut DAT_0081fbb2: [u8; 0];
    static mut DAT_0081fbb6: [u8; 0];
    static mut DAT_0081fbde: [u8; 0];
    static mut DAT_0081bfb6: [u8; 0];
    static mut DAT_0081bfbe: [u8; 0];
    static mut DAT_0081bfc2: [u8; 0];
    static mut DAT_0081bfc6: [u8; 0];
    static mut DAT_0081bfca: [u8; 0];
    static mut DAT_0081bfce: [u8; 0];
    static mut DAT_0081c016: [u8; 0];
    static mut DAT_0083cbe6: [u8; 0];
    static mut DAT_0083cbea: [u8; 0];
    static mut DAT_0083cbee: [u8; 0];
    static mut DAT_0083cc22: [u8; 0];
    static mut DAT_0083cc26: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cdce0(param_1: i32) -> *mut u32;
    fn FUN_004cdbc0() -> i32;
    fn FUN_004ce720();
}

macro_rules! p {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: caller supplies a readable game address.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn w32(p: *mut u8, v: u32) {
    // SAFETY: caller supplies a writable game address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

fn o(p: *mut u8, off: isize) -> *mut u8 {
    p.wrapping_offset(off)
}

fn a(p: *mut u8, off: u32) -> *mut u8 {
    p.wrapping_add(off as usize)
}

unsafe fn zero(mut p: *mut u8, n: u32) {
    let mut c = n as i32;
    while c != 0 {
        // SAFETY: game memory region being cleared.
        unsafe {
            *p = 0;
            *p.add(1) = 0;
            *p.add(2) = 0;
            *p.add(3) = 0;
        }
        p = p.wrapping_add(4);
        c = c.wrapping_sub(1);
    }
}

unsafe fn zero42(q: *mut u8) {
    // SAFETY: game memory record of 0x2a bytes.
    unsafe {
        zero(q, 10);
        let e = q.wrapping_add(40);
        *e = 0;
        *e.add(1) = 0;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f5c90() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the pointers are walked exactly as the original does.
    unsafe {
        if DAT_0081b850 == 2 {
            let mut p5 = p!(DAT_00757d0a);
            loop {
                let p10 = p5.wrapping_add(0x2a);
                zero(p5, 10);
                let e = p5.wrapping_add(40);
                *e = 0;
                *e.add(1) = 0;
                p5 = p10;
                if !(p10 < p!(DAT_00757e2d).wrapping_add(3)) {
                    break;
                }
            }
        } else {
            let mut p14 = p!(DAT_00757cee);
            loop {
                let u2 = r32(p14);
                if u2 != 0 && FUN_004cdce0(u2 as i32).is_null() {
                    zero42(o(p14, -0xe));
                }
                p14 = p14.wrapping_add(0x2a);
                if !(p14 < p!(DAT_00757e3e)) {
                    break;
                }
            }
        }
        *p!(DAT_0081e0a2) = 0;
        *p!(DAT_0081e0a2).add(1) = 0;
        DAT_0081bc64 = 0;
        zero(p!(DAT_0081e0a6), 8);
        zero(p!(DAT_0081e0c6), 8);
        zero(p!(DAT_0081e0e6), 0x30);
        zero(p!(DAT_0081e1a6), 0x30);
        zero(p!(DAT_0081e266), 8);
        zero(p!(DAT_0081e286), 8);
        zero(p!(DAT_0081e2a6), 0x30);
        zero(p!(DAT_0081e366), 0x30);
        zero(p!(DAT_0081e4e6), 0x4b6);
        zero(p!(DAT_0081fa7e), 0x46);

        let mut i_stack_4: i32 = 0;
        let mut u2 = r32(p!(DAT_0081bfba));
        let mut u3 = r32(p!(DAT_0081c012));
        let mut i4 = r32(p!(DAT_0081d326)) as i32;
        if u2 == 1 && u3 == 1 && i4 == 0 {
            let mut p5 = p!(DAT_0081d32e);
            loop {
                w32(p5, 0xffff_ffff);
                p5 = p5.wrapping_add(0x14);
                if !(p5 < p!(DAT_0081d3f6)) {
                    break;
                }
            }
        }
        let mut i_stack_8: i32 = 0;
        let mut p10 = p!(DAT_00757cee);
        let mut q = p!(DAT_0081f802);
        loop {
            u2 = r32(p10);
            if u2 != 0 {
                i_stack_4 = i_stack_4.wrapping_add(1);
                u2 = r32(o(p10, -10));
                u3 = r32(o(p10, -6));
                w32(o(q, -0x36), r32(o(p10, -0xe)));
                let u1 = *o(p10, -2);
                w32(o(q, -0x32), u2);
                w32(o(q, -0x2e), u3);
                *o(q, -0x2a) = u1;
                let mut i7 = r32(o(p10, 12)) as i32;
                let mut i12 = r32(o(p10, 8)) as i32;
                *o(q, -0x2a) = 0;
                w32(o(q, -4), i12 as u32);
                w32(q, i7 as u32);
                i4 = r32(o(p10, 16)) as i32;
                i4 = i4.wrapping_sub(1);
                w32(o(q, 4), i4 as u32);
                i7 = r32(p10) as i32;
                i12 = r32(o(p10, 4)) as i32;
                i4 = r32(o(p10, 20)) as i32;
                i4 = i4.wrapping_sub(1);
                w32(o(q, 8), i4 as u32);
                i4 = r32(o(p10, 24)) as i32;
                i4 = i4.wrapping_sub(1);
                w32(o(q, 12), i4 as u32);
                w32(o(q, -0x24), i7 as u32);
                w32(o(q, -0x20), i12 as u32);
                w32(o(q, -0x1c), 0);
                w32(o(q, -0x18), 0);
                w32(o(q, -0x14), 0);
                w32(o(q, -0x10), 0);
                w32(o(q, -0xc), 0);
                w32(o(q, -8), 0);
                w32(o(q, 0x10), 0);
                let mut i_stack_c = 10;
                let mut off: usize = 0;
                loop {
                    let mut i11 = 8;
                    loop {
                        let v = (p!(DAT_00757870) as *const u8).wrapping_add(off);
                        if v.cast::<i32>().read_unaligned() == i7 {
                            i4 = (p!(DAT_00757680) as *const u8)
                                .wrapping_add(off)
                                .cast::<i32>()
                                .read_unaligned();
                            w32(o(q, -0x28), i4 as u32);
                            u3 = r32(o(q, -4));
                            if u3 == 0 {
                                u2 = r32(q);
                                if u2 != 0xffff_ffff {
                                    let idx = u2.wrapping_add((i4 as u32).wrapping_mul(0x562));
                                    if *a(p!(DAT_00743dc3), idx) == 0 {
                                        w32(q, 0xffff_ffff);
                                    }
                                }
                            }
                        }
                        off = off.wrapping_add(4);
                        i11 -= 1;
                        if i11 == 0 {
                            break;
                        }
                    }
                    i_stack_c -= 1;
                    if i_stack_c == 0 {
                        break;
                    }
                }
                i12 = FUN_004cdbc0();
                if i12 == i7 {
                    DAT_0081bc64 = i_stack_8;
                }
            }
            q = q.wrapping_add(0x16 * 4);
            p10 = p10.wrapping_add(0x2a);
            i_stack_8 = i_stack_8.wrapping_add(1);
            if !(p10 < p!(DAT_00757e3e)) {
                break;
            }
        }

        u2 = r32(p!(DAT_0081bfba));
        u3 = r32(p!(DAT_0081c012));
        if u2 == 3 && u3 == 1 {
            w32(p!(DAT_0081e00e), i_stack_4 as u32);
        }
        let mut u9: u32;
        if u2 == 1 {
            u9 = 0;
            i4 = r32(p!(DAT_0081d326)) as i32;
            if u3 == 1 && i4 == 0 {
                if 0 < DAT_00757a28 {
                    let mut p5 = p!(DAT_0081d32e);
                    let mut p13 = p!(DAT_00757870).cast::<i32>();
                    let mut p14 = p!(DAT_007579d0).cast::<i32>();
                    let mut i7 = DAT_00757a28;
                    loop {
                        let mut i11 = 0;
                        let i12 = p14.read_unaligned();
                        let mut p6 = p13;
                        if 0 < i12 {
                            loop {
                                if p6.read_unaligned() != 0 {
                                    break;
                                }
                                i11 += 1;
                                p6 = p6.wrapping_add(1);
                                if !(i11 < i12) {
                                    break;
                                }
                            }
                            if i11 < i12 {
                                w32(p5, u9);
                            }
                        }
                        p5 = p5.wrapping_add(0x14);
                        p13 = p13.wrapping_add(8);
                        p14 = p14.wrapping_add(1);
                        i7 -= 1;
                        if i7 == 0 {
                            break;
                        }
                    }
                }
                if (DAT_0085e7c4 == 1 || DAT_0081b850 == 2) && 0 < DAT_0075766c {
                    let mut pc = p!(DAT_007438e4);
                    let mut i7 = DAT_0075766c as i32;
                    loop {
                        if *pc != 0 {
                            let b = *o(pc, -0xc) as u32;
                            w32(a(p!(DAT_0081d32e), b.wrapping_mul(20)), u9);
                        }
                        pc = pc.wrapping_add(0x562);
                        i7 -= 1;
                        if i7 == 0 {
                            break;
                        }
                    }
                }
                w32(p!(DAT_0081d3f6), i_stack_4 as u32);
                if DAT_0081b850 == 2 || DAT_0085e7c4 != 0 {
                    w32(p!(DAT_0081d3fa), DAT_00757a28 as u32);
                } else {
                    w32(p!(DAT_0081d3fa), u9);
                    if 0 < DAT_00757a28 {
                        let mut p5 = p!(DAT_0081d32e);
                        let mut p14 = p!(DAT_00757870).cast::<i32>();
                        let mut p13 = p!(DAT_007579d0).cast::<i32>();
                        i_stack_8 = DAT_00757a28;
                        loop {
                            let mut i12 = 0;
                            let i7 = p13.read_unaligned();
                            let mut p6 = p14;
                            if 0 < i7 {
                                loop {
                                    if p6.read_unaligned() != 0 {
                                        break;
                                    }
                                    i12 += 1;
                                    p6 = p6.wrapping_add(1);
                                    if !(i12 < i7) {
                                        break;
                                    }
                                }
                                if i12 < i7 {
                                    i4 = r32(p!(DAT_0081d3fa)) as i32;
                                    i4 = i4.wrapping_add(1);
                                    w32(p!(DAT_0081d3fa), i4 as u32);
                                    w32(p5, u9);
                                }
                            }
                            p5 = p5.wrapping_add(0x14);
                            p14 = p14.wrapping_add(8);
                            p13 = p13.wrapping_add(1);
                            i_stack_8 -= 1;
                            if i_stack_8 == 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        let idx = (DAT_0081bc64 as u32).wrapping_mul(0x58);
        if u2 != 1 && u2 != 2 {
            let i12 = r32(a(p!(DAT_0081f7da), idx));
            u9 = *a(p!(DAT_007438d7), i12.wrapping_mul(0x562)) as u32;
            w32(p!(DAT_0081fb96), u9);
        } else {
            u9 = r32(a(p!(DAT_0081f7fe), idx));
            if u9 == 0 {
                let i7 = r32(a(p!(DAT_0081f802), idx)) as i32;
                let i12 = r32(a(p!(DAT_0081f7da), idx));
                if i7 == -1 || 9 < i7 {
                    u9 = *a(p!(DAT_007438d2), i12.wrapping_mul(0x562)) as u32;
                } else {
                    u9 = *a(
                        p!(DAT_00743dc3),
                        (i7 as u32).wrapping_add(i12.wrapping_mul(0x562)),
                    ) as u32;
                }
            }
            if (u9 as i32) < 1 || (3 < (u9 as i32) && u9 != 0xe) {
                w32(p!(DAT_0081fb96), 1);
                w32(p!(DAT_0081fbda), 0);
            } else {
                w32(p!(DAT_0081fb96), 0);
                w32(p!(DAT_0082e3fe), 0);
            }
        }
        if DAT_0081b850 == 2 {
            u9 = r32(p!(DAT_0081fb96));
            let k = u9.wrapping_mul(0xe824);
            w32(a(p!(DAT_0081fb9e), k), r32(p!(DAT_0081bfb6)));
            w32(a(p!(DAT_0081fba2), k), r32(p!(DAT_0081bfba)));
            u2 = r32(p!(DAT_0081bfba));
            if u2 == 0 {
                w32(a(p!(DAT_0081fba6), k), r32(p!(DAT_0081bfbe)));
            } else if u2 == 1 || u2 == 2 {
                w32(a(p!(DAT_0081fbaa), k), r32(p!(DAT_0081bfc2)));
                w32(a(p!(DAT_0081fbae), k), r32(p!(DAT_0081bfc6)));
            } else {
                w32(a(p!(DAT_0081fbb2), k), r32(p!(DAT_0081bfca)));
                w32(a(p!(DAT_0081fbb6), k), r32(p!(DAT_0081bfce)));
            }
            w32(a(p!(DAT_0081fbda), k), r32(p!(DAT_0081c012)));
            w32(p!(DAT_0083cc22), 0);
            w32(a(p!(DAT_0081fbde), k), r32(p!(DAT_0081c016)));
        } else {
            w32(p!(DAT_0083cbe6), r32(p!(DAT_0081bfb6)));
            w32(p!(DAT_0083cbea), r32(p!(DAT_0081bfba)));
            core::ptr::copy(p!(DAT_0081bfbe) as *const u8, p!(DAT_0083cbee), 20);
            w32(p!(DAT_0083cc22), r32(p!(DAT_0081c012)));
            w32(p!(DAT_0083cc26), r32(p!(DAT_0081c016)));
            w32(p!(DAT_0081fbda), 0);
            w32(p!(DAT_0082e3fe), 0);
        }
        FUN_004ce720();
    }
}
