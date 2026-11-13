// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485f10(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the caller's structure, and the offsets
    // stored in it are treated as addresses exactly as the original does.
    unsafe {
        if param_1.read_unaligned() != 0 {
            param_1.write_unaligned(param_1.read_unaligned().wrapping_add(param_2));
        }
        if param_1.add(5).read_unaligned() != 0 {
            let mut i_var2: i32 = param_1.add(5).read_unaligned().wrapping_add(param_2);
            let mut i_var3: i32 = 0;
            param_1.add(5).write_unaligned(i_var2);
            if 0 < param_1.add(4).read_unaligned() {
                loop {
                    let p = i_var2.wrapping_add(8) as usize as *mut i32;
                    if p.read_unaligned() != 0 {
                        p.write_unaligned(p.read_unaligned().wrapping_add(param_2));
                    }
                    i_var2 = i_var2.wrapping_add(0xc);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < param_1.add(4).read_unaligned()) {
                        break;
                    }
                }
            }
        }
        let mut i_var2: i32 = 0;
        if param_1.add(3).read_unaligned() != 0 {
            let mut i_var3: i32 = 0;
            param_1
                .add(3)
                .write_unaligned(param_1.add(3).read_unaligned().wrapping_add(param_2));
            if 0 < param_1.add(2).read_unaligned() {
                loop {
                    let p =
                        param_1.add(3).read_unaligned().wrapping_add(i_var2) as usize as *mut i32;
                    let i_var1 = p.read_unaligned();
                    if i_var1 != 0 {
                        let p = param_1.add(3).read_unaligned().wrapping_add(i_var2) as usize
                            as *mut i32;
                        p.write_unaligned(i_var1.wrapping_add(param_2));
                        let p = param_1.add(3).read_unaligned().wrapping_add(i_var2) as usize
                            as *mut i32;
                        FUN_00485f10(p.read_unaligned() as usize as *mut i32, param_2);
                    }
                    i_var2 = i_var2.wrapping_add(4);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < param_1.add(2).read_unaligned()) {
                        break;
                    }
                }
            }
        }
    }
}
