// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041bbf0(param_1: *mut u8, param_2: i16) {
    // SAFETY: param_1 points at the caller's table buffer; reads and writes
    // happen at the same offsets as the original.
    unsafe {
        let mut u_var3: u32 = 0;
        let b_var1: u8 = param_1.read();
        if b_var1 != 0 {
            let mut pb_var2 = param_1.byte_add(0x41);
            loop {
                let tmp = pb_var2.cast::<u16>().read_unaligned();
                if (tmp as i32) == (param_2 as i32) {
                    break;
                }
                pb_var2 = pb_var2.byte_add(2);
                u_var3 = u_var3.wrapping_add(1);
                if !(u_var3 < b_var1 as u32) {
                    break;
                }
            }
        }
        if u_var3 != b_var1 as u32 {
            let mut i_var4: i32 = (u_var3 as i32).wrapping_add(1);
            if i_var4 < b_var1 as i32 {
                let mut pb_var2 = param_1.byte_add((i_var4 * 2 + 0x3f) as usize);
                let mut pb_var5 = param_1.byte_add((i_var4 * 4 - 3) as usize);
                loop {
                    let tmp2 = pb_var2.byte_add(2).cast::<u16>().read_unaligned();
                    pb_var2.cast::<u16>().write_unaligned(tmp2);
                    pb_var2 = pb_var2.byte_add(2);
                    i_var4 = i_var4.wrapping_add(1);
                    let tmp4 = pb_var5.byte_add(4).cast::<u32>().read_unaligned();
                    pb_var5.cast::<u32>().write_unaligned(tmp4);
                    pb_var5 = pb_var5.byte_add(4);
                    if !(i_var4 < param_1.read() as i32) {
                        break;
                    }
                }
            }
            param_1.write(param_1.read().wrapping_sub(1));
        }
    }
}
