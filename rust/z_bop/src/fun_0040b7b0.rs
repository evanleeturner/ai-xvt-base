// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00803f80: *mut i32;
    static mut DAT_0066ddcc: *mut i32;
    static mut DAT_005233d8: u32;
    static mut DAT_005233e0: u32;
    static mut DAT_005233e8: u32;
    static mut DAT_0052747c: u16;
    static mut DAT_0066ddc4: u32;
    static mut DAT_0066dde0: u32;
    static mut DAT_009a1fe4: u16;
    static mut DAT_009a7b60: u32;
    static mut DAT_009a8074: u32;
    static mut DAT_009a8e3c: u32;
    static mut DAT_009d682c: u16;
}

type LockFn = unsafe extern "stdcall" fn(*mut i32, i32, *mut core::ffi::c_void, i32, i32) -> i32;
type UnlockFn = unsafe extern "stdcall" fn(*mut i32, u32) -> i32;

unsafe fn vslot(obj: *mut i32, off: usize) -> *const core::ffi::c_void {
    // SAFETY: obj is a game COM-style object whose first word is a vtable.
    unsafe {
        let vt = obj.read() as u32 as usize;
        ((vt + off) as *const u32).read_unaligned() as usize as *const core::ffi::c_void
    }
}

unsafe fn copy(dst: u32, src: u32, n: u32) {
    let mut d = dst;
    let mut s = src;
    let mut c = n >> 2;
    // SAFETY: the game's surfaces are addressed as the original does.
    unsafe {
        while c != 0 {
            ((d as usize) as *mut u32)
                .write_unaligned(((s as usize) as *const u32).read_unaligned());
            s = s.wrapping_add(4);
            d = d.wrapping_add(4);
            c -= 1;
        }
        let mut c = n & 3;
        while c != 0 {
            ((d as usize) as *mut u8).write(((s as usize) as *const u8).read());
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            c -= 1;
        }
    }
}

unsafe fn blit(dst: u32, src: u32, count: u32, d8: u32) {
    // SAFETY: same surfaces as above.
    unsafe {
        if count & 1 == 0 {
            copy(dst, src, count.wrapping_mul(d8));
        } else {
            ((dst as usize) as *mut u16)
                .write_unaligned(((src as usize) as *const u16).read_unaligned());
            if count != 0 {
                copy(
                    dst.wrapping_add(2),
                    src.wrapping_add(2),
                    count.wrapping_sub(1).wrapping_mul(d8),
                );
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040b7b0() {
    let mut a: [u32; 27] = [0; 27];
    let mut p7: u32;
    let i4: u32;
    // SAFETY: globals and the surface objects are the game's own memory.
    unsafe {
        a[0] = 0x6c;
        let mut r: i32;
        let mut pp7: u32;
        let mut pi4: u32;
        loop {
            let f: LockFn = core::mem::transmute(vslot(DAT_00803f80, 100));
            r = f(
                DAT_00803f80,
                0,
                a.as_mut_ptr() as *mut core::ffi::c_void,
                0,
                0,
            );
            pp7 = core::ptr::read_volatile(&a[9]);
            pi4 = core::ptr::read_volatile(&a[4]);
            if r == 0 {
                break;
            }
            if r != -0x7789fde4 {
                return;
            }
        }
        p7 = pp7;
        i4 = pi4;
        a = [0; 27];
        a[0] = 0x6c;
        loop {
            let f: LockFn = core::mem::transmute(vslot(DAT_0066ddcc, 100));
            r = f(
                DAT_0066ddcc,
                0,
                a.as_mut_ptr() as *mut core::ffi::c_void,
                0,
                0,
            );
            if r == 0 {
                break;
            }
            if r != -0x7789fde4 {
                return;
            }
        }
        let d8 = DAT_005233d8;
        let e0 = DAT_005233e0;
        let e8 = DAT_005233e8;
        let pitch2 = core::ptr::read_volatile(&a[4]);

        let mut u8_: u32 = 0;
        p7 = p7.wrapping_add(
            ((DAT_0066dde0.wrapping_sub(e8)) >> 1)
                .wrapping_mul(i4)
                .wrapping_add(((DAT_0066ddc4.wrapping_sub(e0)) >> 1).wrapping_mul(d8)),
        );
        let mut p6: u32 = core::ptr::read_volatile(&a[9]);
        if DAT_009a7b60 != 0 {
            loop {
                blit(p7, p6, e0, d8);
                p6 = p6.wrapping_add(pitch2);
                p7 = p7.wrapping_add(i4);
                u8_ = u8_.wrapping_add(1);
                if !(u8_ < DAT_009a7b60) {
                    break;
                }
            }
        }
        let mut i_stack: i32 = 0;
        let mut pb9: u32 = (DAT_0052747c as u32).wrapping_add(DAT_009a8074);
        if DAT_009a1fe4 != 0 {
            loop {
                let e3c = DAT_009a8e3c;
                if e3c != 0 {
                    blit(p7, p6, e3c, d8);
                    p6 = p6.wrapping_add(e3c.wrapping_mul(d8));
                    p7 = p7.wrapping_add(e3c.wrapping_mul(d8));
                }
                let mut b89: u8 = ((pb9 as usize) as *const u8).read();
                pb9 = pb9.wrapping_add(1);
                let mut u84: u32 = 0;
                let mut pb10 = pb9;
                if DAT_009d682c != 0 {
                    loop {
                        let mut v: u32 = ((pb10 as usize) as *const u8).read() as u32;
                        pb9 = pb10.wrapping_add(1);
                        if v == 0 {
                            v = ((pb9 as usize) as *const u8).read() as u32;
                            pb9 = pb10.wrapping_add(2);
                            if v == 0 {
                                let b1 = ((pb9 as usize) as *const u8).read();
                                pb9 = pb10.wrapping_add(3);
                                v = (b1 as u32).wrapping_add(0x100);
                            }
                            v = v.wrapping_add(0xff);
                        }
                        if (b89 as i8) < 0 {
                            blit(p7, p6, v, d8);
                        }
                        p6 = p6.wrapping_add(v.wrapping_mul(d8));
                        p7 = p7.wrapping_add(v.wrapping_mul(d8));
                        u84 = u84.wrapping_add(v);
                        b89 = b89.wrapping_neg();
                        pb10 = pb9;
                        if !((u84 as i32) < (DAT_009d682c as u32 as i32)) {
                            break;
                        }
                    }
                }
                u84 = u84.wrapping_add(DAT_009a8e3c);
                if u84 < e0 {
                    u84 = e0.wrapping_sub(u84);
                    blit(p7, p6, u84, d8);
                    p6 = p6.wrapping_add(u84.wrapping_mul(d8));
                    p7 = p7.wrapping_add(u84.wrapping_mul(d8));
                    u84 = e0;
                }
                p6 = p6.wrapping_add(pitch2.wrapping_sub(u84.wrapping_mul(d8)));
                p7 = p7.wrapping_add(i4.wrapping_sub(u84.wrapping_mul(d8)));
                i_stack = i_stack.wrapping_add(1);
                if !(i_stack < DAT_009a1fe4 as u32 as i32) {
                    break;
                }
            }
        }
        let mut u8b: u32 = (DAT_009a1fe4 as u32).wrapping_add(DAT_009a7b60);
        if u8b < e8 {
            loop {
                blit(p7, p6, e0, d8);
                p6 = p6.wrapping_add(pitch2);
                p7 = p7.wrapping_add(i4);
                u8b = u8b.wrapping_add(1);
                if !(u8b < e8) {
                    break;
                }
            }
        }
        let f: UnlockFn = core::mem::transmute(vslot(DAT_0066ddcc, 0x80));
        f(DAT_0066ddcc, core::ptr::read_volatile(&a[9]));
        let g: UnlockFn = core::mem::transmute(vslot(DAT_00803f80, 0x80));
        g(DAT_00803f80, core::ptr::read_volatile(&a[4]));
    }
}
