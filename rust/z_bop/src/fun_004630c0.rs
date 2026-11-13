// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004630c0(param_1: *mut u32, param_2: u32) -> u32 {
    let mut p = param_1;
    let mut n = param_2;
    let mut local_4: u32 = 0;
    // SAFETY: param_1 points at the caller's buffer of param_2 bytes; reads
    // are unaligned and sized as the original reads them.
    unsafe {
        if 3 < n {
            let mut count = n >> 2;
            n = n.wrapping_sub(count.wrapping_mul(4));
            loop {
                let x = local_4 ^ p.read_unaligned();
                local_4 = (x << 1) | ((x as i32) < 0) as u32;
                p = p.add(1);
                count = count.wrapping_sub(1);
                if count == 0 {
                    break;
                }
            }
        }
        if n < 2 {
            if n == 1 {
                local_4 ^= u32::from(p.cast::<u8>().read());
                local_4 = (local_4 << 1) | ((local_4 as i32) < 0) as u32;
            }
        } else {
            local_4 ^= u32::from(p.cast::<u16>().read_unaligned());
            local_4 = (local_4 << 1) | ((local_4 as i32) < 0) as u32;
            if n == 3 {
                local_4 ^= u32::from(p.cast::<u8>().byte_add(2).read());
                return (local_4 << 1) | ((local_4 as i32) < 0) as u32;
            }
        }
    }
    local_4
}
