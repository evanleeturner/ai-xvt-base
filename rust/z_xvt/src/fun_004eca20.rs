// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757c90: i32;
    static mut DAT_00757cb0: i32;
    static mut DAT_00743db7: [i8; 1];
    static mut DAT_007438d6: [i8; 1];
    static mut DAT_007579d0: [i32; 1];
    static mut DAT_00757870: [i32; 1];
    static mut DAT_00757cee: [i32; 1];
    static mut DAT_00757680: [i32; 1];
    static mut DAT_00757d02: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ecce0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004eca20(param_1: i32) -> i8 {
    // SAFETY: all globals and the callee are the game's own memory; reads
    // happen at the same addresses and in the same order as the original.
    unsafe {
        let tbl_a = (&raw mut DAT_00743db7) as *mut i8;
        let tbl_b = (&raw mut DAT_007438d6) as *mut i8;

        if param_1 == -1 {
            let u_var1 = FUN_004ecce0(-1);
            if 0 < (u_var1 as i32) && (((u_var1 as i32) < 4) || u_var1 == 0xe) {
                return 0;
            }
            if DAT_00757c90 != 0 {
                let idx = DAT_00757c90.wrapping_add(DAT_00757cb0.wrapping_mul(0x562));
                return tbl_a.offset(idx as isize).read();
            }
            let idx = DAT_00757cb0.wrapping_mul(0x562);
            if tbl_b.offset(idx as isize).read() != 0 {
                return tbl_b.offset(idx as isize).read();
            }
            return 0;
        }

        let mut local_8: i32 = 0;
        let mut i_var3: i32 = 0;
        let mut pi_var4: *mut i32 = (&raw mut DAT_007579d0) as *mut i32;
        let mut local_4: *mut i32 = (&raw mut DAT_00757870) as *mut i32;
        loop {
            let mut i_var2: i32 = 0;
            if 0 < pi_var4.read_unaligned() {
                let mut pi_var5 = local_4;
                loop {
                    let key = ((&raw mut DAT_00757cee) as *mut u8)
                        .offset(param_1.wrapping_mul(0x2a) as isize)
                        .cast::<i32>()
                        .read_unaligned();
                    if pi_var5.read_unaligned() == key {
                        local_8 = ((&raw mut DAT_00757680) as *mut u8)
                            .offset(((i_var2.wrapping_add(i_var3)).wrapping_mul(4)) as isize)
                            .cast::<i32>()
                            .read_unaligned();
                        break;
                    }
                    pi_var5 = pi_var5.wrapping_add(1);
                    i_var2 = i_var2.wrapping_add(1);
                    if !(i_var2 < pi_var4.read_unaligned()) {
                        break;
                    }
                }
            }
            i_var3 = i_var3.wrapping_add(8);
            local_4 = local_4.wrapping_add(8);
            pi_var4 = pi_var4.wrapping_add(1);
            if (pi_var4 as usize) > 0x7579f7 {
                let u_var1 = FUN_004ecce0(param_1);
                if 0 < (u_var1 as i32) && (((u_var1 as i32) < 4) || u_var1 == 0xe) {
                    return 0;
                }
                let i_var3b = ((&raw mut DAT_00757d02) as *mut u8)
                    .offset(param_1.wrapping_mul(0x2a) as isize)
                    .cast::<i32>()
                    .read_unaligned();
                if i_var3b != 0 {
                    let idx = i_var3b.wrapping_add(local_8.wrapping_mul(0x562));
                    return tbl_a.offset(idx as isize).read();
                }
                let idx = local_8.wrapping_mul(0x562);
                if tbl_b.offset(idx as isize).read() != 0 {
                    return tbl_b.offset(idx as isize).read();
                }
                return 0;
            }
        }
    }
}
