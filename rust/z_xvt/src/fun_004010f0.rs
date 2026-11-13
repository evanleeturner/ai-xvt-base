// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da9f8: u16;
    static mut DAT_0071f714: u16;
    static mut DAT_0070a1d0: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_006d498c: u16;
    static mut DAT_006fedd0: u16;
    static mut DAT_006d4a46: u16;
    static mut unaff_EBP: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049c2b0(param_1: i32);
    fn FUN_00486310(param_1: i32);
    fn FUN_004490f0(param_1: i32) -> i32;
    fn FUN_00449790(param_1: i32, param_2: i32) -> i32;
    fn FUN_00486750(param_1: i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004010f0(param_1: u16) {
    // SAFETY: globals and callees are the game's own; pointers computed from
    // the game's table base are read and written where the original does.
    unsafe {
        let mut skip = false;
        if DAT_006da9f8 != param_1 {
            if (DAT_006da9f8 as u32).wrapping_sub(param_1 as u32) != 0xFFFF_FFFF {
                DAT_0071f714 = param_1.wrapping_add(0x7000);
                skip = true;
            } else if param_1 < DAT_006da9f8 {
                DAT_0070a1d0 = param_1;
            }
        }
        if !skip {
            let hi = ((unaff_EBP as u32) >> 0x10) as i16 as i32;
            FUN_0049c2b0(hi.wrapping_shl(16) | (param_1 as i32));
            FUN_00486310(DAT_006d4988.wrapping_add((param_1 as u32).wrapping_mul(0x23) as i32));
        }
        let mut u_var7: u16 = 0;
        DAT_006d498c = 0;
        let i_var2: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let i_var4 = FUN_004490f0(
            ((DAT_006d4988.wrapping_add(3).wrapping_add(i_var2)) as usize as *const u8).read()
                as i32,
        );
        if 0 < i_var4 {
            loop {
                DAT_006d498c = DAT_006d498c.wrapping_add(1);
                let i_var5 = FUN_00449790(
                    ((DAT_006d4988.wrapping_add(3).wrapping_add(i_var2)) as usize as *const u8)
                        .read() as i32,
                    u_var7 as i32,
                );
                if i_var5 == 1 {
                    break;
                }
                u_var7 = u_var7.wrapping_add(1);
                if !((u_var7 as i32) < i_var4) {
                    break;
                }
            }
        }
        let s_var3 = DAT_006fedd0;
        DAT_006d498c = DAT_006d498c.wrapping_sub(1);
        if param_1 < DAT_006da9f8 {
            DAT_006d4a46 = 1;
            DAT_006fedd0 = DAT_0071f714;
        }
        let u_var6 = DAT_006d498c as u32;
        let i_var4 = ((DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var2)) as usize
            as *const i32)
            .read_unaligned();
        let p88 = (i_var4.wrapping_add(0x88)) as usize as *mut u8;
        let u_var1 = p88.read();
        p88.write(1);
        FUN_00486750(DAT_006d4988.wrapping_add(i_var2), u_var6);
        let i_var4b = ((DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var2)) as usize
            as *const i32)
            .read_unaligned();
        (i_var4b.wrapping_add(0x88) as usize as *mut u8).write(u_var1);
        DAT_006fedd0 = s_var3 as u16;
    }
}
