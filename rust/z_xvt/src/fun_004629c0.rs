// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da540: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004629c0(param_1: i32, param_2: u32, param_3: u32) {
    let mut local_14: i32 = 0;
    loop {
        let mut local_10: u32 = param_2;
        let mut u_var5: u32 = local_10;
        let mut i_var1: i32 = 0x7fffffff;
        if param_2 < param_3 {
            // SAFETY: the palette table is the game's own memory; entries
            // param_2..param_3 are read three bytes each, as the original does.
            unsafe {
                let mut pbv: *const u8 = (&raw mut DAT_006da540)
                    .cast::<u8>()
                    .cast_const()
                    .byte_add(param_2.wrapping_mul(3) as usize);
                loop {
                    let r = (((((local_14 >> 5) as u32) >> 6) as u8 & 0x1f).wrapping_mul(2)) as i32;
                    let g = ((local_14 >> 5) as u8 & 0x3f) as i32;
                    let b = ((local_14 as u8 & 0x1f).wrapping_mul(2)) as i32;
                    let i_var2 = r.wrapping_sub(pbv.read() as i32);
                    let i_var3 = g.wrapping_sub(pbv.add(1).read() as i32);
                    let i_var4 = b.wrapping_sub(pbv.add(2).read() as i32);
                    let d = i_var4
                        .wrapping_mul(i_var4)
                        .wrapping_add(i_var3.wrapping_mul(i_var3))
                        .wrapping_add(i_var2.wrapping_mul(i_var2));
                    if d < i_var1 {
                        i_var1 = d;
                        local_10 = u_var5;
                    }
                    pbv = pbv.add(3);
                    u_var5 = u_var5.wrapping_add(1);
                    if u_var5 >= param_3 {
                        break;
                    }
                }
            }
        }
        local_14 = local_14.wrapping_add(1);
        // SAFETY: param_1 is the caller's output buffer of 0x10000 bytes.
        unsafe {
            ((param_1.wrapping_add(local_14).wrapping_sub(1)) as *mut u8).write(local_10 as u8);
        }
        if local_14 >= 0x10000 {
            break;
        }
    }
}
