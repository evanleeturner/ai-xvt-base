// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042bbf0(param_1: *mut u32, param_2: u32) -> u32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut local_4: u32 = 0;
    // SAFETY: param_1 points at the caller's buffer of param_2 bytes; reads
    // happen where the original reads them.
    unsafe {
        if 3 < param_2 {
            let mut u_var1 = param_2 >> 2;
            param_2 = param_2.wrapping_add(u_var1.wrapping_mul(4).wrapping_neg());
            loop {
                let x = local_4 ^ param_1.read_unaligned();
                local_4 = (x << 1) | ((x as i32) < 0) as u32;
                param_1 = param_1.add(1);
                u_var1 = u_var1.wrapping_sub(1);
                if u_var1 == 0 {
                    break;
                }
            }
        }
        if param_2 < 2 {
            if param_2 == 1 {
                local_4 ^= u32::from(param_1.cast::<u8>().read());
                local_4 = (local_4 << 1) | ((local_4 as i32) < 0) as u32;
            }
        } else {
            local_4 ^= u32::from(param_1.cast::<u16>().read_unaligned());
            local_4 = (local_4 << 1) | ((local_4 as i32) < 0) as u32;
            if param_2 == 3 {
                local_4 ^= u32::from(param_1.cast::<u8>().byte_add(2).read());
                return (local_4 << 1) | ((local_4 as i32) < 0) as u32;
            }
        }
    }
    local_4
}
