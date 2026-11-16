// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e731: [i8; 0x40];
    static mut DAT_0085e770: i8;
    static mut DAT_0085e771: [i8; 0x20];
    static mut DAT_0085e6f0: i8;
    static mut DAT_0085e78f: i8;
    static mut DAT_0081b6dc: i32;
    static mut DAT_0081b6e8: i32;
    static mut DAT_00532308: [i8; 8];
    static mut DAT_00532300: [i8; 8];
    static mut DAT_00531c78: [*mut i8; 0x28];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_004c0640(param_1: *mut i8) -> i32;
    fn FUN_004c02a0(param_1: *mut i8);
    fn _srand(seed: u32);
    fn _rand() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn cmp_n(a: *const i8, b: *const i8, n: i32) -> bool {
    let mut count = n;
    let mut i: usize = 0;
    let mut eq = false;
    // SAFETY: both pointers reference buffers at least n bytes long.
    unsafe {
        loop {
            if count == 0 {
                break;
            }
            count -= 1;
            eq = *a.add(i) == *b.add(i);
            i += 1;
            if !eq {
                break;
            }
        }
    }
    eq
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004c9370(param_1: *mut i8) -> u32 {
    let mut b_var2 = false;
    let mut b_var10 = false;
    let mut local_110: [i8; 16] = [0; 16];
    let mut local_100: [i8; 256] = [0; 256];

    // SAFETY: param_1 is the caller's NUL-terminated string; the globals and
    // callees are the game's own.
    unsafe {
        let mut len: i32 = 0;
        while *param_1.add(len as usize) != 0 {
            len += 1;
        }
        let mut i8_ = 0i32;
        let i5 = len;
        if 0 < i5 {
            loop {
                if *param_1.offset(i8_ as isize) == b'"' as i8 {
                    let mut c_var1 = *param_1.offset((i8_ + 1) as isize);
                    i8_ += 1;
                    let mut i4: i32 = 0;
                    while c_var1 != b'"' as i8 && i8_ < i5 && i4 < 0xff {
                        let c = *param_1.offset(i8_ as isize);
                        i8_ += 1;
                        local_100[i4 as usize] = c;
                        c_var1 = *param_1.offset(i8_ as isize);
                        i4 += 1;
                    }
                    local_100[i4 as usize] = 0;
                    if local_100[0] == 1 && local_100[1] == b'=' as i8 {
                        b_var2 = true;
                        _strncpy(
                            (&raw mut DAT_0085e731).cast::<i8>(),
                            local_100.as_ptr().add(2),
                            0x3f,
                        );
                        DAT_0085e770 = 0;
                    } else {
                        local_100[0] = (local_100[1] == b'=' as i8) as i8;
                        if local_100[1] == b'=' as i8 {
                            _strncpy(local_110.as_mut_ptr(), local_100.as_ptr().add(2), 0xc);
                            local_110[12] = 0;
                            b_var10 = true;
                        }
                    }
                }
                i8_ += 1;
                if !(i8_ < i5) {
                    break;
                }
            }
        }
        if !b_var10 {
            DAT_0081b6dc = 0;
            DAT_0081b6e8 = 0;
            return 1;
        }
        let dst = (&raw mut DAT_0085e771).cast::<i8>();
        let mut skip = false;
        if *dst != 0 {
            if FUN_004c0640(dst) != 0 {
                skip = true;
            }
        }
        if !skip {
            let mut src: *const i8 = local_110.as_ptr();
            let m7 = cmp_n(
                local_110.as_ptr(),
                (&raw const DAT_00532308).cast::<i8>(),
                7,
            );
            let rnd = m7
                || cmp_n(
                    local_110.as_ptr(),
                    (&raw const DAT_00532300).cast::<i8>(),
                    5,
                );
            if rnd {
                let seed = GetTickCount();
                _srand(seed);
                let r = _rand();
                let tbl = (&raw const DAT_00531c78).cast::<*mut i8>();
                src = tbl.add((r % 0x28) as usize).read();
            }
            let mut n = 0usize;
            while *src.add(n) != 0 {
                n += 1;
            }
            n += 1;
            let mut k = 0usize;
            while k < n {
                *dst.add(k) = *src.add(k);
                k += 1;
            }
            FUN_004c02a0(dst);
        }
        if !b_var2 {
            DAT_0081b6e8 = 0;
            DAT_0081b6dc = 0;
            return 1;
        }
        DAT_0085e6f0 = 1;
        DAT_0085e78f = 1;
        1
    }
}
