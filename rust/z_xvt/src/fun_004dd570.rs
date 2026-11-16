// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00759657: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dd0b0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd570(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u16) {
    let mut local_54: [i32; 4] = [0; 4];
    let mut local_44: [i32; 4] = [0; 4];
    let mut local_10: i32 = 1;
    let mut local_18: i32 = 1;
    let mut local_14: i32 = 1;

    // SAFETY: globals and callees are the game's own; param_1 points at a
    // rect of four ints; the surface pointer is the game's framebuffer,
    // written where the original does.
    unsafe {
        if !(param_1.read() < param_1.add(2).read()
            && param_1.add(1).read() < param_1.add(3).read())
        {
            return;
        }
        FUN_004dd000(local_44.as_mut_ptr(), param_1);
        FUN_004dd030(local_44.as_mut_ptr(), param_2, param_3);
        if !(local_44[0] < 0x281 && (-1 < local_44[2] && local_44[1] < 0x1e1) && -1 < local_44[3]) {
            return;
        }
        FUN_004dd000(local_54.as_mut_ptr(), local_44.as_mut_ptr());
        FUN_004dd0b0(local_44.as_mut_ptr());
        let i_var1: i32 = (&raw const DAT_0075964b).read_unaligned();
        if local_54[0] != local_44[0] {
            local_18 = 0;
        }
        if local_54[2] != local_44[2] {
            local_14 = 0;
        }
        if local_54[3] != local_44[3] {
            local_10 = 0;
        }
        local_44[2] = (local_44[2] as u32).wrapping_sub(local_44[0] as u32) as i32;
        let mut u_var2: u32 = (local_44[2] as u32).wrapping_add(1);
        let depth: i32 = (&raw const DAT_00759657).read_unaligned();
        let base: u32 = DAT_00758830 as u32;
        if depth == 8 {
            let mut p5: *mut u8 = (local_44[1] as u32)
                .wrapping_mul(i_var1 as u32)
                .wrapping_add(local_44[0] as u32)
                .wrapping_add(base) as usize as *mut u8;
            let u_var4: u8 = param_4 as u8;
            let u_fill: u32 = (u_var4 as u32).wrapping_mul(0x01010101);
            if local_54[1] == local_44[1] {
                let mut p7 = p5;
                let mut n = u_var2 >> 2;
                while n != 0 {
                    p7.cast::<u32>().write_unaligned(u_fill);
                    p7 = p7.wrapping_add(4);
                    n -= 1;
                }
                let mut n = u_var2 & 3;
                while n != 0 {
                    p7.write(u_var4);
                    p7 = p7.wrapping_add(1);
                    n -= 1;
                }
                p5 = p5.wrapping_offset(i_var1 as isize);
            }
            if local_44[1].wrapping_add(1) < local_44[3] {
                local_44[3] =
                    (local_44[3] as u32).wrapping_sub((local_44[1] as u32).wrapping_add(1)) as i32;
                loop {
                    if local_18 != 0 {
                        p5.write(u_var4);
                    }
                    if local_14 != 0 {
                        p5.wrapping_offset(local_44[2] as isize).write(u_var4);
                    }
                    p5 = p5.wrapping_offset(i_var1 as isize);
                    local_44[3] = local_44[3].wrapping_sub(1);
                    if local_44[3] == 0 {
                        break;
                    }
                }
            }
            if local_10 != 0 {
                let mut n = u_var2 >> 2;
                while n != 0 {
                    p5.cast::<u32>().write_unaligned(u_fill);
                    p5 = p5.wrapping_add(4);
                    n -= 1;
                }
                u_var2 &= 3;
                while u_var2 != 0 {
                    p5.write(u_var4);
                    p5 = p5.wrapping_add(1);
                    u_var2 -= 1;
                }
                return;
            }
        } else {
            if depth != 0x10 {
                return;
            }
            let mut p6: *mut u8 = (local_44[1] as u32)
                .wrapping_mul(i_var1 as u32)
                .wrapping_add((local_44[0] as u32).wrapping_mul(2))
                .wrapping_add(base) as usize as *mut u8;
            if local_54[1] == local_44[1] {
                let mut u_var3 = u_var2;
                let mut p8 = p6;
                if 0 < (u_var2 as i32) {
                    while u_var3 != 0 {
                        p8.cast::<u16>().write_unaligned(param_4);
                        p8 = p8.wrapping_add(2);
                        u_var3 -= 1;
                    }
                }
                p6 = p6.wrapping_offset(i_var1 as isize);
            }
            if local_44[1].wrapping_add(1) < local_44[3] {
                local_44[3] =
                    (local_44[3] as u32).wrapping_sub((local_44[1] as u32).wrapping_add(1)) as i32;
                loop {
                    if local_18 != 0 {
                        p6.cast::<u16>().write_unaligned(param_4);
                    }
                    if local_14 != 0 {
                        p6.wrapping_offset((local_44[2] as isize).wrapping_mul(2))
                            .cast::<u16>()
                            .write_unaligned(param_4);
                    }
                    p6 = p6.wrapping_offset(i_var1 as isize);
                    local_44[3] = local_44[3].wrapping_sub(1);
                    if local_44[3] == 0 {
                        break;
                    }
                }
            }
            if local_10 != 0 && 0 < (u_var2 as i32) {
                while u_var2 != 0 {
                    p6.cast::<u16>().write_unaligned(param_4);
                    p6 = p6.wrapping_add(2);
                    u_var2 -= 1;
                }
            }
        }
    }
}
