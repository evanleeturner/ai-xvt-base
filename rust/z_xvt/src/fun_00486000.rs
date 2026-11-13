// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_00486000(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the game's own structure whose fields hold
    // 32-bit addresses of further structures; they are read and written
    // exactly where the original does.
    unsafe {
        if param_1.read_unaligned() != 0 {
            param_1.write_unaligned(param_1.read_unaligned().wrapping_add(param_2));
        }
        if param_1.add(5).read_unaligned() != 0 {
            let v = param_1.add(5).read_unaligned();
            param_1.add(5).write_unaligned(v.wrapping_add(param_2));
        }
        if param_1.add(1).read_unaligned() == 0x14 {
            let pi_var1 = param_1.add(5).read_unaligned() as usize as *const i32;
            if pi_var1.add(1).read_unaligned() == 0 {
                let v = param_1.add(5).read_unaligned();
                param_1.add(5).write_unaligned(v.wrapping_add(param_2));
            }
        }
        let mut i_var3: i32 = 0;
        if param_1.add(3).read_unaligned() != 0 {
            let v = param_1.add(3).read_unaligned();
            param_1.add(3).write_unaligned(v.wrapping_add(param_2));
            if 0 < param_1.add(2).read_unaligned() {
                let mut i_var4: i32 = 0;
                loop {
                    let addr =
                        param_1.add(3).read_unaligned().wrapping_add(i_var4) as usize as *mut i32;
                    let i_var2 = addr.read_unaligned();
                    if i_var2 != 0 {
                        let addr2 = param_1.add(3).read_unaligned().wrapping_add(i_var4) as usize
                            as *mut i32;
                        addr2.write_unaligned(i_var2.wrapping_add(param_2));
                        FUN_00486000(i_var2 as usize as *mut i32, param_2);
                    }
                    i_var4 = i_var4.wrapping_add(4);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < param_1.add(2).read_unaligned()) {
                        break;
                    }
                }
            }
        }
    }
}
