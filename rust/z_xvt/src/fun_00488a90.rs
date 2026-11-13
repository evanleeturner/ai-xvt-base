// SPDX-License-Identifier: MIT

unsafe fn cmp_block(mut a: *const i8, mut b: *const i8) -> bool {
    let mut n: i32 = 0x2000;
    let mut eq: bool = b.is_null();
    loop {
        if n == 0 {
            break;
        }
        n -= 1;
        // SAFETY: the caller supplies buffers of at least 0x2000 readable bytes.
        eq = unsafe { a.read() == b.read() };
        a = a.wrapping_add(1);
        b = b.wrapping_add(1);
        if !eq {
            break;
        }
    }
    eq
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00488a90(param_1: *mut i8, param_2: i32, param_3: i32) -> i32 {
    let mut param_1 = param_1;
    if param_2 == 0 {
        return 0;
    }
    // SAFETY: param_2 and the structures it links to are the game's own
    // memory, read where the original reads them.
    unsafe {
        let rd = |addr: i32| -> i32 { (addr as *const i32).read_unaligned() };
        if rd(param_2.wrapping_add(4)) == 0x14 {
            let p1 = rd(param_2.wrapping_add(0x14));
            let mut iv3 = rd(p1.wrapping_add(0x14)).wrapping_mul(rd(p1.wrapping_add(0x10)));
            if rd(p1.wrapping_add(8)) == iv3 {
                iv3 = rd(p1.wrapping_add(0xc));
            }
            iv3 = p1.wrapping_add(iv3).wrapping_add(0x18);
            if rd(p1.wrapping_add(4)) == 0 {
                if rd(p1) == iv3 {
                    param_1 = param_1.wrapping_add(0x1000);
                    if cmp_block(param_1 as *const i8, iv3.wrapping_add(0x1000) as *const i8) {
                        return iv3;
                    }
                }
            } else {
                param_1 = param_1.wrapping_add(0x1000);
                if cmp_block(param_1 as *const i8, iv3.wrapping_add(0x1000) as *const i8) {
                    return iv3;
                }
            }
        }
        let mut iv3: i32 = 0;
        let mut iv4: i32 = 0;
        if 0 < rd(param_2.wrapping_add(8)) {
            loop {
                let iv2 = rd(rd(param_2.wrapping_add(0xc)).wrapping_add(iv3));
                if param_3 == iv2 {
                    return 0;
                }
                let r = FUN_00488a90(param_1, iv2, param_3);
                if r != 0 {
                    return r;
                }
                iv3 = iv3.wrapping_add(4);
                iv4 = iv4.wrapping_add(1);
                if !(iv4 < rd(param_2.wrapping_add(8))) {
                    break;
                }
            }
        }
        0
    }
}
