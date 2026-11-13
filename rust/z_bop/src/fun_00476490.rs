// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233d8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut i32;
    fn FUN_00476670(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00476490(param_1: *mut i32, param_2: i32, param_3: i32) {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own structures, read and written
    // at the offsets the original uses; callees and globals are the game's.
    unsafe {
        while param_1.add(1).read_unaligned() == 7 {
            param_1 = FUN_00479de0(param_2, param_1.add(5).read_unaligned() as usize as *mut i8);
            if param_1.is_null() {
                return;
            }
        }
        if param_1.add(1).read_unaligned() == 0x14 {
            let pi_var1 = param_1.add(5).read_unaligned() as usize as *mut i32;
            if pi_var1.add(1).read_unaligned() == 0 {
                let mut i_var4 = pi_var1.read_unaligned();
                let mut i_var2 = i_var4;
                if DAT_005233d8 == 2 {
                    i_var2 = i_var4.wrapping_add(0x1000);
                }
                let mut i_var3 = pi_var1
                    .add(5)
                    .read_unaligned()
                    .wrapping_mul(pi_var1.add(4).read_unaligned());
                if pi_var1.add(2).read_unaligned() == i_var3 {
                    i_var3 = pi_var1.add(3).read_unaligned();
                }
                if (i_var2.wrapping_sub(0x18).wrapping_sub(i_var3)) as usize as *mut i32 != pi_var1
                {
                    i_var2 = FUN_00476670(param_2, param_3, i_var4);
                    if i_var2 != 0 {
                        let base = i_var2 as usize as *const i32;
                        i_var2 = base.add(5).read_unaligned();
                        let b = i_var2 as usize as *const i32;
                        i_var4 = b
                            .add(5)
                            .read_unaligned()
                            .wrapping_mul(b.add(4).read_unaligned());
                        if b.add(2).read_unaligned() == i_var4 {
                            i_var4 = b.add(3).read_unaligned();
                        }
                        i_var4 = i_var2.wrapping_add(0x18).wrapping_add(i_var4);
                        if DAT_005233d8 == 2 {
                            i_var4 = i_var4.wrapping_sub(0x1000);
                        }
                        pi_var1.write_unaligned(i_var4);
                    }
                }
            } else {
                pi_var1.add(1).write_unaligned(0);
                let mut i_var2 = pi_var1
                    .add(5)
                    .read_unaligned()
                    .wrapping_mul(pi_var1.add(4).read_unaligned());
                if pi_var1.add(2).read_unaligned() == i_var2 {
                    i_var2 = pi_var1.add(3).read_unaligned();
                }
                i_var2 = (pi_var1 as usize as i32)
                    .wrapping_add(i_var2)
                    .wrapping_add(0x18);
                if DAT_005233d8 == 2 {
                    i_var2 = i_var2.wrapping_sub(0x1000);
                }
                pi_var1.write_unaligned(i_var2);
            }
        }
        let mut i_var2: i32 = 0;
        let mut i_var4: i32 = 0;
        if 0 < param_1.add(2).read_unaligned() {
            loop {
                i_var2 = i_var2.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                let arr = param_1.add(3).read_unaligned();
                let child = ((arr.wrapping_add(-4).wrapping_add(i_var2)) as usize as *const i32)
                    .read_unaligned() as usize as *mut i32;
                FUN_00476490(child, param_2, param_3);
                if !(i_var4 < param_1.add(2).read_unaligned()) {
                    break;
                }
            }
        }
    }
}
