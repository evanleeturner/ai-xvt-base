// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006026bc: i32;
    static mut DAT_0052a350: [f32; 65];
    static mut DAT_005201dc: f32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048e380(param_1: *mut f32, param_2: i32, param_3: i32) {
    // SAFETY: all pointers and globals belong to the game; memory is read and
    // written at the same offsets and in the same order as the original.
    unsafe {
        let mut param_1 = param_1;
        let mut i_var3: i32 = 0;
        if 0 < DAT_006026bc {
            loop {
                let mut i_var5: i32 = 0;
                let mut pi_var4 = param_2.wrapping_add(4) as *const i32;
                let mut pf_var2 = param_2
                    .wrapping_add(4)
                    .wrapping_add(param_3.wrapping_mul(0x40))
                    as *const f32;
                let mut i_var1 = param_3;
                if 0 < param_3 {
                    loop {
                        if pi_var4.read_unaligned() == i_var3
                            || pi_var4.add(1).read_unaligned() == i_var3
                            || pi_var4.add(2).read_unaligned() == i_var3
                            || pi_var4.add(3).read_unaligned() == i_var3
                        {
                            i_var5 = i_var5.wrapping_add(1);
                            if i_var5 == 1 {
                                param_1.write_unaligned(pf_var2.read_unaligned());
                                param_1
                                    .add(1)
                                    .write_unaligned(pf_var2.add(1).read_unaligned());
                                param_1
                                    .add(2)
                                    .write_unaligned(pf_var2.add(2).read_unaligned());
                            } else {
                                param_1.write_unaligned(
                                    param_1.read_unaligned() + pf_var2.read_unaligned(),
                                );
                                param_1.add(1).write_unaligned(
                                    pf_var2.add(1).read_unaligned()
                                        + param_1.add(1).read_unaligned(),
                                );
                                param_1.add(2).write_unaligned(
                                    pf_var2.add(2).read_unaligned()
                                        + param_1.add(2).read_unaligned(),
                                );
                            }
                        }
                        pi_var4 = pi_var4.add(0x10);
                        pf_var2 = pf_var2.add(3);
                        i_var1 = i_var1.wrapping_sub(1);
                        if i_var1 == 0 {
                            break;
                        }
                    }
                }
                if 1 < i_var5 {
                    let local_8: f32 = if i_var5 < 0x41 {
                        let p = &raw mut DAT_0052a350 as *const f32;
                        p.add(i_var5 as usize).read()
                    } else {
                        DAT_005201dc / (i_var5 as f32)
                    };
                    param_1.write_unaligned(param_1.read_unaligned() * local_8);
                    param_1
                        .add(1)
                        .write_unaligned(param_1.add(1).read_unaligned() * local_8);
                    param_1
                        .add(2)
                        .write_unaligned(param_1.add(2).read_unaligned() * local_8);
                }
                param_1 = param_1.add(3);
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < DAT_006026bc) {
                    break;
                }
            }
        }
    }
}
