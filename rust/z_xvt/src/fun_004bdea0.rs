// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbe00(param_1: i8) -> u8;
    fn FUN_004dbe40() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bdea0() -> u32 {
    // SAFETY: the callees are the game's own key-state functions; they take
    // and return plain values.
    unsafe {
        let u_var2 = FUN_004dbe40();
        if (u_var2 as u8) != 0 {
            return u_var2;
        }
        if FUN_004dbe00(0x12) != 0 {
            let mut i_var3: i32 = 0;
            loop {
                if FUN_004dbe00((i_var3 as i8).wrapping_add(0x41)) != 0 {
                    return (i_var3 as u32).wrapping_add(0x7c);
                }
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= 0x1a {
                    break;
                }
            }
            i_var3 = 0;
            loop {
                if FUN_004dbe00((i_var3 as i8).wrapping_add(0x30)) != 0 {
                    return (i_var3 as u32).wrapping_sub(0x6a);
                }
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= 10 {
                    break;
                }
            }
            return 0;
        }
        if FUN_004dbe00(0x10) != 0 {
            let mut i_var3: i32 = 0;
            loop {
                if FUN_004dbe00((i_var3 as i8).wrapping_add(0x70)) != 0 {
                    return (i_var3 as u32).wrapping_sub(0x35);
                }
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= 0xc {
                    break;
                }
            }
            return 0;
        }
        let mut i_var3: i32 = 0;
        loop {
            if FUN_004dbe00((i_var3 as i8).wrapping_add(0x70)) != 0 {
                return (i_var3 as u32).wrapping_sub(0x41);
            }
            i_var3 = i_var3.wrapping_add(1);
            if i_var3 >= 0xc {
                break;
            }
        }
        let table: [(u8, u32); 28] = [
            (0x25, 0xa0),
            (0x27, 0xa1),
            (0x26, 0xa2),
            (0x28, 0xa3),
            (0x2d, 0xa4),
            (0x2e, 0xa5),
            (0x24, 0xa6),
            (0x23, 0xa7),
            (0x21, 0xa8),
            (0x22, 0xa9),
            (0x2c, 0xaa),
            (0x91, 0xab),
            (0x14, 0xad),
            (0x60, 0xae),
            (0x61, 0xaf),
            (0x62, 0xb0),
            (99, 0xb1),
            (100, 0xb2),
            (0x65, 0xb3),
            (0x66, 0xb4),
            (0x67, 0xb5),
            (0x68, 0xb6),
            (0x69, 0xb7),
            (0x90, 0xb8),
            (0x6a, 0xba),
            (0x6b, 0xbc),
            (0x6d, 0xbb),
            (0x6e, 0xbe),
        ];
        let mut idx = 0;
        while idx < 28 {
            if FUN_004dbe00(table[idx].0 as i8) != 0 {
                return table[idx].1;
            }
            idx += 1;
        }
        if FUN_004dbe00(0x6f) != 0 {
            return 0xb9;
        }
        0
    }
}
