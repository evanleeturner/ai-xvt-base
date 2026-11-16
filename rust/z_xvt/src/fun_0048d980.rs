// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048d980(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: param_1 points at the game's structure; its array pointer at
    // offset 0x14 addresses `count` records of 0x10 ints, read as the original does.
    unsafe {
        let rd = |p: *const i32, i: usize| -> i32 { p.add(i).read_unaligned() };
        let count = ((param_1 as u32).wrapping_add(0x10) as usize as *const i32).read_unaligned();
        let base = ((param_1 as u32).wrapping_add(0x14) as usize as *const i32).read_unaligned();
        let mut i_var3: i32 = -1;
        let mut i_var4: i32 = 0;
        let mut p: *const i32 = (base as u32).wrapping_add(4) as usize as *const i32;
        if 0 < count {
            loop {
                let i_var1 = rd(p, 0);
                if i_var1 == param_2 && rd(p, 1) == param_3 {
                    i_var3 = rd(p, 4);
                    break;
                }
                let i_var2 = rd(p, 1);
                if i_var2 == param_2 {
                    if param_3 == i_var1 {
                        i_var3 = rd(p, 4);
                        break;
                    }
                    if rd(p, 2) == param_3 {
                        i_var3 = rd(p, 5);
                        break;
                    }
                }
                if rd(p, 2) == param_2 && param_3 == i_var2 {
                    i_var3 = rd(p, 5);
                    break;
                }
                let i_var2 = rd(p, 3);
                if i_var2 == -1 {
                    if i_var1 == param_2 && param_3 == rd(p, 2) {
                        i_var3 = rd(p, 6);
                        break;
                    }
                    if rd(p, 2) == param_2 && param_3 == i_var1 {
                        i_var3 = rd(p, 6);
                        break;
                    }
                } else {
                    if rd(p, 2) == param_2 && param_3 == i_var2 {
                        i_var3 = rd(p, 6);
                        break;
                    }
                    if i_var2 == param_2 && param_3 == rd(p, 2) {
                        i_var3 = rd(p, 6);
                        break;
                    }
                    if i_var1 == param_2 && param_3 == i_var2 {
                        i_var3 = rd(p, 7);
                        break;
                    }
                    if i_var2 == param_2 && param_3 == i_var1 {
                        i_var3 = rd(p, 7);
                        break;
                    }
                }
                p = p.add(0x10);
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < count) {
                    break;
                }
            }
        }
        if i_var3 == -1 {
            return i_var3;
        }
        i_var4 = i_var4.wrapping_add(1);
        let count = ((param_1 as u32).wrapping_add(0x10) as usize as *const i32).read_unaligned();
        if count <= i_var4 {
            return i_var3;
        }
        loop {
            let i_var1 = rd(p, 0x10);
            if i_var1 == param_2 && rd(p, 0x11) == param_3 {
                return -1;
            }
            let i_var2 = rd(p, 0x11);
            if i_var2 == param_2 {
                if param_3 == i_var1 {
                    return -1;
                }
                if rd(p, 0x12) == param_3 {
                    return -1;
                }
            }
            if rd(p, 0x12) == param_2 && param_3 == i_var2 {
                return -1;
            }
            let i_var2 = rd(p, 0x13);
            if i_var2 == -1 {
                if i_var1 == param_2 && param_3 == rd(p, 0x12) {
                    return -1;
                }
                if rd(p, 0x12) == param_2 && param_3 == i_var1 {
                    return -1;
                }
            } else {
                if rd(p, 0x12) == param_2 && param_3 == i_var2 {
                    return -1;
                }
                if i_var2 == param_2 && param_3 == rd(p, 0x12) {
                    return -1;
                }
                if i_var1 == param_2 && param_3 == i_var2 {
                    return -1;
                }
                if i_var2 == param_2 && param_3 == i_var1 {
                    return -1;
                }
            }
            i_var4 = i_var4.wrapping_add(1);
            p = p.add(0x10);
            let count =
                ((param_1 as u32).wrapping_add(0x10) as usize as *const i32).read_unaligned();
            if count <= i_var4 {
                return i_var3;
            }
        }
    }
}
