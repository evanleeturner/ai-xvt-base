// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_006db7bc: i32;
    static mut DAT_00525518: i32;
    static mut _DAT_006da4fc: i32;
    static mut DAT_0070acc8: i32;
    static mut DAT_00525534: i32;
    static mut DAT_006d4984: i32;
    static mut DAT_007091ae: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004266d0(param_1: *mut i32, param_2: *mut i32) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals are the game's own memory; the pointers are the
    // caller's buffers, read and written exactly where the original does.
    unsafe {
        let mut local_18: i32 = 0;
        let mut pi_var4: *mut i32 = (_DAT_006db7bc
            .wrapping_mul(DAT_00525518)
            .wrapping_add(_DAT_006da4fc.wrapping_mul(DAT_0070acc8))
            .wrapping_add(DAT_00525534)) as u32 as usize
            as *mut i32;
        if DAT_006d4984 != 0 {
            loop {
                let mut local_14: u32 = DAT_007091ae as u32;
                while ((param_1 as usize as u32) & 3) != 0 && local_14 != 0 {
                    local_14 = local_14.wrapping_sub(1);
                    let i_var3 = param_2.read_unaligned();
                    param_2 = param_2.wrapping_add(1);
                    (pi_var4 as *mut u8).write(i_var3 as u8);
                    pi_var4 = pi_var4.wrapping_add(1);
                    param_1 = param_1.wrapping_add(1);
                }
                let mut i_var3: i32 =
                    (local_14.wrapping_add((((local_14 as i32) >> 31) & 3) as u32) as i32) >> 2;
                while i_var3 != 0 {
                    let mut local_c: i32 = i_var3;
                    let mut pi_var5: *mut i32 = param_1;
                    let mut pi_var6: *mut i32 = param_2;
                    loop {
                        if local_c == 0 {
                            break;
                        }
                        local_c = local_c.wrapping_sub(1);
                        let i_var2 = pi_var6.read_unaligned();
                        let i_var1 = pi_var5.read_unaligned();
                        pi_var5 = pi_var5.wrapping_add(1);
                        pi_var6 = pi_var6.wrapping_add(1);
                        if i_var1 != i_var2 {
                            break;
                        }
                    }
                    i_var3 = i_var3.wrapping_sub(local_c.wrapping_add(1));
                    pi_var5 = pi_var4.wrapping_offset(i_var3 as isize);
                    pi_var4 = pi_var5.wrapping_add(1);
                    pi_var5
                        .write_unaligned(param_2.wrapping_offset(i_var3 as isize).read_unaligned());
                    let mut local_8 = param_2.wrapping_offset(i_var3 as isize).wrapping_add(1);
                    let mut local_10 = param_1.wrapping_offset(i_var3 as isize).wrapping_add(1);
                    i_var3 = local_c;
                    param_1 = local_10;
                    param_2 = local_8;
                    if local_c != 0 {
                        loop {
                            pi_var4.write_unaligned(local_8.read_unaligned());
                            pi_var4 = pi_var4.wrapping_add(1);
                            local_8 = local_8.wrapping_add(1);
                            local_10 = local_10.wrapping_add(1);
                            local_c = local_c.wrapping_sub(1);
                            i_var3 = local_c;
                            param_1 = local_10;
                            param_2 = local_8;
                            if local_c == 0 {
                                break;
                            }
                            if local_8.read_unaligned() == local_10.read_unaligned() {
                                break;
                            }
                        }
                    }
                }
                while (local_14 & 3) != 0 {
                    let i_var3 = param_2.read_unaligned();
                    param_2 = param_2.wrapping_add(1);
                    (pi_var4 as *mut u8).write(i_var3 as u8);
                    pi_var4 = pi_var4.wrapping_add(1);
                    param_1 = param_1.wrapping_add(1);
                    local_14 = local_14.wrapping_sub(1);
                }
                local_18 = local_18.wrapping_add(1);
                let diff = (DAT_0070acc8 as u32).wrapping_sub(DAT_007091ae as u32);
                pi_var4 = (pi_var4 as usize as u32).wrapping_add(diff) as usize as *mut i32;
                if !(local_18 < DAT_006d4984) {
                    break;
                }
            }
        }
    }
}
