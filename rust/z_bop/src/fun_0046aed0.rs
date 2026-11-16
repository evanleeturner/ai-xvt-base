// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527010: [i8; 1];
    static mut DAT_00527008: [i8; 1];
    static mut DAT_00527004: i8;
    static mut DAT_0051c4bc: i8;
    static mut DAT_00a07bd8: *mut c_void;
    static mut DAT_009a8e40: [u8; 1];
    static mut DAT_009d1320: [i8; 1];
    static mut DAT_009d136f: i8;
    static mut DAT_009d1370: i8;
    static mut DAT_009d1371: i8;
    static mut DAT_00a07cf0: [*mut u8; 256];
    static mut DAT_009a8068: i32;
    static mut DAT_00525280: [i8; 1];
    static mut DAT_005252d0: i8;
    static mut DAT_005255b8: [i8; 1];
    static mut DAT_00525608: i8;
    static mut DAT_00526050: [i8; 1];
    static mut DAT_005260a0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(p1: *mut i8, p2: *mut c_void, p3: i32, p4: i32);
    fn FUN_0046adf0(p1: *mut i8, p2: *mut c_void) -> i32;
    fn FUN_0046ac80(p1: *mut i8) -> u32;
    fn FUN_0046acb0() -> i32;
    fn FUN_0046acd0(p1: *mut u8) -> i32;
    fn FUN_0046ad30(p1: *mut u8) -> i32;
    fn FUN_0046ad90(p1: *mut u8) -> i32;
    fn _fclose(p1: *mut c_void) -> i32;
    fn _strncpy(p1: *mut i8, p2: *const i8, p3: u32) -> *mut i8;
    fn _fwrite(p1: *const c_void, p2: u32, p3: u32, p4: *mut c_void) -> u32;
}

unsafe fn str_copy(dst: *mut i8, src: *const i8) {
    let mut i: usize = 0;
    loop {
        // SAFETY: caller supplies a NUL-terminated source and a large enough destination.
        let c = unsafe { *src.add(i) };
        unsafe { *dst.add(i) = c };
        if c == 0 {
            break;
        }
        i += 1;
    }
}

unsafe fn str_cat(dst: *mut i8, src: *const i8) {
    let mut n: usize = 0;
    // SAFETY: dst is NUL-terminated.
    while unsafe { *dst.add(n) } != 0 {
        n += 1;
    }
    unsafe { str_copy(dst.add(n), src) };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046aed0(param_1: *mut i8) -> u32 {
    let mut ab_stack_200: [u8; 256] = [0; 256];
    let mut ac_stack_100: [i8; 256] = [0; 256];
    let mut s_stack_204: u32;
    // SAFETY: globals and callees are the game's own; pointers come from the
    // game's tables and the caller's string, used as the original does.
    unsafe {
        let ab = ab_stack_200.as_mut_ptr();
        let ac = ac_stack_100.as_mut_ptr();
        let base = (&raw mut DAT_009a8e40).cast::<u8>();
        let rec = (&raw mut DAT_009d1320).cast::<i8>();
        let f370 = (&raw mut DAT_009d1370).cast::<i8>();
        let f36f = (&raw mut DAT_009d136f).cast::<i8>();
        let f371 = (&raw mut DAT_009d1371).cast::<i8>();
        let ptrs = (&raw mut DAT_00a07cf0).cast::<*mut u8>();

        str_copy(ac, param_1);
        str_cat(ac, (&raw mut DAT_00527010).cast::<i8>());
        FUN_0049e720(ac, (&raw mut DAT_0051c4bc).cast::<c_void>(), 0, 0);
        let p_file = DAT_00a07bd8;
        if p_file.is_null() {
            return 0;
        }
        let mut puvar7: *mut u8 = base;
        'outer: loop {
            if FUN_0046adf0(ab.cast::<i8>(), p_file) == 0 {
                _fclose(p_file);
                return 0;
            }
            if *ab != 0x2a {
                s_stack_204 = FUN_0046ac80(ab.cast::<i8>());
                if s_stack_204 == 0x100 {
                    s_stack_204 = FUN_0046acb0() as u32;
                    if s_stack_204 == 0x100 {
                        _fclose(p_file);
                        return 0;
                    }
                } else if *f370.byte_add(s_stack_204.wrapping_mul(0x55) as usize) == 1 {
                    _fclose(p_file);
                    return 0;
                }
                let off = s_stack_204.wrapping_mul(0x55) as usize;
                _strncpy(rec.byte_add(off), ab.cast::<i8>(), 0x50);
                *f36f.byte_add(off) = 0;
                *f370.byte_add(off) = 1;
                let i_off: i32 = (puvar7 as usize).wrapping_sub(base as usize) as i32;
                f371.byte_add(off).cast::<i32>().write_unaligned(i_off);
                ptrs.add(s_stack_204 as usize).write(puvar7);
                if FUN_0046adf0(ab.cast::<i8>(), p_file) == 0 {
                    _fclose(p_file);
                    return 0;
                }
                let iv = FUN_0046acd0(ab);
                let o = (iv as u32).wrapping_mul(0x52) as usize;
                if *(&raw mut DAT_00525280).cast::<i8>().byte_add(o) == 0 {
                    _fclose(p_file);
                    return 0;
                }
                *puvar7 = *(&raw mut DAT_005252d0).byte_add(o) as u8;
                if FUN_0046adf0(ab.cast::<i8>(), p_file) == 0 {
                    _fclose(p_file);
                    return 0;
                }
                let iv = FUN_0046ad30(ab);
                let o = (iv as u32).wrapping_mul(0x52) as usize;
                if *(&raw mut DAT_005255b8).cast::<i8>().byte_add(o) == 0 {
                    _fclose(p_file);
                    return 0;
                }
                let mut u_var3: u32 = *(&raw mut DAT_00525608).byte_add(o) as u8 as u32;
                puvar7 = puvar7.add(1);
                loop {
                    *puvar7 = u_var3 as u8;
                    let mut puvar8 = puvar7;
                    loop {
                        if FUN_0046adf0(ab.cast::<i8>(), p_file) == 0 {
                            _fclose(p_file);
                            return 0;
                        }
                        let iv = FUN_0046ad90(ab);
                        let o = (iv as u32).wrapping_mul(0x52) as usize;
                        if *(&raw mut DAT_00526050).cast::<i8>().byte_add(o) == 0 {
                            _fclose(p_file);
                            return 0;
                        }
                        puvar7 = puvar8.add(2);
                        *puvar8.add(1) = *(&raw mut DAT_005260a0).byte_add(o) as u8;
                        if iv == 0 {
                            DAT_009a8068 = DAT_009a8068.wrapping_add(1);
                            continue 'outer;
                        }
                        if FUN_0046adf0(ab.cast::<i8>(), p_file) == 0 {
                            _fclose(p_file);
                            return 0;
                        }
                        u_var3 = FUN_0046ac80(ab.cast::<i8>());
                        if u_var3 != 0x100 {
                            break;
                        }
                        let iv = FUN_0046acb0();
                        if iv == 0x100 {
                            _fclose(p_file);
                            return 0;
                        }
                        let o = (iv as u32).wrapping_mul(0x55) as usize;
                        _strncpy(rec.byte_add(o), ab.cast::<i8>(), 0x50);
                        *f36f.byte_add(o) = 0;
                        *f370.byte_add(o) = 0;
                        *puvar7 = iv as u8;
                        puvar8 = puvar7;
                    }
                }
            }
            break;
        }
        _fclose(p_file);
        s_stack_204 = 0;
        loop {
            let o = s_stack_204.wrapping_mul(0x55) as usize;
            if *rec.byte_add(o) != 0 && *f370.byte_add(o) != 1 {
                return 0;
            }
            s_stack_204 = s_stack_204.wrapping_add(1);
            if (s_stack_204 as i32) >= 0x100 {
                break;
            }
        }
        str_copy(ac, param_1);
        str_cat(ac, (&raw mut DAT_00527008).cast::<i8>());
        FUN_0049e720(ac, (&raw mut DAT_00527004).cast::<c_void>(), 0, 1);
        let p_file = DAT_00a07bd8;
        if !p_file.is_null() {
            let ps = &raw mut s_stack_204;
            ps.write(0x5500);
            _fwrite(ps.cast::<c_void>(), 4, 1, p_file);
            _fwrite(rec.cast::<c_void>(), ps.read(), 1, p_file);
            ps.write(0xffff);
            _fwrite(ps.cast::<c_void>(), 4, 1, p_file);
            _fwrite(base.cast::<c_void>(), ps.read(), 1, p_file);
            _fclose(p_file);
        }
    }
    1
}
