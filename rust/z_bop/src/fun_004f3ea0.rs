// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa5df0: i32;
    static mut DAT_00aa615e: i32;
    static mut DAT_00aa62ae: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5ccc: i32;
    static mut DAT_00aa5cb0: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d1400();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f3ea0() {
    // SAFETY: all globals and the callee are the game's own memory; pointers
    // are walked over the same ranges and offsets as the original.
    unsafe {
        if DAT_00b69e30 != 2 {
            FUN_004d1400();
        }
        let mut local_8: *mut i32 = &raw mut DAT_00aa5df0;
        loop {
            let mut i_var2: i32 = 0;
            let mut pi_var4: *mut i32 = &raw mut DAT_00aa615e;
            let end: u32 = (&raw mut DAT_00aa62ae) as usize as u32;
            loop {
                let v = pi_var4.read_unaligned();
                if v != 0 && local_8.read_unaligned() == v {
                    break;
                }
                pi_var4 = pi_var4.byte_add(0x2a);
                i_var2 = i_var2.wrapping_add(1);
                if !((pi_var4 as usize as u32) < end) {
                    break;
                }
            }
            if i_var2 == 8 {
                if 0 < DAT_00aa5e68 {
                    let mut pu_var5: *mut i32 = &raw mut DAT_00aa5ccc;
                    let mut i_var3: i32 = 0;
                    let i_var2b: i32 = local_8.read_unaligned();
                    let mut local_4: i32 = DAT_00aa5e68;
                    let base: *mut i32 = &raw mut DAT_00aa5cb0;
                    loop {
                        let mut i_var7: i32 = 0;
                        loop {
                            let idx = i_var3.wrapping_add(i_var7) as isize;
                            if base.offset(idx).read_unaligned() == i_var2b {
                                if i_var7 < 7 {
                                    let mut i_var6 = 7 - i_var7;
                                    let mut pu_var1 = base.offset(idx);
                                    loop {
                                        i_var6 -= 1;
                                        pu_var1.write_unaligned(pu_var1.add(1).read_unaligned());
                                        pu_var1 = pu_var1.add(1);
                                        if i_var6 == 0 {
                                            break;
                                        }
                                    }
                                }
                                pu_var5.write_unaligned(0);
                            }
                            i_var7 = i_var7.wrapping_add(1);
                            if !(i_var7 < 8) {
                                break;
                            }
                        }
                        pu_var5 = pu_var5.offset(8);
                        i_var3 = i_var3.wrapping_add(8);
                        local_4 = local_4.wrapping_sub(1);
                        if local_4 == 0 {
                            break;
                        }
                    }
                }
                local_8.write_unaligned(0);
            }
            local_8 = local_8.add(1);
            if 0xaa5e0f < (local_8 as usize as u32) {
                return;
            }
        }
    }
}
