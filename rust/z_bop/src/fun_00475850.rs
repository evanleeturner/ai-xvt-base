// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00475850(param_1: i32, param_2: *mut u32, param_3: i32, param_4: u32) {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers come from the game's own structures and are read
    // and written where the original does.
    unsafe {
        while param_2.add(1).read_unaligned() == 7 {
            param_2 = FUN_00479de0(param_3, param_2.add(5).read_unaligned() as *mut i8);
            if param_2.is_null() {
                return;
            }
        }
        if param_2.add(1).read_unaligned() == 0xd {
            let mut pf_var1 = param_2.add(5).read_unaligned() as *mut f32;
            let mut local_4: i32 = 0;
            if 0 < (param_2.add(4).read_unaligned() as i32) {
                loop {
                    let mut pf_var2 = ((param_1 as u32).wrapping_add(0x14) as *const u32)
                        .read_unaligned() as *mut f32;
                    let mut i_var3: i32 = 0;
                    let i_var4 =
                        ((param_1 as u32).wrapping_add(0x10) as *const i32).read_unaligned();
                    let mut b_var5 = i_var4 == 0;
                    if 0 < i_var4 {
                        loop {
                            if pf_var2.read_unaligned() == pf_var1.read_unaligned()
                                && pf_var2.add(1).read_unaligned()
                                    == pf_var1.add(1).read_unaligned()
                            {
                                break;
                            }
                            pf_var2 = pf_var2.wrapping_add(2);
                            i_var3 = i_var3.wrapping_add(1);
                            if !(i_var3 < i_var4) {
                                break;
                            }
                        }
                        b_var5 = i_var4 == i_var3;
                    }
                    if b_var5 {
                        pf_var2.write_unaligned(pf_var1.read_unaligned());
                        pf_var2
                            .add(1)
                            .write_unaligned(pf_var1.add(1).read_unaligned());
                        let cnt = ((param_1 as u32).wrapping_add(0x10) as *mut i32);
                        cnt.write_unaligned(cnt.read_unaligned().wrapping_add(1));
                    }
                    pf_var1 = pf_var1.wrapping_add(2);
                    local_4 = local_4.wrapping_add(1);
                    if !(local_4 < (param_2.add(4).read_unaligned() as i32)) {
                        break;
                    }
                }
            }
        }
        let mut i_var4: i32 = 0;
        if 0 < (param_2.add(2).read_unaligned() as i32) {
            let mut i_var3: i32 = 0;
            loop {
                i_var3 = i_var3.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                let addr = param_2
                    .add(3)
                    .read_unaligned()
                    .wrapping_sub(4)
                    .wrapping_add(i_var3 as u32);
                let child = (addr as *const u32).read_unaligned() as *mut u32;
                FUN_00475850(param_1, child, param_3, param_4);
                if !(i_var4 < (param_2.add(2).read_unaligned() as i32)) {
                    break;
                }
            }
        }
    }
}
