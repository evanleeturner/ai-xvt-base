// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: i32) -> i32;
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a6c0(param_1: i32, mut param_2: i32) -> u32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all globals, callees and pointers belong to the game; reads
    // follow the original's structure layout.
    unsafe {
        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let key = (((param_1 as u32) >> 0x10) << 16) | (idx as i32 as u32);
        let pi_var3 = FUN_0048f4f0(key as i32).cast::<i32>();
        if pi_var3.read_unaligned() != pi_var3 as usize as i32 {
            FUN_00485fa0(pi_var3);
        }
        let local_6: i32 = pi_var3.byte_add(6).cast::<i32>().read_unaligned();
        let local_a: *mut i32 =
            (pi_var3.byte_add(0xA).cast::<u32>().read_unaligned()) as usize as *mut i32;
        let first = local_a.read_unaligned();
        if ((first.wrapping_add(4)) as usize as *const i32).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if local_6 <= param_2 {
            param_2 = local_6.wrapping_sub(1);
        }
        let i_var1: i32 = local_a.offset(param_2 as isize).read_unaligned();
        let i_var4: i32 = 'done: {
            if i_var1 != 0 {
                if ((i_var1.wrapping_add(4)) as usize as *const i32).read_unaligned() == 0x19 {
                    break 'done ((i_var1.wrapping_add(0x14)) as usize as *const i32)
                        .read_unaligned();
                }
                let mut i_var5: i32 = 0;
                let mut i_var6: i32 = 0;
                if 0 < ((i_var1.wrapping_add(8)) as usize as *const i32).read_unaligned() {
                    loop {
                        let arr =
                            ((i_var1.wrapping_add(0xC)) as usize as *const i32).read_unaligned();
                        let v = (arr.wrapping_add(i_var5) as usize as *const i32).read_unaligned();
                        if v != 0 {
                            let r = FUN_00449670(v, pi_var3 as usize as i32);
                            if r != 0 {
                                break 'done r;
                            }
                        }
                        i_var5 = i_var5.wrapping_add(4);
                        i_var6 = i_var6.wrapping_add(1);
                        if i_var6
                            >= ((i_var1.wrapping_add(8)) as usize as *const i32).read_unaligned()
                        {
                            break;
                        }
                    }
                }
            }
            0
        };
        let u_var2 = ((i_var4.wrapping_add(4)) as usize as *const u32).read_unaligned();
        FUN_0048f510();
        u_var2 & 1
    }
}
