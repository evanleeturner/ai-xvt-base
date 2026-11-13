// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005270b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af180(mut param_1: *mut u32, param_2: i32) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: param_1 points at the game's node structure and its child
    // array; the globals and callee are the game's own.
    unsafe {
        loop {
            if param_1.add(1).read_unaligned() != 7 {
                let mut local_4: u32 = (param_1.add(1).read_unaligned() == 0x16) as u32;
                let mut i_var4: i32 = 0;
                if 0 < param_1.add(2).read_unaligned() as i32 {
                    let mut i_var3: u32 = 0;
                    loop {
                        let p_var1 =
                            (param_1.add(3).read_unaligned().wrapping_add(i_var3)) as *const u32;
                        i_var3 = i_var3.wrapping_add(4);
                        i_var4 = i_var4.wrapping_add(1);
                        let u_var2 = FUN_004af180(p_var1.read_unaligned() as *mut u32, param_2);
                        local_4 = local_4.wrapping_add(u_var2);
                        if !(i_var4 < param_1.add(2).read_unaligned() as i32) {
                            break;
                        }
                    }
                }
                return local_4;
            }
            if DAT_005270b0 == 0 {
                param_1 = FUN_00479de0(param_2, param_1.add(5).read_unaligned() as *mut i8);
            } else if *(param_1.add(5).read_unaligned() as *const i8) == 0 {
                param_1 = param_1.read_unaligned() as *mut u32;
            } else {
                let p_var1 = FUN_00479de0(param_2, param_1.add(5).read_unaligned() as *mut i8);
                param_1.write_unaligned(p_var1 as u32);
                *(param_1.add(5).read_unaligned() as *mut u8) = 0;
                param_1 = param_1.read_unaligned() as *mut u32;
            }
            if param_1.is_null() {
                break;
            }
        }
    }
    0
}
