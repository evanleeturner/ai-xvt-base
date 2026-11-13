// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233c4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040dd50(param_1: *mut u16) {
    // SAFETY: param_1 points at the caller's palette/frame buffer, accessed
    // at the same offsets as the original; globals and callee are the game's.
    unsafe {
        if DAT_005233c4 != 2 {
            param_1.add(0x900).write_unaligned(0);
            return;
        }
        let mut i_stack_18: i32 = 0;
        let mut i_stack_14: i32 = 0;
        let mut i_stack_10: i32 = -1;
        let mut pu_var5: *mut u16 = param_1;
        loop {
            let u_var1: u16 = pu_var5.read_unaligned();
            let u_var2: u32 = (u_var1 & 0x1f) as u32;
            let u_var6: u32 = (u_var1 >> 0xb) as u32;
            let u_var3: u32 = ((u_var1 >> 6) & 0x1f) as u32;
            if u_var2
                .wrapping_mul(u_var2)
                .wrapping_add(u_var3.wrapping_mul(u_var3))
                .wrapping_add(u_var6.wrapping_mul(u_var6))
                < 0x20
            {
                pu_var5.write_unaligned(0);
                i_stack_18 = i_stack_18.wrapping_add(1);
                if i_stack_10 == -1 {
                    i_stack_10 = i_stack_14;
                }
            } else {
                let mut i_var7: i32 = 1;
                let mut pu_var10 = pu_var5;
                loop {
                    pu_var10 = pu_var10.add(0x100);
                    let v: u16 = pu_var10.read_unaligned();
                    let i_var9 = ((v & 0x1f) as u32).wrapping_sub(u_var2) as i32;
                    let i_var4 = (((v >> 6) & 0x1f) as u32).wrapping_sub(u_var3) as i32;
                    let i_var8 = ((v >> 0xb) as u32).wrapping_sub(u_var6) as i32;
                    if 0x10
                        < i_var9
                            .wrapping_mul(i_var9)
                            .wrapping_add(i_var4.wrapping_mul(i_var4))
                            .wrapping_add(i_var8.wrapping_mul(i_var8))
                    {
                        pu_var5.write_unaligned(0);
                        i_stack_18 = i_stack_18.wrapping_add(1);
                        if i_stack_10 == -1 {
                            i_stack_10 = i_stack_14;
                        }
                        break;
                    }
                    i_var7 = i_var7.wrapping_add(1);
                    if i_var7 >= 7 {
                        break;
                    }
                }
                if i_var7 == 7 {
                    let n = pu_var5.add(0xa00).read_unaligned();
                    pu_var5.write_unaligned(n);
                }
            }
            pu_var5 = pu_var5.add(1);
            i_stack_14 = i_stack_14.wrapping_add(1);
            if 0xff < i_stack_14 {
                if i_stack_18 < 0x100 {
                    FUN_004079e0();
                } else {
                    FUN_004079e0();
                    i_stack_18 = 0;
                }
                param_1.add(0x100).write_unaligned(i_stack_10 as u16);
                param_1.add(0x900).write_unaligned(i_stack_18 as u16);
                return;
            }
        }
    }
}
