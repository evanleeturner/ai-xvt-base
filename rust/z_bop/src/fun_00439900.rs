// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08374: u16;
    static mut DAT_009ecc34: i32;
    static mut DAT_00a0a2d0: [u8; 1];
    static mut DAT_009e9a67: [u8; 1];
    static mut DAT_009e9a69: [u8; 1];
    static mut DAT_00a09240: [u8; 1];
    static mut DAT_00a0924c: [u8; 1];
    static mut DAT_005233c8: u8;
    static mut DAT_009d1300: *const c_void;
    static mut DAT_00a08c70: *mut i8;
    static mut DAT_009a1ed0: [i8; 1];
    static mut DAT_00a0a7d8: i32;
    static mut DAT_00a0a7dc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0043a5e0();
    fn FUN_004419d0();
    fn FUN_004aba50();
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a9640(a: i8);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9590(a: i32);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_0043c290(a: u16, b: i32);
    fn FUN_004a97f0(a: i32);
    fn FUN_004a9860(a: i32);
    fn FUN_004a9820(a: *mut i8);
    fn FUN_004a9a50(a: *mut i8);
    fn FUN_004abe50();
}

unsafe fn rd16(base: *mut u8, off: u32) -> i16 {
    // SAFETY: the caller passes the game's own table and an offset the original uses.
    unsafe { base.byte_add(off as usize).cast::<i16>().read_unaligned() }
}

unsafe fn copy_str(src: *const i8, dst: *mut i8) {
    // SAFETY: copies the NUL-terminated string including the terminator, unbounded as the original does.
    unsafe {
        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut i = 0;
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00439900() {
    let mut ac_stack_50: [i8; 80] = [0; 80];
    // SAFETY: all globals and callees are the game's own memory and code.
    unsafe {
        FUN_0043a5e0();
        FUN_004419d0();
        let mut u_var4: u32 = DAT_00a08374 as u32;
        let mut i_var3: i32 = DAT_009ecc34;
        let a2d0 = &raw mut DAT_00a0a2d0 as *mut u8;
        let p67 = &raw mut DAT_009e9a67 as *mut u8;
        let p69 = &raw mut DAT_009e9a69 as *mut u8;
        let r40 = &raw mut DAT_00a09240 as *mut u8;
        let r4c = &raw mut DAT_00a0924c as *mut u8;
        let buf = &raw mut DAT_009a1ed0 as *mut i8;

        let s_a = rd16(p67, (i_var3 as u32).wrapping_mul(0x5bd));
        if rd16(a2d0, u_var4.wrapping_mul(2)) != s_a {
            FUN_004aba50();
            FUN_0040e0c0(0, 0x140, 200, 0);
            FUN_004a9640(2);
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9590(0x4e);
            let o = (DAT_00a08374 as u32).wrapping_mul(0xc);
            FUN_004a9520(
                rd16(r40, o),
                rd16(r40, o.wrapping_add(2)),
                rd16(r40, o.wrapping_add(8)).wrapping_add(rd16(r40, o)),
                rd16(r40, o.wrapping_add(10)).wrapping_add(rd16(r40, o.wrapping_add(2))),
            );
            let o2 = (DAT_00a08374 as u32).wrapping_mul(0xc);
            FUN_004a9500(rd16(r40, o2), rd16(r40, o2.wrapping_add(2)));
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            let u_var2 = rd16(p67, (DAT_009ecc34 as u32).wrapping_mul(0x5bd)) as u16;
            let mut pc = DAT_00a08c70;
            if u_var2 != 0xffff {
                FUN_0043c290(u_var2, 3);
                pc = buf;
            }
            copy_str(pc, ac_stack_50.as_mut_ptr());
            FUN_004a97f0(DAT_00a0a7d8);
            FUN_004a9860(0x20);
            FUN_004a9820(ac_stack_50.as_mut_ptr());
            FUN_004a9a50(buf);
            FUN_004abe50();
            i_var3 = DAT_009ecc34;
            u_var4 = DAT_00a08374 as u32;
            let s = rd16(p67, (DAT_009ecc34 as u32).wrapping_mul(0x5bd));
            a2d0.byte_add(u_var4.wrapping_mul(2) as usize)
                .cast::<i16>()
                .write_unaligned(s);
        }
        let s_b = rd16(p69, (i_var3 as u32).wrapping_mul(0x5bd));
        if rd16(a2d0, u_var4.wrapping_mul(2).wrapping_add(2)) != s_b {
            FUN_004aba50();
            FUN_0040e0c0(0, 0x140, 200, 0);
            FUN_004a9640(2);
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9590(0x52);
            let o = (DAT_00a08374 as u32).wrapping_mul(0xc);
            FUN_004a9520(
                rd16(r4c, o),
                rd16(r4c, o.wrapping_add(2)),
                rd16(r4c, o.wrapping_add(8)).wrapping_add(rd16(r4c, o)),
                rd16(r4c, o.wrapping_add(10)).wrapping_add(rd16(r4c, o.wrapping_add(2))),
            );
            let o2 = (DAT_00a08374 as u32).wrapping_mul(0xc);
            FUN_004a9500(rd16(r4c, o2), rd16(r4c, o2.wrapping_add(2)));
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            let mut pc = DAT_00a08c70;
            let u_var2 = rd16(p69, (DAT_009ecc34 as u32).wrapping_mul(0x5bd)) as u16;
            if u_var2 != 0xffff {
                let u2 = rd16(p69, (DAT_009ecc34 as u32).wrapping_mul(0x5bd)) as u16;
                FUN_0043c290(u2, 3);
                pc = buf;
            }
            copy_str(pc, ac_stack_50.as_mut_ptr());
            FUN_004a97f0(DAT_00a0a7dc);
            FUN_004a9860(0x20);
            FUN_004a9820(ac_stack_50.as_mut_ptr());
            FUN_004a9a50(buf);
            FUN_004abe50();
            let s = rd16(p69, (DAT_009ecc34 as u32).wrapping_mul(0x5bd));
            a2d0.byte_add((DAT_00a08374 as u32).wrapping_mul(2).wrapping_add(2) as usize)
                .cast::<i16>()
                .write_unaligned(s);
        }
    }
}
