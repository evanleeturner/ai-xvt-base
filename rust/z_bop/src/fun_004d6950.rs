// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b1f: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d64e0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6950(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u16) {
    let mut local_54: [i32; 4] = [0; 4];
    let mut local_44: [i32; 4] = [0; 4];
    let mut local_10: i32 = 1;
    let mut local_18: i32 = 1;
    let mut local_14: i32 = 1;

    // SAFETY: param_1 points at the caller's rect of four ints; the callees and
    // globals are the game's own; the surface pointer is computed from the
    // game's globals and written where the original writes.
    unsafe {
        if !(param_1.read() < param_1.add(2).read()
            && param_1.add(1).read() < param_1.add(3).read())
        {
            return;
        }
        FUN_004d6480(local_44.as_mut_ptr(), param_1);
        FUN_004d64a0(local_44.as_mut_ptr(), param_2, param_3);
        if !(local_44[0] < 0x281 && (-1 < local_44[2] && local_44[1] < 0x1e1) && -1 < local_44[3]) {
            return;
        }
        FUN_004d6480(local_54.as_mut_ptr(), local_44.as_mut_ptr());
        FUN_004d64e0(local_44.as_mut_ptr());
        let i_var1: i32 = (&raw const DAT_00aa7b13).read_unaligned();
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
        let i_var9: i32 = (&raw const DAT_00aa7b1f).read_unaligned();
        if i_var9 == 8 {
            let base: u32 = (i_var1 as u32)
                .wrapping_mul(local_44[1] as u32)
                .wrapping_add(local_44[0] as u32)
                .wrapping_add(DAT_00aa6cfc as u32);
            let mut puvar4: *mut u8 = base as usize as *mut u8;
            let u_var6: u8 = param_4 as u8;
            let dw_fill: u32 = ((u_var6 as u32) << 24)
                | ((u_var6 as u32) << 16)
                | ((u_var6 as u32) << 8)
                | (u_var6 as u32);
            if local_54[1] == local_44[1] {
                let mut p7 = puvar4;
                let mut n = u_var2 >> 2;
                while n != 0 {
                    p7.cast::<u32>().write_unaligned(dw_fill);
                    p7 = p7.wrapping_add(4);
                    n -= 1;
                }
                let mut n = u_var2 & 3;
                while n != 0 {
                    p7.write(u_var6);
                    p7 = p7.wrapping_add(1);
                    n -= 1;
                }
                puvar4 = puvar4.wrapping_offset(i_var1 as isize);
            }
            if (local_44[1].wrapping_add(1)) < local_44[3] {
                local_44[3] =
                    (local_44[3] as u32).wrapping_sub((local_44[1] as u32).wrapping_add(1)) as i32;
                loop {
                    if local_18 != 0 {
                        puvar4.write(u_var6);
                    }
                    if local_14 != 0 {
                        let a = (local_44[2] as u32).wrapping_add(puvar4 as usize as u32);
                        (a as usize as *mut u8).write(u_var6);
                    }
                    puvar4 = puvar4.wrapping_offset(i_var1 as isize);
                    local_44[3] = local_44[3].wrapping_sub(1);
                    if local_44[3] == 0 {
                        break;
                    }
                }
            }
            if local_10 != 0 {
                let mut n = u_var2 >> 2;
                while n != 0 {
                    puvar4.cast::<u32>().write_unaligned(dw_fill);
                    puvar4 = puvar4.wrapping_add(4);
                    n -= 1;
                }
                u_var2 &= 3;
                while u_var2 != 0 {
                    puvar4.write(u_var6);
                    puvar4 = puvar4.wrapping_add(1);
                    u_var2 -= 1;
                }
                return;
            }
        } else {
            if i_var9 != 0x10 {
                return;
            }
            let base: u32 = (i_var1 as u32)
                .wrapping_mul(local_44[1] as u32)
                .wrapping_add((local_44[0] as u32).wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc as u32);
            let mut puvar5: *mut u16 = base as usize as *mut u16;
            if local_54[1] == local_44[1] {
                let mut u_var3 = u_var2;
                let mut p8 = puvar5;
                if 0 < (u_var2 as i32) {
                    while u_var3 != 0 {
                        p8.write_unaligned(param_4);
                        p8 = p8.wrapping_add(1);
                        u_var3 -= 1;
                    }
                }
                puvar5 = puvar5.wrapping_byte_offset(i_var1 as isize);
            }
            if (local_44[1].wrapping_add(1)) < local_44[3] {
                local_44[3] =
                    (local_44[3] as u32).wrapping_sub((local_44[1] as u32).wrapping_add(1)) as i32;
                loop {
                    if local_18 != 0 {
                        puvar5.write_unaligned(param_4);
                    }
                    if local_14 != 0 {
                        puvar5
                            .wrapping_offset(local_44[2] as isize)
                            .write_unaligned(param_4);
                    }
                    puvar5 = puvar5.wrapping_byte_offset(i_var1 as isize);
                    local_44[3] = local_44[3].wrapping_sub(1);
                    if local_44[3] == 0 {
                        break;
                    }
                }
            }
            if local_10 != 0 && 0 < (u_var2 as i32) {
                while u_var2 != 0 {
                    puvar5.write_unaligned(param_4);
                    puvar5 = puvar5.wrapping_add(1);
                    u_var2 -= 1;
                }
            }
        }
    }
}
