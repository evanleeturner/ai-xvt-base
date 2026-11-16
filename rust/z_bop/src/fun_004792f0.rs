// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004792f0(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: param_1 points at the caller's structure; its record array at
    // +0x14 is read exactly where the original reads it.
    unsafe {
        let cnt = || ((param_1 + 0x10) as *const i32).read_unaligned();
        let mut i3: i32 = -1;
        let mut i4: i32 = 0;
        let mut p = ((param_1 + 0x14) as *const i32)
            .read_unaligned()
            .wrapping_add(4) as *const i32;
        let r = |p: *const i32, k: usize| p.add(k).read_unaligned();
        if 0 < cnt() {
            loop {
                let i1 = r(p, 0);
                if param_2 == i1 && r(p, 1) == param_3 {
                    i3 = r(p, 4);
                    break;
                }
                let i2 = r(p, 1);
                if param_2 == i2 {
                    if param_3 == i1 {
                        i3 = r(p, 4);
                        break;
                    }
                    if param_2 == i2 && r(p, 2) == param_3 {
                        i3 = r(p, 5);
                        break;
                    }
                }
                if param_2 == r(p, 2) && param_3 == i2 {
                    i3 = r(p, 5);
                    break;
                }
                let i2 = r(p, 3);
                if i2 == -1 {
                    if param_2 == i1 && param_3 == r(p, 2) {
                        i3 = r(p, 6);
                        break;
                    }
                    if param_2 == r(p, 2) && param_3 == i1 {
                        i3 = r(p, 6);
                        break;
                    }
                } else {
                    if param_2 == r(p, 2) && param_3 == i2 {
                        i3 = r(p, 6);
                        break;
                    }
                    if param_2 == i2 && param_3 == r(p, 2) {
                        i3 = r(p, 6);
                        break;
                    }
                    if param_2 == i1 && param_3 == i2 {
                        i3 = r(p, 7);
                        break;
                    }
                    if param_2 == i2 && param_3 == i1 {
                        i3 = r(p, 7);
                        break;
                    }
                }
                p = p.add(0x10);
                i4 = i4.wrapping_add(1);
                if !(i4 < cnt()) {
                    break;
                }
            }
        }
        if i3 == -1 {
            return i3;
        }
        i4 = i4.wrapping_add(1);
        if cnt() <= i4 {
            return i3;
        }
        loop {
            let i1 = r(p, 0x10);
            if param_2 == i1 && r(p, 0x11) == param_3 {
                return -1;
            }
            let i2 = r(p, 0x11);
            if param_2 == i2 {
                if param_3 == i1 {
                    return -1;
                }
                if param_2 == i2 && r(p, 0x12) == param_3 {
                    return -1;
                }
            }
            if param_2 == r(p, 0x12) && param_3 == i2 {
                return -1;
            }
            let i2 = r(p, 0x13);
            if i2 == -1 {
                if param_2 == i1 && param_3 == r(p, 0x12) {
                    return -1;
                }
                if param_2 == r(p, 0x12) && param_3 == i1 {
                    return -1;
                }
            } else {
                if param_2 == r(p, 0x12) && param_3 == i2 {
                    return -1;
                }
                if param_2 == i2 && param_3 == r(p, 0x12) {
                    return -1;
                }
                if param_2 == i1 && param_3 == i2 {
                    return -1;
                }
                if param_2 == i2 && param_3 == i1 {
                    return -1;
                }
            }
            i4 = i4.wrapping_add(1);
            p = p.add(0x10);
            if cnt() <= i4 {
                return i3;
            }
        }
    }
}
