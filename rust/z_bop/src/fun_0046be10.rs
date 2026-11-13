// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046be10(param_1: u16, param_2: i16) -> u32 {
    let mut ab: [u8; 12] = [0; 12];
    // SAFETY: all addresses are the game's own memory, read where the original reads;
    // the stack writes go through a raw pointer as the C's unchecked indexing does.
    unsafe {
        let rd = |i2: i32, b: u8| -> i16 {
            ((i2.wrapping_add(0x2ac)
                .wrapping_add((b as i32).wrapping_mul(2))) as *const i16)
                .read_unaligned()
        };
        let mut i3: i32 = 0;
        let base = (&raw mut DAT_009e9670) as i32;
        let idx = base.wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd));
        let a = (idx as *const i32).read_unaligned();
        let b = (a
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8)) as *const i32;
        let c = (b.read_unaligned().wrapping_add(0xa9)) as *const i32;
        let i2: i32 = c.read_unaligned();
        loop {
            let k = ((i3.wrapping_add(0x2a1).wrapping_add(i2)) as *const u8).read();
            ab.as_mut_ptr().add(k as usize).write(i3 as u8);
            i3 = i3.wrapping_add(1);
            if !(i3 < 10) {
                break;
            }
        }
        let mut u7: u16 = 0xffff;
        let mut i6: i32 = 0;
        let u5: u16 = ((i3 as u32) >> 16) as u16;
        loop {
            let b1 = ab[i6 as usize];
            if rd(i2, b1) == 0 {
                if param_1 as u32 == b1 as u32 {
                    if param_2 != -1 {
                        loop {
                            let u4: u32 = ((((u5 as u32) << 16) | param_1 as u32) >> 8) & 0xFFFFFF;
                            if i6 + 1 > 9 {
                                i3 = 0;
                                loop {
                                    if rd(i2, ab[i3 as usize]) != 0 {
                                        return ((u4 << 8) | ab[i3 as usize] as u32) & 0xffff00ff;
                                    }
                                    i3 += 1;
                                    if !(i3 < 10) {
                                        break;
                                    }
                                }
                                i3 = 0;
                                loop {
                                    if rd(i2, ab[i3 as usize]) == 0 {
                                        return ((u4 << 8) | ab[i3 as usize] as u32) & 0xffff00ff;
                                    }
                                    i3 += 1;
                                    if !(i3 < 10) {
                                        break;
                                    }
                                }
                                return ((u5 as u32) << 16) | param_1 as u32;
                            }
                            i3 = i6 + 1;
                            i6 += 1;
                            if rd(i2, ab[i3 as usize]) == 0 {
                                return ((u4 << 8) | ab[i3 as usize] as u32) & 0xffff00ff;
                            }
                        }
                    }
                    if u7 != 0xffff {
                        return ((u5 as u32) << 16) | u7 as u32;
                    }
                    i3 = 9;
                    loop {
                        if rd(i2, ab[i3 as usize]) != 0 {
                            return ((((i3 as u32) >> 8) << 8) | ab[i3 as usize] as u32)
                                & 0xffff00ff;
                        }
                        i3 -= 1;
                        if !(-1 < i3) {
                            break;
                        }
                    }
                    return ((((i3 as u32) >> 8) << 8) | ab[9] as u32) & 0xffff00ff;
                }
                u7 = b1 as u16;
            }
            i6 += 1;
            if i6 > 9 {
                return (u5 as u32) << 0x10;
            }
        }
    }
}
