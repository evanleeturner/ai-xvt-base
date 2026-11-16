// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4974: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_0070a180: i32;
    static mut DAT_0070acd8: i32;
    static mut DAT_00703ad8: i32;
    static mut DAT_0071e6c4: i32;
    static mut DAT_00732f38: i32;
    static mut DAT_00732fe8: i32;
    static mut DAT_0073294c: i32;
    static mut DAT_007329b0: i32;
    static mut DAT_007329b4: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_0052686c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> u32;
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_004127d0(param_1: u32, param_2: u32, param_3: u32) -> u32;
    fn FUN_0040fb70(param_1: i32) -> u32;
    fn FUN_00419190(param_1: u16, param_2: u16) -> u32;
}

#[inline(always)]
fn absu(x: u32) -> u32 {
    if (x as i32) < 0 {
        0u32.wrapping_sub(x)
    } else {
        x
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413010(param_1: u16, param_2: u16) -> u32 {
    // SAFETY: the globals and callees are the game's own memory; all pointer
    // reads are at the offsets the original uses.
    unsafe {
        let mut u_var6: u32 = param_2 as u32;
        let base = DAT_006d4988 as u32;
        let p_tmp = (((param_1 as u32).wrapping_mul(0x23).wrapping_add(0x1f)).wrapping_add(base)
            as usize as *const i32)
            .read_unaligned();
        let s_tmp = ((p_tmp as u32).wrapping_add(0x83) as usize as *const u16).read_unaligned();
        let mut u_var5: u32 = s_tmp as u32;
        if u_var6 <= u_var5 {
            return (DAT_006d4988 as u32) & 0xffff0000;
        }
        let i_var3: i32 = u_var6.wrapping_mul(0x23) as i32;
        let c_var1 = ((DAT_006d4988 as u32)
            .wrapping_add(2)
            .wrapping_add(i_var3 as u32) as usize as *const i8)
            .read();
        if c_var1 == 0x0e {
            return 0;
        }
        if c_var1 == 0x0b {
            return 0;
        }
        if c_var1 != 0x0a && (u_var5 as i32) < DAT_006d4980 {
            let i_var4 = u_var5.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32);
            let i_tmp = (i_var4.wrapping_add(0x1b) as usize as *const i32).read_unaligned();
            if i_tmp == -1 {
                let p2 = (i_var4.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
                let i_tmp2 =
                    ((p2 as u32).wrapping_add(0xa7) as usize as *const i32).read_unaligned();
                u_var5 = FUN_0042b150(i_tmp2);
                let v = (u_var5.wrapping_add(0x18) as usize as *const u16).read_unaligned();
                if v != param_2 {
                    return u_var5 & 0xffff0000;
                }
            }
        }
        let b_var2 = ((DAT_006d4988 as u32)
            .wrapping_add(3)
            .wrapping_add(i_var3 as u32) as usize as *const u8)
            .read();
        FUN_00421c70(u_var6, 0);
        DAT_0070a180 = DAT_00732a2c;
        DAT_0073294c = DAT_00732a2c;
        DAT_0070acd8 = DAT_00732a28;
        DAT_007329b0 = DAT_00732a28;
        DAT_00703ad8 = DAT_00732a24;
        DAT_007329b4 = DAT_00732a24;
        u_var6 = (DAT_006d4974 as u32).wrapping_sub(DAT_00732a2c as u32);
        u_var5 = ((&raw mut DAT_0052686c)
            .cast::<u8>()
            .byte_add((b_var2 as usize).wrapping_mul(0x16))
            .cast::<u16>()
            .read_unaligned()) as u32;
        u_var6 = absu(u_var6);
        if 0x20000 < (u_var6 as i32) {
            return u_var6 & 0xffff0000;
        }
        let mut u_var8 = (DAT_006d4978 as u32).wrapping_sub(DAT_00732a28 as u32);
        u_var8 = absu(u_var8);
        let u_var7 = (u_var6 >> 16) as u16;
        if 0x20000 < (u_var8 as i32) {
            return (u_var7 as u32) << 16;
        }
        let mut u_var10 = (DAT_006d4970 as u32).wrapping_sub(DAT_00732a24 as u32);
        u_var10 = absu(u_var10);
        if 0x20000 < (u_var10 as i32) {
            return (u_var7 as u32) << 16;
        }
        u_var6 = FUN_004127d0(u_var6, u_var8, u_var10);
        if 0x20000 < (u_var6 as i32) {
            return u_var6 & 0xffff0000;
        }
        let u_var9 = absu((DAT_006d4974 as u32).wrapping_sub(DAT_0071e6c4 as u32));
        let u_var11 = absu((DAT_006d4978 as u32).wrapping_sub(DAT_00732f38 as u32));
        let u_var12 = absu((DAT_006d4970 as u32).wrapping_sub(DAT_00732fe8 as u32));
        let u_var13 = absu((DAT_0073294c as u32).wrapping_sub(DAT_0070a180 as u32));
        let u_var14 = absu((DAT_007329b0 as u32).wrapping_sub(DAT_0070acd8 as u32));
        let u_tmp = absu((DAT_007329b4 as u32).wrapping_sub(DAT_00703ad8 as u32));
        u_var8 = FUN_004127d0(
            u_var9.wrapping_add(u_var13).wrapping_add(u_var5),
            u_var11.wrapping_add(u_var14).wrapping_add(u_var5),
            u_var12.wrapping_add(u_tmp).wrapping_add(u_var5),
        );
        if u_var8 < u_var6 {
            return u_var8 & 0xffff0000;
        }
        let c3 = ((DAT_006d4988 as u32)
            .wrapping_add(3)
            .wrapping_add(i_var3 as u32) as usize as *const i8)
            .read();
        if u_var5 < 0x447 && c3 != b':' as i8 {
            let a = (u_var5 as i32) >> 3;
            let b = (u_var5 as i32) >> 2;
            return FUN_0040fb70(a.wrapping_add(b));
        }
        FUN_00419190(param_1, param_2)
    }
}
