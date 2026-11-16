// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f654: i32;
    static mut DAT_00532a38: i32;
    static mut DAT_0081bcb8: i32;
    static mut DAT_00532ab4: i32;
    static mut DAT_00532aac: i32;
    static mut DAT_0081b6fc: u32;
    static mut DAT_0081b700: i32;
    static mut DAT_0085e7c5: i8;
    static mut DAT_00531990: [i8; 1];
    static mut DAT_007585b0: [i8; 100];
    static mut DAT_00758620: i32;
    static mut DAT_00758624: [i8; 1];
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_0081bcf0: [i8; 1];
    static mut DAT_00525e04: [i8; 1];
    static mut DAT_007579b0: [i32; 8];
    static mut DAT_00757870: [i32; 1];
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_00859c2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbea0();
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004d0e60(a: *mut core::ffi::c_void, b: i32, c: i32);
    fn FUN_004dcfd0(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004da920(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004dd7d0(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004de190(a: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004d9660(a: *mut i32, b: *mut i8, c: i32, d: i32, e: i32, f: *mut i8) -> i32;
    fn FUN_004cdbc0() -> i32;
    fn FUN_004cdca0(a: i32) -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: u32);
    fn FUN_004daee0(a: i32, b: i32, c: *mut i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004d9010(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32;
}

unsafe fn xvt_strlen(p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated game string.
    unsafe {
        while *p.add(n) != 0 {
            n += 1;
        }
    }
    n
}

unsafe fn xvt_strcat(dst: *mut i8, src: *const i8) {
    // SAFETY: dst and src are NUL-terminated game buffers, dst large enough.
    unsafe {
        let sl = xvt_strlen(src);
        let dl = xvt_strlen(dst);
        let mut i = 0usize;
        while i <= sl {
            *dst.add(dl + i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d7900(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; buffers are the
    // game's fixed buffers used exactly as the original does.
    unsafe {
        let mut ai: [i32; 4] = [0; 4];
        let mut i_stack_8: i32 = 0;
        let mut i_stack_4: i32 = 0;
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let inp = (&raw mut DAT_007585b0).cast::<i8>();

        if param_1 == 0 {
            FUN_004dbea0();
            DAT_0064f654 = 0;
        }
        if DAT_0081bcb8 == 0 {
            DAT_00532a38 = 0;
        } else {
            FUN_004ddc70(&raw mut i_stack_4, &raw mut i_stack_8);
            if DAT_00532a38 == 0 {
                FUN_004d0e60((&raw mut DAT_00532ab4).cast(), 0, 0);
                FUN_004dcfd0(ai.as_mut_ptr(), 0x22c, 0x5f, 0x24c, 0x69);
                let u = DAT_0081b6fc;
                let p = FUN_004dcfa0(0xcc);
                FUN_004da920(10, p, ai.as_mut_ptr(), u);
                let r = FUN_004dd7d0(ai.as_mut_ptr(), i_stack_4, i_stack_8);
                if r != 0 && (FUN_004db570() != 0 || FUN_004db590() != 0) {
                    if DAT_0085e7c5 != 0 {
                        FUN_004de190((&raw mut DAT_00531990).cast(), 1, 0, 0xff);
                    }
                    DAT_00532a38 = 1;
                }
                FUN_004d0e60((&raw mut DAT_00532aac).cast(), 0, 0);
                FUN_004dcfd0(ai.as_mut_ptr(), 0x208, 0x5f, 0x228, 0x69);
                let p = FUN_004dcfa0(0x185);
                FUN_004da920(10, p, ai.as_mut_ptr(), 0xffff);
            } else {
                FUN_004d0e60((&raw mut DAT_00532aac).cast(), 0, 0);
                FUN_004dcfd0(ai.as_mut_ptr(), 0x208, 0x5f, 0x228, 0x69);
                let u = DAT_0081b6fc;
                let p = FUN_004dcfa0(0x185);
                FUN_004da920(10, p, ai.as_mut_ptr(), u);
                let r = FUN_004dd7d0(ai.as_mut_ptr(), i_stack_4, i_stack_8);
                if r != 0 && (FUN_004db570() != 0 || FUN_004db590() != 0) {
                    if DAT_0085e7c5 != 0 {
                        FUN_004de190((&raw mut DAT_00531990).cast(), 1, 0, 0xff);
                    }
                    DAT_00532a38 = 0;
                }
                FUN_004d0e60((&raw mut DAT_00532ab4).cast(), 0, 0);
                FUN_004dcfd0(ai.as_mut_ptr(), 0x22c, 0x5f, 0x24c, 0x69);
                let p = FUN_004dcfa0(0xcc);
                FUN_004da920(10, p, ai.as_mut_ptr(), 0xffff);
            }
        }
        FUN_004dcfd0(ai.as_mut_ptr(), 0x1cd, 0x196, 0x253, 0x1a8);
        let r = FUN_004d9660(ai.as_mut_ptr(), inp, 100, 0, 0xc, core::ptr::null_mut());
        let mut i_var4: i32;
        'skip: {
            if r == 0 || *inp == 0 {
                break 'skip;
            }
            DAT_00758620 = 0x4d;
            let t = FUN_004cdbc0();
            let i_var5 = FUN_004cdca0(t);
            i_var4 = DAT_00532a38;
            if i_var5 == 0 {
                *buf = 4;
            } else if DAT_00532a38 == 1 {
                *buf = 2;
            } else {
                *buf = 3;
            }
            *buf.add(1) = 0;
            xvt_strcat(buf, (&raw mut DAT_0081bcf0).cast());
            xvt_strcat(buf, (&raw mut DAT_00525e04).cast());
            let len = xvt_strlen(buf);
            *buf.add(len) = 1;
            *buf.add(len + 1) = 0;
            xvt_strcat(buf, inp);
            let len = xvt_strlen(buf);
            let out = (&raw mut DAT_00758624).cast::<i8>();
            let mut k = 0usize;
            while k < len {
                *out.add(k) = *buf.add(k);
                k += 1;
            }
            let total = (len as u32).wrapping_add(1).wrapping_add(3);
            let mut send_zero = false;
            if i_var4 == 0 {
                send_zero = true;
            } else {
                i_var4 = 0;
                let tab = (&raw mut DAT_007579b0).cast::<i32>();
                let mut pi = 0usize;
                loop {
                    let v = FUN_004cdbc0();
                    if v == tab.add(pi).read() {
                        break;
                    }
                    pi += 1;
                    i_var4 += 1;
                    if pi >= 8 {
                        break;
                    }
                }
                if i_var4 == 8 {
                    send_zero = true;
                }
            }
            if send_zero {
                FUN_004cb830(0, (&raw mut DAT_00758620).cast::<u32>(), total);
            } else {
                i_var4 = 0;
                loop {
                    let base = (&raw mut DAT_0081bfb6).cast::<i32>().read_unaligned();
                    let idx = (i_var4 as u32).wrapping_add((base as u32).wrapping_mul(8)) as i32;
                    let e = (&raw mut DAT_00757870)
                        .cast::<i32>()
                        .offset(idx as isize)
                        .read();
                    if e != 0 {
                        FUN_004cb830(e, (&raw mut DAT_00758620).cast::<u32>(), total);
                    }
                    i_var4 += 1;
                    if i_var4 >= 8 {
                        break;
                    }
                }
            }
            let pu = (&raw mut DAT_007585b0).cast::<u32>();
            let mut k = 0usize;
            while k < 0x19 {
                pu.add(k).write(0);
                k += 1;
            }
        }
        FUN_004dcfd0(ai.as_mut_ptr(), 0x1cd, 0x75, 0x253, 0x197);
        let i_var5 = FUN_004daee0(0xc, DAT_0081b700, ai.as_mut_ptr(), 0xffff, 2, 0x400);
        i_var4 = (i_var5 as u32).wrapping_sub(0x14) as i32;
        if i_var5 < 0x15 {
            i_var4 = 0;
        }
        if 0 < i_var4 {
            i_var4 = (i_var4 as u32).wrapping_add(1) as i32;
            if DAT_0064f654 == i_var4 {
                DAT_0064f654 = (DAT_0064f654 as u32).wrapping_sub(1) as i32;
            }
            FUN_004dcfd0(ai.as_mut_ptr(), 0x254, 0x75, 0x25d, 0x197);
            let b = (i_var4 as u32)
                .wrapping_sub(DAT_0064f654 as u32)
                .wrapping_sub(1) as i32;
            let r = FUN_004d9010(ai.as_mut_ptr(), b, i_var4, 0, 5, DAT_00859c2c, 7);
            DAT_0064f654 = (i_var4 as u32).wrapping_sub(r as u32).wrapping_sub(1) as i32;
        }
        FUN_004dcfd0(ai.as_mut_ptr(), 0x1cd, 0x75, 0x253, 0x18e);
        let f = (i_var4 as u32)
            .wrapping_sub(DAT_0064f654 as u32)
            .wrapping_sub(1) as i32;
        FUN_004daee0(0xc, DAT_0081b700, ai.as_mut_ptr(), 0xffff, 2, f);
    }
}
