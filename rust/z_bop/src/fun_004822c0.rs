// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1308: u32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009e9670: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00521f2e: [u32; 1];
    static mut DAT_00a08104: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(param_1: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004822c0(param_1: u16, param_2: i16, param_3: i32) -> u32 {
    let mut in_eax: u32 = 0;
    let mut u_var7: u32 = param_1 as u32;
    // SAFETY: the globals and the pointers they hold belong to the game; every
    // read happens where the original reads it.
    unsafe {
        let mut s_var4: i16 = (DAT_009d1308 as i16).wrapping_add(DAT_00a07ce8 as i16);
        loop {
            let s_var5 = s_var4.wrapping_sub(1);
            if s_var4 == 0 {
                return (in_eax & 0xFFFF0000) | 0xFFFF;
            }
            let u_var6: u16 = (u_var7 as u16).wrapping_add(param_2 as u16);
            u_var7 = u_var6 as u32;
            if u_var6 < 0x8000 {
                if DAT_009d1308.wrapping_add(DAT_00a07ce8) == u_var6 as u32 {
                    u_var7 = 0;
                }
            } else {
                u_var7 = DAT_009d1308
                    .wrapping_sub(1)
                    .wrapping_add(DAT_00a07ce8 & 0xFFFF);
            }
            let u_var2: u32 = u_var7 & 0xFFFF;
            in_eax = param_3.wrapping_mul(0x16F) as u32;
            s_var4 = s_var5;
            let p = ((&raw mut DAT_009e9670) as usize as u32)
                .wrapping_add(param_3.wrapping_mul(0x5BD) as u32) as usize
                as *const u32;
            if p.read_unaligned() == u_var2 {
                continue;
            }
            let i_var3: u32 = DAT_009a1fe8.wrapping_add(u_var2.wrapping_mul(0x23));
            let b3 = (i_var3.wrapping_add(3) as usize as *const u8).read();
            in_eax = ((((u_var2.wrapping_mul(7)) >> 8) << 8) | b3 as u32) & 0xFFFF00FF;
            if (in_eax as i16) == 0 {
                continue;
            }
            in_eax = 0;
            let tbl = (&raw mut DAT_00521f2e) as *const u32;
            if (tbl.add(b3 as usize * 0x1A).read_unaligned() & 1) == 0 {
                continue;
            }
            in_eax = 0;
            let v = (i_var3.wrapping_add(0x1F) as usize as *const i32).read_unaligned();
            if v == 0 {
                break;
            }
            let b2 = (i_var3.wrapping_add(2) as usize as *const u8).read();
            in_eax = (((v as u32) >> 8) << 8) | b2 as u32;
            if b2 == 0x0D {
                continue;
            }
            in_eax = FUN_00484f80(u_var7 as u16);
            if (in_eax as u8) != 0 {
                continue;
            }
            in_eax = (DAT_009a1fe8
                .wrapping_add(0x1F)
                .wrapping_add(u_var2.wrapping_mul(0x23)) as usize
                as *const u32)
                .read_unaligned();
            if (in_eax as usize as *const i8).read() != 0 {
                break;
            }
            DAT_00a08104 = (in_eax.wrapping_add(0xA9) as usize as *const u32).read_unaligned();
            let c_var1 = (DAT_00a08104.wrapping_add(7) as usize as *const i8).read();
            in_eax = ((DAT_00a08104 >> 8) << 8) | (c_var1 as i32 as u32);
            if c_var1 == 3 || c_var1 == 4 {
                continue;
            }
            break;
        }
    }
    (in_eax & 0xFFFF0000) | u_var7
}
