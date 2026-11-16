// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: [u8; 4];
    static mut DAT_00aa7b1f: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6950(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u16);
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d64e0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d67e0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: u16,
    param_5: i32,
) {
    // SAFETY: the callees and globals are the game's own; param_1 points at a
    // rect of four ints and the surface pointer targets the game's framebuffer.
    unsafe {
        if param_5 == 0 {
            FUN_004d6950(param_1, param_2, param_3, param_4);
            return;
        }
        if !(param_1.read() < param_1.add(2).read()
            && param_1.add(1).read() < param_1.add(3).read())
        {
            return;
        }
        let mut local_28: [i32; 4] = [0; 4];
        let p = local_28.as_mut_ptr();
        FUN_004d6480(p, param_1);
        FUN_004d64a0(p, param_2, param_3);
        if !(local_28[0] < 0x281 && local_28[2] > -1 && local_28[1] < 0x1e1 && local_28[3] > -1) {
            return;
        }
        FUN_004d64e0(p);
        let i_var1: i32 = i32::from_le_bytes(DAT_00aa7b13);
        local_28[3] = local_28[3].wrapping_add(1);
        let u_var4: u32 = (local_28[2] as u32)
            .wrapping_sub(local_28[0] as u32)
            .wrapping_add(1);
        let bpp: i32 = i32::from_le_bytes(DAT_00aa7b1f);
        if bpp == 8 {
            let mut local_8: *mut u8 = (i_var1 as u32)
                .wrapping_mul(local_28[1] as u32)
                .wrapping_add(local_28[0] as u32)
                .wrapping_add(DAT_00aa6cfc as u32) as usize
                as *mut u8;
            if local_28[1] < local_28[3] {
                local_28[3] = local_28[3].wrapping_sub(local_28[1]);
                loop {
                    let u_var3 = param_4 as u8;
                    let fill32: u32 = (u_var3 as u32).wrapping_mul(0x0101_0101);
                    let mut puvar5 = local_8;
                    let mut u_var2 = u_var4 >> 2;
                    while u_var2 != 0 {
                        puvar5.cast::<u32>().write_unaligned(fill32);
                        puvar5 = puvar5.wrapping_add(4);
                        u_var2 -= 1;
                    }
                    u_var2 = u_var4 & 3;
                    while u_var2 != 0 {
                        puvar5.write(u_var3);
                        puvar5 = puvar5.wrapping_add(1);
                        u_var2 -= 1;
                    }
                    local_8 = local_8.wrapping_offset(i_var1 as isize);
                    local_28[3] = local_28[3].wrapping_sub(1);
                    if local_28[3] == 0 {
                        break;
                    }
                }
            }
        } else {
            if bpp != 0x10 {
                return;
            }
            let mut local_8: *mut u8 = (i_var1 as u32)
                .wrapping_mul(local_28[1] as u32)
                .wrapping_add((local_28[0] as u32).wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc as u32) as usize
                as *mut u8;
            if local_28[1] < local_28[3] {
                local_28[3] = local_28[3].wrapping_sub(local_28[1]);
                loop {
                    let mut u_var2 = u_var4;
                    let mut puvar5 = local_8;
                    if 0 < (u_var4 as i32) {
                        while u_var2 != 0 {
                            puvar5.cast::<u16>().write_unaligned(param_4);
                            puvar5 = puvar5.wrapping_add(2);
                            u_var2 -= 1;
                        }
                    }
                    local_8 = local_8.wrapping_offset(i_var1 as isize);
                    local_28[3] = local_28[3].wrapping_sub(1);
                    if local_28[3] == 0 {
                        break;
                    }
                }
            }
        }
    }
}
