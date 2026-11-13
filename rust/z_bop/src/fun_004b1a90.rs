// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b1810(param_1: *mut i32);
    fn FUN_004b1890(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b1a90(param_1: *mut i32, param_2: u32) {
    // SAFETY: param_1 points at the game's surface descriptor, whose pixel
    // pointer at index 0x17 addresses a buffer of height rows of pitch bytes;
    // the callees are the game's own.
    unsafe {
        FUN_004b1810(param_1);
        let mut p5: *mut u8 = (param_1.add(0x17).read() as u32) as usize as *mut u8;
        let bpp = param_1.add(9).read();
        if bpp == 8 {
            let mut u4: u32 = 0;
            if param_1.add(4).read() != 0 {
                loop {
                    let mut u6: u32 = param_1.add(3).read() as u32;
                    u4 = u4.wrapping_add(1);
                    let mut p3 = p5;
                    let mut u2 = u6 >> 2;
                    while u2 != 0 {
                        p3.cast::<u32>().write_unaligned(param_2);
                        p3 = p3.wrapping_byte_add(4);
                        u2 = u2.wrapping_sub(1);
                    }
                    u6 &= 3;
                    while u6 != 0 {
                        p3.write(param_2 as u8);
                        p3 = p3.wrapping_byte_add(1);
                        u6 = u6.wrapping_sub(1);
                    }
                    p5 = p5.wrapping_byte_offset(param_1.add(6).read() as isize);
                    if !(u4 < param_1.add(4).read() as u32) {
                        break;
                    }
                }
            }
        } else if bpp == 0x10 {
            let mut u4: u32 = 0;
            if param_1.add(4).read() != 0 {
                loop {
                    let mut u6: u32 = 0;
                    let mut p3 = p5;
                    if param_1.add(3).read() != 0 {
                        loop {
                            p3.cast::<u16>().write_unaligned(param_2 as u16);
                            u6 = u6.wrapping_add(1);
                            p3 = p3.wrapping_byte_add(2);
                            if !(u6 < param_1.add(3).read() as u32) {
                                break;
                            }
                        }
                    }
                    p5 = p5.wrapping_byte_offset(param_1.add(6).read() as isize);
                    u4 = u4.wrapping_add(1);
                    if !(u4 < param_1.add(4).read() as u32) {
                        break;
                    }
                }
            }
        } else if bpp == 0x20 {
            let mut u4: u32 = 0;
            if param_1.add(4).read() != 0 {
                loop {
                    let mut u6: u32 = 0;
                    let mut p3 = p5;
                    if param_1.add(3).read() != 0 {
                        loop {
                            p3.cast::<u32>().write_unaligned(param_2);
                            u6 = u6.wrapping_add(1);
                            p3 = p3.wrapping_byte_add(4);
                            if !(u6 < param_1.add(3).read() as u32) {
                                break;
                            }
                        }
                    }
                    p5 = p5.wrapping_byte_offset(param_1.add(6).read() as isize);
                    u4 = u4.wrapping_add(1);
                    if !(u4 < param_1.add(4).read() as u32) {
                        break;
                    }
                }
            }
        }
        FUN_004b1890(param_1);
    }
}
