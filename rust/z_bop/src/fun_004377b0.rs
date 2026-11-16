// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a6fe0: i16;
    static mut DAT_005233d8: i32;
    static mut DAT_00a00530: [u16; 256];
    static mut DAT_00a0813c: i16;
    static mut DAT_009d8c08: i32;
    static mut DAT_005233f4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004377b0(param_1: i32, param_2: i32, param_3: i32, param_4: u8) {
    // SAFETY: the globals are the game's own memory; the destination is the
    // game's frame buffer, written where the original writes.
    unsafe {
        let i_var2 = param_1.wrapping_add(DAT_009a6fe0 as i32);
        let i_var4 = param_2.wrapping_add(DAT_009a6fe0 as i32);
        if DAT_005233d8 == 2 {
            let u_var1 = DAT_00a00530[param_4 as usize];
            if i_var2 < i_var4 {
                let addr = param_3
                    .wrapping_add(DAT_00a0813c as i32)
                    .wrapping_mul(DAT_009d8c08)
                    .wrapping_add(DAT_005233f4)
                    .wrapping_add(i_var2.wrapping_mul(2));
                let mut p = addr as u32 as usize as *mut u8;
                let mut count = (i_var4.wrapping_sub(i_var2) as u32) >> 1;
                let word = (u_var1 as u32) | ((u_var1 as u32) << 16);
                while count != 0 {
                    p.cast::<u32>().write_unaligned(word);
                    p = p.byte_add(4);
                    count -= 1;
                }
                let mut rem = (i_var4.wrapping_sub(i_var2) & 1) as u32;
                while rem != 0 {
                    p.cast::<u16>().write_unaligned(u_var1);
                    p = p.byte_add(2);
                    rem -= 1;
                }
            }
        } else if i_var2 < i_var4 {
            let addr = i_var2
                .wrapping_add(
                    param_3
                        .wrapping_add(DAT_00a0813c as i32)
                        .wrapping_mul(DAT_009d8c08),
                )
                .wrapping_add(DAT_005233f4);
            let mut p = addr as u32 as usize as *mut u8;
            let mut count = (i_var4.wrapping_sub(i_var2) as u32) >> 2;
            let b = param_4 as u32;
            let word = b | (b << 8) | (b << 16) | (b << 24);
            while count != 0 {
                p.cast::<u32>().write_unaligned(word);
                p = p.byte_add(4);
                count -= 1;
            }
            let mut rem = (i_var4.wrapping_sub(i_var2) & 3) as u32;
            while rem != 0 {
                p.write(param_4);
                p = p.byte_add(1);
                rem -= 1;
            }
        }
    }
}
