// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d499c: i32;
    static mut DAT_006d9d50: u16;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_00421c70(param_1: u32, param_2: i32);
}

#[inline(always)]
unsafe fn rd_i32(addr: u32) -> i32 {
    // SAFETY: the address is a game-owned structure address, read as the original does.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_u16(addr: u32) -> u16 {
    // SAFETY: the address is a game-owned structure address, read as the original does.
    unsafe { (addr as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_u8(addr: u32) -> u8 {
    // SAFETY: the address is a game-owned structure address, read as the original does.
    unsafe { (addr as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429a20(param_1: u32) -> u8 {
    if param_1 == 0xffff {
        return 0;
    }
    // SAFETY: globals and callees are the game's own; all pointers are the
    // game's entry tables, accessed at the same offsets as the original.
    unsafe {
        let entry: u32 = param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32);
        let i_entry1f: i32 = rd_i32(entry.wrapping_add(0x1f));
        if i_entry1f == 0 {
            return if rd_u8(entry.wrapping_add(3)) == 0 {
                0
            } else {
                1
            };
        }
        if rd_u8(entry.wrapping_add(3)) == 0 {
            return 0;
        }
        let i_var3: i32 = rd_i32((i_entry1f as u32).wrapping_add(0xa7));
        if i_var3 == 0 {
            return 1;
        }
        let v3 = i_var3 as u32;
        let i_var2 = FUN_0042b150(i_var3);
        if rd_u8((i_var2 as u32).wrapping_add(0x34)) == 0x15
            && rd_u8((i_var2 as u32).wrapping_add(0x35)) != 0
        {
            return 0;
        }
        let c_var1 = rd_u8(v3.wrapping_add(7));
        if c_var1 == 1 || c_var1 == 3 || c_var1 == 4 {
            return 0;
        }
        if rd_u8(v3.wrapping_add(0x233)) == 3 && rd_u8(v3.wrapping_add(0x237)) != 0 {
            let local_8: u32 = DAT_006d9d50 as u32;
            FUN_00421c70(param_1, 0);
            let mut i_var5 = DAT_00732a2c;
            let mut i_var4 = DAT_00732a28;
            let mut i_var2 = DAT_00732a24;
            FUN_00421c70(local_8, 0);
            i_var5 = i_var5.wrapping_sub(DAT_00732a2c);
            i_var4 = i_var4.wrapping_sub(DAT_00732a28);
            i_var2 = i_var2.wrapping_sub(DAT_00732a24);
            if i_var5 < 0 {
                i_var5 = i_var5.wrapping_neg();
            }
            if i_var4 < 0 {
                i_var4 = i_var4.wrapping_neg();
            }
            if i_var2 < 0 {
                i_var2 = i_var2.wrapping_neg();
            }
            if i_var4 < i_var5 {
                DAT_006d499c = (i_var4 >> 1).wrapping_add(i_var5);
            } else {
                DAT_006d499c = (i_var5 >> 1).wrapping_add(i_var4);
            }
            if i_var2 < DAT_006d499c {
                i_var2 >>= 1;
            } else {
                DAT_006d499c >>= 1;
            }
            DAT_006d499c = DAT_006d499c.wrapping_add(i_var2);
            let c = rd_u8(
                (DAT_006d9d50 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(2)
                    .wrapping_add(DAT_006d4988 as u32),
            );
            if c == 0 || c == 1 || c == 2 {
                if 0x8000 < DAT_006d499c {
                    return 0;
                }
            } else if 0x10000 < DAT_006d499c {
                return 0;
            }
        }
        let i_var2: u32 = (DAT_006d4988 as u32).wrapping_add(param_1.wrapping_mul(0x23));
        let mut i_tmp = rd_i32(i_var2.wrapping_add(0x1b));
        if i_tmp != -1 {
            return 1;
        }
        i_tmp = rd_i32(i_var2.wrapping_add(0x1f));
        i_tmp = rd_i32((i_tmp as u32).wrapping_add(0xa7));
        let us_a = rd_u16((i_tmp as u32).wrapping_add(199));
        let us_b = rd_u16(v3.wrapping_add(0xc3));
        if us_a < us_b { 0 } else { 1 }
    }
}
