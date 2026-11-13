// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00486000(param_1: *mut i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485fa0(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's own structure; offsets are read and
    // written unaligned exactly as the original does.
    unsafe {
        let mut i_var3: i32 = 0;
        let base = param_1 as i32;
        let i_var5: i32 = base.wrapping_sub(param_1.read_unaligned());
        param_1.write_unaligned(param_1.read_unaligned().wrapping_add(i_var5));
        let p10 = param_1.byte_add(10);
        if p10.read_unaligned() != 0 {
            let mut i_var4: i32 = 0;
            p10.write_unaligned(p10.read_unaligned().wrapping_add(i_var5));
            if 0 < param_1.byte_add(6).read_unaligned() {
                loop {
                    let pi_var2 = (p10.read_unaligned().wrapping_add(i_var3)) as *mut i32;
                    let i_var1 = pi_var2.read_unaligned();
                    if i_var1 != 0 {
                        pi_var2.write_unaligned(i_var1.wrapping_add(i_var5));
                        FUN_00486000(pi_var2, i_var5);
                    }
                    i_var3 = i_var3.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4 < param_1.byte_add(6).read_unaligned()) {
                        break;
                    }
                }
            }
        }
    }
}
