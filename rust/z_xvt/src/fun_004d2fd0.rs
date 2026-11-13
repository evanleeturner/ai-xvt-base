// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f440: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3030(param_1: *mut u8, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2fd0(param_1: *mut u8, param_2: i32, param_3: i32) {
    let mut param_1 = param_1;
    // SAFETY: param_3 points at the caller's descriptor, param_1 at its pixel
    // bytes, param_2 at its table; the global is the game's own table.
    unsafe {
        let i_var1 = ((param_3.wrapping_add(4)) as *const i32).read_unaligned();
        let mut i_var4 = ((param_3.wrapping_add(8)) as *const i32).read_unaligned();
        let mut pu_var5 = &raw mut DAT_0064f440;
        let mut i_var3: i32 = 0x80;
        while i_var3 != 0 {
            pu_var5.write_unaligned(0x1000100);
            pu_var5 = pu_var5.add(1);
            i_var3 = i_var3.wrapping_sub(1);
        }
        if 0 < i_var4 {
            loop {
                i_var3 = i_var1;
                let mut pb_var6 = param_1;
                if 0 < i_var1 {
                    loop {
                        param_1 = pb_var6.add(1);
                        let b = pb_var6.read() as u32;
                        let arg = param_2.wrapping_add((b as i32).wrapping_mul(4)) as *mut u8;
                        let u_var2 = FUN_004d3030(arg, b as i32);
                        pb_var6.write(u_var2 as u8);
                        i_var3 = i_var3.wrapping_sub(1);
                        pb_var6 = param_1;
                        if i_var3 == 0 {
                            break;
                        }
                    }
                }
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
        }
    }
}
