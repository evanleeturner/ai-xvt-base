// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa5df0: i32;
    static mut DAT_00aa615e: i32;
    static mut DAT_00aa62ae: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5cb0: i32;
    static mut DAT_00aa5cb4: i32;
    static mut DAT_00aa5ac0: i32;
    static mut DAT_00aa5ac4: i32;
    static mut DAT_00aa5ccc: i32;
    static mut DAT_00aa5adc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d1400();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fa420() {
    // SAFETY: all globals and the callee are the game's own memory; every
    // access is at the same address and width the original uses.
    unsafe {
        if DAT_00b69e30 != 2 {
            FUN_004d1400();
        }
        let b_cb0 = (&raw mut DAT_00aa5cb0) as *mut u8;
        let b_cb4 = (&raw mut DAT_00aa5cb4) as *mut u8;
        let b_ac0 = (&raw mut DAT_00aa5ac0) as *mut u8;
        let b_ac4 = (&raw mut DAT_00aa5ac4) as *mut u8;
        let b_ccc = (&raw mut DAT_00aa5ccc) as *mut u8;
        let b_adc = (&raw mut DAT_00aa5adc) as *mut u8;

        let mut local_8: *mut i32 = &raw mut DAT_00aa5df0;
        loop {
            let mut i_var5: i32 = 0;
            let mut pi_var2: usize = (&raw mut DAT_00aa615e) as usize;
            let i_var1: i32 = local_8.read_unaligned();
            loop {
                if (pi_var2 as *const i32).read_unaligned() == i_var1 {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(0x2a);
                i_var5 = i_var5.wrapping_add(1);
                if pi_var2 >= ((&raw mut DAT_00aa62ae) as usize) {
                    break;
                }
            }

            if i_var5 == 8 {
                if 0 < DAT_00aa5e68 {
                    let mut i_var5: usize = 0;
                    let mut local_4: i32 = DAT_00aa5e68;
                    loop {
                        let mut i_var4: i32 = 0;
                        let mut i_var6: usize = i_var5;
                        loop {
                            if b_cb0.byte_add(i_var6).cast::<i32>().read_unaligned() == i_var1 {
                                if i_var4 < 7 {
                                    let mut i_var7: i32 = 7 - i_var4;
                                    let mut i_var3: usize = i_var6;
                                    loop {
                                        i_var7 = i_var7.wrapping_sub(1);
                                        let v =
                                            b_cb4.byte_add(i_var3).cast::<i32>().read_unaligned();
                                        b_cb0.byte_add(i_var3).cast::<i32>().write_unaligned(v);
                                        let w =
                                            b_ac4.byte_add(i_var3).cast::<i32>().read_unaligned();
                                        b_ac0.byte_add(i_var3).cast::<i32>().write_unaligned(w);
                                        i_var3 = i_var3.wrapping_add(4);
                                        if i_var7 == 0 {
                                            break;
                                        }
                                    }
                                }
                                b_ccc.byte_add(i_var5).cast::<i32>().write_unaligned(0);
                                b_adc.byte_add(i_var5).cast::<i32>().write_unaligned(-1);
                            }
                            i_var6 = i_var6.wrapping_add(4);
                            i_var4 = i_var4.wrapping_add(1);
                            if i_var4 >= 8 {
                                break;
                            }
                        }
                        i_var5 = i_var5.wrapping_add(0x20);
                        local_4 = local_4.wrapping_sub(1);
                        if local_4 == 0 {
                            break;
                        }
                    }
                }
                local_8.write_unaligned(0);
            }
            local_8 = local_8.wrapping_add(1);
            if (local_8 as usize) > 0xaa5e0f {
                return;
            }
        }
    }
}
