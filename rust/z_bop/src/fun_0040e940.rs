// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e940(
    param_1: *mut u8,
    param_2: i32,
    param_3: u32,
    param_4: u32,
) -> u32 {
    let mut param_3 = param_3;
    let mut local_10: i32 = 0x7fffffff;
    if param_3 < param_4 {
        let mut pb_var2 =
            param_2.wrapping_add(param_3.wrapping_mul(3) as i32) as usize as *const u8;
        let mut u_var3: u32 = param_3;
        loop {
            // SAFETY: param_1 points at 3 color bytes and pb_var2 walks the
            // caller's 3-byte palette entries, as in the original.
            let i_var1: i32 = unsafe {
                let a1 = u32::from(*param_1.add(1));
                let b1 = u32::from(*pb_var2.add(1));
                let a0 = u32::from(*param_1);
                let b0 = u32::from(*pb_var2);
                let a2 = u32::from(*param_1.add(2));
                let b2 = u32::from(*pb_var2.add(2));
                let d1 = a1.wrapping_sub(b1);
                let d0 = a0.wrapping_sub(b0);
                let d2 = a2.wrapping_sub(b2);
                d1.wrapping_mul(d1)
                    .wrapping_add(d0.wrapping_mul(d0))
                    .wrapping_add(d2.wrapping_mul(d2)) as i32
            };
            if i_var1 < local_10 {
                param_3 = u_var3;
                local_10 = i_var1;
            }
            pb_var2 = pb_var2.wrapping_add(3);
            u_var3 = u_var3.wrapping_add(1);
            if u_var3 >= param_4 {
                break;
            }
        }
    }
    param_3
}
