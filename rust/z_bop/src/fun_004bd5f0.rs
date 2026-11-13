// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052addc: [i8; 1];
    static mut DAT_0051c4bc: [i8; 1];
    static mut DAT_00bb72a0: i32;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b69d1e: [i8; 1];
    static mut DAT_00bb2820: [i8; 1];
    static mut DAT_00bb2821: [i8; 1];
    static mut DAT_00bb2835: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_004cc590(param_1: i32) -> u32;
    fn _fgets(param_1: *mut i8, param_2: i32, param_3: i32) -> *mut i8;
    fn _atoi(param_1: *mut i8) -> i32;
}

unsafe fn slen(mut p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
            n += 1;
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bd5f0() -> i32 {
    let mut local_100 = [0i8; 256];
    // SAFETY: all globals and callees are the game's own memory; pointers
    // walk the game's buffers exactly as the original does.
    unsafe {
        let file = FUN_004cc4a0(
            (&raw mut DAT_0052addc).cast::<i8>(),
            (&raw mut DAT_0051c4bc).cast::<i8>(),
        );
        if file == 0 {
            return 0;
        }
        DAT_00bb72a0 = 0;
        let l = local_100.as_mut_ptr();
        loop {
            let mut p = _fgets((&raw mut DAT_00b69d20).cast::<i8>(), 0x80, file);
            if p.is_null() {
                FUN_004cc590(file);
                return 1;
            }
            let n = slen((&raw mut DAT_00b69d20).cast::<i8>() as *const i8);
            let last = (&raw mut DAT_00b69d1e).cast::<i8>().add(n + 1);
            if *last == 10 {
                *last = 0;
            }
            let mut c = *p;
            let mut i: i32 = 0;
            while c != 0x20
                && {
                    c = *p;
                    c != 0
                }
                && i < 0x100
            {
                p = p.add(1);
                *l.offset(i as isize) = c;
                c = *p;
                i += 1;
            }
            *l.offset(i as isize) = 0;
            p = p.add(1);
            let v = _atoi(l);
            let idx = DAT_00bb72a0.wrapping_mul(0x95);
            *(&raw mut DAT_00bb2820).cast::<i8>().offset(idx as isize) = v as i8;
            c = *p;
            i = 0;
            while c != 0x20
                && {
                    c = *p;
                    c != 0
                }
                && i < 0x100
            {
                p = p.add(1);
                *l.offset(i as isize) = c;
                c = *p;
                i += 1;
            }
            *l.offset(i as isize) = 0;
            let off = DAT_00bb72a0.wrapping_mul(0x95);
            let cnt = slen(l as *const i8) + 1;
            let dst = (&raw mut DAT_00bb2821).cast::<i8>().offset(off as isize);
            let mut k = 0usize;
            while k < cnt {
                *dst.add(k) = *l.add(k);
                k += 1;
            }
            p = p.add(1);
            let cnt2 = slen(p as *const i8) + 1;
            let dst2 = (&raw mut DAT_00bb2835).cast::<i8>().offset(off as isize);
            k = 0;
            while k < cnt2 {
                *dst2.add(k) = *p.add(k);
                k += 1;
            }
            DAT_00bb72a0 = DAT_00bb72a0.wrapping_add(1);
        }
    }
}
