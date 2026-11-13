// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8078: u16;
    static mut DAT_009ed664: u16;
    static mut DAT_009d8110: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe6: u32;
    static mut DAT_00a00870: [i32; 1];
    static mut DAT_00a00874: [i32; 1];
    static mut DAT_009cc450: i16;
    static mut DAT_009a20a6: u16;
    static mut unaff_EBP: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041fc20(param_1: u32);
    fn FUN_00472400(param_1: u32);
    fn FUN_004adc40(param_1: i32) -> i32;
    fn FUN_004ae2a0(param_1: i32, param_2: i32) -> i32;
    fn FUN_004728d0(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042b380(param_1: u16) {
    // SAFETY: all globals, callees and the game-structure addresses
    // derived from them belong to the game; accessed as the original does.
    unsafe {
        'lab: {
            if param_1 != DAT_009a8078 {
                if (DAT_009a8078 as u32).wrapping_sub(param_1 as u32) != 0xFFFF_FFFF {
                    DAT_009ed664 = param_1.wrapping_add(0x7000);
                    break 'lab;
                }
                if param_1 < DAT_009a8078 {
                    DAT_009d8110 = param_1;
                }
            }
            FUN_0041fc20(((unaff_EBP >> 16) << 16) | param_1 as u32);
            FUN_00472400(DAT_009a1fe8.wrapping_add((param_1 as u32).wrapping_mul(0x23)));
        }
        let i_var3: u32 = (param_1 as u32).wrapping_mul(0x23);
        DAT_009a1fe6 = 0;
        let arr0 = &raw mut DAT_00a00870 as *mut i32;
        let arr1 = &raw mut DAT_00a00874 as *mut i32;
        let mut b_var1 = ((DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var3)) as *const u8).read();
        let mut i_var5: i32;
        if b_var1 < 0x49 {
            i_var5 = arr0
                .wrapping_add((b_var1 as usize).wrapping_mul(0x65))
                .read();
        } else {
            i_var5 = FUN_004adc40(b_var1 as i32);
        }
        let mut u_var9: u32 = 0;
        if 0 < i_var5 {
            loop {
                DAT_009a1fe6 = DAT_009a1fe6.wrapping_add(1);
                b_var1 = ((DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var3)) as *const u8).read();
                let u_var6 = b_var1 as u32;
                let i_var7: i32;
                if b_var1 < 0x49 {
                    let mut u_var8 = u_var9;
                    let cnt = arr0
                        .wrapping_add((u_var6 as usize).wrapping_mul(0x65))
                        .read();
                    if cnt <= u_var9 as i32 {
                        u_var8 = (cnt as u32).wrapping_sub(1);
                    }
                    i_var7 = arr1
                        .wrapping_add(
                            (u_var8 as usize).wrapping_add((u_var6 as usize).wrapping_mul(0x65)),
                        )
                        .read();
                } else {
                    i_var7 = FUN_004ae2a0(u_var6 as i32, u_var9 as i32);
                }
                if i_var7 == 1 {
                    break;
                }
                u_var9 = (u_var9 as u16).wrapping_add(1) as u32;
                if !((u_var9 as i32) < i_var5) {
                    break;
                }
            }
        }
        let s_var4 = DAT_009cc450;
        DAT_009a1fe6 = DAT_009a1fe6.wrapping_sub(1);
        if param_1 < DAT_009a8078 {
            DAT_009a20a6 = 1;
            DAT_009cc450 = DAT_009ed664 as i16;
        }
        let u_var9 = DAT_009a1fe6;
        i_var5 =
            ((DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var3)) as *const i32).read_unaligned();
        let p = (i_var5 as u32).wrapping_add(0x8a) as *mut u8;
        let u_var2 = p.read();
        p.write(1);
        FUN_004728d0(DAT_009a1fe8.wrapping_add(i_var3), u_var9);
        let i2 =
            ((DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var3)) as *const i32).read_unaligned();
        ((i2 as u32).wrapping_add(0x8a) as *mut u8).write(u_var2);
        DAT_009cc450 = s_var4;
    }
}
