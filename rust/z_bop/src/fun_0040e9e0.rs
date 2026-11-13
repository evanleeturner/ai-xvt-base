// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e940(param_1: *mut u8, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e9e0(param_1: i32, param_2: u32, param_3: u32) {
    let mut buf: [u8; 3] = [0; 3];
    let mut i_var2: i32 = 0;
    loop {
        let sh = i_var2 >> 5;
        buf[1] = (sh as u8) & 0x3f;
        buf[0] = ((((sh as u32) >> 6) as u8) & 0x1f).wrapping_mul(2);
        let i_var3 = i_var2.wrapping_add(1);
        buf[2] = ((i_var3 as u8) & 0x1f).wrapping_mul(2);
        // SAFETY: the callee is the game's own and reads the three key bytes
        // in buf; the output byte is written at the caller's buffer.
        unsafe {
            let u_var1 = FUN_0040e940(buf.as_mut_ptr(), 0x9a7bc0, param_2, param_3);
            ((param_1.wrapping_sub(1).wrapping_add(i_var3)) as usize as *mut i8)
                .write(u_var1 as i8);
        }
        i_var2 = i_var3;
        if i_var3 >= 0x10000 {
            break;
        }
    }
}
