// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d6959: i32;
    static mut DAT_009e967c: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d8c88: i32;
    static mut DAT_009fd458: i32;
    static mut DAT_009e968a: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(param_1: u16) -> u32;
    fn FUN_00482eb0(param_1: u16, param_2: u16, param_3: u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00436bc0(param_1: u32) {
    // SAFETY: all addresses are the game's own memory, read where the original reads.
    unsafe {
        let base = DAT_009a1fe8 as u32;
        let i_var1: u32 = (param_1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(base) as usize as *const u32)
            .read_unaligned();
        let sel = ((i_var1.wrapping_add(0x88)) as usize as *const u8).read();
        let b_var3: u8 = match sel {
            0 => 0x3f,
            1 | 4 => 0x37,
            2 => 0x33,
            _ => 0x3b,
        };

        let idx = DAT_009ecc34 as u32;
        if DAT_009d6959 == 0 {
            let u_var2: u32 = u32::from(
                ((&raw mut DAT_009e967c as usize as u32).wrapping_add(idx.wrapping_mul(0x5bd))
                    as usize as *const u16)
                    .read_unaligned(),
            );
            let p = (i_var1.wrapping_add(0xa9) as usize as *const u32).read_unaligned();
            let flag = (p.wrapping_add(0x1f6).wrapping_add(u_var2) as usize as *const u8).read();
            if flag == 0 {
                let b = ((param_1 & 0xffff)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x16)
                    .wrapping_add(base) as usize as *const u8)
                    .read();
                let e = (&raw mut DAT_009d8c88 as usize as u32)
                    .wrapping_add(u32::from(b).wrapping_mul(0x566).wrapping_mul(4));
                let v = u32::from((e as usize as *const i32).read_unaligned() as u8);
                let b_var4 = if v == u_var2 {
                    false
                } else {
                    let i2 = v.wrapping_add(u_var2.wrapping_mul(0x1e5));
                    let a =
                        (&raw mut DAT_009fd458 as usize as u32).wrapping_add(i2.wrapping_mul(4));
                    (a as usize as *const i32).read_unaligned() == 0
                };
                if b_var4 {
                    return;
                }
            }
        }

        let u_var2 = FUN_00484f80(param_1 as u16);
        if (u_var2 as u8) == 0 {
            let w = u32::from(
                ((&raw mut DAT_009e968a as usize as u32).wrapping_add(idx.wrapping_mul(0x5bd))
                    as usize as *const u16)
                    .read_unaligned(),
            );
            if w != param_1 {
                FUN_00482eb0(param_1 as u16, 0xffff, b_var3);
            }
        }
    }
}
