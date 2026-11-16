// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8a60: [i8; 0];
    static mut DAT_0052772c: [i8; 4];
    static mut DAT_00527744: [i8; 0];
    static mut s_D__BalanceOfPower__00527730: [i8; 19];
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004ccf40() -> *mut i8;
    fn FUN_004ccb90() -> i8;
    fn FUN_0049e060() -> i8;
    fn FUN_0049eb60(param_1: i32);
}

unsafe fn len_incl(mut p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        loop {
            n = n.wrapping_add(1);
            let c = p.read();
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
    }
    n
}

unsafe fn copy_fwd(mut dst: *mut i8, mut src: *const i8, n: u32) {
    // SAFETY: caller guarantees both regions cover n bytes.
    unsafe {
        let mut q = n >> 2;
        while q != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            q -= 1;
        }
        let mut r = n & 3;
        while r != 0 {
            dst.write(src.read());
            src = src.add(1);
            dst = dst.add(1);
            r -= 1;
        }
    }
}

unsafe fn str_cpy(dst: *mut i8, src: *const i8) {
    // SAFETY: src is NUL-terminated, dst is large enough.
    unsafe {
        let n = len_incl(src);
        copy_fwd(dst, src, n);
    }
}

unsafe fn str_cat(dst: *mut i8, src: *const i8) {
    // SAFETY: both are NUL-terminated, dst is large enough.
    unsafe {
        let n = len_incl(src);
        let end = dst.add(len_incl(dst) as usize - 1);
        copy_fwd(end, src, n);
    }
}

unsafe fn try_open(path: *mut i8, mode: *mut i8) -> bool {
    // SAFETY: calls the game's fopen wrapper and stores into its global.
    unsafe {
        let mut i = 4;
        while i != 0 {
            DAT_00a07bd8 = FID_conflict___wfopen(path, mode);
            i -= 1;
            if !DAT_00a07bd8.is_null() {
                return true;
            }
        }
    }
    false
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049e720(
    param_1: *mut i8,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
) -> u32 {
    // SAFETY: the globals and callees are the game's own; the string
    // pointers are the caller's NUL-terminated strings.
    unsafe {
        let buf = (&raw mut DAT_009d8a60).cast::<i8>();
        str_cpy(buf, param_1);
        loop {
            if param_4 != 2 {
                if try_open(param_1, param_2) {
                    return 1;
                }
                let p = FUN_004ccf40();
                str_cpy(buf, p);
                str_cat(buf, (&raw const DAT_00527744).cast::<i8>());
                str_cat(buf, param_1);
                if try_open(buf, param_2) {
                    return 1;
                }
            }
            if param_4 != 1 {
                let s = (&raw const s_D__BalanceOfPower__00527730).cast::<i8>();
                let mut k = 0usize;
                while k < 19 {
                    buf.add(k).write(s.add(k).read());
                    k += 1;
                }
                buf.write(FUN_004ccb90());
                str_cat(buf, param_1);
                if try_open(buf, param_2) {
                    return 1;
                }
                buf.cast::<u32>()
                    .write_unaligned((&raw const DAT_0052772c).cast::<u32>().read_unaligned());
                buf.write(FUN_004ccb90());
                str_cat(buf, param_1);
                if try_open(buf, param_2) {
                    return 1;
                }
            }
            if param_3 == 0 {
                DAT_00a07bd8 = core::ptr::null_mut();
                return 0;
            }
            loop {
                let c = FUN_0049e060();
                if c == b'R' as i8 || c == b'r' as i8 {
                    break;
                }
                if c == b'F' as i8 || c == b'f' as i8 {
                    FUN_0049eb60(1);
                    DAT_00a07bd8 = core::ptr::null_mut();
                    return 0;
                }
            }
        }
    }
}
