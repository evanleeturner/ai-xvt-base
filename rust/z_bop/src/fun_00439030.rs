// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: u32;
    static mut DAT_009e9a77: i8;
    static mut DAT_009e9670: i8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08374: u16;
    static mut DAT_00a08ca4: i8;
    static mut DAT_00a08ca6: i8;
    static mut DAT_00a08ca0: i8;
    static mut DAT_00a08ca2: i8;
    static mut DAT_00a08ca8: i8;
    static mut DAT_009a20ae: u8;
    static mut DAT_00a09f58: i8;
    static mut DAT_00a09250: i8;
    static mut DAT_00a09252: i8;
    static mut DAT_00a0924c: i8;
    static mut DAT_00a0924e: i8;
    static mut DAT_00a09254: i8;
    static mut DAT_00a0a134: *mut u8;
    static mut DAT_00a0a130: *mut u8;
    static mut DAT_00a09244: i8;
    static mut DAT_00a09246: i8;
    static mut DAT_00a09240: i8;
    static mut DAT_00a09242: i8;
    static mut DAT_00a09248: i8;
    static mut DAT_00a0a13c: *mut u8;
    static mut DAT_00a0a138: *mut u8;
    static mut DAT_005215e0: i8;
    static mut DAT_005215dc: i8;
    static mut DAT_00521540: i8;
    static mut DAT_00a08e8e: i8;
    static mut DAT_00a08e8c: i8;
    static mut DAT_009a6fe4: *const c_void;
    static mut DAT_00a092a4: i8;
    static mut DAT_00a092a0: i8;
    static mut DAT_00a092a2: i8;
    static mut DAT_00a092a8: i8;
    static mut DAT_00a092aa: i8;
    static mut DAT_00a092a6: i8;
    static mut DAT_00a0a18c: i8;
    static mut DAT_005233ec: i32;
    static mut DAT_00a09288: i8;
    static mut DAT_00a0928a: i8;
    static mut DAT_00a09290: i8;
    static mut DAT_00a09292: i8;
    static mut DAT_009d1300: *const c_void;
    static mut DAT_009a1ed0: i8;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_009ec464: u8;
    static mut DAT_00a08ec8: i16;
    static mut DAT_00a08eca: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9640(c: i8);
    fn FUN_004a9590(v: u32);
    fn FUN_004a95c0(v: u32);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9960(p: *mut u8);
    fn FUN_004278c0(p: *mut c_void) -> u64;
    fn FUN_0043c290(a: u16, b: u8);
    fn FUN_004a9a50(p: *mut c_void);
}

unsafe fn rs(p: *mut i8, off: u32) -> i16 {
    // SAFETY: the game's own table memory, read at a byte offset.
    unsafe { p.byte_add(off as usize).cast::<i16>().read_unaligned() }
}

unsafe fn ru(p: *mut i8, off: u32) -> u16 {
    // SAFETY: the game's own table memory, read at a byte offset.
    unsafe { p.byte_add(off as usize).cast::<u16>().read_unaligned() }
}

unsafe fn rp(p: *mut i8, off: u32) -> *mut u8 {
    // SAFETY: the game's own table memory, read at a byte offset.
    unsafe { p.byte_add(off as usize).cast::<*mut u8>().read_unaligned() }
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address comes from the game's own structures.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00439030(param_1: u32) {
    // SAFETY: all globals and callees are the game's own; pointers read
    // from them are dereferenced where the original does.
    unsafe {
        if DAT_009ecc34 != (param_1 & 0xffff) {
            return;
        }
        let c = (&raw mut DAT_009e9a77)
            .byte_add(((param_1 & 0xffff).wrapping_mul(0x5bd)) as usize)
            .read();
        if !(c == 0 || c == 0x13) {
            return;
        }
        let t0 = (&raw mut DAT_009e9670)
            .byte_add(DAT_009ecc34.wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        let t1 = t0
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32);
        let t2 = rd32(t1);
        let iv = rd32(t2.wrapping_add(0xa9));
        let u_var9 = (iv.wrapping_add(0x1e3) as usize as *const u16).read_unaligned();
        let u_var1 = (iv.wrapping_add(0x1e1) as usize as *const u16).read_unaligned();
        FUN_004a9640(0);
        let mut u14: u32 = 0x7a;
        FUN_004a9590(0x2f);
        loop {
            let i6 = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
            if rs(&raw mut DAT_00a08ca4, i6) != 0 {
                let go = match u14 {
                    0x7a => (u_var1 & 0x200) != 0,
                    0x7b => {
                        if (u_var9 & 1) == 0 {
                            false
                        } else {
                            (u_var1 & 0x800) != 0
                        }
                    }
                    0x7c => (u_var1 & 0x400) != 0,
                    0x7d => {
                        if (u_var9 & 0x100) == 0 {
                            false
                        } else {
                            (u_var1 & 0x1000) != 0
                        }
                    }
                    _ => true,
                };
                if go {
                    FUN_004a95c0(ru(&raw mut DAT_00a08ca6, i6) as u32);
                    let k = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
                    FUN_004a9500(rs(&raw mut DAT_00a08ca0, k), rs(&raw mut DAT_00a08ca2, k));
                    let i6 = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
                    FUN_004a9520(
                        rs(&raw mut DAT_00a08ca0, i6),
                        rs(&raw mut DAT_00a08ca2, i6),
                        rs(&raw mut DAT_00a08ca8, i6).wrapping_add(rs(&raw mut DAT_00a08ca0, i6)),
                        (DAT_009a20ae as u16 as i16).wrapping_add(rs(&raw mut DAT_00a08ca2, i6)),
                    );
                    FUN_004a9960(rp(&raw mut DAT_00a09f58, u14.wrapping_mul(4)));
                }
            }
            let u8_ = (u14 as u16).wrapping_add(1);
            u14 = u8_ as u32;
            if !(u8_ < 0x7e) {
                break;
            }
        }
        if (u_var1 & 0x40) != 0 {
            let m = (DAT_00a08374 as u32).wrapping_mul(12);
            if rs(&raw mut DAT_00a09250, m) != 0 {
                if DAT_00a08374 != 0 {
                    FUN_004a9590(0x4a);
                }
                let m = (DAT_00a08374 as u32).wrapping_mul(12);
                FUN_004a95c0(ru(&raw mut DAT_00a09252, m) as u32);
                FUN_004a9500(rs(&raw mut DAT_00a0924c, m), rs(&raw mut DAT_00a0924e, m));
                let i6 = (DAT_00a08374 as u32).wrapping_mul(12);
                FUN_004a9520(
                    rs(&raw mut DAT_00a0924c, i6),
                    rs(&raw mut DAT_00a0924e, i6),
                    rs(&raw mut DAT_00a09254, i6).wrapping_add(rs(&raw mut DAT_00a0924c, i6)),
                    (DAT_009a20ae as u16 as i16).wrapping_add(rs(&raw mut DAT_00a0924e, i6)),
                );
                let mut pb = DAT_00a0a134;
                if ru(
                    &raw mut DAT_00a09250,
                    (DAT_00a08374 as u32).wrapping_mul(12),
                ) < 5
                {
                    pb = DAT_00a0a130;
                }
                FUN_004a9960(pb);
            }
            let m = (DAT_00a08374 as u32).wrapping_mul(12);
            if rs(&raw mut DAT_00a09244, m) != 0 {
                if DAT_00a08374 != 0 {
                    FUN_004a9590(0x4a);
                }
                let m = (DAT_00a08374 as u32).wrapping_mul(12);
                FUN_004a95c0(ru(&raw mut DAT_00a09246, m) as u32);
                FUN_004a9500(rs(&raw mut DAT_00a09240, m), rs(&raw mut DAT_00a09242, m));
                let i6 = (DAT_00a08374 as u32).wrapping_mul(12);
                FUN_004a9520(
                    rs(&raw mut DAT_00a09240, i6),
                    rs(&raw mut DAT_00a09242, i6),
                    rs(&raw mut DAT_00a09248, i6).wrapping_add(rs(&raw mut DAT_00a09240, i6)),
                    (DAT_009a20ae as u16 as i16).wrapping_add(rs(&raw mut DAT_00a09242, i6)),
                );
                let mut pb = DAT_00a0a13c;
                if ru(
                    &raw mut DAT_00a09244,
                    (DAT_00a08374 as u32).wrapping_mul(12),
                ) < 5
                {
                    pb = DAT_00a0a138;
                }
                FUN_004a9960(pb);
            }
            let c11: i8 = if DAT_00a08374 == 0 { 2 } else { 0 };
            FUN_004a9640(c11);
            let u10 = FUN_004278c0((&raw mut DAT_005215e0).cast());
            let s_var4 = u10 as i16;
            FUN_004a9590(0x4a);
            FUN_004a95c0(0x2c);
            let m = (DAT_00a08374 as u32).wrapping_mul(12);
            let s_var2 = rs(&raw mut DAT_00a08e8e, m);
            let s_var3 = rs(&raw mut DAT_00a08e8c, m);
            let s_var5 = (DAT_009a20ae as u16 as i16).wrapping_add(s_var2);
            let u10 = FUN_004278c0((&raw mut DAT_005215dc).cast());
            FUN_004a9520(
                s_var3.wrapping_add(s_var4),
                s_var2,
                (u10 as i16).wrapping_add(s_var3).wrapping_add(s_var4),
                s_var5,
            );
            let m = (DAT_00a08374 as u32).wrapping_mul(12);
            FUN_004a9500(
                rs(&raw mut DAT_00a08e8c, m).wrapping_add(s_var4),
                rs(
                    &raw mut DAT_00a08e8e,
                    (DAT_00a08374 as u32).wrapping_mul(12),
                ),
            );
            let f: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
            f(0x25);
        }
        if (u_var1 & 0x20) != 0 {
            let mut u14: u32 = 0;
            loop {
                let i6 = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
                if rs(&raw mut DAT_00a092a4, i6) != 0 {
                    FUN_004a9520(
                        rs(&raw mut DAT_00a092a0, i6),
                        rs(&raw mut DAT_00a092a2, i6),
                        rs(&raw mut DAT_00a092a8, i6).wrapping_add(rs(&raw mut DAT_00a092a0, i6)),
                        (DAT_009a20ae as u16 as i16).wrapping_add(rs(&raw mut DAT_00a092a2, i6)),
                    );
                    let k = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
                    FUN_004a9500(rs(&raw mut DAT_00a092a0, k), rs(&raw mut DAT_00a092a2, k));
                    let k = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
                    FUN_004a9590(ru(&raw mut DAT_00a092aa, k) as u32);
                    let k = (DAT_00a08374 as u32).wrapping_add(u14).wrapping_mul(12);
                    FUN_004a95c0(ru(&raw mut DAT_00a092a6, k) as u32);
                    FUN_004a9960(rp(&raw mut DAT_00a0a18c, u14.wrapping_mul(4)));
                }
                let u9 = (u14 as u16).wrapping_add(1);
                u14 = u9 as u32;
                if !(u9 < 2) {
                    break;
                }
            }
        }
        FUN_004a95c0(0x2c);
        FUN_004a9640(0);
        let b12: u8 = if DAT_005233ec == 0x13 { 7 } else { 3 };
        let t0 = (&raw mut DAT_009e9670)
            .byte_add(DAT_009ecc34.wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        FUN_0043c290(t0 as u16, b12);
        let m = (DAT_00a08374 as u32).wrapping_mul(12);
        FUN_004a9500(rs(&raw mut DAT_00a09288, m), rs(&raw mut DAT_00a0928a, m));
        let i6 = (DAT_00a08374 as u32).wrapping_mul(12);
        FUN_004a9520(
            rs(&raw mut DAT_00a09288, i6),
            rs(&raw mut DAT_00a0928a, i6),
            rs(&raw mut DAT_00a09290, i6).wrapping_add(rs(&raw mut DAT_00a09288, i6)),
            rs(&raw mut DAT_00a09292, i6).wrapping_add(rs(&raw mut DAT_00a0928a, i6)),
        );
        let g: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
        g();
        FUN_004a9a50((&raw mut DAT_009a1ed0).cast());
        let c = (&raw mut DAT_009e9a77)
            .byte_add(((param_1 & 0xffff).wrapping_mul(0x5bd)) as usize)
            .read();
        if c == 0 {
            FUN_004a9640(2);
            FUN_004a9520(0, 0, DAT_009a7bac as i16, DAT_009a8c14 as i16);
            FUN_004a95c0(0x40);
            let v: u32 = if DAT_005233ec == 0x13 { 0x4d } else { 0x4e };
            FUN_004a9590(v);
            DAT_009ec464 = 0;
            let u13 = DAT_00a08eca;
            let u10 = FUN_004278c0((&raw mut DAT_00521540).cast());
            FUN_004a9500((u10 as i16).wrapping_add(DAT_00a08ec8), u13);
            let f: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
            f(0x3a);
        }
    }
}
